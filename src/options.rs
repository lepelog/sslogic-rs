use std::fs::File;
use std::convert::TryInto;
use std::error::Error;
use std::collections::HashSet;
use serde_yaml::{Mapping, Value};

pub struct RandomizerOptions {
    options: Vec<RandomizerOption>,
}

#[derive(Debug)]
pub struct RandomizerOption {
    name: String,
    command: String,
    inner: SpecialOptions,
    help: String,
}

#[derive(Debug)]
pub enum SpecialOptions {
    Boolean{value: bool, default: bool},
    UInt{value: usize, default: usize, bits: u8, min: Option<usize>, max: Option<usize>},
    Multichoice{value: HashSet<String>, default: HashSet<String>, all_values: Vec<String>},
    Singlechoice{value: String, default: String, all_values: Vec<String>, bits: u8},
    SinglechoiceUInt{value: usize, default: usize, all_values: Vec<usize>, bits: u8},
}

fn get_str_key_from_yaml<'a>(mapping: &'a Mapping, key: &str) -> Result<&'a Value, Box<dyn Error>> {
    Ok(mapping.get(&Value::String(key.into())).ok_or_else(|| format!("{} did not exist!", key))?)
}

fn get_str_key_as_str_from_yaml(mapping: &Mapping, key: &str) -> Result<String, Box<dyn Error>> {
    Ok(get_str_key_from_yaml(mapping, key)?.as_str().unwrap().to_string())
}

impl RandomizerOptions {
    pub fn parse_option_file() -> Result<Self, Box<dyn Error>> {
        let mut result = Vec::new();
        let f = File::open("options.yaml")?;
        let mapped: Vec<Mapping> = serde_yaml::from_reader(f)?;
        for opt in mapped {
            let name = get_str_key_as_str_from_yaml(&opt, "name")?;
            let command = get_str_key_as_str_from_yaml(&opt, "command")?;
            let help = get_str_key_as_str_from_yaml(&opt, "help")?;
            let option_type = get_str_key_as_str_from_yaml(&opt, "type")?;
            let special_opts = match option_type.as_str() {
                "boolean" => {
                    let default = get_str_key_from_yaml(&opt, "default")?.as_bool().unwrap_or(false);
                    SpecialOptions::Boolean{default, value: default}
                },
                "int" => {
                    if name == "Seed" {
                        continue;
                    }
                    let default = get_str_key_from_yaml(&opt, "default")?.as_u64()
                        .and_then(|v| v.try_into().ok()).ok_or_else(|| format!("default value of {} is not valid!", name))?;
                    let bits = get_str_key_from_yaml(&opt, "bits")?.as_u64()
                        .and_then(|v| v.try_into().ok()).ok_or_else(|| format!("bits value of {} is not valid!", name))?;
                    // field is optional
                    let min = get_str_key_from_yaml(&opt, "min").ok().and_then(|m| m.as_u64()).and_then(|v| v.try_into().ok());
                    let max = get_str_key_from_yaml(&opt, "max").ok().and_then(|m| m.as_u64()).and_then(|v| v.try_into().ok());
                    SpecialOptions::UInt{
                        default,
                        value: default,
                        bits,
                        min,
                        max,
                    }
                },
                "multichoice" => {
                    let default: HashSet<_> = get_str_key_from_yaml(&opt, "default")?
                        .as_sequence().unwrap().iter().map(|val| val.as_str().unwrap().to_string()).collect();
                    let choices = get_str_key_from_yaml(&opt, "choices")?
                        .as_sequence().unwrap().iter().map(|val| val.as_str().unwrap().to_string()).collect();
                    SpecialOptions::Multichoice {
                        default: default.clone(),
                        value: default,
                        all_values: choices,
                    }
                },
                "singlechoice" => {
                    let default_value = get_str_key_from_yaml(&opt, "default")?;
                    let bits = get_str_key_from_yaml(&opt, "bits")?.as_u64()
                        .and_then(|v| v.try_into().ok()).ok_or_else(|| format!("bits value of {} is not valid!", name))?;
                    if default_value.is_u64() {
                        let default = default_value.as_u64().unwrap().try_into()?;
                        let choices = get_str_key_from_yaml(&opt, "choices")?
                            .as_sequence().unwrap().iter().map(|val| val.as_u64().unwrap().try_into().unwrap()).collect();
                        SpecialOptions::SinglechoiceUInt {
                            default,
                            value: default,
                            all_values: choices,
                            bits,
                        }
                    } else {
                        let default = default_value.as_str().unwrap().to_string();
                        let choices = get_str_key_from_yaml(&opt, "choices")?
                            .as_sequence().unwrap().iter().map(|val| val.as_str().unwrap().to_string()).collect();
                        SpecialOptions::Singlechoice {
                            default: default.clone(),
                            value: default,
                            all_values: choices,
                            bits,
                        }
                    }
                },
                _ => {
                    println!("unknown opt type: {}", name);
                    continue;
                }
            };
            result.push(RandomizerOption {
                name,
                command,
                help,
                inner: special_opts,
            });
        }
        return Ok(RandomizerOptions {
            options: result
        });
    }

    pub fn get_options(&self) -> &Vec<RandomizerOption> {
        &self.options
    }

    // checks, if a specific option has been enabled
    // returns None, if the option can't be found or is not a boolean option
    pub fn get_option_enabled(&self, option_name: &str) -> Option<bool> {
        for opt in self.options.iter() {
            match opt.inner {
                SpecialOptions::Boolean{value, ..} => {
                    if opt.command == option_name {
                        return Some(value);
                    }
                },
                _ => {}
            }
        }
        None
    }

    // return the uint value of an option, if that option exists
    pub fn get_option_uint(&self, option_name: &str) -> Option<usize> {
        for opt in self.options.iter() {
            match opt.inner {
                SpecialOptions::UInt{value, ..} => {
                    if opt.command == option_name {
                        return Some(value);
                    }
                },
                _ => {}
            }
        }
        None
    }

    // returns the choice str for an option, if that option exists
    pub fn get_option_choice_str(&self, option_name: &str) -> Option<&String> {
        for opt in self.options.iter() {
            match &opt.inner {
                SpecialOptions::Singlechoice{value, ..} => {
                    if opt.command == option_name {
                        return Some(&value);
                    }
                },
                _ => {}
            }
        }
        None
    }

    pub fn get_option_choice_uint(&self, option_name: &str) -> Option<usize> {
        for opt in self.options.iter() {
            match opt.inner {
                SpecialOptions::SinglechoiceUInt{value, ..} => {
                    if opt.command == option_name {
                        return Some(value);
                    }
                },
                _ => {}
            }
        }
        None
    }

    pub fn get_option_choices(&self, option_name: &str) -> Option<&HashSet<String>> {
        for opt in self.options.iter() {
            match &opt.inner {
                SpecialOptions::Multichoice{value, ..} => {
                    if opt.command == option_name {
                        return Some(value);
                    }
                },
                _ => {}
            }
        }
        None
    }
}