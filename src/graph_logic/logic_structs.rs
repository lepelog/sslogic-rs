use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::num::NonZeroU16;

use serde::Deserialize;

use super::logic_expression::LogicElement;
use super::logic_loader::CheckEntry;

// parsing the definition files directly retrieves an initial placement
// for example events are directly placed (for example "Skyloft - Water Refill" yields the "Water Refill" "item" )

// give every string in the logic a unique ID
// helps, so that we can clone stuff more cheaply
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct LogicKey(pub NonZeroU16);

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct AreaKey {
    pub stage: LogicKey,
    pub area: LogicKey,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct LocationKey {
    pub stage: LogicKey,
    pub area: LogicKey,
    pub location: LogicKey,
}

// TODO: this should really be bitflags, one for night one for day
#[derive(PartialEq, Eq, Hash, Debug, Clone, Deserialize)]
pub enum AllowedToD {
    None,
    Day,
    Night,
    Both,
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

    pub fn to_allowed(&self) -> AllowedToD {
        match self {
            TimeOfDay::Day => AllowedToD::Day,
            TimeOfDay::Night => AllowedToD::Night,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub struct CheckKey {
    pub area_key: AreaKey,
    pub check: LogicKey,
}

impl CheckKey {
    pub fn dbg_to_string(&self, mapper: &LogicKeyMapper) -> String {
        format!(
            "{} - {}",
            self.area_key.stage.name(mapper),
            self.check.name(mapper)
        )
    }
}

impl LogicKey {
    pub fn stage_with_area(&self, area: &LogicKey) -> AreaKey {
        AreaKey {
            stage: self.clone(),
            area: area.clone(),
        }
    }

    pub fn name<'a>(&self, mapper: &'a LogicKeyMapper) -> &'a str {
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

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub struct StageExit {
    // stage we are in
    pub stage: LogicKey,
    // stage this exit leads to
    pub to_stage: LogicKey,
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

    pub fn to_stage_exit(&self) -> StageExit {
        StageExit {
            stage: self.area.stage.clone(),
            to_stage: self.to_area.stage.clone(),
            disambiguation: self.disambiguation.clone(),
        }
    }
}

// TODO: move
pub struct LogicBase {
    mapper: LogicKeyMapper,
    areas: HashMap<AreaKey, Area>,
}

impl LogicBase {
    pub fn get<'a>(&'a self, area_key: &AreaKey) -> Option<&'a Area> {
        self.areas.get(area_key)
    }

    pub fn get_location<'a>(
        &'a self,
        area_key: &AreaKey,
        location: &LogicKey,
    ) -> Result<&'a LogicElement, String> {
        self.areas
            .get(area_key)
            .ok_or_else(|| format!("can't find area {}", area_key.name(&self.mapper)))
            .and_then(|area| {
                area.locations.get(location).ok_or_else(|| {
                    format!(
                        "can't find location {} in {}",
                        location.name(&self.mapper),
                        area_key.name(&self.mapper)
                    )
                })
            })
    }

    pub fn get_location_mut<'a>(
        &'a mut self,
        area_key: &AreaKey,
        location: &LogicKey,
    ) -> Result<&'a mut LogicElement, String> {
        self.areas
            .get_mut(area_key)
            .ok_or_else(|| format!("can't find area {}", area_key.name(&self.mapper)))
            .and_then(|area| {
                area.locations.get_mut(location).ok_or_else(|| {
                    format!(
                        "can't find location {} in {}",
                        location.name(&self.mapper),
                        area_key.name(&self.mapper)
                    )
                })
            })
    }

    pub fn create_edit_scope(&mut self) -> LogicEdit<'_> {
        LogicEdit {
            logic_base: self,
            location_changes: HashMap::new(),
            exit_changes: HashMap::new(),
        }
    }

    pub fn dbg_area(&self, area_key: &AreaKey) -> String {
        area_key.name(&self.mapper)
    }
}

pub struct LogicEdit<'a> {
    logic_base: &'a mut LogicBase,
    location_changes: HashMap<(AreaKey, LogicKey), LogicElement>,
    exit_changes: HashMap<(AreaKey, PassagewayKey), LogicElement>,
}

impl<'a> LogicEdit<'a> {
    pub fn set_location_logic(
        &mut self,
        area_key: AreaKey,
        location: LogicKey,
        new_logic: LogicElement,
    ) -> Result<(), String> {
        let elem = self.logic_base.get_location_mut(&area_key, &location)?;
        // only save the old element if we haven't saved the original
        match self.location_changes.entry((area_key, location)) {
            Entry::Vacant(vacant) => {
                vacant.insert(elem.clone());
            }
            _ => (),
        }
        *elem = new_logic;
        Ok(())
    }

    pub fn get_logic(&self) -> &LogicBase {
        self.logic_base
    }
}

