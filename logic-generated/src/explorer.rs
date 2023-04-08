use std::collections::{hash_map::Entry, HashMap, HashSet};

use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    items::Item,
    logic::{Area, Entrance, Exit, Location, Event},
    logic_static::{AreaBitset, BitSetCompatible, ItemCollection, LocationBitset, TimeOfDay, EventBitset},
    requirements::{Options, Requirements},
};

#[derive(Clone)]
pub struct Inventory {
    reachable_areas_day: AreaBitset,
    reachable_areas_night: AreaBitset,
    items: ItemCollection,
    events: EventBitset,
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
    pub fn has_event(&self, event: Event) -> bool {
        self.events.has(event)
    }
}

pub struct Placement {
    pub initial_inventory: Inventory,
    pub initial_items: HashMap<Item, usize>,
    pub locations: HashMap<Location, Item>,
    pub connections: HashMap<Exit, Entrance>,
    pub rev_connections: HashMap<Entrance, Exit>,
}

impl Placement {
    pub fn collect_initial_item(&mut self, item: Item) {
        self.initial_inventory.items.collect(item);
        *self.initial_items.entry(item).or_default() += 1;
    }

    pub fn get_item_at(&self, location: Location) -> Option<Item> {
        self.locations.get(&location).copied()
    }

    pub fn get_connected_exit(&self, entrance: Entrance) -> Option<Exit> {
        self.rev_connections.get(&entrance).copied()
    }

    pub fn get_connected_entrance(&self, exit: Exit) -> Option<Entrance> {
        self.connections.get(&exit).copied()
    }

    pub fn connect(&mut self, entrance: Entrance, exit: Exit) {
        self.connections.insert(exit, entrance);
        self.rev_connections.insert(entrance, exit);
    }
}

pub struct Explorer<'a> {
    inventory: Inventory,
    collected_locations: LocationBitset,
    requirements: &'a Requirements<'a>,
    placement: &'a Placement,
    options: &'a Options,
}

impl<'a> Explorer<'a> {
    pub fn create(
        requirements: &'a Requirements<'a>,
        placement: &'a Placement,
        options: &'a Options,
    ) -> Self {
        Explorer {
            inventory: placement.initial_inventory.clone(),
            collected_locations: Default::default(),
            requirements,
            placement,
            options,
        }
    }

    fn explore_areas(&mut self) -> bool {
        let mut did_change = false;
        // check, if we can reach a new area
        for &area in Area::ALL {
            let already_reached_tod = self.inventory.get_area_tod(area);
            let possible_tod = area.possible_tod();
            if already_reached_tod.contains(possible_tod) {
                // this area is already fully accessible, no need to check it further
                continue;
            }

            // try to sleep
            if !already_reached_tod.is_empty() && area.can_sleep() {
                self.inventory.reachable_areas_day.insert(area);
                self.inventory.reachable_areas_night.insert(area);
                did_change = true;
                // we now have reached everything possible, no need to check more
                continue;
            }

            // check all entrances to this area
            for entrance in area.entrances() {
                if let Some(exit) = self.placement.rev_connections.get(&entrance) {
                    // we check this area for both ToD, to account for forced ToD changes
                    // if the area we're currently checking has a forced ToD, we don't care what ToD
                    // the exit is taken
                    if !possible_tod.is_all() {
                        // ToD is forced
                        // check if this area can be entered, ToD of the previous area doesn't matter
                        if self.requirements.check(
                            exit.requirement_key(),
                            &self.inventory,
                            self.options,
                            TimeOfDay::all(),
                        ) {
                            self.inventory.insert_area_tod(area, possible_tod);
                            did_change = true;
                        }
                    } else {
                        // ToD is not forced
                        // check for both day and night if the exit can be taken
                        for tod in [TimeOfDay::Day, TimeOfDay::Night] {
                            if self.requirements.check(
                                exit.requirement_key(),
                                &self.inventory,
                                self.options,
                                tod,
                            ) {
                                // We might be able to save another area collection iteration by trying
                                // to sleep immediately
                                if area.can_sleep() {
                                    self.inventory.insert_area_tod(area, TimeOfDay::all());
                                } else {
                                    self.inventory.insert_area_tod(area, tod);
                                }
                                did_change = true;
                            }
                        }
                    }
                }
            }

            for (_exit_area, req_key) in area.logic_entrances() {
                // ToD is not forced
                // check for both day and night if the exit can be taken
                for tod in [TimeOfDay::Day, TimeOfDay::Night] {
                    if self
                        .requirements
                        .check(*req_key, &self.inventory, self.options, tod)
                    {
                        if area.can_sleep() {
                            self.inventory.insert_area_tod(area, TimeOfDay::all());
                        } else {
                            self.inventory.insert_area_tod(area, tod);
                        }
                        did_change = true;
                    }
                }
            }
        }
        did_change
    }

