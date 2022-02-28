#![feature(str_split_once)]
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

use crate::inventory::Inventory;
use crate::items::Item;
use crate::RandomizerOptions;

#[derive(Deserialize)]
pub struct LogicLocationEntryYaml {
    #[serde(rename(deserialize = "Need"))]
    need: String,
    #[serde(rename(deserialize = "original item"))]
    original_item: String,
}

#[derive(Debug)]
pub enum LogicElement {
    AndExpression(Vec<LogicElement>),
    OrExpression(Vec<LogicElement>),
    MacroRequirement(String),
    FixedValue(bool),
    SingleItem(Item),
    CountableItem(Item, usize),
    OptionEnabledRequirement(String),
    OptionDisabledRequirement(String),
    OptionIsRequirement(String, String),
    OptionIsNotRequirement(String, String),
    OptionContainsRequirement(String, String),
    OptionDoesNotContainRequirement(String, String),
}

fn find_closing_parenthesis(s: &str, start_index: usize) -> Option<usize> {
    // assert that we start with the parenthesis already open
    let mut par_depth = 1;
    for (idx, c) in s.chars().enumerate().skip(start_index) {
        match c {
            '(' => par_depth += 1,
            ')' => {
                par_depth -= 1;
                if par_depth == 0 {
                    return Some(idx);
                }
            }
            _ => {}
        }
    }
    None
}

#[derive(PartialEq)]
enum LogicType {
    None,
    Or,
    And,
}

impl LogicElement {
    // Can Access Skyview & (Practice Sword | (Water Scale & Bomb Bag)) & (Slingshot | Clawshots | Beetle | Bow) & SV Small Key x1
    pub fn parse(string: &str) -> Result<Self, Box<dyn Error>> {
        let mut logic_type = LogicType::None;
        let mut current_rest = string;
        let mut current_level = Vec::new();
        let split_chars: &[_] = &['&', '|'];
        // until exhausted
        while current_rest != "" {
            if current_rest.starts_with('(') {
                // handle nested logic expression
                let closing_p = find_closing_parenthesis(current_rest, 1).ok_or_else(|| {
                    format!("Closing parenthesis not found on string '{}'", current_rest)
                })?;
                current_level.push(LogicElement::parse(&current_rest[1..closing_p])?);
                current_rest = current_rest[closing_p + 1..].trim();
                continue;
            }
            // find either '&' or '|', only the same type can be on one level
            if let Some(pos) = current_rest.find(split_chars) {
                match &current_rest[pos..=pos] {
                    "&" => {
                        if logic_type == LogicType::Or {
                            return Err("& and | mixed!".into());
                        }
                        logic_type = LogicType::And;
                    }
                    "|" => {
                        if logic_type == LogicType::And {
                            return Err("& and | mixed!".into());
                        }
                        logic_type = LogicType::Or;
                    }
                    _ => unreachable!(),
                };
                // add requirement to list
                let requirement = current_rest[..pos].trim();
                if requirement != "" {
                    current_level.push(LogicElement::parse_base_requirement(requirement)?);
                }
                current_rest = current_rest[pos + 1..].trim();
            } else {
                // if they can't be found, the rest is one base expression and we're done
                if current_rest != "" {
                    current_level.push(LogicElement::parse_base_requirement(current_rest.trim())?);
                }
                break;
            }
        }
        if current_level.len() == 1 {
            return Ok(current_level.remove(0));
        }
        match logic_type {
            LogicType::And => {
                return Ok(LogicElement::AndExpression(current_level));
            }
            LogicType::Or => {
                return Ok(LogicElement::OrExpression(current_level));
            }
            _ => {
                return Err(
                    format!("no logic type, that should not be possible: {}", string).into(),
                )
            }
        }
    }

    fn parse_base_requirement(s: &str) -> Result<Self, Box<dyn Error>> {
        if s == "Nothing" {
            return Ok(LogicElement::FixedValue(true));
        }
        if s == "Impossible" {
            return Ok(LogicElement::FixedValue(false));
        }
        // simple item requirement
        if let Some(item) = Item::try_from(s).ok() {
            return Ok(LogicElement::SingleItem(item));
        }
        // item count requirement
        if let Some((item_part, count_part)) = s.rsplit_once(" ") {
            if count_part.starts_with("x") {
                if let Ok(count) = count_part[1..].parse::<usize>() {
                    if let Ok(item) = Item::try_from(item_part) {
                        return Ok(LogicElement::CountableItem(item, count));
                    }
                }
            }
        }
        // option requirements
        if s.starts_with("Option \"") {
            if s.ends_with("\" Enabled") {
                return Ok(LogicElement::OptionEnabledRequirement(
                    s["Option \"".len()..s.len() - "\" Enabled".len()].to_string(),
                ));
            }
            if s.ends_with("\" Disabled") {
                return Ok(LogicElement::OptionDisabledRequirement(
                    s["Option \"".len()..s.len() - "\" Disabled".len()].to_string(),
                ));
            }
            if s.ends_with('"') {
                match s.find("\" Is \"") {
                    Some(pos) => {
                        let option_name = s["Option \"".len()..pos].to_string();
                        let option_value = s[pos + "\" Is \"".len()..s.len() - 1].to_string();
                        return Ok(LogicElement::OptionIsRequirement(option_name, option_value));
                    }
                    None => {}
                }
                match s.find("\" Is Not \"") {
                    Some(pos) => {
                        let option_name = s["Option \"".len()..pos].to_string();
                        let option_value = s[pos + "\" Is Not \"".len()..s.len() - 1].to_string();
                        return Ok(LogicElement::OptionIsNotRequirement(
                            option_name,
                            option_value,
                        ));
                    }
                    None => {}
                }
                match s.find("\" Contains \"") {
                    Some(pos) => {
                        let option_name = s["Option \"".len()..pos].to_string();
                        let option_value = s[pos + "\" Contains \"".len()..s.len() - 1].to_string();
                        return Ok(LogicElement::OptionContainsRequirement(
                            option_name,
                            option_value,
                        ));
                    }
                    None => {}
                }
                match s.find("\" Does Not Contain \"") {
                    Some(pos) => {
                        let option_name = s["Option \"".len()..pos].to_string();
                        let option_value =
                            s[pos + "\" Does Not Contain \"".len()..s.len() - 1].to_string();
                        return Ok(LogicElement::OptionDoesNotContainRequirement(
                            option_name,
                            option_value,
                        ));
                    }
                    None => {}
                }
            }
        }
        return Ok(LogicElement::MacroRequirement(s.to_string()));
    }

