use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::num::NonZeroU16;

use serde::Deserialize;

use super::logic_expression::LogicElement;

// parsing the definition files directly retrieves an initial placement
// for example events are directly placed (for example "Skyloft - Water Refill" yields the "Water Refill" "item" )

// give every string in the logic a unique ID
// helps, so that we can clone stuff more cheaply
#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub struct LogicKey(pub NonZeroU16);

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub struct AreaKey {
    pub stage: LogicKey,
    pub area: LogicKey,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Deserialize)]
pub enum AllowedToD {
    Day,
    Night,
    Both,
    None,
}

impl AllowedToD {
    pub fn allows(&self, tod: &TimeOfDay) -> bool {
        match self {
            AllowedToD::Day => *tod == TimeOfDay::Day,
            AllowedToD::Night => *tod == TimeOfDay::Night,
            AllowedToD::Both => true,
            AllowedToD::None => false,
        }
    }

    pub fn all_allowed(&self) -> &[TimeOfDay] {
        match self {
            AllowedToD::Day => &[TimeOfDay::Day],
            AllowedToD::Night => &[TimeOfDay::Night],
            AllowedToD::Both => &[TimeOfDay::Day, TimeOfDay::Night],
            AllowedToD::None => &[],
        }
    }

    pub fn add_tod(&mut self, tod: &TimeOfDay) {
        match self {
            AllowedToD::Day => {
                if tod == &TimeOfDay::Night {
                    *self = AllowedToD::Both;
                }
            }
            AllowedToD::Night => {
                if tod == &TimeOfDay::Day {
                    *self = AllowedToD::Both;
                }
            }
            AllowedToD::Both => (),
            AllowedToD::None => match tod {
                &TimeOfDay::Day => *self = AllowedToD::Day,
                &TimeOfDay::Night => *self = AllowedToD::Night,
            },
        }
    }
}