impl<'a> Drop for LogicEdit<'a> {
    fn drop(&mut self) {
        // restore overwritten logic elements
        for ((area_key, location), logic_element) in self.location_changes.drain() {
            *self
                .logic_base
                .areas
                .get_mut(&area_key)
                .unwrap()
                .locations
                .get_mut(&location)
                .unwrap() = logic_element;
        }
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

impl Area {
    pub fn check_exit_req(
        &self,
        psgw_key: &PassagewayKey,
        inventory: &Inventory,
        tod: &TimeOfDay,
    ) -> bool {
        self.exits
            .get(psgw_key)
            .unwrap()
            .check_requirement_met(inventory, tod)
    }
    pub fn check_location_req(
        &self,
        location: &LogicKey,
        inventory: &Inventory,
        tod: &TimeOfDay,
    ) -> bool {
        self.locations
            .get(location)
            .unwrap()
            .check_requirement_met(inventory, tod)
    }
}

struct PlandoCheckEntry {
    locations: Vec<CheckKey>,
    items: Vec<LogicKey>,
}

pub trait Placement {
    fn get_connected_area<'a>(&'a self, exit: &Exit) -> Option<&'a Entrance>;
    fn set_connected_area(&mut self, exit: Exit, entrance: Entrance);
    fn remove_connected_area(&mut self, exit: &Exit);
    fn get_filled_check<'a>(&'a self, check: &CheckKey) -> Option<&'a LogicKey>;
    fn set_filled_check(&mut self, check: CheckKey, key: LogicKey);
    fn remove_filled_check(&mut self, check: &CheckKey);
}

#[derive(Debug, Clone, Default)]
pub struct BasePlacement {
    pub connected_areas: HashMap<Exit, Entrance>,
    pub filled_checks: HashMap<CheckKey, LogicKey>,
}

impl BasePlacement {
    pub fn new(
        connected_areas: HashMap<Exit, Entrance>,
        filled_checks: HashMap<CheckKey, LogicKey>,
    ) -> Self {
        Self {
            connected_areas,
            filled_checks,
        }
    }

    pub fn into_inner(self) -> (HashMap<Exit, Entrance>, HashMap<CheckKey, LogicKey>) {
        let Self {
            connected_areas,
            filled_checks,
        } = self;
        (connected_areas, filled_checks)
    }

    pub fn consume_other(&mut self, other: (HashMap<Exit, Entrance>, HashMap<CheckKey, LogicKey>)) {
        self.connected_areas.extend(other.0);
        self.filled_checks.extend(other.1);
    }
}

impl Placement for BasePlacement {
    fn get_connected_area<'a>(&'a self, exit: &Exit) -> Option<&'a Entrance> {
        self.connected_areas.get(exit)
    }

    fn set_connected_area(&mut self, exit: Exit, entrance: Entrance) {
        self.connected_areas.insert(exit, entrance);
    }

    fn remove_connected_area(&mut self, exit: &Exit) {
        self.connected_areas.remove(exit);
    }

    fn get_filled_check<'a>(&'a self, check: &CheckKey) -> Option<&'a LogicKey> {
        self.filled_checks.get(check)
    }

    fn set_filled_check(&mut self, check: CheckKey, key: LogicKey) {
        self.filled_checks.insert(check, key);
    }

    fn remove_filled_check(&mut self, check: &CheckKey) {
        self.filled_checks.remove(check);
    }
}

#[derive(Debug, Clone, Default)]
pub struct RecursivePlacement<'a> {
    connected_areas: HashMap<Exit, Entrance>,
    filled_checks: HashMap<CheckKey, LogicKey>,
    parent: Option<&'a BasePlacement>,
}

impl<'a> RecursivePlacement<'a> {
    pub fn with_parent(placement: &'a BasePlacement) -> Self {
        RecursivePlacement {
            connected_areas: HashMap::new(),
            filled_checks: HashMap::new(),
            parent: Some(placement),
        }
    }

    pub fn new(
        connected_areas: HashMap<Exit, Entrance>,
        filled_checks: HashMap<CheckKey, LogicKey>,
    ) -> Self {
        RecursivePlacement {
            connected_areas,
            filled_checks,
            parent: None,
        }
    }

    pub fn into_inner(self) -> (HashMap<Exit, Entrance>, HashMap<CheckKey, LogicKey>) {
        let RecursivePlacement {
            connected_areas,
            filled_checks,
            ..
        } = self;
        (connected_areas, filled_checks)
    }

    pub fn consume_other(&mut self, other: (HashMap<Exit, Entrance>, HashMap<CheckKey, LogicKey>)) {
        self.connected_areas.extend(other.0);
        self.filled_checks.extend(other.1);
    }

    pub fn clear(&mut self) {
        self.connected_areas.clear();
        self.filled_checks.clear();
    }
}

