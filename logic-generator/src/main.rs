use std::{
    any::Any,
    borrow::Cow,
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    fmt::{format, Debug},
    fs::File,
    io::{self, BufReader, Write},
};

use anyhow::Context;
use heck::ToUpperCamelCase;
use indexmap::IndexMap;
use regex::Regex;
use requirement_expression::{RequirementExpression, RequirementToD};
use serde::Deserialize;

mod requirement_expression;

//
// START yaml stuff
//

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum TimeOfDay {
    Day,
    Night,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Door {
    Left,
    Right,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Area {
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
pub struct Stage {
    force_tod: Option<TimeOfDay>,
    // stage: String,
    #[serde(default)]
    areas: BTreeMap<String, Area>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Region {
    force_tod: Option<TimeOfDay>,
    #[serde(default)]
    stages: BTreeMap<String, Stage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OrigEntrance {
  stage: String,
  room: u8,
  layer: u8,
  entrance: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ScenEntry {
    stage: String,
    room: u8,
    index: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EntranceTableEntry {
    stage: String,
    to_stage: String,
    disambiguation: Option<String>,
    door: Option<Door>,
    orig: OrigEntrance,
    scens: Vec<ScenEntry>,
}

//
// END yaml stuff
//

pub fn convert_to_upper_camel_case(s: &str) -> String {
    let mut result = s.chars()
        .filter(|c| *c != '\'')
        .collect::<String>()
        .to_upper_camel_case();
    if result.as_bytes()[0].is_ascii_digit() {
        result.insert(0, 'X');
    }
    result
}

//
// START enum stuff
//

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct NameAndEnumName {
    name: String,
    enum_name: String,
}

impl NameAndEnumName {
    pub fn from_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            enum_name: convert_to_upper_camel_case(name),
        }
    }
}

struct AreaEnumMember {
    name: NameAndEnumName,
    stage: String,
    force_tod: Option<TimeOfDay>,
    can_sleep: bool,
    logic_exits: Vec<String>,
    locations: Vec<String>,
    exits: Vec<String>,
}

struct StageEnumMember {
    name: NameAndEnumName,
    region: String,
    force_tod: Option<TimeOfDay>,
    areas: Vec<String>,
}

struct RegionEnumMember {
    name: NameAndEnumName,
    force_tod: Option<TimeOfDay>,
    areas: Vec<String>,
}

struct LocationEnumMember {
    name: NameAndEnumName,
    area: String,
    requirement: RequirementExpression,
}

struct ExitEnumMember {
    name: NameAndEnumName,
    area: String,
    to: String,
    disambiguation: Option<String>,
    // entrance enum
    vanilla_entrance: String,
    // entrance enum
    maybe_coupled_entrance: String,
    // exit enum
    door_connection: Option<String>,
    requirement: RequirementExpression,
}

struct EntranceEnumMember {
    name: NameAndEnumName,
    area: String,
}

fn print_common<'a, W: Write>(
    out: &mut W,
    typename: &str,
    items: impl Iterator<Item = &'a NameAndEnumName> + Clone,
) -> io::Result<()> {
    writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
    writeln!(out, "pub enum {} {{", typename)?;

    // print enum
    for item in items.clone() {
        writeln!(out, "{},", item.enum_name)?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "impl Into<usize> for {} {{", typename)?;
    writeln!(out, "fn into(self) -> usize {{")?;
    writeln!(out, "self as usize")?;
    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    writeln!(out, "impl BitSetCompatible for {} {{", typename)?;
    writeln!(out, "const ALL: &'static [{}] = &[", typename)?;

    for item in items.clone() {
        writeln!(out, "{}::{},", typename, item.enum_name)?;
    }
    writeln!(out, "];")?;
    writeln!(out, "}}")?;

    writeln!(out, "impl {} {{", typename)?;

    //
    writeln!(out, "pub fn name(&self) -> &'static str {{")?;
    writeln!(out, "match self {{")?;

    for item in items {
        writeln!(
            out,
            "{}::{} => \"{}\",",
            typename, item.enum_name, item.name
        )?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    writeln!(out, "}}")?;

    Ok(())
}

fn print_regions<W: Write>(out: &mut W, regions: &[RegionEnumMember]) -> io::Result<()> {
    print_common(out, "Region", regions.iter().map(|r| &r.name))?;

    writeln!(out, "impl Region {{")?;

    //
    writeln!(out, "pub fn areas(&self) -> &'static [Area] {{")?;
    writeln!(out, "match self {{")?;

    for region in regions {
        writeln!(out, "Region::{} => &[", region.name.enum_name)?;
        for area in &region.areas {
            writeln!(out, "Area::{},", area)?;
        }
        writeln!(out, "],")?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    //
    // writeln!(out, "pub fn force_tod(&self) -> Option<TimeOfDay> {{")?;
    // writeln!(out, "use TimeOfDay::*;")?;
    // writeln!(out, "match self {{")?;

    // for region in regions {
    //     writeln!(
    //         out,
    //         "Region::{} => {:?},",
    //         region.enum_name, region.force_tod
    //     )?;
    // }

    // writeln!(out, "}}")?;
    // writeln!(out, "}}")?;

    writeln!(out, "}}")?;
    Ok(())
}

fn print_stages<W: Write>(out: &mut W, stages: &[StageEnumMember]) -> io::Result<()> {
    print_common(out, "Stage", stages.iter().map(|r| &r.name))?;

    writeln!(out, "impl Stage {{")?;

    //
    writeln!(out, "pub fn region(&self) -> Region {{")?;
    writeln!(out, "match self {{")?;

    for stage in stages {
        writeln!(
            out,
            "Stage::{} => Region::{},",
            stage.name.enum_name, stage.region
        )?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    //
    writeln!(out, "pub fn areas(&self) -> &'static [Area] {{")?;
    writeln!(out, "match self {{")?;

    for stage in stages {
        writeln!(out, "Stage::{} => &[", stage.name.enum_name)?;
        for area in &stage.areas {
            writeln!(out, "Area::{},", area)?;
        }
        writeln!(out, "],")?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    //
    // writeln!(out, "pub fn force_tod(&self) -> Option<TimeOfDay> {{")?;
    // writeln!(out, "use TimeOfDay::*;")?;
    // writeln!(out, "match self {{")?;

    // for stage in stages {
    //     writeln!(out, "Stage::{} => {:?},", stage.enum_name, stage.force_tod)?;
    // }

    // writeln!(out, "}}")?;
    // writeln!(out, "}}")?;

    writeln!(out, "}}")?;
    Ok(())
}

fn print_areas<W: Write>(
    out: &mut W,
    areas: &[AreaEnumMember],
    entrances: &BTreeMap<String, BTreeMap<String, ()>>,
    logic_entrances: &BTreeMap<String, BTreeMap<String, ()>>,
) -> io::Result<()> {
    print_common(out, "Area", areas.iter().map(|r| &r.name))?;

    writeln!(out, "pub struct AreaVal {{
        pub stage: Stage,
        pub possible_tod: TimeOfDay,
        pub can_sleep: bool,
        pub exits: &'static [Exit],
        pub entrances: &'static [Entrance],
        pub logic_entrance_areas: &'static [Area],
        pub logic_exit_areas: &'static [Area],
        pub locations: &'static [Location],
    }}")?;

    writeln!(out, "impl Area {{")?;

    //
    writeln!(out, "pub fn get(&self) -> &'static AreaVal {{")?;
    writeln!(out, "match self {{")?;

    for area in areas {
        let tod = match area.force_tod {
            Some(TimeOfDay::Day) => "Day",
            Some(TimeOfDay::Night) => "Night",
            None => "Both",
        };
        use std::fmt::Write;
        let mut exits = String::new();
        for exit in &area.exits {
            writeln!(exits, "Exit::{},", exit).unwrap();
        }
        
        let mut entrances_str = String::new();
        if let Some(entrances) = entrances.get(&area.name.enum_name) {
            for entrance in entrances.keys() {
                writeln!(entrances_str, "Entrance::{},", entrance).unwrap();
            }
        }

        let mut logic_exits = String::new();
        for area in &area.logic_exits {
            writeln!(logic_exits, "Area::{},", area).unwrap();
        }

        let mut logic_entrances_str = String::new();
        if let Some(entrances) = logic_entrances.get(&area.name.enum_name) {
            for area in entrances.keys() {
                writeln!(logic_entrances_str, "Area::{},", area).unwrap();
            }
        }
        let mut locations = String::new();
        for location in &area.locations {
            writeln!(locations, "Location::{},", location).unwrap();
        }
        writeln!(
            out,
            "Area::{} => &AreaVal {{
                stage: Stage::{},
                possible_tod: TimeOfDay::{},
                can_sleep: {},
                exits: & [{}],
                entrances: & [{}],
                logic_exit_areas: & [{}],
                logic_entrance_areas: & [{}],
                locations: &[{}],
            }},",
            area.name.enum_name,
            area.stage,
            tod,
            area.can_sleep,
            exits,
            entrances_str,
            logic_exits,
            logic_entrances_str,
            locations,
        )?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    writeln!(out, "}}")?;
    Ok(())
}

fn print_locations<W: Write>(out: &mut W, locations: &[LocationEnumMember]) -> io::Result<()> {
    print_common(out, "Location", locations.iter().map(|l| &l.name))?;

    writeln!(out, "impl Location {{")?;

    //
    writeln!(out, "pub fn area(&self) -> Area {{")?;
    writeln!(out, "match self {{")?;

    for location in locations {
        writeln!(
            out,
            "Location::{} => Area::{},",
            location.name.enum_name, location.area
        )?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    writeln!(out, "}}")?;
    Ok(())
}

fn print_events<W: Write>(
    out: &mut W,
    events: &BTreeMap<NameAndEnumName, RequirementExpression>,
) -> io::Result<()> {
    print_common(out, "Event", events.keys())?;

    Ok(())
}

fn print_exits<W: Write>(out: &mut W, exits: &[ExitEnumMember], entrances: &[EntranceEnumMember]) -> io::Result<()> {
    print_common(out, "Exit", exits.iter().map(|l| &l.name))?;

    writeln!(out, "pub struct ExitVal {{
        pub area: Area,
        pub to: Area,
        pub disambiguation: Option<&'static str>,
        pub vanilla_entrance: Entrance,
        pub coupled_entrance: Option<Entrance>,
        pub door_connection: DoorConnection<Exit>,
    }}")?;

    writeln!(out, "impl Exit {{")?;

    //
    writeln!(out, "pub fn area(&self) -> Area {{")?;
    writeln!(out, "match self {{")?;

    for exit in exits {
        writeln!(out, "Exit::{} => Area::{},", exit.name.enum_name, exit.area)?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    //
    writeln!(out, "pub fn get(&self) -> &'static ExitVal {{")?;
    writeln!(out, "match self {{")?;

    for exit in exits {
        writeln!(
            out,
            "Exit::{} => &ExitVal {{
                area: Area::{},
                to: Area::{},
                vanilla_entrance: Entrance::{},", exit.name.enum_name, exit.area, exit.to, exit.vanilla_entrance)?;
        if let Some(d) = exit.disambiguation.as_ref() {
            writeln!(out, "disambiguation: Some(\"{d}\"),")?;
        } else {
            writeln!(out, "disambiguation: None,")?;
        }
        if let Some(entrance) = entrances.iter().find(|entrance| exit.maybe_coupled_entrance == entrance.name.name) {
            writeln!(out, "coupled_entrance: Some(Entrance::{}),", entrance.name.enum_name)?;
        } else {
            writeln!(out, "coupled_entrance: None,")?;
        }
        if let Some(dc) = exit.door_connection.as_ref() {
            writeln!(out, "door_connection: DoorConnection::{dc},")?;
        } else {
            writeln!(out, "door_connection: DoorConnection::No,")?;
        }
        writeln!(out, "}},")?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    writeln!(out, "}}")?;
    Ok(())
}

fn print_entrances<W: Write>(out: &mut W, entrances: &[EntranceEnumMember]) -> io::Result<()> {
    print_common(out, "Entrance", entrances.iter().map(|l| &l.name))?;

    writeln!(out, "impl Entrance {{")?;

    //
    writeln!(out, "pub fn area(&self) -> Area {{")?;
    writeln!(out, "match self {{")?;

    for entrance in entrances {
        writeln!(
            out,
            "Entrance::{} => Area::{},",
            entrance.name.enum_name, entrance.area
        )?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    writeln!(out, "}}")?;
    Ok(())
}

fn read_logic_into(name: &str, map: &mut BTreeMap<String, Region>) -> anyhow::Result<()> {
    let reader =
        BufReader::new(File::open(format!("../bitless/{name}.yaml")).context(name.to_string())?);
    let region: BTreeMap<String, Region> = serde_yaml::from_reader(reader)?;
    map.extend(region);
    Ok(())
}

fn parse_macro_file(
    item_names: &HashSet<String>,
) -> anyhow::Result<HashMap<String, RequirementExpression>> {
    let reader = BufReader::new(File::open(format!("../bitless/macros.yaml")).context("macros")?);
    let raw_macros: IndexMap<String, String> = serde_yaml::from_reader(reader)?;
    let mut macros = HashMap::new();
    for (macro_name, macro_req) in &raw_macros {
        let parsed =
            RequirementExpression::parse(macro_req, None, &[&macros], &item_names).unwrap();
        println!("{:?}", parsed);
        macros.insert(macro_name.clone(), parsed);
    }
    Ok(macros)
}

fn parse_entrance_table() -> anyhow::Result<Vec<EntranceTableEntry>> {
    let reader = BufReader::new(File::open("../entrance_table2.yaml").context("entrance_table2.yaml")?);
    serde_yaml::from_reader(reader).context("entrance_table")
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum ItemKind {
    Single,
    Counted,
    Consumable,
}

#[derive(Debug, Deserialize)]
struct ItemYamlEntry {
    id: usize,
    name: String,
    oarc: serde_yaml::Value,
    getarcname: String,
    getmodelname: String,
    r#type: ItemKind,
}

fn process_items() -> anyhow::Result<HashSet<String>> {
    let reader = BufReader::new(File::open("items.yaml")?);
    let mut raw_items: Vec<ItemYamlEntry> = serde_yaml::from_reader(reader)?;
    raw_items.sort_by_key(|i| i.r#type);

    let single_count = raw_items
        .iter()
        .filter(|i| i.r#type == ItemKind::Single)
        .count();
    let counted_count = raw_items
        .iter()
        .filter(|i| i.r#type == ItemKind::Counted)
        .count();

    println!("single: {}, counted: {}", single_count, counted_count);

    let out = &mut File::create("../logic-core/src/generated/items.rs")?;
    writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
    writeln!(out, "pub enum Item {{")?;

    // print enum
    for item in &raw_items {
        writeln!(out, "{},", convert_to_upper_camel_case(&item.name))?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "impl Item {{")?;
    writeln!(out, "pub const ALL: &'static [Item] = &[")?;

    for item in &raw_items {
        writeln!(out, "Item::{},", convert_to_upper_camel_case(&item.name))?;
    }
    writeln!(out, "];")?;

    //
    writeln!(out, "pub fn name(&self) -> &'static str {{")?;
    writeln!(out, "match self {{")?;

    for item in &raw_items {
        writeln!(
            out,
            "Item::{} => \"{}\",",
            convert_to_upper_camel_case(&item.name),
            item.name
        )?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    writeln!(out, "}}")?;

    writeln!(out, "pub const SINGLE_ITEM_COUNT: usize = {single_count};")?;
    writeln!(
        out,
        "pub const COUNTED_ITEM_COUNT: usize = {counted_count};"
    )?;

    let names = raw_items.iter().map(|i| i.name.clone()).collect();

    Ok(names)
}

fn print_base_requirements<'a, W: Write>(
    out: &mut W,
    requirements: impl Iterator<Item = (String, &'a RequirementExpression)> + Clone,
) -> io::Result<()> {
    out.write_all(b"use std::collections::HashMap;\n")?;
    out.write_all(b"use super::{items::Item,logic::{Area, Exit, Event, Location}};\n")?;
    out.write_all(b"use crate::{logic_static::TimeOfDay,requirements::RequirementExpression, RequirementKey};\n")?;

    out.write_all(
        b"pub fn get_requirements() -> HashMap<RequirementKey, RequirementExpression<'static>> {\n",
    )?;
    writeln!(out, "HashMap::from([")?;

    for (name, req) in requirements {
        write!(out, "({},", name)?;
        req.write_reqs(out)?;
        writeln!(out, "),")?;
    }
    writeln!(out, "])}}")?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let item_names = process_items()?;

    let entrance_table = parse_entrance_table()?;

    let macros = parse_macro_file(&item_names)?;

    let stage_exit_re = Regex::new(r#"([^-]+) - ([^(-]+)( \(([^-]+)\))?$"#)?;
    let mut regions: BTreeMap<String, Region> = BTreeMap::new();

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

    // area: list of entrances (written as area)
    let mut entrances: BTreeMap<String, BTreeMap<String, ()>> = BTreeMap::new();
    // exit_area_name: (entrance_area_name)
    let mut logic_entrances: BTreeMap<String, BTreeMap<String, ()>> = BTreeMap::new();

    let mut area_enums = Vec::new();
    let mut stage_enums = Vec::new();
    let mut region_enums = Vec::new();
    let mut location_enums = Vec::new();
    let mut exit_enums = Vec::new();
    let mut entrance_enums = Vec::new();
    let mut logic_exit_requirements = Vec::new();
    let mut events: BTreeMap<NameAndEnumName,RequirementExpression> =
        BTreeMap::new();
    // let mut exit_enums = Vec::new();
    // println!("{faron:?}");
    for (region_name, region) in regions.iter() {
        let region_force_tod = region.force_tod.clone();
        // println!("{}: {:?}", region_name, region.force_tod);
        let region_name = NameAndEnumName::from_name(region_name);

        let mut region_area_names = Vec::new();
        for (stage_name, stage) in region.stages.iter() {
            let stage_force_tod = stage.force_tod.or(region_force_tod);

            let stage_name = NameAndEnumName::from_name(stage_name);
            let mut area_names = Vec::new();
            for (area_name, area) in stage.areas.iter() {
                let area_force_tod = area.force_tod.or(stage_force_tod);
                let area_enum_name_with_stage = format!(
                    "{}_{}",
                    stage_name.enum_name,
                    convert_to_upper_camel_case(area_name)
                );

                let area_name = NameAndEnumName {
                    name: area_name.clone(),
                    enum_name: area_enum_name_with_stage,
                };

                let mut locations = Vec::new();
                let mut exits = Vec::new();
                // other_area_name
                let mut logic_exits = Vec::new();

                let mut local_macros = HashMap::new();

                for (macro_name, macro_req) in area.macros.iter() {
                    let resolved_req = RequirementExpression::parse(
                        macro_req,
                        Some(&area_name.enum_name),
                        &[&macros, &local_macros],
                        &item_names,
                    )
                    .unwrap();
                    local_macros.insert(
                        macro_name.clone(),
                        RequirementExpression::And(vec![
                            RequirementExpression::Area(
                                area_name.enum_name.clone(),
                                RequirementToD::Any,
                            ),
                            resolved_req,
                        ]),
                    );
                }

                for (logic_exit_area, req) in area.logic_exits.iter() {
                    let requirement_name = format!(
                        "RequirementKey::LogicExit{{ from: Area::{}, to: Area::{}_{}}}",
                        area_name.enum_name,
                        stage_name.enum_name,
                        convert_to_upper_camel_case(logic_exit_area)
                    );

                    let resolved_req = RequirementExpression::parse(
                        req,
                        Some(&area_name.enum_name),
                        &[&macros, &local_macros],
                        &item_names,
                    )
                    .unwrap();
                    logic_exit_requirements.push((
                        requirement_name.clone(),
                        RequirementExpression::And(vec![
                            RequirementExpression::Area(
                                area_name.enum_name.clone(),
                                RequirementToD::Any,
                            ),
                            resolved_req,
                        ]),
                    ));
                    let other_area_name = format!(
                        "{}_{}",
                        stage_name.enum_name,
                        convert_to_upper_camel_case(logic_exit_area)
                    );
                    logic_exits.push(other_area_name.clone());
                    logic_entrances
                        .entry(other_area_name)
                        .or_default()
                        .insert(area_name.enum_name.clone(), ());
                }

                for (exit_name, exit_req) in area.map_exits.iter() {
                    if let Some(captures) = stage_exit_re.captures(&exit_name) {
                        let other_stagename = &captures[1];
                        let other_areaname = &captures[2];
                        let disambiguation = captures.get(4).map(|c| c.as_str());
                        let mut full_exit_enum_name = format!(
                            "{}_To_{}",
                            stage_name.enum_name,
                            convert_to_upper_camel_case(other_stagename)
                        );
                        let mut full_exit_name =
                            format!("{} -> {}", stage_name.name, other_stagename,);
                        // TODO: the mapping of exit to entrance doesn't necessarily need to
                        // be 1:1, refactor again when full ER
                        let mut full_entrance_enum_name = format!(
                            "{}_From_{}",
                            convert_to_upper_camel_case(other_stagename),
                            stage_name.enum_name,
                        );
                        let mut full_entrance_name =
                            format!("{} (from {})", other_stagename, stage_name.name,);
                        // this entrance might not actually exist, that will be checked later
                        let mut full_coupled_entrance_name =
                            format!("{} (from {})", stage_name.name, other_stagename,);
                        if let Some(disambiguation) = disambiguation {
                            use std::fmt::Write;
                            write!(
                                &mut full_exit_enum_name,
                                "_{}",
                                convert_to_upper_camel_case(disambiguation)
                            )?;
                            write!(
                                &mut full_entrance_enum_name,
                                "_{}",
                                convert_to_upper_camel_case(disambiguation)
                            )?;
                            write!(&mut full_exit_name, " ({})", disambiguation)?;
                            write!(&mut full_entrance_name, " ({})", disambiguation)?;
                            write!(&mut full_coupled_entrance_name, " ({})", disambiguation)?;
                        }

                        let other_area_enum_name = format!(
                            "{}_{}",
                            convert_to_upper_camel_case(other_stagename),
                            convert_to_upper_camel_case(other_areaname)
                        );
                        let exit_name = NameAndEnumName {
                            name: full_exit_name,
                            enum_name: full_exit_enum_name,
                        };
                        let entrance_name = NameAndEnumName {
                            name: full_entrance_name,
                            enum_name: full_entrance_enum_name,
                        };
                        entrances
                            .entry(other_area_enum_name.clone())
                            .or_default()
                            .insert(entrance_name.enum_name.clone(), ());
                        exits.push(exit_name.enum_name.clone());

                        let resolved_req = RequirementExpression::parse(
                            exit_req,
                            Some(&area_name.enum_name),
                            &[&macros, &local_macros],
                            &item_names,
                        )
                        .unwrap();

                        // n is probably low enough for quadratic complexity
                        let matching_exits: Vec<_> = entrance_table.iter().filter(|entry| {
                            entry.stage == stage_name.enum_name && entry.to_stage == other_stagename
                        }).collect();

                        exit_enums.push(ExitEnumMember {
                            name: exit_name,
                            area: area_name.enum_name.clone(),
                            to: other_area_enum_name.clone(),
                            disambiguation: disambiguation.map(|d| d.to_string()),
                            maybe_coupled_entrance: full_coupled_entrance_name,
                            door_connection: None,
                            vanilla_entrance: entrance_name.enum_name.clone(),
                            requirement: RequirementExpression::And(vec![
                                RequirementExpression::Area(
                                    area_name.enum_name.clone(),
                                    RequirementToD::Any,
                                ),
                                resolved_req,
                            ]),
                        });
                        entrance_enums.push(EntranceEnumMember {
                            name: entrance_name,
                            area: other_area_enum_name.clone(),
                        });
                    }
                    // println!("{exit_name}");
                    // println!("{captures:?}");
                    // let mut exit_name = convert_to_upper_camel_case(exit_name);
                    // let full_exit_name = format!("{}_To_{}", area_name_with_stage, exit_name, )
                }

                for (location_name, location_req) in &area.locations {
                    let location_name_with_region = format!(
                        "{}_{}",
                        region_name.enum_name,
                        convert_to_upper_camel_case(location_name)
                    );
                    let location_name = NameAndEnumName {
                        name: location_name.clone(),
                        enum_name: location_name_with_region,
                    };

                    let resolved_req = RequirementExpression::parse(
                        location_req,
                        Some(&area_name.enum_name),
                        &[&macros, &local_macros],
                        &item_names,
                    )
                    .unwrap();

                    locations.push(location_name.enum_name.clone());
                    location_enums.push(LocationEnumMember {
                        name: location_name,
                        requirement: RequirementExpression::And(vec![
                            RequirementExpression::Area(
                                area_name.enum_name.clone(),
                                RequirementToD::Any,
                            ),
                            resolved_req,
                        ]),
                        area: area_name.enum_name.clone(),
                    });
                }

                for (event_name, event_req) in area.events.iter() {
                    let event_name = NameAndEnumName::from_name(event_name);
                    let resolved_req = RequirementExpression::parse(
                        event_req,
                        Some(&area_name.enum_name),
                        &[&macros, &local_macros],
                        &item_names,
                    )
                    .unwrap();
                    let full_requirement = RequirementExpression::And(vec![
                        RequirementExpression::Area(
                            area_name.enum_name.clone(),
                            RequirementToD::Any,
                        ),
                        resolved_req,
                    ]);
                    // build the combined event requirements; if it already exists, add it to the OR chain
                    match events.entry(event_name) {
                        std::collections::btree_map::Entry::Occupied(mut existing_req) => {
                            match existing_req.get_mut() {
                                RequirementExpression::Or(variants) => variants.push(full_requirement),
                                req_ref => {
                                    let new_requirement = RequirementExpression::Or(vec![req_ref.clone(), full_requirement]);
                                    existing_req.insert(new_requirement);
                                },
                            }
                        },
                        std::collections::btree_map::Entry::Vacant(entry) => {
                            entry.insert(full_requirement);
                        }
                    }
                }

                area_names.push(area_name.enum_name.clone());
                region_area_names.push(area_name.enum_name.clone());
                if area.can_sleep && area.force_tod.is_some() {
                    panic!(
                        "that doesn't make sense: can_sleep = true, but forced ToD in {}",
                        area_name.enum_name
                    );
                }
                area_enums.push(AreaEnumMember {
                    locations,
                    can_sleep: area.can_sleep,
                    force_tod: area_force_tod,
                    name: area_name,
                    stage: stage_name.enum_name.clone(),
                    logic_exits,
                    exits,
                });
            }

            // TODO: skyloft being in 3 regions makes things dumb
            if stage_name.enum_name == "Skyloft"
                && stage_enums
                    .iter()
                    .any(|s: &StageEnumMember| s.name.enum_name == "Skyloft")
            {
                continue;
            }
            stage_enums.push(StageEnumMember {
                name: stage_name,
                region: region_name.enum_name.clone(),
                force_tod: stage_force_tod,
                areas: area_names,
            });
        }
        region_enums.push(RegionEnumMember {
            name: region_name,
            areas: region_area_names,
            force_tod: region_force_tod,
        });
    }

    let mut out = File::create("../logic-core/src/generated/logic.rs")?;
    writeln!(&mut out, "#![allow(non_camel_case_types)]")?;
    writeln!(
        &mut out,
        "use crate::logic_static::{{TimeOfDay, BitSetCompatible, DoorConnection}};"
    )?;

    print_regions(&mut out, &region_enums)?;
    print_stages(&mut out, &stage_enums)?;
    print_areas(&mut out, &area_enums, &entrances, &logic_entrances)?;
    print_locations(&mut out, &location_enums)?;
    print_events(&mut out, &events)?;

    // make an iterator of (name, requirement) out of everything that needs logical requirements
    let requirements = logic_exit_requirements
        .iter()
        .map(|(s, r)| (s.clone(), r))
        .chain(
            location_enums
                .iter()
                .map(|l| (format!("RequirementKey::Location(Location::{})", l.name.enum_name), &l.requirement)),
        )
        .chain(
            exit_enums
                .iter()
                .map(|e| (format!("RequirementKey::Exit(Exit::{})", e.name.enum_name), &e.requirement)),
        )
        .chain(events.iter().map(|(s, r)| (format!("RequirementKey::Event(Event::{})", s.enum_name), r)));
    // make sure all events exist
    let mut all_events = HashSet::new();
    for (name, requirement) in requirements.clone() {
        requirement.get_all_base_reqs(&mut all_events);
    }
    // for event in &all_events {
    //     if !events.contains_key(*event) {
    //         println!("{event}");
    //     }
    // }
    let mut out_requirements =
        File::create("../logic-core/src/generated/generated_requirements.rs")?;
    print_base_requirements(&mut out_requirements, requirements)?;
    print_exits(&mut out, &exit_enums, &entrance_enums)?;
    print_entrances(&mut out, &entrance_enums)?;

    Ok(())
}
