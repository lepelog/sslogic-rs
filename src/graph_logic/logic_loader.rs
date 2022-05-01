use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::path::PathBuf;

use serde::Deserialize;
use serde_yaml::{Mapping, Value};

use crate::options::RandomizerOptions;

use super::logic_structs::{AreaKey, CheckKey};
use super::{
    logic_expression::LogicElement,
    logic_structs::{AllowedToD, Area, LogicKey, LogicKeyMapper, PassagewayKey},
};

fn parse_force_tod(mapping: &Mapping) -> Result<Option<AllowedToD>, String> {
    Ok(
        match mapping
            .get(&Value::String("force-tod".into()))
            .and_then(|v| v.as_str())
        {
            None => None,
            Some("Day") => Some(AllowedToD::Day),
            Some("Night") => Some(AllowedToD::Night),
            Some(s) => return Err(format!("invalid forced ToD: {}", s).into()),
        },
    )
}

pub fn do_parse(
    logic_key_mapper: &mut LogicKeyMapper,
) -> Result<(HashMap<AreaKey, Area>, HashMap<CheckKey, LogicKey>), Box<dyn std::error::Error>> {
    let mut paths: Vec<PathBuf> = read_dir("bitless")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<_, std::io::Error>>()?;
    // need to make sure to not introduce randomness here
    paths.sort();
    println!("{:?}", paths);
    let macro_file = paths
        .iter()
        .filter(|p| p.ends_with("macros.yaml"))
        .next()
        .unwrap();
    let f = File::open(macro_file)?;
    let mapping: Mapping = serde_yaml::from_reader(f)?;
    let macros = parse_macro_file(&mapping, logic_key_mapper)?;
    let mut areas = HashMap::new();
    let mut placement = HashMap::new();
    for entry in paths.iter() {
        if entry.ends_with("macros.yaml") || entry.extension().filter(|e| e == &"yaml").is_none() {
            continue;
        }
        let f = File::open(entry)?;
        let mapping: Mapping = serde_yaml::from_reader(f)?;
        parse_file(
            &macros,
            &mapping,
            logic_key_mapper,
            &mut areas,
            &mut placement,
        )?;
    }
    Ok((areas, placement))
}

pub fn parse_macro_file(
    mapping: &Mapping,
    logic_key_mapper: &mut LogicKeyMapper,
) -> Result<HashMap<String, LogicElement>, Box<dyn std::error::Error>> {
    let mut result = HashMap::new();
    for (raw_macro_name, raw_expr) in mapping {
        let macro_name = raw_macro_name.as_str().unwrap().to_string();
        let expr = LogicElement::parse(raw_expr.as_str().unwrap(), &result, logic_key_mapper)?;
        result.insert(macro_name, expr);
    }
    Ok(result)
}

pub fn parse_file(
    global_macros: &HashMap<String, LogicElement>,
    mapping: &Mapping,
    logic_key_mapper: &mut LogicKeyMapper,
    areas: &mut HashMap<AreaKey, Area>,
    placement: &mut HashMap<CheckKey, LogicKey>,
) -> Result<(), Box<dyn std::error::Error>> {
    for (raw_region_name, raw_region) in mapping {
        let region_name = raw_region_name.as_str().unwrap();
        let region_mapping = raw_region.as_mapping().unwrap();
        parse_region(
            region_name,
            global_macros,
            region_mapping,
            logic_key_mapper,
            areas,
            placement,
        )?;
    }
    Ok(())
}

pub fn parse_region(
    region_name: &str,
    global_macros: &HashMap<String, LogicElement>,
    mapping: &Mapping,
    logic_key_mapper: &mut LogicKeyMapper,
    areas: &mut HashMap<AreaKey, Area>,
    placement: &mut HashMap<CheckKey, LogicKey>,
) -> Result<(), Box<dyn std::error::Error>> {
    let local_allowed_tod = parse_force_tod(mapping)?;
    let raw_stages = mapping
        .get(&Value::String("stages".to_string()))
        .and_then(|v| v.as_mapping())
        .unwrap();
    for (raw_stage_name, raw_stage) in raw_stages {
        let stage_name = raw_stage_name.as_str().unwrap();
        let stage_mapping = raw_stage.as_mapping().unwrap();
        parse_stage(
            region_name,
            stage_name,
            &local_allowed_tod,
            global_macros,
            stage_mapping,
            logic_key_mapper,
            areas,
            placement,
        )?;
    }
    Ok(())
}