impl Default for AllowedToD {
    fn default() -> Self {
        AllowedToD::Both
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum TimeOfDay {
    Day,
    Night,
}

impl TimeOfDay {
    pub const ALL: [TimeOfDay; 2] = [TimeOfDay::Day, TimeOfDay::Night];
    pub fn with_forced(&self, forced_tod: &AllowedToD) -> Self {
        match forced_tod {
            &AllowedToD::Both => self.clone(),
            &AllowedToD::Day => TimeOfDay::Day,
            &AllowedToD::Night => TimeOfDay::Night,
            &AllowedToD::None => panic!("AllowedToD::None not allowed here!"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct CheckKey {
    pub area_key: AreaKey,
    pub check: LogicKey,
}

impl LogicKey {
    pub fn stage_with_area(&self, area: &LogicKey) -> AreaKey {
        AreaKey {
            stage: self.clone(),
            area: area.clone(),
        }
    }

    pub fn name<'a>(&self, mapper: &'a LogicKeyMapper) -> &'a String {
        mapper.convert_to_string(self).unwrap()
    }
}

impl AreaKey {
    pub fn area_localize(&self, local: &TimeOfDay) -> LocalizedAreaKey {
        LocalizedAreaKey {
            area_key: self.clone(),
            local: local.clone(),
        }
    }

    pub fn area_with_check(&self, check: &LogicKey) -> CheckKey {
        CheckKey {
            area_key: self.clone(),
            check: check.clone(),
        }
    }

    pub fn area_to_passageway(&self, disambiguation: &Option<LogicKey>) -> PassagewayKey {
        PassagewayKey {
            other_area: self.clone(),
            disambiguation: disambiguation.clone(),
        }
    }

    pub fn area_exit_to(&self, other_area: &AreaKey, disambiguation: &Option<LogicKey>) -> Exit {
        Exit {
            area: self.clone(),
            to_area: other_area.clone(),
            disambiguation: disambiguation.clone(),
        }
    }

    pub fn area_exit_to_psgw(&self, psgw: &PassagewayKey) -> Exit {
        self.area_exit_to(&psgw.other_area, &psgw.disambiguation)
    }

    pub fn area_entrance_from(
        &self,
        other_area: &AreaKey,
        disambiguation: &Option<LogicKey>,
    ) -> Entrance {
        Entrance {
            area: self.clone(),
            from_area: other_area.clone(),
            disambiguation: disambiguation.clone(),
        }
    }

    pub fn area_entrance_from_psgw(&self, psgw: &PassagewayKey) -> Entrance {
        self.area_entrance_from(&psgw.other_area, &psgw.disambiguation)
    }

    pub fn name<'a>(&self, mapper: &'a LogicKeyMapper) -> String {
        let stage = mapper.convert_to_string(&self.stage).unwrap();
        let area = mapper.convert_to_string(&self.area).unwrap();
        format!("{} - {}", stage, area)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct LocalizedAreaKey {
    pub area_key: AreaKey,
    pub local: TimeOfDay,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct PassagewayKey {
    pub other_area: AreaKey,
    pub disambiguation: Option<LogicKey>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub struct Entrance {
    // area we are in
    pub area: AreaKey,
    // area this entrance came from
    pub from_area: AreaKey,
    pub disambiguation: Option<LogicKey>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub struct Exit {
    // area we are in
    pub area: AreaKey,
    // area this exit leads to
    pub to_area: AreaKey,
    pub disambiguation: Option<LogicKey>,
}

impl Entrance {
    pub fn dbg_to_string(&self, mapper: &LogicKeyMapper) -> String {
        let from_area_name = self.from_area.name(mapper);
        let to_area_name = self.area.name(mapper);
        if let Some(disambig) = &self.disambiguation {
            let disambig_name = mapper.convert_to_string(disambig).unwrap();
            format!(
                "{} (from {}, {})",
                to_area_name, from_area_name, disambig_name
            )
        } else {
            format!("{} (from {})", to_area_name, from_area_name)
        }
    }

    pub fn reverse(&self) -> Entrance {
        Entrance {
            area: self.from_area.clone(),
            from_area: self.area.clone(),
            disambiguation: self.disambiguation.clone(),
        }
    }

    pub fn to_area_psgw(&self) -> (AreaKey, PassagewayKey) {
        (
            self.area.clone(),
            PassagewayKey {
                other_area: self.from_area.clone(),
                disambiguation: self.disambiguation.clone(),
            },
        )
    }

    // the other side of the passageway
    pub fn to_exit(&self) -> Exit {
        Exit {
            area: self.from_area.clone(),
            to_area: self.area.clone(),
            disambiguation: self.disambiguation.clone(),
        }
    }

    // when you immediately go through the exit which entrance you came from
    pub fn to_this_side_exit(&self) -> Exit {
        Exit {
            area: self.area.clone(),
            to_area: self.from_area.clone(),
            disambiguation: self.disambiguation.clone(),
        }
    }
}

impl Exit {
    pub fn dbg_to_string(&self, mapper: &LogicKeyMapper) -> String {
        let from_area_name = self.area.name(mapper);
        let to_area_name = self.to_area.name(mapper);
        if let Some(disambig) = &self.disambiguation {
            let disambig_name = mapper.convert_to_string(disambig).unwrap();
            format!("{} to {} ({})", from_area_name, to_area_name, disambig_name)
        } else {
            format!("{} to {}", from_area_name, to_area_name)
        }
    }

    pub fn reverse(&self) -> Exit {
        Exit {
            area: self.to_area.clone(),
            to_area: self.area.clone(),
            disambiguation: self.disambiguation.clone(),
        }
    }

    pub fn to_area_psgw(&self) -> (AreaKey, PassagewayKey) {
        (
            self.area.clone(),
            PassagewayKey {
                other_area: self.to_area.clone(),
                disambiguation: self.disambiguation.clone(),
            },
        )
    }

    // the other side of the passageway
    pub fn to_entrance(&self) -> Entrance {
        Entrance {
            area: self.to_area.clone(),
            from_area: self.area.clone(),
            disambiguation: self.disambiguation.clone(),
        }
    }

    // when you immediately go through the exit which entrance you came from
    pub fn to_this_side_entrance(&self) -> Entrance {
        Entrance {
            area: self.area.clone(),
            from_area: self.to_area.clone(),
            disambiguation: self.disambiguation.clone(),
        }
    }

    pub fn to_stages(
        &self,
        areas: &HashMap<AreaKey, Area>,
    ) -> (LogicKey, LogicKey, Option<LogicKey>) {
        (
            self.area.stage.clone(),
            self.to_area.stage.clone(),
            self.disambiguation.clone(),
        )
    }
}

pub struct Area {
    pub region: LogicKey,
    pub stage: LogicKey,
    pub name: LogicKey,
    pub allowed_tod: AllowedToD,
    pub can_sleep: bool,
    pub locations: HashMap<LogicKey, LogicElement>,
    pub exits: HashMap<PassagewayKey, LogicElement>,
}

#[derive(Debug, Clone, Default)]
pub struct Placement {
    pub connected_areas: HashMap<Exit, Entrance>,
    pub filled_checks: HashMap<CheckKey, LogicKey>,
}

#[derive(Debug, Clone, Default)]
pub struct Inventory {
    pub areas: HashMap<AreaKey, AllowedToD>,
    pub items_events_counts: HashMap<LogicKey, usize>,
}

pub fn collect_items(items_events_counts: &mut HashMap<LogicKey, usize>, item: LogicKey) {
    *items_events_counts.entry(item).or_default() += 1;
}

impl Inventory {
    pub fn collect_item(&mut self, item: LogicKey) {
        collect_items(&mut self.items_events_counts, item);
    }

    pub fn check_owned_item(&self, item: &LogicKey) -> bool {
        self.items_events_counts.get(item).cloned().unwrap_or(0) >= 1
    }

    pub fn check_item_count(&self, item: &LogicKey, count: usize) -> bool {
        self.items_events_counts.get(item).cloned().unwrap_or(0) >= count
    }

    pub fn add_area_tod(&mut self, area_key: &AreaKey, tod: &TimeOfDay) {
        self.areas
            .entry(area_key.clone())
            .or_insert(AllowedToD::None)
            .add_tod(tod);
    }

    pub fn check_area_tod(&self, area_key: &AreaKey, tod: &TimeOfDay) -> bool {
        self.areas
            .get(area_key)
            .map(|allowed| allowed.allows(tod))
            .unwrap_or(false)
    }
}

#[derive(Default)]
pub struct LogicKeyMapper {
    // key is index + 1 (to be able to use NonZeroU16)
    name_to_num: HashMap<String, LogicKey>,
    num_to_name: Vec<String>,
}

impl LogicKeyMapper {
    pub fn convert_to_num(&mut self, s: &str) -> LogicKey {
        if let Some(num) = self.name_to_num.get(s) {
            return num.clone();
        }
        let next_num = LogicKey(
            NonZeroU16::try_from(u16::try_from(self.num_to_name.len() + 1).unwrap()).unwrap(),
        );
        self.num_to_name.push(s.into());
        self.name_to_num.insert(s.into(), next_num.clone());
        next_num
    }

    pub fn convert_to_area(&mut self, stage_name: &str, area_part_name: &str) -> AreaKey {
        let stage = self.convert_to_num(stage_name);
        let area_part = self.convert_to_num(area_part_name);
        AreaKey {
            stage,
            area: area_part,
        }
    }

    pub fn convert_to_num_assuming_present<'a>(&self, s: &'a str) -> Result<LogicKey, &'a str> {
        self.name_to_num.get(s).cloned().ok_or(s)
    }

    pub fn convert_to_area_assuming_present<'a>(
        &self,
        stage_name: &'a str,
        area_part_name: &'a str,
    ) -> Result<AreaKey, &'a str> {
        self.name_to_num
            .get(stage_name)
            .cloned()
            .ok_or(stage_name)
            .and_then(|stage_key| {
                self.name_to_num
                    .get(area_part_name)
                    .cloned()
                    .ok_or(area_part_name)
                    .map(|area_part_key| AreaKey {
                        stage: stage_key,
                        area: area_part_key,
                    })
            })
    }

    pub fn convert_to_string(&self, num: &LogicKey) -> Option<&String> {
        self.num_to_name.get((u16::from(num.0) - 1) as usize)
    }

    pub fn len(&self) -> usize {
        self.num_to_name.len()
    }
}

#[cfg(test)]
mod tests {
    use super::LogicKeyMapper;
    #[test]
    fn works() {
        let mut mapper = LogicKeyMapper::default();
        let first_key = mapper.convert_to_num("First Key");
        let second_key = mapper.convert_to_num("Second Key");
        let first_key_again = mapper.convert_to_num("First Key");

        assert_eq!(first_key, first_key_again);
        assert_ne!(first_key, second_key);
        assert_eq!(
            Some(&"First Key".to_string()),
            mapper.convert_to_string(&first_key)
        );
        assert_eq!(
            Some(&"Second Key".to_string()),
            mapper.convert_to_string(&second_key)
        );
    }
}