    pub fn get_all_base_reqs(&self, all_base_reqs: &mut HashSet<String>) {
        match self {
            LogicElement::MacroRequirement(s) => {
                all_base_reqs.insert(s.clone());
            }
            LogicElement::OrExpression(exprs) | LogicElement::AndExpression(exprs) => {
                for expr in exprs {
                    expr.get_all_base_reqs(all_base_reqs);
                }
            }
            _ => {}
        };
    }

    pub fn check_requirement_met(
        &self,
        options: &RandomizerOptions,
        inventory: &Inventory,
        macros: &HashMap<String, LogicElement>,
        generated_macros: &HashMap<String, LogicElement>,
    ) -> bool {
        match self {
            LogicElement::FixedValue(val) => {
                return *val;
            }
            LogicElement::SingleItem(item) => {
                return inventory.has_item(*item);
            }
            LogicElement::CountableItem(item, req_count) => {
                return inventory.get_item_count(*item) >= *req_count;
            }
            LogicElement::OptionEnabledRequirement(opt) => {
                return options.get_option_enabled(&opt).unwrap();
            }
            LogicElement::OptionDisabledRequirement(opt) => {
                return !options.get_option_enabled(&opt).unwrap();
            }
            LogicElement::OptionIsRequirement(opt, val) => {
                return options.get_option_choice_str(&opt).unwrap() == val;
            }
            LogicElement::OptionIsNotRequirement(opt, val) => {
                return options.get_option_choice_str(&opt).unwrap() != val;
            }
            LogicElement::OptionContainsRequirement(opt, val) => {
                return options.get_option_choices(&opt).unwrap().contains(val);
            }
            LogicElement::OptionDoesNotContainRequirement(opt, val) => {
                return !options.get_option_choices(&opt).unwrap().contains(val);
            }
            LogicElement::OrExpression(requirements) => {
                // compute all for test
                return requirements.iter().fold(false, |last, cur| {
                    last || cur.check_requirement_met(options, inventory, macros, generated_macros)
                });
                // return requirements.iter().any(|cur| cur.check_requirement_met(options, inventory));
            }
            LogicElement::AndExpression(requirements) => {
                // compute all for test
                return requirements.iter().fold(true, |last, cur| {
                    last && cur.check_requirement_met(options, inventory, macros, generated_macros)
                });
                // return requirements.iter().all(|cur| cur.check_requirement_met(options, inventory));
            }
            LogicElement::MacroRequirement(macr) => {
                match macros.get(macr).or_else(|| generated_macros.get(macr)) {
                    Some(req) => {
                        return req.check_requirement_met(
                            options,
                            inventory,
                            macros,
                            generated_macros,
                        );
                    }
                    None => {
                        println!("macro not found: {}", macr);
                        return false;
                    }
                }
            }
        };
    }
}

pub struct LogicFiles {
    pub location_names: Vec<String>,
    pub requirement_map: HashMap<String, LogicElement>,
}

impl LogicFiles {
    pub fn read_logic() -> Result<Self, Box<dyn Error>> {
        let f = File::open("SS Rando Logic - Item Location.yaml")?;
        // read to mapping first, to preserve order
        let result: Mapping = serde_yaml::from_reader(f)?;
        let mut ordered_location_list = Vec::with_capacity(result.len());
        let mut locs_and_macros: HashMap<String, LogicElement> =
            HashMap::with_capacity(result.len());
        for (key, value) in result.iter() {
            let loc_name = key.as_str().unwrap().to_string();
            ordered_location_list.push(loc_name.clone());
            let loc_need = value.get("Need").unwrap().as_str().unwrap().to_string();
            locs_and_macros.insert(loc_name, LogicElement::parse(&loc_need)?);
        }
        let f = File::open("SS Rando Logic - Macros.yaml")?;
        let mut macros: HashMap<String, String> = serde_yaml::from_reader(f)?;
        locs_and_macros.reserve(macros.len());
        for (macro_name, macro_str) in macros.drain() {
            locs_and_macros.insert(macro_name, LogicElement::parse(&macro_str)?);
        }
        Ok(LogicFiles {
            location_names: ordered_location_list,
            requirement_map: locs_and_macros,
        })
    }
}