pub fn parse_stage(
    region_name: &str,
    stage_name: &str,
    force_tod: &Option<AllowedToD>,
    global_macros: &HashMap<String, LogicElement>,
    mapping: &Mapping,
    logic_key_mapper: &mut LogicKeyMapper,
    areas: &mut HashMap<AreaKey, Area>,
    placement: &mut HashMap<CheckKey, LogicKey>,
) -> Result<(), Box<dyn std::error::Error>> {
    let local_allowed_tod = parse_force_tod(mapping)?.or_else(|| force_tod.clone());
    let raw_areas = mapping
        .get(&Value::String("areas".to_string()))
        .and_then(|v| v.as_mapping())
        .unwrap();
    for (raw_area_name, raw_area) in raw_areas {
        let area_name = raw_area_name.as_str().unwrap();
        let area_mapping = raw_area.as_mapping().unwrap();
        parse_area(
            region_name,
            stage_name,
            area_name,
            &local_allowed_tod,
            global_macros,
            area_mapping,
            logic_key_mapper,
            areas,
            placement,
        )?;
    }
    Ok(())
}

pub fn parse_area(
    region_name: &str,
    stage_name: &str,
    area_name: &str,
    force_tod: &Option<AllowedToD>,
    global_macros: &HashMap<String, LogicElement>,
    mapping: &Mapping,
    logic_key_mapper: &mut LogicKeyMapper,
    areas: &mut HashMap<AreaKey, Area>,
    placement: &mut HashMap<CheckKey, LogicKey>,
) -> Result<(), Box<dyn std::error::Error>> {
    let self_stage_key = logic_key_mapper.convert_to_num(stage_name);
    let self_area_part_key = logic_key_mapper.convert_to_num(area_name);
    let self_area_key = self_stage_key.stage_with_area(&self_area_part_key);
    let mut local_macros = HashMap::new();
    if let Some(raw_macros) = mapping
        .get(&Value::String("macros".to_string()))
        .and_then(|v| v.as_mapping())
    {
        for (raw_name, raw_expr) in raw_macros {
            let name = raw_name.as_str().unwrap().to_string();
            let expr =
                LogicElement::parse(raw_expr.as_str().unwrap(), global_macros, logic_key_mapper)?;
            local_macros.insert(name, expr);
        }
    }
    // TODO: why doesn't extend work?
    for (name, expr) in global_macros.iter() {
        local_macros.insert(name.clone(), expr.clone());
    }
    let mut locations = HashMap::new();
    if let Some(raw_locs) = mapping
        .get(&Value::String("locations".into()))
        .and_then(|v| v.as_mapping())
    {
        for (raw_name, raw_expr) in raw_locs {
            let name = raw_name.as_str().unwrap().to_string();
            let check_key = logic_key_mapper.convert_to_num(&name);
            let expr =
                LogicElement::parse(raw_expr.as_str().unwrap(), &local_macros, logic_key_mapper)?;
            locations.insert(check_key, expr);
        }
    }
    if let Some(raw_events) = mapping
        .get(&Value::String("events".into()))
        .and_then(|v| v.as_mapping())
    {
        for (raw_name, raw_expr) in raw_events {
            let name = raw_name.as_str().unwrap().to_string();
            let check_key = logic_key_mapper.convert_to_num(&name);
            let expr =
                LogicElement::parse(raw_expr.as_str().unwrap(), &local_macros, logic_key_mapper)?;
            locations.insert(check_key.clone(), expr);
            // events, if you think of them as locations, have their
            // own name as "item"
            placement.insert(
                CheckKey {
                    area_key: self_area_key.clone(),
                    check: check_key.clone(),
                },
                check_key,
            );
        }
    }
    let mut exits = HashMap::new();
    if let Some(raw_map_exits) = mapping
        .get(&Value::String("map-exits".into()))
        .and_then(|v| v.as_mapping())
    {
        for (raw_name, raw_expr) in raw_map_exits {
            let other_name = raw_name.as_str().unwrap();
            let (other_stage_name, other_area_name) = other_name.split_once(" - ").unwrap();
            let (other_area_name, disambiguation) =
                if let Some((mut other_area_name, mut disambiguation)) =
                    other_area_name.split_once('(')
                {
                    // disambiguation, sometimes there are multiple entrance
                    // connections between stages
                    assert_eq!(disambiguation.chars().last(), Some(')'));
                    disambiguation = disambiguation[..disambiguation.len() - 1].trim();
                    other_area_name = other_area_name.trim();
                    (
                        other_area_name,
                        Some(logic_key_mapper.convert_to_num(disambiguation)),
                    )
                } else {
                    (other_area_name, None)
                };
            let other_area = logic_key_mapper.convert_to_area(other_stage_name, other_area_name);
            let expr =
                LogicElement::parse(raw_expr.as_str().unwrap(), &local_macros, logic_key_mapper)?;
            exits.insert(
                PassagewayKey {
                    other_area,
                    disambiguation,
                },
                expr,
            );
        }
    }
    if let Some(raw_logic_exits) = mapping
        .get(&Value::String("logic-exits".into()))
        .and_then(|v| v.as_mapping())
    {
        for (raw_name, raw_expr) in raw_logic_exits {
            let other_area_name = raw_name.as_str().unwrap();
            let other_area = logic_key_mapper.convert_to_area(stage_name, other_area_name);
            let expr =
                LogicElement::parse(raw_expr.as_str().unwrap(), &local_macros, logic_key_mapper)?;
            exits.insert(
                PassagewayKey {
                    other_area,
                    disambiguation: None,
                },
                expr,
            );
        }
    }
    let can_sleep = mapping
        .get(&Value::String("can-sleep".into()))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let local_allowed_tod = parse_force_tod(mapping)?;
    areas.insert(
        self_area_key,
        Area {
            exits,
            locations,
            name: logic_key_mapper.convert_to_num(area_name),
            stage: logic_key_mapper.convert_to_num(stage_name),
            region: logic_key_mapper.convert_to_num(region_name),
            allowed_tod: local_allowed_tod
                .or_else(|| force_tod.clone())
                .unwrap_or(AllowedToD::Both),
            can_sleep,
        },
    );
    Ok(())
}

