use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::{
    generated::items::Item,
    generated::logic::{Area, Entrance, Event, Exit, Location},
    logic_static::{
        AreaBitset, BitSetCompatible, EventBitset, ItemCollection, LocationBitset, TimeOfDay,
    },
    requirements::{Requirements}, Options,
};

#[derive(Debug, Clone, Default)]
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
    pub fn insert_item(&mut self, item: Item) {
        self.items.collect(item);
    }
    pub fn insert_items(&mut self, item: Item, count: u8) {
        self.items.collect_multiple(item, count);
    }
    pub fn has_event(&self, event: Event) -> bool {
        self.events.has(event)
    }
    pub fn insert_event(&mut self, event: Event) {
        self.events.insert(event);
    }
}

#[derive(Clone)]
pub struct Placement {
    pub initial_items: HashMap<Item, usize>,
    pub initial_events: EventBitset,
    pub initial_entrance: Option<(Entrance, TimeOfDay)>,
    pub locations: HashMap<Location, Item>,
    pub connections: HashMap<Exit, Entrance>,
    pub rev_connections: HashMap<Entrance, Exit>,
}

impl Placement {
    pub fn new() -> Self {
        Placement {
            connections: Default::default(),
            initial_entrance: None,
            initial_events: Default::default(),
            initial_items: Default::default(),
            locations: Default::default(),
            rev_connections: Default::default(),
        }
    }
    pub fn collect_initial_item(&mut self, item: Item) {
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

    pub fn set_location(&mut self, location: Location, item: Item) {
        let prev = self.locations.insert(location, item);
        // TODO: maybe should be an error instead
        assert!(prev.is_none());
    }

    pub fn get_initial_inventory(&self) -> Inventory {
        let mut inventory = Inventory::default();
        inventory.events.clone_from(&self.initial_events);
        if let Some((entrance, tod)) = self.initial_entrance {
            inventory.insert_area_tod(entrance.area(), tod);
        }
        for (item, count) in &self.initial_items {
            // TODO: clamp instead of unwrap
            inventory.insert_items(*item, (*count).try_into().unwrap());
        }
        inventory
    }
}

#[derive(Clone)]
pub struct Explorer<'a> {
    pub inventory: Inventory,
    pub collected_locations: LocationBitset,
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
            inventory: placement.get_initial_inventory(),
            collected_locations: Default::default(),
            requirements,
            placement,
            options,
        }
    }

    pub fn insert_item(&mut self, item: Item) {
        self.inventory.insert_item(item);
    }

    pub fn insert_area_tod(&mut self, area: Area, tod: TimeOfDay) {
        self.inventory.insert_area_tod(area, tod);
    }

    fn explore_areas(&mut self) -> bool {
        explore_areas(self.requirements, self.placement, self.options, &mut self.inventory)
    }

    fn collect_events(&mut self) -> bool {
        collect_events(self.requirements, self.options, &mut self.inventory)
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

    fn explore(&mut self) -> bool {
        let mut explored_new = self.explore_areas();
        explored_new |= self.collect_items();
        explored_new |= self.collect_events();
        explored_new
    }

    pub fn can_reach(&mut self, location: Location) -> bool {
        loop {
            if self.requirements.check(
                location.requirement_key(),
                &self.inventory,
                self.options,
                TimeOfDay::all(),
            ) {
                return true;
            }
            if !self.explore() {
                // nothing new found, check is unreachable
                return false;
            }
        }
    }
}

#[derive(Clone)]
pub struct SphereExplorer<'a> {
    pub inventory: Inventory,
    pub collected_locations: LocationBitset,
    spheres: Vec<Vec<(Location, Item)>>,
    requirements: &'a Requirements<'a>,
    placement: &'a Placement,
    options: &'a Options,
}

