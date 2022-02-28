use crate::packedbits::{PackedBitsReader, PackedBitsWriter};
use serde_yaml::{Mapping, Value};
use std::collections::HashSet;
use std::convert::TryInto;
use std::error::Error;
use std::fmt::Write;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct RandomizerOptions {
    pub seed: Option<usize>,
    options: Vec<RandomizerOption>,
}

#[derive(Debug, Clone)]
pub struct RandomizerOption {
    pub name: String,
    pub command: String,
    pub permalink: bool,
    pub inner: SpecialOptions,
    pub help: String,
}

#[derive(Debug, Clone)]
pub enum SpecialOptions {
    Boolean {
        value: bool,
        default: bool,
    },
    UInt {
        value: usize,
        default: usize,
        bits: u8,
        min: Option<usize>,
        max: Option<usize>,
    },
    Multichoice {
        value: HashSet<String>,
        default: HashSet<String>,
        all_values: Vec<String>,
    },
    Singlechoice {
        value: String,
        default: String,
        all_values: Vec<String>,
        bits: u8,
    },
    SinglechoiceUInt {
        value: usize,
        default: usize,
        all_values: Vec<usize>,
        bits: u8,
    },
}

#[derive(Debug)]
pub enum SetOptionError {
    DoesNotExist,
    WrongOptionType,
    NotInAvailableChoices,
}

fn get_str_key_from_yaml<'a>(mapping: &'a Mapping, key: &str) -> Result<&'a Value, Box<dyn Error>> {
    Ok(mapping
        .get(&Value::String(key.into()))
        .ok_or_else(|| format!("{} did not exist!", key))?)
}