pub fn specialize_for_options(areas: &mut HashMap<AreaKey, Area>, options: &RandomizerOptions) {
    for area in areas.values_mut() {
        for expr in area.exits.values_mut() {
            expr.specialize_for_options_macros(options);
        }
        for expr in area.locations.values_mut() {
            expr.specialize_for_options_macros(options);
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PassagewayEntry {
    pub stage: String,
    pub to_stage: String,
    pub disambiguation: Option<String>,
    pub door: Option<String>,
    #[serde(rename = "type")]
    pub pswg_type: Option<String>,
}

pub fn load_passageway_defs() -> Result<Vec<PassagewayEntry>, Box<dyn std::error::Error>> {
    let f = File::open("entrance_table2.yaml")?;
    let result: Vec<PassagewayEntry> = serde_yaml::from_reader(f)?;
    Ok(result)
}

#[derive(Deserialize)]
struct RawCheckEntry {
    #[serde(rename = "type")]
    types: String,
}

pub fn load_check_defs(
    mapper: &LogicKeyMapper,
) -> Result<HashMap<LogicKey, Vec<String>>, Box<dyn std::error::Error>> {
    let f = File::open("checks.yaml")?;
    let result: HashMap<String, RawCheckEntry> = serde_yaml::from_reader(f)?;
    Ok(result
        .into_iter()
        .map(|(name, raw)| {
            let (_region, name) = name.split_once(" - ").unwrap();
            let logic_key = mapper.convert_to_num_assuming_present(name).unwrap();
            let types = raw.types.split(',').map(|t| t.trim().to_string()).collect();
            (logic_key, types)
        })
        .collect())
}

#[cfg(test)]
mod tests {

    use std::collections::{HashMap, HashSet};

    use rand::prelude::*;
    use rand_pcg::Pcg64;
    use serde::Deserialize;

    use crate::{
        graph_logic::{
            logic_algorithms::{
                do_exploration, get_entrance_exits, get_exits, get_first_unreachable_loc,
                get_progress_item_list, place_entrances_coupled, FillError,
            },
            logic_expression::LogicElement,
            logic_loader::specialize_for_options,
            logic_structs::{
                Entrance, Exit, Inventory, LogicKey, LogicKeyMapper, PassagewayKey, Placement,
                TimeOfDay,
            },
        },
        options::RandomizerOptions,
    };

    use super::{do_parse, load_passageway_defs};

    #[test]
    pub fn test_do_parse() {
        let mut logic_key_mapper = LogicKeyMapper::default();
        let (mut areas, placement) = do_parse(&mut logic_key_mapper).unwrap();
        println!("area count: {}", areas.len());
        let mut area_names: Vec<_> = areas
            .iter()
            .map(|(key, area)| (key.name(&logic_key_mapper), &area.allowed_tod))
            .collect();
        // area_names.sort_by_key(|(area, _)| area);
        for (area, tod) in area_names {
            println!("{}: {:?}", area, tod);
        }
        println!("\nLogic strings: {}", logic_key_mapper.len());

        let options = RandomizerOptions::parse_option_file().unwrap();

        for opt in options.get_options() {
            println!("{}", opt.command);
        }

        specialize_for_options(&mut areas, &options);

        for area in areas.values() {
            for (check, expr) in area.locations.iter() {
                let mut out = String::new();
                expr.dbg_to_string(&mut out, &logic_key_mapper);
                let loc = logic_key_mapper.convert_to_string(check).unwrap();
                println!("{}: {}", loc, out);
            }
        }
    }
    #[test]
    pub fn test_areas_exist() {
        // find exits without entrance
        let mut logic_key_mapper = LogicKeyMapper::default();
        let (areas, _) = do_parse(&mut logic_key_mapper).unwrap();
        let (_, exits) = get_entrance_exits(&areas);

        // test that all areas do exist
        for exit in exits.iter() {
            if areas.get(&exit.to_area).is_none() {
                println!(
                    "can't find entrance area for {}",
                    exit.dbg_to_string(&logic_key_mapper)
                );
            }
        }
    }

    #[test]
    pub fn test_lonely_exits() {
        // find exits without entrance
        let mut logic_key_mapper = LogicKeyMapper::default();
        let (areas, _) = do_parse(&mut logic_key_mapper).unwrap();
        let (entrances, _) = get_entrance_exits(&areas);
        // (from stage, to stage, disambiguation)
        let entrance_set: HashSet<_> = entrances
            .iter()
            .map(|entrance| {
                (
                    areas.get(&entrance.from_area).unwrap().stage.clone(),
                    areas.get(&entrance.area).unwrap().stage.clone(),
                    entrance.disambiguation.clone(),
                )
            })
            .collect();
        let exit_set: HashSet<_> = entrances
            .iter()
            .map(|entrance| {
                (
                    areas.get(&entrance.area).unwrap().stage.clone(),
                    areas.get(&entrance.from_area).unwrap().stage.clone(),
                    entrance.disambiguation.clone(),
                )
            })
            .collect();
        let p = |key: &LogicKey| logic_key_mapper.convert_to_string(key).unwrap();
        for exit in exit_set {
            if !entrance_set.contains(&exit) {
                println!(
                    "{:?} to {:?} ({:?})",
                    p(&exit.0),
                    p(&exit.1),
                    exit.2.map(|k| p(&k))
                );
            }
        }
    }

    #[test]
    pub fn test_all_loc_reachable() {
        // find exits without entrance
        let mut logic_key_mapper = LogicKeyMapper::default();
        let (mut areas, mut item_placement) = do_parse(&mut logic_key_mapper).unwrap();
        let mut entrance_connections: HashMap<Exit, Entrance> = HashMap::new();
        let exits = get_exits(&areas);

        let exit_map: Vec<_> = exits.iter().cloned().collect();

        for exit in exits.iter() {
            // all vanilla
            entrance_connections.insert(exit.clone(), exit.to_entrance());
        }

        let mut rng = Pcg64::seed_from_u64(0);

        let mut placement = Placement {
            connected_areas: entrance_connections,
            filled_checks: item_placement,
        };

        let mut inventory = Inventory {
            areas: HashMap::new(),
            items_events_counts: HashMap::new(),
        };

        // begin seach from central skyloft
        let skyloft_central = logic_key_mapper
            .convert_to_area_assuming_present("Skyloft", "Central Outside")
            .unwrap();

        inventory.add_area_tod(&skyloft_central, &TimeOfDay::Day);

        // add all progress items
        for item_key in get_progress_item_list(&logic_key_mapper) {
            inventory.collect_item(item_key);
        }

        // used for initial statue rando
        for item in &[
            "Sealed Grounds Statue",
            "Eldin Entrance Statue",
            "Lanayru Mine Entry Statue",
        ] {
            let item_key = logic_key_mapper
                .convert_to_num_assuming_present(item)
                .unwrap();
            inventory.collect_item(item_key);
        }

        // set required dungeons to 0
        let sealed_temple_main = logic_key_mapper
            .convert_to_area_assuming_present("Sealed Temple", "Main")
            .unwrap();
        let beat_req_dungeons = logic_key_mapper
            .convert_to_num_assuming_present("Beat Required Dungeons")
            .unwrap();

        *areas
            .get_mut(&sealed_temple_main)
            .unwrap()
            .locations
            .get_mut(&beat_req_dungeons)
            .unwrap() = LogicElement::FixedValue(true);

        let options = RandomizerOptions::parse_option_file().unwrap();
        specialize_for_options(&mut areas, &options);

        do_exploration(&areas, &placement, &mut inventory);

        // find out what areas we still can't reach
        for (area_key, area) in areas.iter() {
            if !inventory.check_area_tod(area_key, &TimeOfDay::Day)
                && !inventory.check_area_tod(area_key, &TimeOfDay::Night)
            {
                println!("unreachable: {}", area_key.name(&logic_key_mapper));
            }
        }

        // for (area_key, allowed_tod) in inventory.areas.iter() {
        //     println!(
        //         "reachable: {} ({:?})",
        //         area_key.name(&logic_key_mapper),
        //         allowed_tod
        //     );
        // }

        // find out what checks we can't reach
        if let Some(unreach) = get_first_unreachable_loc(&areas, &inventory) {
            println!("unreachable: {}", unreach.check.name(&logic_key_mapper));
        }
    }

    #[test]
    pub fn shuffle_test() {
        // find exits without entrance
        let mut logic_key_mapper = LogicKeyMapper::default();
        let (mut areas, mut item_placement) = do_parse(&mut logic_key_mapper).unwrap();
        let mut entrance_connections: HashMap<Exit, Entrance> = HashMap::new();
        let exits = get_exits(&areas);

        let mut seen_passageways = HashSet::new();

        let passageway_defs = load_passageway_defs().unwrap();
        let mut psgw_set = HashSet::new();
        for p in passageway_defs.iter() {
            if p.pswg_type.is_some() {
                continue;
            }
            // ignore one set of doors for now
            if p.door.as_ref().map_or(false, |d| d == "Right") {
                continue;
            }
            let from_key = logic_key_mapper
                .convert_to_num_assuming_present(&p.stage)
                .unwrap();
            let to_key = logic_key_mapper
                .convert_to_num_assuming_present(&p.to_stage)
                .unwrap();
            let disambig = p
                .disambiguation
                .as_ref()
                .map(|d| logic_key_mapper.convert_to_num_assuming_present(d).unwrap());
            psgw_set.insert((from_key, to_key, disambig));
        }

        for exit in exits.iter() {
            let from_stage = areas.get(&exit.area).unwrap().stage.clone();
            let to_stage = areas.get(&exit.to_area).unwrap().stage.clone();
            if from_stage == to_stage {
                continue;
            }
            let psgw = (from_stage, to_stage, exit.disambiguation.clone());
            if psgw_set.contains(&psgw) && seen_passageways.contains(&psgw) {
                println!(
                    "duplicate passageway: {}, {}",
                    psgw.0.name(&logic_key_mapper),
                    psgw.1.name(&logic_key_mapper)
                );
            } else {
                seen_passageways.insert(psgw);
            }
        }

        let exit_map: HashSet<_> = exits.iter().cloned().collect();

        let mut entrances_to_shuffle = Vec::new();

        for exit in exits.iter() {
            let stages_pswg = exit.to_stages(&areas);
            if stages_pswg.0 == stages_pswg.1 {
                // inside a stage, always vanilla
                entrance_connections.insert(exit.clone(), exit.to_entrance());
            } else {
                // ensure this is a 2 way entrance
                let opposite_way = exit.reverse();
                if psgw_set.contains(&stages_pswg) && exit_map.contains(&opposite_way) {
                    // different stage, needs to be randomized
                    entrances_to_shuffle.push(exit.to_entrance());
                } else {
                    // one way, not randomized
                    entrance_connections.insert(exit.clone(), exit.to_entrance());
                }
            }
        }

        let mut rng = Pcg64::seed_from_u64(0);

        // due to areas being a hash map, this is essentially random
        // otherwise
        // logic keys are stable though
        entrances_to_shuffle.sort();

        let mut placement = Placement {
            connected_areas: entrance_connections,
            filled_checks: item_placement,
        };

        let mut inventory = Inventory {
            areas: HashMap::new(),
            items_events_counts: HashMap::new(),
        };

        // start search from central skyloft
        let skyloft_central = logic_key_mapper
            .convert_to_area_assuming_present("Skyloft", "Central Outside")
            .unwrap();

        inventory.add_area_tod(&skyloft_central, &TimeOfDay::Day);

        // assume all items
        for item_key in get_progress_item_list(&logic_key_mapper) {
            inventory.collect_item(item_key);
        }

        // assume vanilla starting statues
        for item in &[
            "Sealed Grounds Statue",
            "Eldin Entrance Statue",
            "Lanayru Mine Entry Statue",
        ] {
            let item_key = logic_key_mapper
                .convert_to_num_assuming_present(item)
                .unwrap();
            inventory.collect_item(item_key);
        }

        // set required dungeons to 0
        let sealed_temple_main = logic_key_mapper
            .convert_to_area_assuming_present("Sealed Temple", "Main")
            .unwrap();
        let beat_req_dungeons = logic_key_mapper
            .convert_to_num_assuming_present("Beat Required Dungeons")
            .unwrap();

        *areas
            .get_mut(&sealed_temple_main)
            .unwrap()
            .locations
            .get_mut(&beat_req_dungeons)
            .unwrap() = LogicElement::FixedValue(true);

        let options = RandomizerOptions::parse_option_file().unwrap();

        specialize_for_options(&mut areas, &options);

        do_exploration(&areas, &placement, &mut inventory);

        let new_placement = loop {
            let mut new_placement = placement.clone();
            let mut entrances = entrances_to_shuffle.clone();
            if let Err(FillError::NoValidExitLeft(entr)) = place_entrances_coupled(
                &mut rng,
                &areas,
                &mut new_placement,
                &inventory,
                &mut entrances,
                &logic_key_mapper,
            ) {
                println!("error placing {}", entr.dbg_to_string(&logic_key_mapper));
            } else {
                break new_placement;
            }
        };

        for (exit, entrance) in new_placement.connected_areas.iter() {
            // quadratic complexity let's go
            if entrances_to_shuffle.contains(&entrance) {
                println!(
                    "{:<70}: {}",
                    entrance.dbg_to_string(&logic_key_mapper),
                    exit.dbg_to_string(&logic_key_mapper)
                );
            }
        }

        // println!()
    }
}
