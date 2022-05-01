use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Write;

use crate::options::RandomizerOptions;

use super::logic_structs::{Inventory, LogicKey, LogicKeyMapper, TimeOfDay};

#[derive(Debug, Clone)]
pub enum LogicElement {
    AndExpression(Vec<LogicElement>),
    OrExpression(Vec<LogicElement>),
    LogicKeyRequirement(LogicKey),
    LogicKeyCountRequirement(LogicKey, usize),
    FixedValue(bool),
    TimeOfDay(TimeOfDay),
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
    pub fn parse(
        string: &str,
        macros: &HashMap<String, LogicElement>,
        logic_key_mapper: &mut LogicKeyMapper,
    ) -> Result<Self, Box<dyn Error>> {
        let mut logic_type = LogicType::None;
        let mut current_rest = string.trim();
        let mut current_level = Vec::new();
        let split_chars: &[_] = &['&', '|'];
        // until exhausted
        while current_rest != "" {
            if current_rest.starts_with('(') {
                // handle nested logic expression
                let closing_p = find_closing_parenthesis(current_rest, 1).ok_or_else(|| {
                    format!("Closing parenthesis not found on string '{}'", current_rest)
                })?;
                current_level.push(LogicElement::parse(
                    &current_rest[1..closing_p],
                    macros,
                    logic_key_mapper,
                )?);
                current_rest = current_rest[closing_p + 1..].trim();
                continue;
            }
            // find either '&' or '|', only the same type can be on one level
            if let Some(pos) = current_rest.find(split_chars) {
                match &current_rest[pos..=pos] {
                    "&" => {
                        if logic_type == LogicType::Or {
                            return Err(format!("& and | mixed for {}", string).into());
                        }
                        logic_type = LogicType::And;
                    }
                    "|" => {
                        if logic_type == LogicType::And {
                            return Err(format!("& and | mixed for {}", string).into());
                        }
                        logic_type = LogicType::Or;
                    }
                    _ => unreachable!(),
                };
                // add requirement to list
                let requirement = current_rest[..pos].trim();
                if requirement != "" {
                    current_level.push(LogicElement::parse_base_requirement(
                        requirement,
                        macros,
                        logic_key_mapper,
                    )?);
                }
                current_rest = current_rest[pos + 1..].trim();
            } else {
                // if they can't be found, the rest is one base expression and we're done
                if current_rest != "" {
                    current_level.push(LogicElement::parse_base_requirement(
                        current_rest.trim(),
                        macros,
                        logic_key_mapper,
                    )?);
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

    fn parse_base_requirement(
        s: &str,
        macros: &HashMap<String, LogicElement>,
        logic_key_mapper: &mut LogicKeyMapper,
    ) -> Result<Self, Box<dyn Error>> {
        if s == "Nothing" {
            return Ok(LogicElement::FixedValue(true));
        }
        if s == "Impossible" {
            return Ok(LogicElement::FixedValue(false));
        }
        if s == "Daytime" {
            return Ok(LogicElement::TimeOfDay(TimeOfDay::Day));
        }
        if s == "Nighttime" {
            return Ok(LogicElement::TimeOfDay(TimeOfDay::Night));
        }
        // macros
        if let Some(elem) = macros.get(s) {
            return Ok(elem.clone());
        }
        // item count requirement
        if let Some((item_part, count_part)) = s.rsplit_once(" ") {
            if count_part.starts_with("x") {
                if let Ok(count) = count_part[1..].parse::<usize>() {
                    let key = logic_key_mapper.convert_to_num(item_part);
                    return Ok(LogicElement::LogicKeyCountRequirement(key, count));
                }
            }
        }
        // trick requirements
        if s.ends_with(" Trick") {
            return Ok(LogicElement::OptionContainsRequirement(
                "enabled-tricks-bitless".to_string(), // TODO: glitched and er tricks
                s[..s.len() - " Trick".len()].to_string(),
            ));
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
        return Ok(LogicElement::LogicKeyRequirement(
            logic_key_mapper.convert_to_num(s),
        ));
    }

    pub fn get_all_base_reqs(&self, all_base_reqs: &mut HashSet<LogicKey>) {
        match self {
            LogicElement::LogicKeyRequirement(key)
            | LogicElement::LogicKeyCountRequirement(key, _) => {
                all_base_reqs.insert(key.clone());
            }
            LogicElement::OrExpression(exprs) | LogicElement::AndExpression(exprs) => {
                for expr in exprs {
                    expr.get_all_base_reqs(all_base_reqs);
                }
            }
            _ => {}
        };
    }

    pub fn specialize_for_options_macros(&mut self, options: &RandomizerOptions) {
        match self {
            LogicElement::OptionEnabledRequirement(opt) => {
                *self = LogicElement::FixedValue(options.get_option_enabled(&opt).expect(opt));
            }
            LogicElement::OptionDisabledRequirement(opt) => {
                *self = LogicElement::FixedValue(!options.get_option_enabled(&opt).expect(opt));
            }
            LogicElement::OptionIsRequirement(opt, val) => {
                *self = LogicElement::FixedValue(
                    options.get_option_choice_str(&opt).expect(opt) == val,
                );
            }
            LogicElement::OptionIsNotRequirement(opt, val) => {
                *self = LogicElement::FixedValue(
                    options.get_option_choice_str(&opt).expect(opt) != val,
                );
            }
            LogicElement::OptionContainsRequirement(opt, val) => {
                *self = LogicElement::FixedValue(
                    options.get_option_choices(&opt).expect(opt).contains(val),
                );
            }
            LogicElement::OptionDoesNotContainRequirement(opt, val) => {
                *self = LogicElement::FixedValue(
                    !options.get_option_choices(&opt).expect(opt).contains(val),
                );
            }
            LogicElement::AndExpression(elems) => {
                for elem in elems.iter_mut() {
                    elem.specialize_for_options_macros(options);
                }
                // if any value is false, the entire expression is false
                if elems
                    .iter()
                    .any(|e| matches!(e, &LogicElement::FixedValue(false)))
                {
                    *self = LogicElement::FixedValue(false);
                } else {
                    // remove all values that are true
                    elems.retain(|e| !matches!(e, &LogicElement::FixedValue(true)));
                    // if there aren't any elements anymore, the entire expression is true
                    if elems.len() == 0 {
                        *self = LogicElement::FixedValue(true);
                    } else if elems.len() == 1 {
                        *self = elems.remove(0);
                    }
                }
            }
            LogicElement::OrExpression(elems) => {
                for elem in elems.iter_mut() {
                    elem.specialize_for_options_macros(options);
                }
                // if any value is true, the entire expression is true
                if elems
                    .iter()
                    .any(|e| matches!(e, &LogicElement::FixedValue(true)))
                {
                    *self = LogicElement::FixedValue(true);
                } else {
                    // remove all values that are false
                    elems.retain(|e| !matches!(e, &LogicElement::FixedValue(false)));
                    // if there aren't any elements anymore, the entire expression is false
                    if elems.len() == 0 {
                        *self = LogicElement::FixedValue(false);
                    } else if elems.len() == 1 {
                        *self = elems.remove(0);
                    }
                }
            }
            _ => (),
        };
    }

    pub fn check_requirement_met(&self, inventory: &Inventory, tod: &TimeOfDay) -> bool {
        match self {
            LogicElement::FixedValue(val) => {
                return *val;
            }
            LogicElement::TimeOfDay(this_tod) => {
                return tod == this_tod;
            }
            LogicElement::LogicKeyCountRequirement(key, count) => {
                return inventory.check_item_count(key, *count);
            }
            LogicElement::LogicKeyRequirement(key) => {
                return inventory.check_owned_item(key);
            }
            LogicElement::OrExpression(requirements) => {
                // compute all for test
                return requirements.iter().fold(false, |last, cur| {
                    last || cur.check_requirement_met(inventory, tod)
                });
                // return requirements.iter().any(|cur| cur.check_requirement_met(options, inventory));
            }
            LogicElement::AndExpression(requirements) => {
                // compute all for test
                return requirements.iter().fold(true, |last, cur| {
                    last && cur.check_requirement_met(inventory, tod)
                });
                // return requirements.iter().all(|cur| cur.check_requirement_met(options, inventory));
            }
            _ => panic!("{:?} is not properly simplified!", self),
        };
    }

    pub fn dbg_to_string(&self, s: &mut String, mapper: &LogicKeyMapper) {
        match self {
            LogicElement::LogicKeyCountRequirement(key, count) => {
                write!(
                    s,
                    "LogicKeyCountRequirement({}, {})",
                    mapper.convert_to_string(key).unwrap(),
                    count
                )
                .unwrap();
            }
            LogicElement::LogicKeyRequirement(key) => {
                write!(
                    s,
                    "LogicKeyRequirement({})",
                    mapper.convert_to_string(key).unwrap()
                )
                .unwrap();
            }
            LogicElement::AndExpression(requirements) => {
                write!(s, "AndExpression([").unwrap();
                for req in requirements {
                    req.dbg_to_string(s, mapper);
                    s.push(',');
                }
                s.pop();
                write!(s, "])").unwrap();
            }
            LogicElement::OrExpression(requirements) => {
                write!(s, "OrExpression([").unwrap();
                for req in requirements {
                    req.dbg_to_string(s, mapper);
                    s.push(',');
                }
                s.pop();
                write!(s, "])").unwrap();
            }
            _ => write!(s, "{:?}", self).unwrap(),
        };
    }
}