impl<'a> Placement for RecursivePlacement<'a> {
    fn get_connected_area<'b>(&'b self, exit: &Exit) -> Option<&'b Entrance> {
        self.connected_areas
            .get(exit)
            .or_else(|| self.parent.and_then(|p| p.get_connected_area(exit)))
    }

    fn set_connected_area(&mut self, exit: Exit, entrance: Entrance) {
        self.connected_areas.insert(exit, entrance);
    }

    // only removes from the top layer
    fn remove_connected_area(&mut self, exit: &Exit) {
        self.connected_areas.remove(exit);
    }

    fn get_filled_check<'b>(&'b self, check: &CheckKey) -> Option<&'b LogicKey> {
        self.filled_checks
            .get(check)
            .or_else(|| self.parent.and_then(|p| p.get_filled_check(check)))
    }

    fn set_filled_check(&mut self, check: CheckKey, key: LogicKey) {
        self.filled_checks.insert(check, key);
    }

    // only removes from the top layer
    fn remove_filled_check(&mut self, check: &CheckKey) {
        self.filled_checks.remove(check);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Inventory<'a> {
    pub areas: HashMap<AreaKey, AllowedToD>,
    pub items_events_counts: HashMap<LogicKey, usize>,
    pub parent: Option<&'a Inventory<'a>>,
}

pub fn collect_items(items_events_counts: &mut HashMap<LogicKey, usize>, item: LogicKey) {
    *items_events_counts.entry(item).or_default() += 1;
}

impl Inventory<'static> {
    pub fn new(
        areas: HashMap<AreaKey, AllowedToD>,
        items_events_counts: HashMap<LogicKey, usize>,
    ) -> Self {
        Self {
            areas,
            items_events_counts,
            parent: None,
        }
    }
}

impl<'a> Inventory<'a> {
    pub fn collect_item(&mut self, item: LogicKey) {
        collect_items(&mut self.items_events_counts, item);
    }

    pub fn check_owned_item(&self, item: &LogicKey) -> bool {
        self.items_events_counts.get(item).cloned().unwrap_or(0) >= 1
            || self.parent.map_or(false, |p| p.check_owned_item(item))
    }

    pub fn check_item_count(&self, item: &LogicKey, count: usize) -> bool {
        let self_owned = self.items_events_counts.get(item).cloned().unwrap_or(0);
        let left = count.saturating_sub(self_owned);
        if left == 0 {
            return true;
        }
        self.parent
            .map_or(false, |p| p.check_item_count(item, left))
    }

    pub fn add_area_tod(&mut self, area_key: &AreaKey, tod: &TimeOfDay) {
        match self.areas.entry(area_key.clone()) {
            Entry::Occupied(occupied) => {
                occupied.into_mut().add_tod(tod);
            }
            Entry::Vacant(vacant) => {
                vacant.insert(tod.to_allowed());
            }
        };
    }

    pub fn check_area_tod(&self, area_key: &AreaKey, tod: &TimeOfDay) -> bool {
        self.areas
            .get(area_key)
            .map(|allowed| allowed.allows(tod))
            .unwrap_or(false)
            || self
                .parent
                .map_or(false, |p| p.check_area_tod(area_key, tod))
    }

    pub fn clear(&mut self) {
        self.areas.clear();
        self.items_events_counts.clear();
    }

    pub fn make_child(&'a self) -> Self {
        Inventory {
            areas: HashMap::new(),
            items_events_counts: HashMap::new(),
            parent: Some(self),
        }
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

    pub fn map(&self, s: &str) -> LogicKey {
        self.convert_to_num_assuming_present(s).unwrap()
    }

    pub fn convert_to_string(&self, num: &LogicKey) -> Option<&str> {
        self.num_to_name
            .get((u16::from(num.0) - 1) as usize)
            // the compiler told me to do this
            .map(|x| &**x)
    }

    pub fn len(&self) -> usize {
        self.num_to_name.len()
    }

    pub fn dbg_logic<I>(&self, iter: I) -> String
    where
        I: Iterator<Item = LogicKey>,
    {
        iter.map(|k| self.convert_to_string(&k))
            .fold(String::new(), |mut acc, elem| {
                acc.push_str(elem.unwrap_or("INVALID"));
                acc.push_str(",");
                acc
            })
    }

    pub fn dbg_areas<I>(&self, iter: I) -> String
    where
        I: Iterator<Item = AreaKey>,
    {
        iter.map(|k| k.name(self))
            .fold(String::new(), |mut acc, elem| {
                acc.push_str(&elem);
                acc.push_str(",");
                acc
            })
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
        assert_eq!(Some("First Key"), mapper.convert_to_string(&first_key));
        assert_eq!(Some("Second Key"), mapper.convert_to_string(&second_key));
    }
}
