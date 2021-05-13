#![feature(str_split_once)]
use std::error::Error;
use std::io::prelude::*;
use std::io::{BufReader};
use std::fs::File;
use std::collections::{HashMap, BTreeMap, HashSet};
use std::convert::TryFrom;

use serde::{Serialize, Deserialize};
use serde_yaml::Mapping;

use crate::items::Item;


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
    BaseRequirement(String),
    SingleItem(Item),
    CountableItem(Item, usize),
    OptionEnabledRequirement(String),
    OptionDisabledRequirement(String),
    OptionValueRequirement(String),
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
            },
            _ => {},
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
        let mut logicType = LogicType::None;
        let mut current_rest = string;
        let mut current_level = Vec::new();
        let split_chars: &[_] = &['&','|'];
        // until exhausted
        while current_rest != "" {
            if current_rest.starts_with('(') {
                // handle nested logic expression
                let closing_p = find_closing_parenthesis(current_rest, 1)
                    .ok_or_else(|| format!("Closing parenthesis not found on string '{}'", current_rest))?;
                current_level.push(LogicElement::parse(&current_rest[1..closing_p])?);
                current_rest =  current_rest[closing_p+1..].trim();
                continue;
            }
            // find either '&' or '|', only the same type can be on one level
            if let Some(pos) = current_rest.find(split_chars) {
                match &current_rest[pos..=pos] {
                    "&" => {
                        if logicType == LogicType::Or {
                            return Err("& and | mixed!".into());
                        }
                        logicType = LogicType::And;
                    },
                    "|" => {
                        if logicType == LogicType::And {
                            return Err("& and | mixed!".into());
                        }
                        logicType = LogicType::Or;
                    },
                    _ => unreachable!(),
                };
                // add requirement to list
                let requirement = current_rest[..pos].trim();
                if requirement != "" {
                    current_level.push(LogicElement::parse_base_requirement(requirement)?);
                }
                current_rest = current_rest[pos+1..].trim();
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
        match logicType {
            LogicType::And => {
                return Ok(LogicElement::AndExpression(current_level));
            },
            LogicType::Or => {
                return Ok(LogicElement::OrExpression(current_level));
            },
            _ => return Err(format!("no logic type, that should not be possible: {}", string).into()),
        }
    }

    fn parse_base_requirement(s: &str) -> Result<Self, Box<dyn Error>> {
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
        if s.starts_with("Option \"") && s.ends_with("\" Enabled") {
            return Ok(LogicElement::OptionEnabledRequirement(s["Option \"".len()..s.len()-"\" Enabled".len()].to_string()))
        }
        return Ok(LogicElement::BaseRequirement(s.to_string()));
    }

    pub fn get_all_base_reqs(&self, all_base_reqs: &mut HashSet<String>) {
        match self {
            LogicElement::BaseRequirement(s) => {
                all_base_reqs.insert(s.clone());
            },
            LogicElement::OrExpression(exprs) | LogicElement::AndExpression(exprs) => {
                for expr in exprs {
                    expr.get_all_base_reqs(all_base_reqs);
                }
            }
            _ => {},
        };
    }
}

pub struct LogicFiles {
    pub location_names: Vec<String>,
    pub requirement_map: HashMap<String, LogicElement>,
}

impl LogicFiles {
    pub fn read_logic() -> Result<Self, Box<dyn Error>>{
        let f = File::open("SS Rando Logic - Item Location.yaml")?;
        // read to mapping first, to preserve order
        let result: Mapping = serde_yaml::from_reader(f)?;
        let mut ordered_location_list = Vec::with_capacity(result.len());
        let mut locs_and_macros: HashMap<String, LogicElement> = HashMap::with_capacity(result.len());
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