fn get_str_key_as_str_from_yaml(mapping: &Mapping, key: &str) -> Result<String, Box<dyn Error>> {
    Ok(get_str_key_from_yaml(mapping, key)?
        .as_str()
        .unwrap()
        .to_string())
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
            let is_permalink = get_str_key_from_yaml(&opt, "permalink")
                .ok()
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            let special_opts = match option_type.as_str() {
                "boolean" => {
                    let default = get_str_key_from_yaml(&opt, "default")?
                        .as_bool()
                        .unwrap_or(false);
                    SpecialOptions::Boolean {
                        default,
                        value: default,
                    }
                }
                "int" => {
                    if name == "Seed" {
                        continue;
                    }
                    let default = get_str_key_from_yaml(&opt, "default")?
                        .as_u64()
                        .and_then(|v| v.try_into().ok())
                        .ok_or_else(|| format!("default value of {} is not valid!", name))?;
                    let bits = get_str_key_from_yaml(&opt, "bits")?
                        .as_u64()
                        .and_then(|v| v.try_into().ok())
                        .ok_or_else(|| format!("bits value of {} is not valid!", name))?;
                    // field is optional
                    let min = get_str_key_from_yaml(&opt, "min")
                        .ok()
                        .and_then(|m| m.as_u64())
                        .and_then(|v| v.try_into().ok());
                    let max = get_str_key_from_yaml(&opt, "max")
                        .ok()
                        .and_then(|m| m.as_u64())
                        .and_then(|v| v.try_into().ok());
                    SpecialOptions::UInt {
                        default,
                        value: default,
                        bits,
                        min,
                        max,
                    }
                }
                "multichoice" => {
                    let default: HashSet<_> = get_str_key_from_yaml(&opt, "default")?
                        .as_sequence()
                        .unwrap()
                        .iter()
                        .map(|val| val.as_str().unwrap().to_string())
                        .collect();
                    let choices = get_str_key_from_yaml(&opt, "choices")?
                        .as_sequence()
                        .unwrap()
                        .iter()
                        .map(|val| val.as_str().unwrap().to_string())
                        .collect();
                    SpecialOptions::Multichoice {
                        default: default.clone(),
                        value: default,
                        all_values: choices,
                    }
                }
                "singlechoice" => {
                    let default_value = get_str_key_from_yaml(&opt, "default")?;
                    let bits = get_str_key_from_yaml(&opt, "bits")?
                        .as_u64()
                        .and_then(|v| v.try_into().ok())
                        .ok_or_else(|| format!("bits value of {} is not valid!", name))?;
                    if default_value.is_u64() {
                        let default = default_value.as_u64().unwrap().try_into()?;
                        let choices = get_str_key_from_yaml(&opt, "choices")?
                            .as_sequence()
                            .unwrap()
                            .iter()
                            .map(|val| val.as_u64().unwrap().try_into().unwrap())
                            .collect();
                        SpecialOptions::SinglechoiceUInt {
                            default,
                            value: default,
                            all_values: choices,
                            bits,
                        }
                    } else {
                        let default = default_value.as_str().unwrap().to_string();
                        let choices = get_str_key_from_yaml(&opt, "choices")?
                            .as_sequence()
                            .unwrap()
                            .iter()
                            .map(|val| val.as_str().unwrap().to_string())
                            .collect();
                        SpecialOptions::Singlechoice {
                            default: default.clone(),
                            value: default,
                            all_values: choices,
                            bits,
                        }
                    }
                }
                _ => {
                    println!("unknown opt type: {}", name);
                    continue;
                }
            };
            result.push(RandomizerOption {
                name,
                command,
                permalink: is_permalink,
                help,
                inner: special_opts,
            });
        }
        return Ok(RandomizerOptions {
            seed: None,
            options: result,
        });
    }

    pub fn get_options(&self) -> &Vec<RandomizerOption> {
        &self.options
    }

    pub fn to_permalink(&self) -> String {
        let mut writer = PackedBitsWriter::new();
        for option in self.options.iter() {
            if !option.permalink {
                continue;
            }
            match &option.inner {
                SpecialOptions::Boolean { value, .. } => {
                    writer.write(if *value { 1 } else { 0 }, 1);
                }
                SpecialOptions::UInt { value, bits, .. } => {
                    writer.write(*value, *bits);
                }
                SpecialOptions::Multichoice {
                    value, all_values, ..
                } => {
                    for val in all_values.iter() {
                        writer.write(if value.contains(val) { 1 } else { 0 }, 1);
                    }
                }
                SpecialOptions::Singlechoice {
                    value,
                    all_values,
                    bits,
                    ..
                } => {
                    // if this is not the array, something else is messed up
                    let index = all_values.iter().position(|v| v == value).unwrap();
                    writer.write(index, *bits);
                }
                SpecialOptions::SinglechoiceUInt {
                    value,
                    all_values,
                    bits,
                    ..
                } => {
                    let index = all_values.iter().position(|v| v == value).unwrap();
                    writer.write(index, *bits);
                }
            }
        }
        writer.flush();
        let mut out = writer.to_base64();
        if let Some(seed) = self.seed {
            write!(out, "#{}", seed).unwrap();
        }
        return out;
    }

    pub fn update_from_permalink(&mut self, s: &str) -> Result<(), Box<dyn Error>> {
        let mut reader = match s.find("#") {
            None => PackedBitsReader::from_base64(s)?,
            Some(seed_str_idx) => {
                self.seed = Some(s[seed_str_idx + 1..].parse()?);
                PackedBitsReader::from_base64(&s[..seed_str_idx])?
            }
        };
        // let mut reader = PackedBitsReader::from_base64(s)?;
        for option in self.options.iter_mut() {
            if !option.permalink {
                continue;
            }
            let option_name = &option.name;
            match &mut option.inner {
                SpecialOptions::Boolean { value, .. } => {
                    *value = reader.read(1)? == 1;
                }
                SpecialOptions::UInt { value, bits, .. } => {
                    *value = reader.read(*bits)?;
                }
                SpecialOptions::Multichoice {
                    value, all_values, ..
                } => {
                    let mut new_val = HashSet::new();
                    for val in all_values.iter() {
                        if reader.read(1)? == 1 {
                            new_val.insert(val.clone());
                        }
                    }
                    *value = new_val;
                }
                SpecialOptions::Singlechoice {
                    value,
                    all_values,
                    bits,
                    ..
                } => {
                    let index = reader.read(*bits)?;
                    *value = all_values.get(index).cloned().ok_or_else(|| {
                        format!("index {} out of range for {}", index, option_name)
                    })?;
                }
                SpecialOptions::SinglechoiceUInt {
                    value,
                    all_values,
                    bits,
                    ..
                } => {
                    let index = reader.read(*bits)?;
                    *value = all_values.get(index).cloned().ok_or_else(|| {
                        format!("index {} out of range for {}", index, option_name)
                    })?;
                }
            }
        }
        Ok(())
    }

    // checks, if a specific option has been enabled
    // returns None, if the option can't be found or is not a boolean option
    pub fn get_option_enabled(&self, option_name: &str) -> Option<bool> {
        for opt in self.options.iter() {
            match opt.inner {
                SpecialOptions::Boolean { value, .. } => {
                    if opt.command == option_name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }
        None
    }

    // return the uint value of an option, if that option exists
    pub fn get_option_uint(&self, option_name: &str) -> Option<usize> {
        for opt in self.options.iter() {
            match opt.inner {
                SpecialOptions::UInt { value, .. } => {
                    if opt.command == option_name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }
        None
    }

    // returns the choice str for an option, if that option exists
    pub fn get_option_choice_str(&self, option_name: &str) -> Option<&String> {
        for opt in self.options.iter() {
            match &opt.inner {
                SpecialOptions::Singlechoice { value, .. } => {
                    if opt.command == option_name {
                        return Some(&value);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn get_option_choice_uint(&self, option_name: &str) -> Option<usize> {
        for opt in self.options.iter() {
            match opt.inner {
                SpecialOptions::SinglechoiceUInt { value, .. } => {
                    if opt.command == option_name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn get_option_choices(&self, option_name: &str) -> Option<&HashSet<String>> {
        for opt in self.options.iter() {
            match &opt.inner {
                SpecialOptions::Multichoice { value, .. } => {
                    if opt.command == option_name {
                        return Some(value);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn set_option_bool(
        &mut self,
        option_name: &str,
        new_value: bool,
    ) -> Result<(), SetOptionError> {
        for opt in self.options.iter_mut() {
            if opt.command == option_name {
                match &mut opt.inner {
                    SpecialOptions::Boolean { value, .. } => {
                        *value = new_value;
                        return Ok(());
                    }
                    _ => {
                        return Err(SetOptionError::WrongOptionType);
                    }
                }
            }
        }
        Err(SetOptionError::DoesNotExist)
    }

    pub fn set_option_uint(
        &mut self,
        option_name: &str,
        new_value: usize,
    ) -> Result<(), SetOptionError> {
        for opt in self.options.iter_mut() {
            if opt.command == option_name {
                match &mut opt.inner {
                    SpecialOptions::UInt {
                        value, min, max, ..
                    } => {
                        if let Some(min_val) = min {
                            if *min_val > new_value {
                                return Err(SetOptionError::NotInAvailableChoices);
                            }
                        }
                        if let Some(max_val) = max {
                            if *max_val < new_value {
                                return Err(SetOptionError::NotInAvailableChoices);
                            }
                        }
                        *value = new_value;
                        return Ok(());
                    }
                    SpecialOptions::SinglechoiceUInt {
                        value, all_values, ..
                    } => {
                        if !all_values.contains(&new_value) {
                            return Err(SetOptionError::NotInAvailableChoices);
                        }
                        *value = new_value;
                        return Ok(());
                    }
                    _ => {
                        return Err(SetOptionError::WrongOptionType);
                    }
                }
            }
        }
        Err(SetOptionError::DoesNotExist)
    }

    pub fn set_option_str(
        &mut self,
        option_name: &str,
        new_value: &str,
    ) -> Result<(), SetOptionError> {
        for opt in self.options.iter_mut() {
            if opt.command == option_name {
                match &mut opt.inner {
                    SpecialOptions::Singlechoice {
                        value, all_values, ..
                    } => {
                        if !all_values.iter().any(|v| v == new_value) {
                            return Err(SetOptionError::NotInAvailableChoices);
                        }
                        *value = new_value.to_owned();
                        return Ok(());
                    }
                    _ => {
                        return Err(SetOptionError::WrongOptionType);
                    }
                }
            }
        }
        Err(SetOptionError::DoesNotExist)
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        for opt in self.options.iter() {
            match &opt.inner {
                SpecialOptions::Boolean { value, .. } => {
                    writeln!(out, "{}: {}", opt.name, value).unwrap();
                }
                SpecialOptions::UInt { value, .. } => {
                    writeln!(out, "{}: {}", opt.name, value).unwrap();
                }
                SpecialOptions::Multichoice {
                    value, all_values, ..
                } => {
                    let chosen = all_values
                        .iter()
                        .filter(|&v| value.contains(v))
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", ");
                    writeln!(out, "{}: {}", opt.name, chosen).unwrap();
                }
                SpecialOptions::Singlechoice { value, .. } => {
                    writeln!(out, "{}: {}", opt.name, value).unwrap();
                }
                SpecialOptions::SinglechoiceUInt { value, .. } => {
                    writeln!(out, "{}: {}", opt.name, value).unwrap();
                }
            }
        }
        out
    }
}
