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

//
// END yaml stuff
//

pub fn convert_to_upper_camel_case(s: &str) -> String {
    s.chars()
        .filter(|c| *c != '\'')
        .collect::<String>()
        .to_upper_camel_case()
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
    logic_exits: Vec<(String, String)>,
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
    stages: Vec<String>,
}

struct LocationEnumMember {
    name: NameAndEnumName,
    area: String,
    requirement: RequirementExpression,
}

struct ExitEnumMember {
    name: NameAndEnumName,
    area: String,
    vanilla_entrance: String,
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
    writeln!(out, "pub fn stages(&self) -> &'static [Stage] {{")?;
    writeln!(out, "match self {{")?;

    for region in regions {
        writeln!(out, "Region::{} => &[", region.name.enum_name)?;
        for stage in &region.stages {
            writeln!(out, "Stage::{},", stage)?;
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
    logic_entrances: &BTreeMap<String, BTreeMap<String, String>>,
) -> io::Result<()> {
    print_common(out, "Area", areas.iter().map(|r| &r.name))?;

    writeln!(out, "impl Area {{")?;

    //
    writeln!(out, "pub fn stage(&self) -> Stage {{")?;
    writeln!(out, "match self {{")?;

    for area in areas {
        writeln!(
            out,
            "Area::{} => Stage::{},",
            area.name.enum_name, area.stage
        )?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    //
    writeln!(out, "pub fn possible_tod(&self) -> TimeOfDay {{")?;
    writeln!(out, "match self {{")?;

    for area in areas {
        if let Some(tod) = area.force_tod {
            writeln!(
                out,
                "Area::{} => TimeOfDay::{:?},",
                area.name.enum_name, tod
            )?;
        } else {
            writeln!(out, "Area::{} => TimeOfDay::Both,", area.name.enum_name)?;
        }
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    //
    writeln!(out, "pub fn can_sleep(&self) -> bool {{")?;
    writeln!(out, "match self {{")?;

    for area in areas {
        writeln!(out, "Area::{} => {},", area.name.enum_name, area.can_sleep)?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    //
    writeln!(out, "pub fn exits(&self) -> &'static [Exit] {{")?;
    writeln!(out, "match self {{")?;

    for area in areas {
        writeln!(out, "Area::{} => &[", area.name.enum_name)?;
        for exit in &area.exits {
            writeln!(out, "Exit::{},", exit)?;
        }
        writeln!(out, "],")?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    // //
    // writeln!(out, "pub fn exits(&self) -> impl Iterator<Item = Exit> + '_ {{")?;
    // writeln!(
    //     out,
    //     "self.exit_areas().iter().map(|other| Exit {{ from: *self, to: *other }})"
    // )?;
    // writeln!(out, "}}")?;

    //
    writeln!(out, "pub fn entrances(&self) -> &'static [Entrance] {{")?;
    writeln!(out, "match self {{")?;

    for area in areas {
        writeln!(out, "Area::{} => &[", area.name.enum_name)?;
        if let Some(entrances) = entrances.get(&area.name.enum_name) {
            for entrance in entrances.keys() {
                writeln!(out, "Entrance::{},", entrance)?;
            }
        }
        writeln!(out, "],")?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    // //
    // writeln!(out, "pub fn entrances(&self) -> impl Iterator<Item = Entrance> + '_ {{")?;
    // writeln!(
    //     out,
    //     "self.entrance_areas().iter().map(|area| Entrance {{ from: *area, to: *self }})"
    // )?;
    // writeln!(out, "}}")?;

    //
    writeln!(
        out,
        "pub fn logic_entrances(&self) -> &'static [(Area, RequirementKey)] {{"
    )?;
    writeln!(out, "match self {{")?;

    for area in areas {
        writeln!(out, "Area::{} => &[", area.name.enum_name)?;
        if let Some(entrance_areas) = logic_entrances.get(&area.name.enum_name) {
            for (entrance_area, req_key) in entrance_areas.iter() {
                writeln!(
                    out,
                    "(Area::{}, RequirementKey::{}),",
                    entrance_area, req_key
                )?;
            }
        }
        writeln!(out, "],")?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    //
    writeln!(
        out,
        "pub fn logic_exits(&self) -> &'static [(Area, RequirementKey)] {{"
    )?;
    writeln!(out, "match self {{")?;

    for area in areas {
        writeln!(out, "Area::{} => &[", area.name.enum_name)?;
        for (exit_area, req_key) in area.logic_exits.iter() {
            writeln!(out, "(Area::{}, RequirementKey::{}),", exit_area, req_key)?;
        }
        writeln!(out, "],")?;
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

    //
    writeln!(out, "pub fn requirement_key(&self) -> RequirementKey {{")?;
    writeln!(out, "match self {{")?;

    for location in locations {
        writeln!(
            out,
            "Location::{} => RequirementKey::{},",
            location.name.enum_name, location.name.enum_name
        )?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    writeln!(out, "}}")?;
    Ok(())
}

fn print_events<W: Write>(
    out: &mut W,
    events: &BTreeMap<NameAndEnumName, Vec<(String, RequirementExpression)>>,
) -> io::Result<()> {
    print_common(out, "Event", events.keys())?;

    writeln!(out, "impl Event {{")?;

    //
    writeln!(
        out,
        "pub fn requirements(&self) -> &'static[RequirementKey] {{"
    )?;
    writeln!(out, "match self {{")?;

    for (event_name, requirements) in events {
        writeln!(out, "Event::{} => &[", event_name.enum_name)?;
        for (requirement_name, _) in requirements {
            writeln!(out, "RequirementKey::{},", requirement_name)?;
        }
        writeln!(out, "],")?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    writeln!(out, "}}")?;
    Ok(())
}

fn print_exits<W: Write>(out: &mut W, exits: &[ExitEnumMember]) -> io::Result<()> {
    print_common(out, "Exit", exits.iter().map(|l| &l.name))?;

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
    writeln!(out, "pub fn vanilla_entrance(&self) -> Entrance {{")?;
    writeln!(out, "match self {{")?;

    for exit in exits {
        writeln!(
            out,
            "Exit::{} => Entrance::{},",
            exit.name.enum_name, exit.vanilla_entrance
        )?;
    }

    writeln!(out, "}}")?;
    writeln!(out, "}}")?;

    //
    writeln!(out, "pub fn requirement_key(&self) -> RequirementKey {{")?;
    writeln!(out, "match self {{")?;

    for exit in exits {
        writeln!(
            out,
            "Exit::{} => RequirementKey::{},",
            exit.name.enum_name, exit.name.enum_name
        )?;
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
    let reader = BufReader::new(File::open(format!("items.yaml"))?);
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

    let mut out = &mut File::create("../logic-core/src/generated/items.rs")?;
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

fn print_requirements<'a, W: Write>(
    out: &mut W,
    requirements: impl Iterator<Item = (&'a str, &'a RequirementExpression)> + Clone,
) -> io::Result<()> {
    writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
    writeln!(out, "pub enum RequirementKey {{")?;

    // print enum
    for (name, _) in requirements.clone() {
        writeln!(out, "{},", name)?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "impl RequirementKey {{")?;
    writeln!(out, "pub const ALL: &'static [RequirementKey] = &[")?;

    for (name, _) in requirements.clone() {
        writeln!(out, "RequirementKey::{},", name)?;
    }
    writeln!(out, "];")?;

    writeln!(out, "}}")?;

    Ok(())
}

fn print_base_requirements<'a, W: Write>(
    out: &mut W,
    requirements: impl Iterator<Item = (&'a str, &'a RequirementExpression)> + Clone,
) -> io::Result<()> {
    out.write_all(b"use std::collections::HashMap;\n")?;
    out.write_all(b"use super::{items::Item,logic::{Area, RequirementKey, Event}};\n")?;
    out.write_all(b"use crate::{logic_static::TimeOfDay,requirements::RequirementExpression};\n")?;

    out.write_all(
        b"pub fn get_requirements() -> HashMap<RequirementKey, RequirementExpression<'static>> {\n",
    )?;
    writeln!(out, "HashMap::from([")?;

    for (name, req) in requirements {
        write!(out, "(RequirementKey::{},", name)?;
        req.write_reqs(out)?;
        writeln!(out, "),")?;
    }
    writeln!(out, "])}}")?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let item_names = process_items()?;

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
    // exit_area_name: (entrance_area_name: requirement_key_name)
    let mut logic_entrances: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();

    let mut area_enums = Vec::new();
    let mut stage_enums = Vec::new();
    let mut region_enums = Vec::new();
    let mut location_enums = Vec::new();
    let mut exit_enums = Vec::new();
    let mut entrance_enums = Vec::new();
    let mut logic_exit_requirements = Vec::new();
    let mut events: BTreeMap<NameAndEnumName, Vec<(String, RequirementExpression)>> =
        BTreeMap::new();
    // let mut exit_enums = Vec::new();
    // println!("{faron:?}");
    for (region_name, region) in regions.iter() {
        let region_force_tod = region.force_tod.clone();
        // println!("{}: {:?}", region_name, region.force_tod);
        let region_name = NameAndEnumName::from_name(region_name);

        let mut stage_names = Vec::new();
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
                // (other_area_name, requirement_key_name)
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
                        "{}_To_{}",
                        area_name.enum_name,
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
                    logic_exits.push((other_area_name.clone(), requirement_name.clone()));
                    logic_entrances
                        .entry(other_area_name)
                        .or_default()
                        .insert(area_name.enum_name.clone(), requirement_name);
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

                        exit_enums.push(ExitEnumMember {
                            name: exit_name,
                            area: area_name.enum_name.clone(),
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
                    let area_local_event_name =
                        format!("{}_{}", area_name.enum_name, event_name.enum_name);
                    let resolved_req = RequirementExpression::parse(
                        event_req,
                        Some(&area_name.enum_name),
                        &[&macros, &local_macros],
                        &item_names,
                    )
                    .unwrap();
                    events.entry(event_name).or_default().push((
                        area_local_event_name,
                        RequirementExpression::And(vec![
                            RequirementExpression::Area(
                                area_name.enum_name.clone(),
                                RequirementToD::Any,
                            ),
                            resolved_req,
                        ]),
                    ));
                }

                // area_names.push(area_enum_name_with_stage.clone());
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
            stage_names.push(stage_name.enum_name.clone());

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
            stages: stage_names,
            force_tod: region_force_tod,
        });
    }

    let mut out = File::create("../logic-core/src/generated/logic.rs")?;
    writeln!(&mut out, "#![allow(non_camel_case_types)]")?;
    writeln!(
        &mut out,
        "use crate::logic_static::{{TimeOfDay, BitSetCompatible}};"
    )?;

    print_regions(&mut out, &region_enums)?;
    print_stages(&mut out, &stage_enums)?;
    print_areas(&mut out, &area_enums, &entrances, &logic_entrances)?;
    print_locations(&mut out, &location_enums)?;
    print_events(&mut out, &events)?;

    // make an iterator of (name, requirement) out of everything that needs logical requirements
    let requirements = logic_exit_requirements
        .iter()
        .map(|(s, r)| (s.as_str(), r))
        .chain(
            location_enums
                .iter()
                .map(|l| (l.name.enum_name.as_str(), &l.requirement)),
        )
        .chain(
            exit_enums
                .iter()
                .map(|e| (e.name.enum_name.as_str(), &e.requirement)),
        )
        .chain(events.values().flatten().map(|(s, r)| (s.as_str(), r)));
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
    print_requirements(&mut out, requirements.clone())?;
    let mut out_requirements =
        File::create("../logic-core/src/generated/generated_requirements.rs")?;
    print_base_requirements(&mut out_requirements, requirements)?;
    print_exits(&mut out, &exit_enums)?;
    print_entrances(&mut out, &entrance_enums)?;

    Ok(())
}
