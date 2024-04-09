use std::{marker::PhantomData, sync::atomic::AtomicUsize};

use bitflags::bitflags;

use crate::{
    generated::{items::{Item, COUNTED_ITEM_COUNT, SINGLE_ITEM_COUNT}, logic::{Area, Event, Location, Stage}}, Entrance, Exit
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TimeOfDay: u8 {
        const Day   = 1 << 0;
        const Night = 1 << 1;
        const Both   = Self::Day.bits() | Self::Night.bits();
    }
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

const SLOT_COUNT: usize = SINGLE_ITEM_COUNT / usize::BITS as usize + 1;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ItemCollection {
    slots: [usize; SLOT_COUNT],
    counted: [u8; COUNTED_ITEM_COUNT],
}

impl ItemCollection {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_filled() -> Self {
        ItemCollection {
            slots: [usize::MAX; SLOT_COUNT],
            counted: [u8::MAX; COUNTED_ITEM_COUNT],
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
        if let Some(num) = item_num.checked_sub(SINGLE_ITEM_COUNT) {
            if let Some(count_ref) = self.counted.get_mut(num) {
                *count_ref = count_ref.saturating_add(count);
            }
        } else {
            // if it's a flag we don't have to care about the count
            let item_num = item as usize;
            let slot = item_num / usize::BITS as usize;
            let shift = item_num % usize::BITS as usize;
            self.slots[slot] |= 1 << shift;
        }
    }
    pub fn check(&self, item: Item) -> u8 {
        let item_num = item as usize;
        if let Some(num) = item_num.checked_sub(SINGLE_ITEM_COUNT) {
            self.counted.get(num).copied().unwrap_or_default()
        } else {
            let item_num = item as usize;
            let slot = item_num / usize::BITS as usize;
            let shift = item_num % usize::BITS as usize;
            (self.slots[slot] & (1 << shift) != 0).into()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DoorConnection<T: Copy> {
    No,
    Left(T),
    Right(T),
}

impl<T: Copy> DoorConnection<T> {
    pub fn is_no(&self) -> bool {
        matches!(self, DoorConnection::No)
    }
    pub fn has_left_neighbor(&self) -> bool {
        matches!(self, DoorConnection::Left(_))
    }
    pub fn has_right_neighbor(&self) -> bool {
        matches!(self, DoorConnection::Right(_))
    }
    pub fn get_left_neighbor(&self) -> Option<T> {
        match self {
            DoorConnection::Left(val) => Some(*val),
            _ => None,
        }
    }
    pub fn get_right_neighbor(&self) -> Option<T> {
        match self {
            DoorConnection::Right(val) => Some(*val),
            _ => None,
        }
    }
    pub fn is_left_door(&self) -> bool {
        self.has_right_neighbor()
    }
    pub fn is_right_door(&self) -> bool {
        self.has_left_neighbor()
    }
    pub fn is_opposite<U: Copy>(&self, other: DoorConnection<U>) -> bool {
        match (self, other) {
            (DoorConnection::No, DoorConnection::No) => true,
            (DoorConnection::Left(_), DoorConnection::Right(_)) => true,
            (DoorConnection::Right(_), DoorConnection::Left(_)) => true,
            _ => false,
        }
    }
    pub fn is_same<U: Copy>(&self, other: DoorConnection<U>) -> bool {
        match (self, other) {
            (DoorConnection::No, DoorConnection::No) => true,
            (DoorConnection::Left(_), DoorConnection::Left(_)) => true,
            (DoorConnection::Right(_), DoorConnection::Right(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequirementKey {
    Location(Location),
    Exit(Exit),
    Event(Event),
    LogicExit { from: Area, to: Area },
}

impl Location {
    pub fn requirement_key(&self) -> RequirementKey {
        RequirementKey::Location(*self)
    }
}

impl Exit {
    pub fn requirement_key(&self) -> RequirementKey {
        RequirementKey::Exit(*self)
    }
}

impl Event {
    pub fn requirement_key(&self) -> RequirementKey {
        RequirementKey::Event(*self)
    }
}

impl Area {
    pub fn stage(&self) -> Stage {
        self.get().stage
    }
    pub fn possible_tod(&self) -> TimeOfDay {
        self.get().possible_tod
    }
    pub fn can_sleep(&self) -> bool {
        self.get().can_sleep
    }
    pub fn exits(&self) -> &'static [Exit] {
        self.get().exits
    }
    pub fn entrances(&self) -> &'static [Entrance] {
        self.get().entrances
    }
    pub fn logic_exit_areas(&self) -> &'static [Area] {
        self.get().logic_exit_areas
    }
    pub fn logic_entrance_areas(&self) -> &'static [Area] {
        self.get().logic_entrance_areas
    }
    pub fn locations(&self) -> &'static [Location] {
        self.get().locations
    }
    pub fn logic_exits(&self) -> impl Iterator<Item = (Area, RequirementKey)> + '_ {
        self.logic_exit_areas().iter().map(|to_area| (*to_area, RequirementKey::LogicExit { from: *self, to: *to_area }))
    }
    pub fn logic_entrances(&self) -> impl Iterator<Item = (Area, RequirementKey)> + '_ {
        self.logic_entrance_areas().iter().map(|from_area| (*from_area, RequirementKey::LogicExit { from: *from_area, to: *self }))
    }
}

impl Exit {
    pub fn disambiguation(&self) -> Option<&'static str> {
        self.get().disambiguation
    }
    pub fn vanilla_entrance(&self) -> Entrance {
        self.get().vanilla_entrance
    }
    pub fn coupled_entrance(&self) -> Option<Entrance> {
        self.get().coupled_entrance
    }
    pub fn to(&self) -> Area {
        self.get().to
    }
    pub fn door_connection(&self) -> DoorConnection<Exit> {
        self.get().door_connection
    }
}
