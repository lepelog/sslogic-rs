use std::{collections::{HashMap, HashSet}, marker::PhantomData};

use crate::generated::{Area, Event, Exit, Item, Location, Stage, Options};

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TimeOfDay: u8 {
        const Day   = 1 << 0;
        const Night = 1 << 1;
        const Both   = Self::Day.bits() | Self::Night.bits();
    }
}

impl TimeOfDay {
    pub fn from_force_tod(tod: Option<ForceToD>) -> Self {
        match tod {
            None => TimeOfDay::all(),
            Some(ForceToD::Day) => TimeOfDay::Day,
            Some(ForceToD::Night) => TimeOfDay::Night,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForceToD {
    Day,
    Night,
}

pub trait BitSetCompatible: Sized + Clone + Copy + Into<usize> + 'static {
    const ALL: &'static [Self];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitSet<T: BitSetCompatible, const N: usize> {
    slots: [usize; N],
    _phantom: PhantomData<T>,
}

impl<T: BitSetCompatible, const N: usize> Default for BitSet<T, N> {
    fn default() -> Self {
        BitSet {
            slots: [0; N],
            _phantom: PhantomData,
        }
    }
}

impl<T: BitSetCompatible, const N: usize> BitSet<T, N> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_all_set() -> Self {
        BitSet {
            slots: [usize::MAX; N],
            _phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, t: T) {
        let num: usize = t.into();
        let slot = num / usize::BITS as usize;
        let shift = num % usize::BITS as usize;
        self.slots[slot] |= 1 << shift;
    }

    pub fn has(&self, t: T) -> bool {
        let num: usize = t.into();
        let slot = num / usize::BITS as usize;
        let shift = num % usize::BITS as usize;
        self.slots[slot] & (1 << shift) != 0
    }

    pub fn get_all_set(&self) -> impl Iterator<Item = T> + '_ {
        T::ALL.iter().map(|a| *a).filter(|a| self.has(*a))
    }

    pub fn get_all_unset(&self) -> impl Iterator<Item = T> + '_ {
        T::ALL.iter().map(|a| *a).filter(|a| !self.has(*a))
    }
}

pub type AreaBitset = BitSet<Area, { Area::ALL.len() / usize::BITS as usize + 1 }>;
pub type EventBitset = BitSet<Event, { Event::ALL.len() / usize::BITS as usize + 1 }>;
pub type StageBitset = BitSet<Stage, { Stage::ALL.len() / usize::BITS as usize + 1 }>;
pub type LocationBitset = BitSet<Location, { Location::ALL.len() / usize::BITS as usize + 1 }>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemCollection {
    count: [u8; Item::ALL.len()],
}

impl Default for ItemCollection {
    fn default() -> Self {
        ItemCollection {
            count: [0; Item::ALL.len()],
        }
    }
}

impl ItemCollection {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_filled() -> Self {
        ItemCollection {
            count: [u8::MAX; Item::ALL.len()],
        }
    }
    pub fn collect(&mut self, item: Item) {
        self.collect_multiple(item, 1);
    }
    pub fn collect_multiple(&mut self, item: Item, count: u8) {
        if count == 0 {
            return;
        }
        let item_num = item as usize;
        if let Some(count_ref) = self.count.get_mut(item_num) {
            *count_ref = count_ref.saturating_add(count);
        }
    }
    pub fn check(&self, item: Item) -> u8 {
        let item_num = item as usize;
        self.count.get(item_num).copied().unwrap_or_default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Inventory {
    // TODO: should probably be not pub
    pub reachable_areas_day: AreaBitset,
    pub reachable_areas_night: AreaBitset,
    pub items: ItemCollection,
    pub events: EventBitset,
}



impl Inventory {
    pub fn get_area_tod(&self, area: Area) -> TimeOfDay {
        let mut tod = TimeOfDay::empty();
        if self.reachable_areas_day.has(area) {
            tod |= TimeOfDay::Day;
        }
        if self.reachable_areas_night.has(area) {
            tod |= TimeOfDay::Night;
        }
        tod
    }
    pub fn insert_area_tod(&mut self, area: Area, tod: TimeOfDay) {
        if tod.contains(TimeOfDay::Day) {
            self.reachable_areas_day.insert(area);
        }
        if tod.contains(TimeOfDay::Night) {
            self.reachable_areas_night.insert(area);
        }
    }
    pub fn item_count(&self, item: Item) -> u8 {
        self.items.check(item)
    }
    pub fn insert_item(&mut self, item: Item) {
        self.items.collect(item);
    }
    pub fn insert_items(&mut self, item: Item, count: u8) {
        self.items.collect_multiple(item, count);
    }
    pub fn has_event(&self, event: Event) -> bool {
        self.events.has(event)
    }
    pub fn insert_event(&mut self, event: Event) {
        self.events.insert(event);
    }
}

pub struct Requirements<'a> {
    parent: Option<&'a Requirements<'a>>,
    pub requirements: std::collections::HashMap<RequirementKey, RequirementExpression<'a>>,
}

impl Requirements<'static> {
    pub fn new() -> Self {
        Self::new_from_map(Default::default())
    }

    pub fn new_from_map(
        requirements: HashMap<RequirementKey, RequirementExpression<'static>>,
    ) -> Self {
        Self {
            parent: None,
            requirements,
        }
    }
}

impl<'a> Requirements<'a> {
    pub fn check(
        &self,
        requirement: RequirementKey,
        inventory: &Inventory,
        options: &Options,
        allowed_tod: TimeOfDay,
    ) -> bool {
        if let Some(req) = self.requirements.get(&requirement) {
            req.check(inventory, options, allowed_tod)
        } else if let Some(parent) = self.parent {
            parent.check(requirement, inventory, options, allowed_tod)
        } else {
            false
        }
    }