pub fn collect_events(requirements: &Requirements<'_>, options: &Options, inventory: &mut Inventory) -> bool {
    let mut did_change = false;
    'outer: for event in Event::ALL {
        if !inventory.has_event(*event) {
            for requirement in event.requirements() {
                if requirements.check(*requirement, &inventory, options, TimeOfDay::all()) {
                    inventory.insert_event(*event);
                    did_change = true;
                    continue 'outer;
                }
            }
        }
    }
    did_change
}

pub fn explore_areas(requirements: &Requirements<'_>, placement: &Placement, options: &Options, inventory: &mut Inventory) -> bool {
    let mut did_change = false;
    for &area in Area::ALL {
        let already_reached_tod = inventory.get_area_tod(area);
        let possible_tod = area.possible_tod();
        if already_reached_tod.contains(possible_tod) {
            // this area is already fully accessible, no need to check it further
            continue;
        }

        // try to sleep
        if !already_reached_tod.is_empty() && area.can_sleep() {
            inventory.reachable_areas_day.insert(area);
            inventory.reachable_areas_night.insert(area);
            did_change = true;
            // we now have reached everything possible, no need to check more
            continue;
        }

        // check all entrances to this area
        for entrance in area.entrances() {
            if let Some(exit) = placement.rev_connections.get(&entrance) {
                // we check this area for both ToD, to account for forced ToD changes
                // if the area we're currently checking has a forced ToD, we don't care what ToD
                // the exit is taken
                if !possible_tod.is_all() {
                    // ToD is forced
                    // check if this area can be entered, ToD of the previous area doesn't matter
                    if requirements.check(
                        exit.requirement_key(),
                        &inventory,
                        options,
                        TimeOfDay::all(),
                    ) {
                        inventory.insert_area_tod(area, possible_tod);
                        did_change = true;
                    }
                } else {
                    // ToD is not forced
                    // check for both day and night if the exit can be taken
                    for tod in [TimeOfDay::Day, TimeOfDay::Night] {
                        // only check logic if the tod isn't already reached
                        if !already_reached_tod.contains(tod) {
                            if requirements.check(
                                exit.requirement_key(),
                                &inventory,
                                options,
                                tod,
                            ) {
                                // We might be able to save another area collection iteration by trying
                                // to sleep immediately
                                if area.can_sleep() {
                                    inventory.insert_area_tod(area, TimeOfDay::all());
                                } else {
                                    inventory.insert_area_tod(area, tod);
                                }
                                did_change = true;
                            }
                        }
                    }
                }
            }
        }

        for (_exit_area, req_key) in area.logic_entrances() {
            // ToD is not forced
            // check for both day and night if the exit can be taken
            for tod in [TimeOfDay::Day, TimeOfDay::Night] {
                // only check logic if the tod isn't already reached
                if !already_reached_tod.contains(tod) {
                    if requirements
                        .check(*req_key, &inventory, options, tod)
                    {
                        if area.can_sleep() {
                            inventory.insert_area_tod(area, TimeOfDay::all());
                        } else {
                            inventory.insert_area_tod(area, tod);
                        }
                        did_change = true;
                    }
                }
            }
        }
    }
    did_change
}

