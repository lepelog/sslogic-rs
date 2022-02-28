use crate::logic_files::LogicElement;
use crate::Inventory;
use crate::Item;
use crate::LogicFiles;
use crate::RandomizerOptions;
use crate::{ALL_DUNGEONS, ALL_DUNGEON_ENTRANCES, ALL_PROGRESS_ITEMS};
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::seq::IteratorRandom;
use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use std::collections::{HashMap, HashSet};

pub struct FilledCheck {
    pub name: String,
    pub placed_item: Option<Item>,
    prerandomized_item: Option<Item>,
}

// avoid copying static resources
pub struct ItemRandomizeContext<'a> {
    logic_file: &'a LogicFiles,
    options: &'a RandomizerOptions,
    custom_macros: HashMap<String, LogicElement>,
    pub filled_checks: Vec<FilledCheck>,
}

#[derive(Debug)]
pub enum RandomizerError {
    NoUsefulItem,
}

fn pop_random_item<T>(vec: &mut Vec<T>, rng: &mut Pcg64) -> T {
    vec.swap_remove(rng.gen_range(0..vec.len()))
}

impl<'a> ItemRandomizeContext<'a> {
    pub fn create(logic_file: &'a LogicFiles, options: &'a RandomizerOptions) -> Self {
        ItemRandomizeContext {
            logic_file,
            options,
            custom_macros: HashMap::new(),
            filled_checks: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.custom_macros = HashMap::new();
        self.filled_checks = Vec::new();
    }

    pub fn prepare(&mut self) {
        // entrance connections, vanilla only
        for (dungeon, entrance) in ALL_DUNGEONS.iter().zip(ALL_DUNGEON_ENTRANCES.iter()) {
            // entrances
            self.custom_macros.insert(
                format!("Can Access {}", dungeon),
                LogicElement::MacroRequirement(format!("Can Access {}", entrance)),
            );
            // exits
            self.custom_macros.insert(
                format!("Can Beat {}", entrance),
                LogicElement::MacroRequirement(format!("Can Beat {}", dungeon)),
            );
        }
        // beat game macro
        let mut beat_game =
            "Can Access Sealed Temple & Can Open GOT After Raising & Can Raise Gate of Time"
                .to_owned();
        beat_game.push_str("& Can Beat Skyview");
        beat_game.push_str("& Can Beat Sandship");
        self.custom_macros.insert(
            "Can Access Past".to_owned(),
            LogicElement::parse(&beat_game).unwrap(),
        );
    }

    pub fn random_fill(&mut self) {
        let seed = rand::thread_rng().next_u64();
        println!("seed: {}", seed);
        let mut rng = Pcg64::seed_from_u64(seed);
        let mut try_count = 0;
        while !self.try_random_fill(&mut rng) {
            if try_count % 10000 == 0 {
                println!("no success try {}", try_count);
            }
            try_count += 1;
            self.reset();
            self.prepare();
        }
        println!("success try {}", try_count);
    }

    pub fn try_random_fill(&mut self, rng: &mut Pcg64) -> bool {
        let mut all_progress_items = Vec::new();
        for itm in ALL_PROGRESS_ITEMS {
            if *itm == Item::GratitudeCrystal {
                continue;
            }
            // if *itm == Item::AmberTablet || *itm == Item::RubyTablet || *itm == Item::EmeraldTablet {
            //     continue;
            // }
            for _ in 0..itm.get_orig_game_count() {
                all_progress_items.push(itm.clone());
            }
        }
        let mut remaining_unreachable_locations: Vec<_> = self
            .logic_file
            .location_names
            .iter()
            .map(|n| FilledCheck {
                name: n.to_owned(),
                placed_item: None,
                prerandomized_item: None,
            })
            .collect();
        for check in remaining_unreachable_locations.iter_mut() {
            if check.name.contains(" - Crystal") {
                check.placed_item = Some(Item::GratitudeCrystal);
            }
        }
        for itm in all_progress_items.iter() {
            let mut index = rng.gen_range(0..remaining_unreachable_locations.len());
            while remaining_unreachable_locations
                .get(index)
                .unwrap()
                .placed_item
                .is_some()
            {
                index = rng.gen_range(0..remaining_unreachable_locations.len());
            }
            remaining_unreachable_locations
                .get_mut(index)
                .unwrap()
                .placed_item = Some(*itm);
        }
        let mut inventory = Inventory::new();
        // inventory.collect(Item::AmberTablet);
        // inventory.collect(Item::RubyTablet);
        // inventory.collect(Item::EmeraldTablet);
        let beat_game_cond =
            LogicElement::MacroRequirement("Can Reach and Defeat Demise".to_owned());
        // let beat_game_cond = LogicElement::MacroRequirement("Skyview - Ruby Tablet".to_owned());
        let mut found_new = true;
        while !beat_game_cond.check_requirement_met(
            self.options,
            &inventory,
            &self.logic_file.requirement_map,
            &self.custom_macros,
        ) {
            if !found_new {
                // println!("Could not finish the game, inventory: {:?}", inventory);
                // println!("reachable areas: {}", self.filled_checks.len());
                return false;
            }
            // track reachable
            let mut idx = 0;
            found_new = false;
            while idx < remaining_unreachable_locations.len() {
                // if the unreachable location is now reachable, remove it from the unreachable list and add it to the
                // reachable list, only increase the index if the location can not be reached cause the swap swaps it with the last location
                if self
                    .logic_file
                    .requirement_map
                    .get(&remaining_unreachable_locations[idx].name)
                    .unwrap()
                    .check_requirement_met(
                        self.options,
                        &inventory,
                        &self.logic_file.requirement_map,
                        &self.custom_macros,
                    )
                {
                    let reached_check = remaining_unreachable_locations.swap_remove(idx);
                    if let Some(reached_item) = reached_check.placed_item {
                        inventory.collect(reached_item);
                    }
                    self.filled_checks.push(reached_check);
                    found_new = true;
                } else {
                    idx += 1;
                }
            }
        }
        return true;
    }

    pub fn randomize(&mut self) -> Result<(), RandomizerError> {
        let mut all_progress_items = Vec::new();
        for itm in ALL_PROGRESS_ITEMS {
            if *itm == Item::GratitudeCrystal {
                continue;
            }
            // if *itm == Item::AmberTablet || *itm == Item::RubyTablet || *itm == Item::EmeraldTablet {
            //     continue;
            // }
            for _ in 0..itm.get_orig_game_count() {
                all_progress_items.push(itm.clone());
            }
        }
        let seed = rand::thread_rng().next_u64();
        let mut inventory = Inventory::new();
        // inventory.collect(Item::AmberTablet);
        // inventory.collect(Item::EmeraldTablet);
        // inventory.collect(Item::RubyTablet);
        println!("seed: {}", seed);
        let mut rng = Pcg64::seed_from_u64(seed);
        let mut remaining_unreachable_locations: Vec<_> = self
            .logic_file
            .location_names
            .iter()
            .map(|n| FilledCheck {
                name: n.to_owned(),
                placed_item: None,
                prerandomized_item: None,
            })
            .collect();
        for check in remaining_unreachable_locations.iter_mut() {
            if check.name.contains(" - Crystal") {
                check.prerandomized_item = Some(Item::GratitudeCrystal);
            }
        }
        let mut remaining_reachable_locations = Vec::new();
        while all_progress_items.len() > 0 {
            // first, make sure to update reachable
            let mut idx = 0;
            let mut found_new_prerandomized = false;
            while idx < remaining_unreachable_locations.len() {
                // if the unreachable location is now reachable, remove it from the unreachable list and add it to the
                // reachable list, only increase the index if the location can not be reached cause the swap swaps it with the last location
                if self
                    .logic_file
                    .requirement_map
                    .get(&remaining_unreachable_locations[idx].name)
                    .unwrap()
                    .check_requirement_met(
                        self.options,
                        &inventory,
                        &self.logic_file.requirement_map,
                        &self.custom_macros,
                    )
                {
                    let mut reached_check = remaining_unreachable_locations.swap_remove(idx);
                    // if this has a prerandomized item, collect it and that location is done
                    if let Some(item) = reached_check.prerandomized_item {
                        inventory.collect(item.clone());
                        reached_check.placed_item = Some(item);
                        self.filled_checks.push(reached_check);
                        found_new_prerandomized = true;
                    } else {
                        remaining_reachable_locations.push(reached_check);
                    }
                } else {
                    idx += 1;
                }
            }

            // a prerandomized item might open up new locations, so continue
            if found_new_prerandomized {
                continue;
            }

            if remaining_reachable_locations.len() == 0 {
                println!("{:?}", inventory);
            }

            // find a random reachable check
            let mut check_to_place = pop_random_item(&mut remaining_reachable_locations, &mut rng);

            // place whatever over 10 open checks, otherwise place something useful
            let next_chosen_item = if remaining_reachable_locations.len() < 10 {
                all_progress_items.shuffle(&mut rng);
                if let Some(item) = self.get_and_remove_first_useful_item(
                    &mut all_progress_items,
                    &inventory,
                    &remaining_unreachable_locations,
                ) {
                    item
                } else if remaining_reachable_locations.len() > 1 {
                    pop_random_item(&mut all_progress_items, &mut rng)
                } else {
                    return Err(RandomizerError::NoUsefulItem);
                }
            } else {
                pop_random_item(&mut all_progress_items, &mut rng)
            };

            inventory.collect(next_chosen_item);

            // println!("placed {:?} at {}", next_chosen_item, check_to_place.name);

            check_to_place.placed_item = Some(next_chosen_item);

            self.filled_checks.push(check_to_place);
        }
        Ok(())
    }

    fn get_and_remove_first_useful_item(
        &self,
        items: &mut Vec<Item>,
        inventory: &Inventory,
        unreachable_locations: &Vec<FilledCheck>,
    ) -> Option<Item> {
        let mut idx = 0;
        let mut copied_inv: Inventory = inventory.clone();
        while idx < items.len() {
            let current_item = items[idx];
            copied_inv.collect(current_item);
            if unreachable_locations.iter().any(|loc| {
                loc.prerandomized_item.is_none()
                    && self
                        .logic_file
                        .requirement_map
                        .get(&loc.name)
                        .unwrap()
                        .check_requirement_met(
                            self.options,
                            &copied_inv,
                            &self.logic_file.requirement_map,
                            &self.custom_macros,
                        )
            }) {
                return Some(items.swap_remove(idx));
            }

            copied_inv.remove(current_item);
            idx += 1;
        }
        None
    }
}