    pub fn create_layer(&'a self) -> Requirements<'a> {
        Requirements {
            parent: Some(self),
            requirements: Default::default(),
        }
    }

    pub fn get_owned_requirement(
        &self,
        requirement: RequirementKey,
    ) -> Option<RequirementExpression<'a>> {
        match self.requirements.get(&requirement) {
            Some(req) => Some(req.clone()),
            None => self.parent.and_then(|parent| {
                parent
                    .get_requirement(requirement)
                    .map(|r| RequirementExpression::Ref(r))
            }),
        }
    }

    pub fn get_requirement(
        &'a self,
        requirement: RequirementKey,
    ) -> Option<&'a RequirementExpression<'a>> {
        self.requirements
            .get(&requirement)
            .or_else(|| self.parent.and_then(|p| p.get_requirement(requirement)))
    }

    pub fn set_requirement(
        &mut self,
        requirement_key: RequirementKey,
        requirement_expression: RequirementExpression<'a>,
    ) {
        self.requirements
            .insert(requirement_key, requirement_expression);
    }

    /// Add the requirement of "requirement" to "combined_requirement"
    /// useful, for example, to temporarily mark the need to get a certain gossip stone for reaching another check:
    /// `req.combine.requirement(RequirementKey::GossipStoneInSummit, RequirementKey::KnightAcademy_FledgesGift)`
    pub fn combine_requirement(
        &mut self,
        requirement: RequirementKey,
        combined_requirement: RequirementKey,
    ) {
        // TODO: the requirements should really exist in all cases, but should there be
        // a default just in case? But this should be fine since it signals a bug
        let combined = RequirementExpression::And(vec![
            self.get_owned_requirement(requirement).unwrap(),
            self.get_owned_requirement(combined_requirement).unwrap(),
        ]);
        self.requirements.insert(combined_requirement, combined);
    }
}

#[derive(Clone)]
pub enum RequirementExpression<'a> {
    And(Vec<RequirementExpression<'a>>),
    Or(Vec<RequirementExpression<'a>>),
    Item(Item, u8),
    Event(Event),
    Area(Area, TimeOfDay),
    Fixed(bool),
    Option(fn(&Options) -> bool),
    Ref(&'a RequirementExpression<'a>),
}



impl<'a> RequirementExpression<'a> {
    pub fn check(&self, inventory: &Inventory, options: &Options, allowed_tod: TimeOfDay) -> bool {
        match self {
            RequirementExpression::And(exprs) => exprs
                .iter()
                .all(|e| e.check(inventory, options, allowed_tod)),
            RequirementExpression::Or(exprs) => exprs
                .iter()
                .any(|e| e.check(inventory, options, allowed_tod)),
            RequirementExpression::Item(item, count) => inventory.item_count(*item) >= *count,
            RequirementExpression::Event(event) => inventory.has_event(*event),
            RequirementExpression::Area(area, tod) => !inventory
                .get_area_tod(*area)
                .intersection(allowed_tod)
                .intersection(*tod)
                .is_empty(),
            RequirementExpression::Fixed(val) => *val,
            RequirementExpression::Option(f) => f(options),
            RequirementExpression::Ref(req) => req.check(inventory, options, allowed_tod),
        }
    }

    pub fn remove_used_items(
        &self,
        inventory: &Inventory,
        options: &Options,
        items: &mut HashSet<Item>,
    ) {
        // if the requirement is met, don't consider items used
        if !self.check(inventory, options, TimeOfDay::all()) {
            match self {
                RequirementExpression::And(exprs) | RequirementExpression::Or(exprs) => {
                    for expr in exprs.iter() {
                        expr.remove_used_items(inventory, options, items);
                    }
                }
                RequirementExpression::Item(item, ..) => {
                    items.remove(item);
                }
                RequirementExpression::Event(..) => (),
                RequirementExpression::Area(..) => (),
                RequirementExpression::Fixed(..) => (),
                RequirementExpression::Option(..) => (),
                RequirementExpression::Ref(req) => req.remove_used_items(inventory, options, items),
            };
        }
    }

    pub fn owned(&self) -> RequirementExpression<'static> {
        match self {
            RequirementExpression::And(reqs) => 
                RequirementExpression::And(reqs.iter().map(|req| req.owned()).collect()),
            RequirementExpression::Or(reqs) =>
                RequirementExpression::Or(reqs.iter().map(|req| req.owned()).collect()),
            RequirementExpression::Item(item_id, count) => RequirementExpression::Item(
                *item_id, *count),
            RequirementExpression::Event(event_id) => RequirementExpression::Event(*event_id),
            RequirementExpression::Area(area, tod) => RequirementExpression::Area(*area, *tod),
            RequirementExpression::Fixed(value) => RequirementExpression::Fixed(*value),
            RequirementExpression::Option(f) => RequirementExpression::Option(*f),
            RequirementExpression::Ref(inner) => inner.owned(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequirementKey {
    Exit(Exit),
    LogicExit { from: Area, to: Area },
    Location(Location),
    Event(Event),
}

impl From<Exit> for RequirementKey {
    fn from(value: Exit) -> Self {
        Self::Exit(value)
    }
}

impl From<Location> for RequirementKey {
    fn from(value: Location) -> Self {
        Self::Location(value)
    }
}

impl From<Event> for RequirementKey {
    fn from(value: Event) -> Self {
        Self::Event(value)
    }
}

impl <T> From<&T> for RequirementKey where T: Into<RequirementKey> + Copy {
    fn from(value: &T) -> Self {
        (*value).into()
    }
}
