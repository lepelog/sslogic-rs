use std::collections::HashMap;

use crate::Item;

#[derive(Debug, Clone)]
pub struct Inventory {
    inventory: HashMap<Item, usize>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            inventory: HashMap::new(),
        }
    }

    pub fn collect(&mut self, item: Item) {
        let old_count = self.inventory.entry(item).or_insert(0);
        *old_count += 1;
    }

    pub fn remove(&mut self, item: Item) {
        // does nothing if the item isn't in the map or if the owned count is already 0
        self.inventory.entry(item).and_modify(|item_count| {
            if *item_count > 0 {
                *item_count -= 1;
            }
        });
    }

    pub fn has_item(&self, item: Item) -> bool {
        return self.inventory.get(&item).unwrap_or(&0) > &0;
    }

    pub fn get_item_count(&self, item: Item) -> usize {
        *self.inventory.get(&item).unwrap_or(&0)
    }
}
