use std::collections::{HashMap, HashSet};

use rand::{seq::SliceRandom, Rng};
use snafu::Snafu;

use crate::{explorer::{Explorer, Placement}, generated::{Item, Location, Options}, logic_static::Requirements};

/// The algorithm works as follows:
///
/// first, all plando entries (this also includes entries from settings) are shuffled, but fixed placements get the top spot,
/// then plando placements with only one combinations, then startitems and then the rest
///
/// 1. fixed placements (1 combination)
/// 2. startitems
/// 3. rest
///
/// then every entry gets processed, all possible item/location combinations are checked if the logic works and then chosen by the weight
///
/// if the entry is not possible because another setting (non plando) occupies a locations (for example a key could be in the place of a locations where a sword is placed)
/// or already placed an item (I can't think of a scenario here), then that's an error and we try again
///
/// startitems are an exception, if they lead to an error (and really only if all items are either placed by this setting or startitems) then just fill the locations normally
/// for fixed locations there is also the option to mark the spot as "vacant", so to act as if the check was already obtained
///
/// as soon as an item should be placed that is "tainted" (placed by another setting, again, this isn't anywhere in current logic), error
/// as soon as a location is already filled and "tainted" (filled by another setting, "another" is redundant because if this setting filled it, it wouldn't try to fill it again)
///  we error out, because there is a setting conflict
/// struct WeightedItem

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Cannot place items {items_left} at {impossible_locations}"))]
    NoLocationLeft {
        impossible_locations: String,
        items_left: String,
    },
    #[snafu(display("Locations already filled by another setting: {impossible_locations}"))]
    SettingsConflictLocation { impossible_locations: String },
    #[snafu(display("Items already placed by another setting: {impossible_items}"))]
    SettingsConflictItem { impossible_items: String },
    #[snafu(display("Conflict between settings and plando"))]
    PlandoSettingsConflict,
    #[snafu(display("Conflict between plando entries"))]
    PlandoEntryConflict,
    #[snafu(display("No location item combination possible"))]
    PlandoNoLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LocationOrStart {
    Start,
    Location(Location),
}

impl From<Location> for LocationOrStart {
    fn from(value: Location) -> Self {
        Self::Location(value)
    }
}

impl LocationOrStart {
    pub fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }

    pub fn is_location(&self) -> bool {
        matches!(self, Self::Location(..))
    }

    pub fn as_location(&self) -> Option<Location> {
        match self {
            Self::Start => None,
            Self::Location(loc) => Some(*loc),
        }
    }

    pub fn is_location_and(&self, f: impl FnOnce(Location) -> bool) -> bool {
        match self {
            Self::Start => false,
            Self::Location(location) => f(*location),
        }
    }

    pub fn is_start_or(&self, f: impl FnOnce(Location) -> bool) -> bool {
        match self {
            Self::Start => true,
            Self::Location(location) => f(*location),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ItemOrVacant {
    Vacant,
    Item(Item),
}

impl ItemOrVacant {
    pub fn is_vacant(&self) -> bool {
        matches!(self, Self::Vacant)
    }

    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item(..))
    }

    pub fn as_item(&self) -> Option<Item> {
        match self {
            Self::Vacant => None,
            Self::Item(item) => Some(*item),
        }
    }

    pub fn is_item_and(&self, f: impl FnOnce(Item) -> bool) -> bool {
        match self {
            Self::Vacant => false,
            Self::Item(item) => f(*item),
        }
    }

    pub fn is_vacant_or(&self, f: impl FnOnce(Item) -> bool) -> bool {
        match self {
            Self::Vacant => true,
            Self::Item(item) => f(*item),
        }
    }
}