    fn collect_items(&mut self) -> bool {
        let mut did_change = false;
        for (location, item) in &self.placement.locations {
            // only consider new, reachable locations
            if !self.collected_locations.has(*location)
                && self.requirements.check(
                    location.requirement_key(),
                    &self.inventory,
                    self.options,
                    TimeOfDay::all(),
                )
            {
                self.collected_locations.insert(*location);
                self.inventory.items.collect(*item);
                did_change = true;
            }
        }
        did_change
    }

    pub fn can_reach(&mut self, location: Location) -> bool {
        loop {
            let explored_new = self.explore_areas();
            let collected_new = self.collect_items();
            if !explored_new && !collected_new {
                // nothing new found, check is unreachable
                return false;
            }
            if self.requirements.check(
                location.requirement_key(),
                &self.inventory,
                self.options,
                TimeOfDay::all(),
            ) {
                return true;
            }
        }
    }
}

enum PlacementRestrictionPrio {
    High,
    Low,
}

#[derive(Clone, Copy)]
enum LocOrStart {
    Location(Location),
    StartItem,
}

impl LocOrStart {
    pub fn is_start_or_else(&self, f: impl FnOnce(Location) -> bool) -> bool {
        match self {
            LocOrStart::StartItem => true,
            LocOrStart::Location(loc) => f(*loc),
        }
    }
}

struct PlacementRestriction {
    high_prio: bool,
    error_if_unmet: bool,
    locations: Vec<(LocOrStart, u8)>,
    items: Vec<(Item, u8)>,
}

fn fast_rand_remove<T: Sized, R: Rng>(v: &mut Vec<T>, rng: &mut R) -> Option<T> {
    if v.len() == 0 {
        None
    } else {
        Some(v.swap_remove(rng.gen_range(0..v.len())))
    }
}

fn place(
    restrictions: &mut Vec<PlacementRestriction>,
    placement: &mut Placement,
    locations: &mut HashSet<Location>,
    items: &mut HashMap<Item, u8>,
    requirements: &Requirements,
    options: &Options,
) {
    let mut rng = thread_rng();

    // TODO: restriction shuffle and order
    // first, place restrictions
    while let Some(mut restriction) = restrictions.pop() {
        // remove all locations that are not in the list anymore
        restriction
            .locations
            .retain(|(loc, _)| loc.is_start_or_else(|loc| locations.contains(&loc)));
        // remove all items that are not in the list anymore
        restriction
            .items
            .retain(|(item, _)| items.contains_key(item));

        if restriction.locations.is_empty() || restriction.items.is_empty() {
            if restriction.error_if_unmet {
                panic!("Unmet :(, should be an error");
            } else {
                println!("unmet, continue");
                continue;
            }
        }
        // get all possible combinations
        let mut possible_combinations = Vec::new();
        for (loc_to_fill_or_start, loc_weight) in restriction.locations.iter() {
            for (item_to_place, item_weight) in restriction.items.iter() {
                if loc_to_fill_or_start.is_start_or_else(|loc_to_fill| {
                    // explore, using all items that aren't placed yet, except the one we want to place
                    // assumed fill
                    let mut explorer = Explorer::create(requirements, placement, options);
                    for (item, item_count) in items.iter() {
                        let count = if item == item_to_place {
                            *item_count - 1
                        } else {
                            *item_count
                        };
                        explorer.inventory.items.collect_multiple(*item, count);
                    }
                    explorer.can_reach(loc_to_fill)
                }) {
                    possible_combinations.push((
                        *loc_to_fill_or_start,
                        *item_to_place,
                        (*loc_weight as usize) * (*item_weight as usize),
                    ));
                }
            }
        }
        if let Ok((chosen_loc, chosen_item, _)) =
            possible_combinations.choose_weighted(&mut rng, |(_, _, weight)| *weight)
        {
            // we selected a possible combination, remove the location from the available ones
            // remove placed item, dropping keys if their value reaches 0
            match items.entry(*chosen_item) {
                Entry::Occupied(mut val) => {
                    if *val.get() >= 1 {
                        *val.get_mut() -= 1;
                    } else {
                        val.remove();
                    }
                }
                _ => (),
            }
            match chosen_loc {
                LocOrStart::Location(loc) => {
                    locations.remove(loc);
                    placement.locations.insert(*loc, *chosen_item);
                }
                LocOrStart::StartItem => {
                    // TODO: inventory isn't really the right place for this,
                    // as item duplicates might not be accounted for
                    placement.initial_inventory.items.collect(*chosen_item);
                }
            }
        } else {
            println!("sad");
        }
    }
}

fn test(requirements: &Requirements, placement: &Placement, options: &Options) -> bool {
    let mut explorer = Explorer::create(requirements, placement, options);
    explorer.can_reach(Location::SealedGrounds_ChestInsideSealedTemple)
}
