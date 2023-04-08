use std::marker::PhantomData;

use bitflags::bitflags;

use crate::{
    items::{Item, COUNTED_ITEM_COUNT, SINGLE_ITEM_COUNT},
    logic::{Area, Location, Stage, Event},
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

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
pub struct ItemCollection {
    slots: [usize; SLOT_COUNT],
    counted: [u8; COUNTED_ITEM_COUNT],
}

impl ItemCollection {
    pub fn new() -> Self {
        ItemCollection {
            slots: Default::default(),
            counted: Default::default(),
        }
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