impl From<Item> for ItemOrVacant {
    fn from(value: Item) -> Self {
        Self::Item(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeightedItem {
    item: ItemOrVacant,
    weight: u8,
}

impl WeightedItem {
    pub fn simple(item: Item) -> Self {
        WeightedItem {
            item: item.into(),
            weight: 1,
        }
    }
    pub fn vacant() -> Self {
        WeightedItem {
            item: ItemOrVacant::Vacant,
            weight: 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeightedLocation {
    location: LocationOrStart,
    weight: u8,
}

impl WeightedLocation {
    pub fn simple(location: Location) -> Self {
        WeightedLocation {
            location: location.into(),
            weight: 1,
        }
    }
    pub fn start() -> Self {
        WeightedLocation {
            location: LocationOrStart::Start,
            weight: 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WeightedCombination {
    item: WeightedItem,
    location: WeightedLocation,
}

pub enum PlandoEntry {
    Fixed {
        item: Item,
        location: Location,
        vacant_fallback: bool,
    },
    Flex {
        items: Vec<WeightedItem>,
        locations: Vec<WeightedLocation>,
        count: usize,
        is_plando: bool,
    },
}

pub fn place_items_plando(
    rng: &mut impl Rng,
    mut plando_entries: Vec<PlandoEntry>,
    requirements: &Requirements<'_>,
    placement: &mut Placement,
    options: &Options,
    locations: &mut HashSet<Location>,
    items: &mut HashMap<Item, u8>,
    progress_locations: &HashSet<Location>,
    progress_items: &HashSet<Item>,
) -> Result<(), Error> {
    let mut settings_placed_items: HashSet<Item> = HashSet::new();
    let mut settings_filled_locations: HashSet<Location> = HashSet::new();
    plando_entries.shuffle(rng);
    // this makes sure that entries with the same item * locations length are near another, but random
    fn rate_entry(entry: &PlandoEntry) -> usize {
        match entry {
            PlandoEntry::Fixed { .. } => 3,
            PlandoEntry::Flex {
                items, locations, ..
            } => {
                if items.len() == 1 && locations.len() == 1 {
                    return 2;
                } else if locations.iter().all(|loc| loc.location.is_start()) {
                    return 1; // then startitems
                } else {
                    return 0;
                }
            }
        }
    }
    plando_entries.sort_by(|a, b| rate_entry(a).cmp(&rate_entry(b)));
    while let Some(current_entry) = plando_entries.pop() {
        let (count, mut entry_items, mut entry_locations, is_plando, allows_vacant) =
            match current_entry {
                PlandoEntry::Fixed {
                    item,
                    location: locations,
                    vacant_fallback,
                } => (
                    1usize,
                    vec![WeightedItem {
                        item: ItemOrVacant::Item(item),
                        weight: 1,
                    }],
                    vec![WeightedLocation {
                        location: LocationOrStart::Location(locations),
                        weight: 1,
                    }],
                    false,
                    vacant_fallback,
                ),
                PlandoEntry::Flex {
                    items,
                    locations,
                    count,
                    is_plando,
                } => (count, items, locations, is_plando, false),
            };
        let mut new_settings_placed_items = Vec::new();
        for placed_count in 0..count {
            let mut unplaced_items: Vec<_> = entry_items
                .iter()
                .filter(|placable| {
                    placable
                        .item
                        .is_vacant_or(|item| *items.get(&item).unwrap_or(&0) > 0)
                })
                .copied()
                .collect();
            let mut unfilled_locations: Vec<_> = entry_locations
                .iter()
                .filter(|place| place.location.is_start_or(|loc| locations.contains(&loc)))
                .copied()
                .collect();
            if unplaced_items.is_empty()
                && entry_items.iter().any(|placable| {
                    placable
                        .item
                        .is_item_and(|item| settings_placed_items.contains(&item))
                })
            {
                return Err(Error::SettingsConflictItem {
                    impossible_items: format!(
                        "{:?}",
                        entry_items
                    ),
                });
            }
            if unfilled_locations.is_empty()
                && entry_locations.iter().any(|place| {
                    place
                        .location
                        .is_location_and(|loc| settings_filled_locations.contains(&loc))
                })
            {
                return Err(Error::SettingsConflictLocation {
                    impossible_locations: format!(
                        "{:?}", entry_locations
                    ),
                });
            }
            if unplaced_items.is_empty() || unfilled_locations.is_empty() {
                // settings errors are already handled, so this has to be because of plando or startitems
                // but if this is a plando entry, error out, since the wish of the user should be respected
                if is_plando {
                    return Err(Error::PlandoEntryConflict);
                } else if allows_vacant {
                    todo!(
                        "this entry allows using vacant instead of the item, not implemented yet"
                    );
                } else {
                    println!("{:?}", unplaced_items);
                    println!("{:?}", unfilled_locations);
                    return Err(Error::PlandoSettingsConflict);
                }
            }
            let combination_count = unplaced_items.len() * unfilled_locations.len();
            let mut possible_combinations = Vec::with_capacity(combination_count);
            for item_or_vacant in unplaced_items.iter() {
                match item_or_vacant.item {
                    ItemOrVacant::Vacant => {
                        // always possible
                        for loc in unfilled_locations.iter() {
                            possible_combinations.push(WeightedCombination {
                                item: *item_or_vacant,
                                location: *loc,
                            });
                        }
                    }
                    ItemOrVacant::Item(item_to_place) => {
                        // explore, using all items that aren't placed yet, except the one we want to place
                        // assumed fill
                        let mut explorer = Explorer::create(requirements, placement, options);
                        for (item, item_count) in items.iter() {
                            let count = if *item == item_to_place {
                                *item_count - 1
                            } else {
                                *item_count
                            };
                            explorer.inventory.insert_items(*item, count);
                        }
                        for loc_or_start in unfilled_locations.iter() {
                            // if we're placing a progress item, make sure we only consider progress locations
                            if !progress_items.contains(&item_to_place)
                                || loc_or_start
                                    .location
                                    .is_start_or(|l| progress_locations.contains(&l))
                            {
                                if loc_or_start
                                    .location
                                    .is_start_or(|loc_to_fill| explorer.can_reach(loc_to_fill))
                                {
                                    possible_combinations.push(WeightedCombination {
                                        location: *loc_or_start,
                                        item: *item_or_vacant,
                                    });
                                }
                            }
                        }
                    }
                }
            }
            if let Ok(combination) = possible_combinations.choose_weighted(rng, |combo| {
                (combo.item.weight as u16) * (combo.location.weight as u16)
            }) {
                // TODO: debug impl
                println!("placing {:?} at {:?}", combination.item.item, combination.location.location);
                // remove it from remaining items
                entry_items.swap_remove(
                    entry_items
                        .iter()
                        .position(|i| i == &combination.item)
                        .unwrap(),
                );
                entry_locations.swap_remove(
                    entry_locations
                        .iter()
                        .position(|l| l == &combination.location)
                        .unwrap(),
                );
                // remove item from pool (if it wasn't a vacant marker)
                match combination.item.item {
                    ItemOrVacant::Item(item) => {
                        if let Some(count) = items.get_mut(&item) {
                            *count = count.saturating_sub(1);
                        }
                        // we don't consider startitems to be errors if we run out of them
                        // delay treating an item as setting based until after each entry
                        if is_plando
                            && combination.location.location.is_location()
                            && !settings_placed_items.contains(&item)
                        {
                            new_settings_placed_items.push(item);
                        }
                    }
                    _ => (),
                }
                // remove location from pool (if it wasn't the start)
                match combination.location.location {
                    LocationOrStart::Location(loc) => {
                        locations.remove(&loc);
                        settings_filled_locations.insert(loc);
                        placement.set_location(loc, combination.item.item.as_item().unwrap());
                    }
                    LocationOrStart::Start => {
                        let count = placement.initial_items.entry(
                            combination.item.item.as_item().unwrap()
                        ).or_default();
                        *count = count.saturating_add(1);
                    },
                }
            } else {
                return Err(Error::PlandoNoLocation);
            }
        }
        settings_placed_items.extend(new_settings_placed_items);
    }
    Ok(())
}
