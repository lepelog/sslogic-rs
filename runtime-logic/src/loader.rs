use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    fs::File,
    io::BufReader,
};

use anyhow::{bail, Context};
use indexmap::IndexMap;
use regex::Regex;
use serde::Deserialize;

use crate::{
    requirements::{RequirementExpression, Requirements},
    structure::{
        self, Area, AreaId, ConnectionShuffleType, ContextLoadable, DoorConnection, DoubleDoor,
        Entrance, EntranceId, EntrancePatchInfo, Event, EventId, Exit, ExitId, ExitPatchInfo, Item,
        ItemId, Location, LocationId, LocationKind, LogicContext, Region, RegionId, RequirementKey,
        Stage, StageId,
    },
};

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum TimeOfDay {
    Day,
    Night,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AreaYaml {
    force_tod: Option<TimeOfDay>,
    #[serde(default)]
    locations: BTreeMap<String, String>,
    #[serde(default)]
    can_sleep: bool,
    #[serde(default)]
    events: BTreeMap<String, String>,
    #[serde(default)]
    map_exits: BTreeMap<String, String>,
    #[serde(default)]
    logic_exits: BTreeMap<String, String>,
    #[serde(default)]
    macros: IndexMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StageYaml {
    force_tod: Option<TimeOfDay>,
    // stage: String,
    #[serde(default)]
    areas: BTreeMap<String, AreaYaml>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RegionYaml {
    force_tod: Option<TimeOfDay>,
    #[serde(default)]
    stages: BTreeMap<String, StageYaml>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ItemYaml {
    id: u16,
    name: String,
    oarc: Vec<String>,
    getarcname: String,
    getmodelname: String,
    //   type: Single
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConnectionYaml {
    stage: String,
    to_stage: String,
    disambiguation: Option<String>,
    door: Option<DoubleDoor>,
    orig: EntranceYaml,
    scens: Vec<ExitYaml>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntranceYaml {
    stage: String,
    room: u8,
    layer: u8,
    entrance: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ExitYaml {
    stage: String,
    room: u8,
    index: u8,
}

#[derive(Debug, Deserialize)]
pub struct CheckYaml {
    #[serde(rename = "original item")]
    orig_item: String,
    #[serde(rename = "Paths")]
    #[serde(default)]
    paths: Vec<String>,
    #[serde(rename = "type")]
    types: String,
}

fn read_logic_into(name: &str, map: &mut BTreeMap<String, RegionYaml>) -> anyhow::Result<()> {
    let reader = BufReader::new(
        File::open(format!("../bitless/{name}.yaml")).with_context(|| name.to_string())?,
    );
    let region: BTreeMap<String, RegionYaml> = serde_yaml::from_reader(reader)?;
    map.extend(region);
    Ok(())
}

fn parse_macro_file() -> anyhow::Result<IndexMap<String, String>> {
    let reader = BufReader::new(File::open(format!("../bitless/macros.yaml")).context("macros")?);
    let raw_macros: IndexMap<String, String> = serde_yaml::from_reader(reader)?;

    Ok(raw_macros)
}

pub fn read_item_file() -> anyhow::Result<HashMap<ItemId, Item>> {
    let reader =
        BufReader::new(File::open(format!("../logic-generator/items.yaml")).context("items")?);
    let items_yaml: Vec<ItemYaml> = serde_yaml::from_reader(reader)?;

    let items = items_yaml
        .into_iter()
        .map(|item_yaml| {
            let item_id = ItemId(item_yaml.id);
            let item = Item {
                id: item_id,
                name: item_yaml.name,
            };
            (item_id, item)
        })
        .collect();

    Ok(items)
}

pub fn read_entrance_table() -> anyhow::Result<Vec<ConnectionYaml>> {
    let reader =
        BufReader::new(File::open(format!("../entrance_table2.yaml")).context("entrance_table")?);
    let connections: Vec<ConnectionYaml> = serde_yaml::from_reader(reader)?;

    Ok(connections)
}

pub fn read_checks() -> anyhow::Result<IndexMap<String, CheckYaml>> {
    let reader = BufReader::new(File::open(format!("../checks.yaml")).context("checks")?);
    let result: IndexMap<String, CheckYaml> = serde_yaml::from_reader(reader)?;

    Ok(result)
}

pub fn load_logic() -> anyhow::Result<(LogicContext, Requirements<'static>)> {
    let macros = parse_macro_file()?;
    let items = read_item_file()?;
    let checks = read_checks()?;

    let stage_exit_re = Regex::new(r#"([^-]+) - ([^(-]+)( \(([^-]+)\))?$"#)?;
    let mut regions: BTreeMap<String, RegionYaml> = BTreeMap::new();

    read_logic_into("Ancient Cistern", &mut regions)?;
    read_logic_into("Earth Temple", &mut regions)?;
    read_logic_into("Eldin", &mut regions)?;
    read_logic_into("Faron", &mut regions)?;
    read_logic_into("Fire Sanctuary", &mut regions)?;
    read_logic_into("Lanayru Mining Facility", &mut regions)?;
    read_logic_into("Lanayru", &mut regions)?;
    read_logic_into("Sandship", &mut regions)?;
    read_logic_into("Sky Keep", &mut regions)?;
    read_logic_into("Sky", &mut regions)?;
    read_logic_into("Skyloft", &mut regions)?;
    read_logic_into("Skyview", &mut regions)?;

    let entrance_table = read_entrance_table()?;

    let item_names: HashMap<&str, ItemId> = items
        .iter()
        .map(|(id, item)| (item.name.as_str(), *id))
        .collect();

    let mut new_regions = Vec::new();
    let mut stages: Vec<Stage> = Vec::new();
    let mut areas = Vec::new();
    let mut locations = Vec::new();

    let mut unparsed_requirements: Vec<(RequirementKey, &str)> = Vec::new();
    let mut unparsed_events: HashMap<&str, Vec<(AreaId, &str)>> = HashMap::new();
    let mut area_rev_lookup: HashMap<(&str, &str), AreaId> = HashMap::new();
    let mut area_macros: HashMap<AreaId, &IndexMap<String, String>> = HashMap::new();

    for (region_name, region) in regions.iter() {
        let region_id = RegionId(new_regions.len() as u16);
        let mut region_areas = Vec::new();

        for (stage_name, stage) in region.stages.iter() {
            let (stage_id, stage_areas) =
                if let Some(existing_stage) = stages.iter_mut().find(|s| &s.name == stage_name) {
                    (existing_stage.id, &mut existing_stage.areas)
                } else {
                    let stage_id = StageId(stages.len() as u16);
                    stages.push(Stage {
                        areas: Vec::new(),
                        id: stage_id,
                        name: stage_name.clone(),
                    });
                    (stage_id, &mut stages.last_mut().unwrap().areas)
                };

            for (area_name, area) in stage.areas.iter() {
                let area_id = AreaId(areas.len() as u16);
                let mut area_locations = Vec::new();

                stage_areas.push(area_id);
                region_areas.push(area_id);

                area_rev_lookup.insert((&stage_name, &area_name), area_id);

                for (location_name, requirement) in area.locations.iter() {
                    let location_id = LocationId(locations.len() as u16);

                    area_locations.push(location_id);
                    unparsed_requirements
                        .push((RequirementKey::Location(location_id), &requirement));

                    let display_name = format!("{region_name} - {location_name}");

                    let kind = if let Some(check) = checks.get(&display_name) {
                        LocationKind::Check {
                            vanilla_item: *item_names
                                .get(check.orig_item.as_str())
                                .with_context(|| check.orig_item.clone())?,
                            patches: Vec::new(),
                        }
                    } else if display_name.contains("Gossip Stone") {
                        LocationKind::GossipStone {
                            text_path: "TODO".to_string(),
                        }
                    } else {
                        bail!("location: {display_name}")
                    };

                    locations.push(Location {
                        area: area_id,
                        display_name,
                        name: location_name.clone(),
                        id: location_id,
                        kind,
                    });
                }

                for (event_name, requirement) in area.events.iter() {
                    unparsed_events
                        .entry(&event_name)
                        .or_default()
                        .push((area_id, &requirement));
                }

                if !area.macros.is_empty() {
                    area_macros.insert(area_id, &area.macros);
                }

                areas.push(Area {
                    id: area_id,
                    can_sleep: area.can_sleep,
                    name: area_name.clone(),
                    time_of_day: area
                        .force_tod
                        .or(stage.force_tod)
                        .or(region.force_tod)
                        .map_or(structure::TimeOfDay::Both, |tod| {
                            if tod == TimeOfDay::Day {
                                structure::TimeOfDay::Day
                            } else {
                                structure::TimeOfDay::Night
                            }
                        }),
                    locations: area_locations,
                    logic_entrances: Vec::new(),
                    logic_exits: Vec::new(),
                    map_entrances: Vec::new(),
                    map_exits: Vec::new(),
                    region: region_id,
                    stage: stage_id,
                });
            }
        }

        new_regions.push(Region {
            areas: region_areas,
            id: region_id,
            name: region_name.clone(),
        });
    }

    let mut exits = Vec::new();
    let mut entrances: Vec<Entrance> = Vec::new();

    for (region_name, region) in regions.iter() {
        for (stage_name, stage) in region.stages.iter() {
            for (area_name, area) in stage.areas.iter() {
                let area_id = area_rev_lookup[&(stage_name.as_str(), area_name.as_str())];

                for (exit_area, requirement) in area.logic_exits.iter() {
                    let other_area_id = area_rev_lookup[&(stage_name.as_str(), exit_area.as_str())];
                    unparsed_requirements.push((
                        RequirementKey::LogicExit {
                            from: area_id,
                            to: other_area_id,
                        },
                        &requirement,
                    ));
                    areas[area_id.0 as usize].logic_exits.push(other_area_id);
                    areas[other_area_id.0 as usize]
                        .logic_entrances
                        .push(area_id);
                }

                for (exit_name, requirement) in area.map_exits.iter() {
                    if let Some(captures) = stage_exit_re.captures(&exit_name) {
                        let other_stagename = &captures[1];
                        let other_areaname = &captures[2];
                        let disambiguation = captures.get(4).map(|c| c.as_str());
                        let other_area_id = area_rev_lookup[&(other_stagename, other_areaname)];

                        let mut has_no_connection_def = true;

                        let mut create_connection = |connection_def: Option<&ConnectionYaml>| {
                            let double_door = connection_def
                                .and_then(|c| c.door)
                                .unwrap_or(DoubleDoor::No);

                            let format_disambiguation = match (disambiguation, double_door) {
                                (None, DoubleDoor::No) => format!(""),
                                (None, _) => format!(" ({double_door:?} Door)"),
                                (Some(disambiguation), DoubleDoor::No) => {
                                    format!(" ({disambiguation})")
                                }
                                (Some(disambiguation), _) => {
                                    format!(" ({disambiguation} {double_door:?} Door)")
                                }
                            };

                            let entrance_id = EntranceId(entrances.len() as u16);

                            let connection_shuffle_type = if connection_def.is_some() {
                                ConnectionShuffleType::Other
                            } else {
                                ConnectionShuffleType::Never
                            };

                            entrances.push(Entrance {
                                id: entrance_id,
                                area: other_area_id,
                                from: area_id,
                                disambiguation: disambiguation.map(|s| s.to_string()),
                                display_name: format!(
                                    "{other_stagename} from {stage_name}{}",
                                    format_disambiguation
                                ),
                                door_connection: DoorConnection::No, // maybe filled later
                                connection_shuffle_type,
                                patch_info: connection_def.map(|info| EntrancePatchInfo {
                                    entrance_id: info.orig.entrance,
                                    layer: info.orig.layer,
                                    room: info.orig.room,
                                    stage_name: info.orig.stage.clone(),
                                }),
                            });

                            let exit_id = ExitId(exits.len() as u16);

                            unparsed_requirements
                                .push((RequirementKey::Exit(exit_id), &requirement));

                            exits.push(Exit {
                                id: exit_id,
                                area: area_id,
                                disambiguation: disambiguation.map(|s| s.to_string()),
                                display_name: format!(
                                    "{stage_name} to {other_stagename}{format_disambiguation}"
                                ),
                                to: other_area_id,
                                coupled_entrance: None, // to be filled later
                                door_connection: DoorConnection::No, // maybe filled later
                                connection_shuffle_type,
                                vanilla_entrance: Some(entrance_id),
                                patch_info: connection_def
                                    .map(|info| {
                                        info.scens
                                            .iter()
                                            .map(|scen| ExitPatchInfo {
                                                exit_idx: scen.index,
                                                room: scen.room,
                                                stage_name: scen.stage.clone(),
                                            })
                                            .collect()
                                    })
                                    .unwrap_or_default(),
                            });

                            areas[area_id.0 as usize].map_exits.push(exit_id);
                            areas[other_area_id.0 as usize]
                                .map_entrances
                                .push(entrance_id);
                        };

                        let mut connection_defs: Vec<_> = entrance_table
                            .iter()
                            .filter(|entr| {
                                &entr.stage == stage_name
                                    && &entr.to_stage == other_stagename
                                    && disambiguation
                                        == entr.disambiguation.as_ref().map(|x| x.as_str())
                            })
                            .collect();

                        if connection_defs.len() == 2 {
                            // right and left
                            assert!(
                                connection_defs
                                    .iter()
                                    .filter(|c| c.door == Some(DoubleDoor::Left))
                                    .count()
                                    == 1
                            );
                            assert!(
                                connection_defs
                                    .iter()
                                    .filter(|c| c.door == Some(DoubleDoor::Right))
                                    .count()
                                    == 1
                            );
                            // first create right, then left
                            if connection_defs[0].door != Some(DoubleDoor::Right) {
                                connection_defs.swap(0, 1);
                            }
                            assert!(connection_defs[0].door == Some(DoubleDoor::Right));
                            create_connection(Some(connection_defs[0]));
                            create_connection(Some(connection_defs[1]));
                            let left_entrance_no = entrances.len() - 1;
                            let right_entrance_no = entrances.len() - 2;
                            entrances[left_entrance_no].door_connection =
                                DoorConnection::Right(EntranceId(right_entrance_no as u16));
                            entrances[right_entrance_no].door_connection =
                                DoorConnection::Left(EntranceId(left_entrance_no as u16));
                            let left_exit_no = exits.len() - 1;
                            let right_exit_no = exits.len() - 2;
                            exits[left_exit_no].door_connection =
                                DoorConnection::Right(ExitId(right_exit_no as u16));
                            exits[right_exit_no].door_connection =
                                DoorConnection::Left(ExitId(left_exit_no as u16));
                        } else if connection_defs.len() == 1 {
                            assert!(connection_defs[0].door == None);
                            create_connection(Some(connection_defs[0]));
                        } else if connection_defs.len() == 0 {
                            // this is kinda dumb but we don't mention connections that should always be vanilla in the entrance file
                            // so to still create entries for them, we have to do it separately

                            // disambiguation doesn't make sense if the connection has no definition, that's a bug
                            // if disambiguation.is_some() {
                            //     bail!("invalid disambiguation on {exit_name}");
                            // }

                            create_connection(None);
                        }
                    } else {
                        panic!("invalid stage exit: {exit_name}");
                    }
                }
            }
        }
    }

    fn opposite(dd: DoubleDoor) -> DoubleDoor {
        match dd {
            DoubleDoor::Left => DoubleDoor::Right,
            DoubleDoor::No => DoubleDoor::No,
            DoubleDoor::Right => DoubleDoor::Left,
        }
    }

    // couple entrances
    for exit in exits.iter_mut() {
        if let Some(entrance) = entrances.iter().find(|entrance| {
            entrance.area == exit.area
                && entrance.from == exit.to
                && entrance.disambiguation == exit.disambiguation
                && entrance.door_connection.is_opposite(exit.door_connection)
        }) {
            exit.coupled_entrance = Some(entrance.id);
        }
    }

    let events: Vec<Event> = unparsed_events
        .keys()
        .enumerate()
        .map(|(id, event)| Event {
            id: EventId(id as u16),
            name: event.to_string(),
        })
        .collect();

    let event_names: HashMap<&str, EventId> = events
        .iter()
        .map(|event| (event.name.as_str(), event.id))
        .collect();

    // parse requirements
    // parse macros first

    let mut parsed_macros: HashMap<&str, RequirementExpression<'static>> = HashMap::new();
    for (name, req_string) in &macros {
        let req = RequirementExpression::parse(
            &req_string,
            None,
            &[&parsed_macros],
            &item_names,
            &event_names,
        )?;
        parsed_macros.insert(&name, req);
    }

    // parse local area macros
    let parsed_area_macros: HashMap<AreaId, HashMap<&str, RequirementExpression<'static>>> =
        area_macros
            .iter()
            .map(|(area, macros)| {
                let mut for_area = HashMap::new();
                for (name, req_string) in macros.iter() {
                    let req = RequirementExpression::parse(
                        &req_string,
                        None,
                        &[&parsed_macros, &for_area],
                        &item_names,
                        &event_names,
                    )
                    .unwrap();
                    for_area.insert(&name, req);
                }
                (*area, for_area)
            })
            .collect();

    let mut requirements: HashMap<RequirementKey, RequirementExpression<'static>> = HashMap::new();

    let empty_macros = HashMap::new();
    for (req_key, req_string) in unparsed_requirements.iter() {
        let area_id = match req_key {
            RequirementKey::Exit(exit_id) => exits[exit_id.0 as usize].area,
            RequirementKey::LogicExit { from, .. } => *from,
            RequirementKey::Location(location_id) => locations[location_id.0 as usize].area,
            RequirementKey::Event(_) => unreachable!(),
        };

        let req = RequirementExpression::parse(
            &req_string,
            Some(area_id),
            &[
                &parsed_macros,
                parsed_area_macros.get(&area_id).unwrap_or(&empty_macros),
            ],
            &item_names,
            &event_names,
        )?;
        requirements.insert(*req_key, req);
    }

    for (event, raw_reqs) in unparsed_events.iter() {
        let event_id = event_names[event];
        let mut reqs = Vec::new();
        for (area_id, req_string) in raw_reqs {
            let req = RequirementExpression::parse(
                &req_string,
                Some(*area_id),
                &[
                    &parsed_macros,
                    parsed_area_macros.get(area_id).unwrap_or(&empty_macros),
                ],
                &item_names,
                &event_names,
            )?;
            reqs.push(req);
        }
        requirements.insert(
            RequirementKey::Event(event_id),
            RequirementExpression::Or(reqs),
        );
    }

    let ctx = LogicContext {
        areas,
        exits,
        entrance: entrances,
        events,
        items,
        locations,
        regions: new_regions,
        stages,
    };

    for i in &ctx.areas {
        assert!(i.id == i.id.ctx(&ctx).id);
    }

    for i in &ctx.exits {
        assert!(i.id == i.id.ctx(&ctx).id);
    }

    for i in &ctx.entrance {
        assert!(i.id == i.id.ctx(&ctx).id);
    }

    for i in &ctx.events {
        assert!(i.id == i.id.ctx(&ctx).id);
    }

    for (i, item) in &ctx.items {
        assert!(i == &item.id);
    }

    for i in &ctx.locations {
        assert!(i.id == i.id.ctx(&ctx).id);
    }

    for i in &ctx.regions {
        assert!(i.id == i.id.ctx(&ctx).id);
    }

    for i in &ctx.stages {
        assert!(i.id == i.id.ctx(&ctx).id);
    }

    let reqs = Requirements::new_from_map(requirements);

    Ok((ctx, reqs))
}
