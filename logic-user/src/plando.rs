use std::collections::{HashMap, HashSet};

use log::debug;
use logic_core::{Explorer, Item, Location, Options, Placement, Requirements};
use rand::{seq::SliceRandom, Rng};

use crate::error::PlacementError;

pub enum ItemOrVacant {
    Item(Item),
    Vacant,
}

#[derive(Debug, Clone, Copy)]
pub enum LocationOrStart {
    Location(Location),
    Start,
}

impl From<Location> for LocationOrStart {
    fn from(value: Location) -> Self {
        LocationOrStart::Location(value)
    }
}

impl LocationOrStart {
    pub fn is_start_or_else(&self, f: impl FnOnce(Location) -> bool) -> bool {
        match self {
            LocationOrStart::Start => true,
            LocationOrStart::Location(loc) => f(*loc),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlandoEntry {
    pub name: &'static str,
    pub count: usize,
    pub items: Vec<(Item, u8)>,
    pub locations: Vec<(LocationOrStart, u8)>,
}

pub fn do_plando<R: Rng>(
    rng: &mut R,
    entries: &mut Vec<PlandoEntry>,
    requirements: &Requirements<'_>,
    placement: &mut Placement,
    options: &Options,
    locations: &mut HashSet<Location>,
    items: &mut HashMap<Item, u8>,
) -> Result<(), PlacementError>{
    // this makes sure that entries with the same item * locations length are after another, but random
    entries.shuffle(rng);
    entries.sort_by(|a, b| {
        (a.items.len() * a.locations.len()).cmp(&(b.items.len() * b.locations.len())).reverse()
    });

    while let Some(mut entry) = entries.pop() {
        debug!("Processing {entry:?}");
        for i in 0..entry.count {
            entry
                .items
                .retain(|(item, _)| items.get(item).map_or(false, |count| *count > 0));
            entry
                .locations
                .retain(|(loc, _)| loc.is_start_or_else(|loc| locations.contains(&loc)));
            let combination_count = entry.items.len() * entry.locations.len();
            let mut possible_combinations = Vec::with_capacity(combination_count);
            for (item_to_place, item_weight) in &entry.items {
                // explore, using all items that aren't placed yet, except the one we want to place
                // assumed fill
                let mut explorer = Explorer::create(requirements, placement, options);
                for (item, item_count) in items.iter() {
                    let count = if item == item_to_place {
                        *item_count - 1
                    } else {
                        *item_count
                    };
                    explorer.inventory.insert_items(*item, count);
                }
                for (loc_or_start, loc_weight) in &entry.locations {
                    if loc_or_start.is_start_or_else(|loc_to_fill| {
                        explorer.can_reach(loc_to_fill)
                    }) {
                        possible_combinations.push((
                            *loc_or_start,
                            *item_to_place,
                            (*item_weight as usize) * (*loc_weight as usize),
                        ));
                    }
                }
            }
            debug!(
                "possible combinations {}/{}",
                possible_combinations.len(),
                combination_count
            );
            if let Ok((loc_to_fill, item_to_place, _)) =
                possible_combinations.choose_weighted(rng, |(_, _, weight)| *weight)
            {
                debug!("place {:?} at {:?}", item_to_place, loc_to_fill);
                *items.get_mut(item_to_place).unwrap() -= 1;
                match loc_to_fill {
                    LocationOrStart::Start => {
                        *placement.initial_items.entry(*item_to_place).or_default() += 1;
                    }
                    LocationOrStart::Location(loc) => {
                        locations.remove(loc);
                        placement.locations.insert(*loc, *item_to_place);
                    }
                }
            } else {
                debug!("Encountered placement error, aborting plando placement");
                return Err(PlacementError::PlandoNoLocation);
            }
        }
    }
    Ok(())
}
