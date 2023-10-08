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

    pub fn loc(&self) -> Option<&Location> {
        match self {
            LocationOrStart::Start => None,
            LocationOrStart::Location(loc) => Some(loc),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementEntryErrorHandling {
    /// aka already collected
    Vacant,
    /// just let a normal item be randomized there
    Ignore,
    /// cause the randomization to fail
    Error,
}

#[derive(Debug, Clone)]
pub enum PlandoEntry {
    // single item single location
    // does not allow a different item at the target location
    // (not even for plando)
    // these are never used for plando
    Fixed {
        name: &'static str,
        item: Item,
        location: Location,
        // instead of erroring because the item can't be placed,
        // make the spot vacant instead
        use_vacant: bool,
    },
    // description for this entry (for example "dungeon progress", "vanilla crystals")
    Flex {
        name: &'static str,
        count: usize,
        // list of items and their weights, when placing the same item multiple times, it has to appear multiple
        // times in this list as well
        items: Vec<(Item, u8)>,
        // list of locations and their weights
        locations: Vec<(LocationOrStart, u8)>,
        // maybe just make it an enum "source"?
        // if this entry originates from a plando
        is_plando: bool,
    },
}

impl PlandoEntry {
    pub fn get_name(&self) -> &'static str {
        match self {
            PlandoEntry::Fixed { name, .. } => name,
            PlandoEntry::Flex { name, .. } => name,
        }
    }
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
    pub fn add_fixed(
        &mut self,
        name: &'static str,
        item: Item,
        location: Location,
        vacant_possible: bool,
    ) {
        self.push(PlandoEntry::Fixed {
            name,
            item,
            location,
            use_vacant: vacant_possible,
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
        self.push(PlandoEntry::Flex {
            name,
            count,
            items: repeat((item, 1)).take(count).collect(),
            locations,
            is_plando: false,
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
    setting_filled_locations: &mut HashSet<Location>,
    // although there can be copies of items, it doesn't matter
    // if an item got setting placed once or multiple times
    setting_placed_items: &mut HashSet<Item>,
    progress_locations: &HashSet<Location>,
    progress_items: &HashSet<Item>,
) -> Result<(), PlacementError> {
    // this makes sure that entries with the same item * locations length are after another, but random
    fn rate_entry(entry: &PlandoEntry) -> usize {
        match entry {
            PlandoEntry::Fixed { .. } => 3,
            PlandoEntry::Flex {
                items, locations, ..
            } => {
                if items.len() == 1 && locations.len() == 1 {
                    return 2;
                } else if locations
                    .iter()
                    .all(|(loc, _)| *loc == LocationOrStart::Start)
                {
                    return 1; // then startitems
                } else {
                    return 0;
                }
            }
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
        debug!("Processing {}", entry.get_name());
        // TODO: mark spots that were placed by non plando placements (settings)
        // TODO: mark items that were placed by non plando
        let (name, count, mut entry_items, mut entry_locations, error_handling, is_plando): (
            &str,
            usize,
            Vec<(Item, u8)>,
            Vec<(LocationOrStart, u8)>,
            PlacementEntryErrorHandling,
            bool,
        ) = match entry {
            PlandoEntry::Fixed {
                name,
                item,
                location,
                use_vacant,
            } => (
                name,
                1,
                vec![(item, 1)],
                vec![(LocationOrStart::Location(location), 1)],
                if use_vacant {
                    PlacementEntryErrorHandling::Vacant
                } else {
                    PlacementEntryErrorHandling::Error
                },
                false,
            ),
            PlandoEntry::Flex {
                name,
                count,
                items,
                locations,
                is_plando,
            } => (
                name,
                count,
                items,
                locations,
                PlacementEntryErrorHandling::Ignore,
                is_plando,
            ),
        };
        for placed_count in 0..count {
            let mut unplaced_items = entry_items.clone();
            let mut unplaced_locations = entry_locations.clone();
            unplaced_items.retain(|(item, _)| items.get(item).map_or(false, |count| *count > 0));
            unplaced_locations
                .retain(|(loc, _)| loc.is_start_or_else(|loc| locations.contains(&loc)));
            // if one of item or location were used by another non plando entry and there is now nothing to choose from, error
            if (unplaced_items.is_empty()
                && entry_items
                    .iter()
                    .any(|(item, _)| setting_placed_items.contains(item)))
                || (unplaced_locations.is_empty()
                    && entry_locations
                        .iter()
                        .filter_map(|(l, _)| l.loc())
                        .any(|l| setting_filled_locations.contains(l)))
            {
                println!("setting conflict");
                debug!(
                    "Encountered placement error, either no item or no location left, aborting plando placement"
                );
                debug!(
                    "remaining items: {:?}, remaining locations: {:?}",
                    entry_items, entry_locations
                );
                return Err(PlacementError::PlandoNoLocation);
            }
            // if any of item or location are empty, it wasn't because of another setting
            if unplaced_items.is_empty() || unplaced_locations.is_empty() {
                if is_plando {
                    debug!(
                        "Encountered placement error on plando requirement, aborting plando placement"
                    );
                    debug!(
                        "remaining items: {:?}, remaining locations: {:?}",
                        entry_items, entry_locations
                    );
                    return Err(PlacementError::PlandoNoLocation);
                }
                match error_handling {
                    PlacementEntryErrorHandling::Error => {
                        debug!(
                            "Encountered placement error due to error placement, aborting plando placement"
                        );
                        debug!(
                            "remaining items: {:?}, remaining locations: {:?}",
                            entry_items, entry_locations
                        );
                        return Err(PlacementError::PlandoNoLocation);
                    },
                    PlacementEntryErrorHandling::Ignore => {
                        debug!(
                            "No item or location left, continuing anyways"
                        );
                        break;
                    },
                    PlacementEntryErrorHandling::Vacant => {
                        debug!(
                            "No item or location left, marking location as vacant"
                        );
                        for loc in unplaced_locations.iter().filter_map(|(l, _)| l.loc()) {
                            // TODO actually implement
                            placement.set_location(*loc, Item::GreenRupee);
                        }
                        break;
                    }
                }
            }
            let combination_count = unplaced_items.len() * unplaced_locations.len();
            let mut possible_combinations = Vec::with_capacity(combination_count);
            for (item_to_place, item_weight) in &unplaced_items {
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
                for (loc_or_start, loc_weight) in &unplaced_locations {
                    // if we're placing a progress item, make sure we only consider progress locations
                    if !progress_items.contains(item_to_place)
                        || loc_or_start.is_start_or_else(|l| progress_locations.contains(&l))
                    {
                        if loc_or_start
                            .is_start_or_else(|loc_to_fill| explorer.can_reach(loc_to_fill))
                        {
                            possible_combinations.push((
                                *loc_or_start,
                                *item_to_place,
                                (*item_weight as usize) * (*loc_weight as usize),
                            ));
                        }
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
                // TODO an item could appear multiple times  with different weights
                let item_pos = entry_items
                    .iter()
                    .position(|i| i.0 == *item_to_place)
                    .unwrap();
                entry_items.swap_remove(item_pos);
                let loc_pos = entry_locations
                    .iter()
                    .position(|l| l.0 == *loc_to_fill)
                    .unwrap();
                entry_locations.swap_remove(loc_pos);
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
            // } else if placed_count >= entry.min_count {
            //     debug!("Encountered placement error, but min is reached, continuing");
            //     break;
            } else {
                // we couldn't place an item, check what to do now
                // if error_handling == PlacementEntryErrorHandling::Error {
                // hm, if we could recover that would haven been handled already
                // if error handling is error, there is no recovery
                debug!(
                    "Encountered placement error due to fixed placement, aborting plando placement"
                );
                debug!(
                    "remaining items: {:?}, remaining locations: {:?}",
                    entry_items, entry_locations
                );
                return Err(PlacementError::PlandoNoLocation);
                // } else if is_plando {
                //     // if this is a plando entry we can't fulfill, that's an error
                //     debug!("Encountered placement error from plando entry, aborting plando placement");
                //     debug!(
                //         "remaining items: {:?}, remaining locations: {:?}",
                //         entry_items, entry_locations
                //     );
                //     return Err(PlacementError::PlandoNoLocation);
                // } else {
                //     // this is not a plando entry, and if only plando entries affected this
                // }
            }
        }
    }
    Ok(())
}