pub fn collect_spheres(requirements: &Requirements<'_>, placement: &Placement, options: &Options) -> Vec<Vec<(Location, Item)>> {
    let mut inventory = placement.get_initial_inventory();
    let mut collected_locations = LocationBitset::new();

    let mut spheres = Vec::new();
    let mut overall_did_change = true;
    while overall_did_change {
        overall_did_change = false;
        let mut cur_sphere = Vec::new();
        // fully explore areas
        while explore_areas(requirements, placement, options, &mut inventory) {
            overall_did_change = true;
        }
        // fully collect events
        while collect_events(requirements, options, &mut inventory) {
            overall_did_change = true;
        }
        // all locations/item reachable now are the current sphere
        for (location, item) in &placement.locations {
            // only consider new, reachable locations
            if !collected_locations.has(*location)
                && requirements.check(
                    location.requirement_key(),
                    &inventory,
                    options,
                    TimeOfDay::all(),
                )
            {
                collected_locations.insert(*location);
                cur_sphere.push((*location, *item));
            }
        }
        if !cur_sphere.is_empty() {
            overall_did_change = true;
            for (_, item) in &cur_sphere {
                inventory.insert_item(*item);
            }
            spheres.push(cur_sphere);
        }
    }
    spheres
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

// fn fast_rand_remove<T: Sized, R: Rng>(v: &mut Vec<T>, rng: &mut R) -> Option<T> {
//     if v.len() == 0 {
//         None
//     } else {
//         Some(v.swap_remove(rng.gen_range(0..v.len())))
//     }
// }

// fn place(
//     restrictions: &mut Vec<PlacementRestriction>,
//     placement: &mut Placement,
//     locations: &mut HashSet<Location>,
//     items: &mut HashMap<Item, u8>,
//     requirements: &Requirements,
//     options: &Options,
// ) {
//     let mut rng = thread_rng();

//     // TODO: restriction shuffle and order
//     // first, place restrictions
//     while let Some(mut restriction) = restrictions.pop() {
//         // remove all locations that are not in the list anymore
//         restriction
//             .locations
//             .retain(|(loc, _)| loc.is_start_or_else(|loc| locations.contains(&loc)));
//         // remove all items that are not in the list anymore
//         restriction
//             .items
//             .retain(|(item, _)| items.contains_key(item));

//         if restriction.locations.is_empty() || restriction.items.is_empty() {
//             if restriction.error_if_unmet {
//                 panic!("Unmet :(, should be an error");
//             } else {
//                 println!("unmet, continue");
//                 continue;
//             }
//         }
//         // get all possible combinations
//         let mut possible_combinations = Vec::new();
//         for (loc_to_fill_or_start, loc_weight) in restriction.locations.iter() {
//             for (item_to_place, item_weight) in restriction.items.iter() {
//                 if loc_to_fill_or_start.is_start_or_else(|loc_to_fill| {
//                     // explore, using all items that aren't placed yet, except the one we want to place
//                     // assumed fill
//                     let mut explorer = Explorer::create(requirements, placement, options);
//                     for (item, item_count) in items.iter() {
//                         let count = if item == item_to_place {
//                             *item_count - 1
//                         } else {
//                             *item_count
//                         };
//                         explorer.inventory.items.collect_multiple(*item, count);
//                     }
//                     explorer.can_reach(loc_to_fill)
//                 }) {
//                     possible_combinations.push((
//                         *loc_to_fill_or_start,
//                         *item_to_place,
//                         (*loc_weight as usize) * (*item_weight as usize),
//                     ));
//                 }
//             }
//         }
//         if let Ok((chosen_loc, chosen_item, _)) =
//             possible_combinations.choose_weighted(&mut rng, |(_, _, weight)| *weight)
//         {
//             // we selected a possible combination, remove the location from the available ones
//             // remove placed item, dropping keys if their value reaches 0
//             match items.entry(*chosen_item) {
//                 Entry::Occupied(mut val) => {
//                     if *val.get() >= 1 {
//                         *val.get_mut() -= 1;
//                     } else {
//                         val.remove();
//                     }
//                 }
//                 _ => (),
//             }
//             match chosen_loc {
//                 LocOrStart::Location(loc) => {
//                     locations.remove(loc);
//                     placement.locations.insert(*loc, *chosen_item);
//                 }
//                 LocOrStart::StartItem => {
//                     // TODO: inventory isn't really the right place for this,
//                     // as item duplicates might not be accounted for
//                     placement.initial_inventory.items.collect(*chosen_item);
//                 }
//             }
//         } else {
//             println!("sad");
//         }
//     }
// }

// fn test(requirements: &Requirements, placement: &Placement, options: &Options) -> bool {
//     let mut explorer = Explorer::create(requirements, placement, options);
//     explorer.can_reach(Location::SealedGrounds_ChestInsideSealedTemple)
// }
