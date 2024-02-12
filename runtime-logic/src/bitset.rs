use std::marker::PhantomData;

use rust_dense_bitset::DenseBitSetExtended;

use crate::structure::ItemId;

pub trait BitSetCompatible {
    fn to_num(&self) -> u16;
}

#[derive(Clone)]
pub struct BitSet<T: BitSetCompatible> {
    bitset: DenseBitSetExtended,
    _type: PhantomData<T>,
}

impl<T: BitSetCompatible> Default for BitSet<T> {
    fn default() -> Self {
        Self {
            bitset: DenseBitSetExtended::new(),
            _type: PhantomData,
        }
    }
}

impl<T: BitSetCompatible> BitSet<T> {
    pub fn new(bits: usize) -> Self {
        Self {
            bitset: DenseBitSetExtended::with_capacity(bits),
            _type: PhantomData,
        }
    }

    pub fn insert(&mut self, t: T) {
        self.bitset.insert_u64(1, t.to_num() as usize, 1);
    }

    pub fn has(&self, t: T) -> bool {
        self.bitset.extract_u64(t.to_num() as usize, 1) == 1
    }
}

#[derive(Clone)]
pub struct ItemCollection {
    counts: Vec<u8>,
}

impl Default for ItemCollection {
    fn default() -> Self {
        ItemCollection::new()
    }
}

impl ItemCollection {
    pub fn new() -> Self {
        Self {
            counts: vec![0; 255],
        } // todo improve?
    }
    pub fn collect(&mut self, item: ItemId) {
        self.collect_multiple(item, 1);
    }
    pub fn collect_multiple(&mut self, item: ItemId, count: u8) {
        if count == 0 {
            return;
        }
        if let Some(stored) = self.counts.get_mut(item.to_num() as usize) {
            *stored = stored.saturating_add(count);
        }
    }
    pub fn check(&self, item: ItemId) -> u8 {
        self.counts
            .get(item.to_num() as usize)
            .cloned()
            .unwrap_or_default()
    }
}
