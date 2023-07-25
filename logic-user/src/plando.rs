use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
    ops::{Deref, DerefMut},
};

use log::debug;
use logic_core::{options::Options, Explorer, Item, Location, Placement, Requirements};
use rand::{seq::SliceRandom, Rng};

use crate::error::PlacementError;

pub enum ItemOrVacant {
    Item(Item),
    Vacant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub min_count: usize,
    pub max_count: usize,
    pub items: Vec<(Item, u8)>,
    pub locations: Vec<(LocationOrStart, u8)>,
}

#[derive(Debug, Default)]
pub struct PlandoEntries(Vec<PlandoEntry>);

impl Deref for PlandoEntries {
    type Target = Vec<PlandoEntry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlandoEntries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PlandoEntries {
    pub fn add_fixed(&mut self, name: &'static str, item: Item, location: Location) {
        self.push(PlandoEntry {
            name,
            min_count: 1,
            max_count: 1,
            items: vec![(item, 1)],
            locations: vec![(location.into(), 1)],
        });
    }

    pub fn add_area_restricted(
        &mut self,
        name: &'static str,
        item: Item,
        locations: impl IntoIterator<Item = Location>,
        count: usize,
    ) {
        let locations = locations.into_iter().map(|loc| (loc.into(), 1)).collect();
        self.push(PlandoEntry {
            name,
            min_count: count,
            max_count: count,
            items: repeat((item, 1)).take(count).collect(),
            locations,
        });
    }
}

pub fn do_plando<R: Rng>(
    rng: &mut R,
    entries: &mut Vec<PlandoEntry>,
    requirements: &Requirements<'_>,
    placement: &mut Placement,
    options: &Options,
    locations: &mut HashSet<Location>,
    items: &mut HashMap<Item, u8>,
) -> Result<(), PlacementError> {
    // this makes sure that entries with the same item * locations length are after another, but random
    fn rate_entry(entry: &PlandoEntry) -> usize {
        if entry.items.len() == 1 && entry.locations.len() == 1 {
            return 2; // highest prio
        } else if entry
            .locations
            .iter()
            .all(|(loc, _)| *loc == LocationOrStart::Start)
        {
            return 1; // then startitems
        } else {
            return 0;
        }
    }
    entries.shuffle(rng);
    entries.sort_by(|a, b| rate_entry(a).cmp(&rate_entry(b)));
    // entries.sort_by(|a, b| {
    //     (a.items.len() * a.locations.len())
    //         .cmp(&(b.items.len() * b.locations.len()))
    //         .reverse()
    // });

    while let Some(mut entry) = entries.pop() {
        // debug!("Processing {entry:?}");
        debug!("Processing {}", entry.name);
        for placed_count in 0..entry.max_count {
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
                    if loc_or_start.is_start_or_else(|loc_to_fill| explorer.can_reach(loc_to_fill))
                    {
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
                // remove now filled location and item from entry
                let item_pos = entry
                    .items
                    .iter()
                    .position(|i| i.0 == *item_to_place)
                    .unwrap();
                entry.items.swap_remove(item_pos);
                let loc_pos = entry
                    .locations
                    .iter()
                    .position(|l| l.0 == *loc_to_fill)
                    .unwrap();
                entry.locations.swap_remove(loc_pos);
                // remove now filled location and item from pool
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
            } else if placed_count >= entry.min_count {
                debug!("Encountered placement error, but min is reached, continuing");
                break;
            } else {
                debug!("Encountered placement error, aborting plando placement");
                debug!(
                    "remaining items: {:?}, remaining locations: {:?}",
                    entry.items, entry.locations
                );
                return Err(PlacementError::PlandoNoLocation);
            }
        }
    }
    Ok(())
}
