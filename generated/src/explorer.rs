use std::collections::{HashMap, HashSet};

use crate::{generated::{Area, Entrance, Event, Exit, Item, Location, Options}, logic_static::{BitSetCompatible, EventBitset, Inventory, LocationBitset, RequirementKey, Requirements, TimeOfDay}};

#[derive(Clone)]
pub struct Placement {
    pub initial_items: HashMap<Item, usize>,
    pub initial_events: EventBitset,
    pub initial_entrance: Option<(Entrance, TimeOfDay)>,
    pub locations: HashMap<Location, Item>,
    pub connections: HashMap<Exit, Entrance>,
    pub rev_connections: HashMap<Entrance, Vec<Exit>>,
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

    pub fn get_connected_entrance(&self, exit: Exit) -> Option<Entrance> {
        self.connections.get(&exit).copied()
    }

    pub fn connect(&mut self, entrance: Entrance, exit: Exit) {
        let prev = self.connections.insert(exit, entrance);
        // TODO error instead
        assert!(prev.is_none());
        self.rev_connections.entry(entrance).or_default().push(exit);
    }

    pub fn disconnect_exit(&mut self, exit: Exit) -> Option<Entrance> {
        if let Some(entrance_id) = self.connections.remove(&exit) {
            self.rev_connections
                .entry(entrance_id)
                .and_modify(|exits| exits.retain(|e| *e != exit));
            Some(entrance_id)
        } else {
            None
        }
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
            inventory.insert_area_tod(entrance.get().area, tod);
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
    rev_connections: HashMap<Entrance, Exit>,
    item_placement: HashMap<Location, Item>,
    banned_areas: Option<&'a HashSet<Area>>,
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
            banned_areas: None,
            rev_connections: HashMap::new(),
            item_placement: HashMap::new(),
            requirements,
            placement,
            options,
        }
    }

    pub fn set_banned_areas(&'a mut self, banned_areas: &'a HashSet<Area>) {
        self.banned_areas = Some(banned_areas);
    }

    pub fn insert_item(&mut self, item: Item) {
        self.inventory.insert_item(item);
    }

    pub fn insert_items(&mut self, item: Item, count: u8) {
        self.inventory.insert_items(item, count);
    }

    pub fn insert_area_tod(&mut self, area: Area, tod: TimeOfDay) {
        self.inventory.insert_area_tod(area, tod);
    }

    fn explore_areas(&mut self) -> bool {
        explore_areas(
            self.requirements,
            self.placement,
            self.options,
            &mut self.inventory,
            self.banned_areas,
            &self.rev_connections,
        )
    }

    fn collect_events(&mut self) -> bool {
        collect_events(
            self.requirements,
            self.options,
            &mut self.inventory,
        )
    }

    fn collect_items(&mut self) -> bool {
        let mut did_change = false;
        for (location, item) in self
            .placement
            .locations
            .iter()
            .chain(self.item_placement.iter())
        {
            // only consider new, reachable locations
            if !self.collected_locations.has(*location)
                && self.requirements.check(
                    location.into(), // TODO: use into
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

    pub fn max_explore(&mut self) {
        while self.explore() {}
    }

    fn explore(&mut self) -> bool {
        let mut explored_new = self.explore_areas();
        explored_new |= self.collect_items();
        explored_new |= self.collect_events();
        explored_new
    }

    // TODO: impl into RequirementKey
    pub fn can_reach(&mut self, location: Location) -> bool {
        self.can_reach_requirement(RequirementKey::Location(location))
    }

    pub fn can_reach_area(&mut self, area: Area) -> TimeOfDay {
        let full_tod = TimeOfDay::from_force_tod(area.get().time_of_day);
        loop {
            let tod = self.inventory.get_area_tod(area);
            if tod.contains(full_tod) {
                return tod;
            }
            if !self.explore() {
                // nothing new found, area is max reached
                return tod;
            }
        }
    }

    pub fn can_reach_requirement(&mut self, req: RequirementKey) -> bool {
        loop {
            if self
                .requirements
                .check(req, &self.inventory, self.options, TimeOfDay::all())
            {
                return true;
            }
            if !self.explore() {
                // nothing new found, check is unreachable
                return false;
            }
        }
    }

    pub fn get_unreachable(&mut self) -> Option<Location> {
        for location in Location::ALL.iter() {
            if !self.can_reach(*location) {
                return Some(*location);
            }
        }
        None
    }

    pub fn set_connection(&mut self, exit: Exit, entrance: Entrance) {
        self.rev_connections.insert(entrance, exit);
    }

    pub fn set_location(&mut self, location: Location, item: Item) {
        self.item_placement.insert(location, item);
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

pub fn collect_events(
    requirements: &Requirements<'_>,
    options: &Options,
    inventory: &mut Inventory,
) -> bool {
    let mut did_change = false;
    'outer: for event in Event::ALL.iter() {
        if !inventory.has_event(*event) {
            // TODO: previously events had multiple requirements, not actually needed?
            if requirements.check(event.into(), &inventory, options, TimeOfDay::all()) {
                inventory.insert_event(*event);
                did_change = true;
                continue 'outer;
            }
        }
    }
    did_change
}

pub fn explore_areas(
    requirements: &Requirements<'_>,
    placement: &Placement,
    options: &Options,
    inventory: &mut Inventory,
    banned_areas: Option<&HashSet<Area>>,
    rev_connections: &HashMap<Entrance, Exit>,
) -> bool {
    let mut did_change = false;
    for area in Area::ALL.iter() {
        if let Some(banned) = banned_areas {
            if banned.contains(area) {
                continue;
            }
        }
        // println!("checking {} - {}", area.stage.name(ctx), area.name);
        let already_reached_tod = inventory.get_area_tod(*area);
        let possible_tod = TimeOfDay::from_force_tod(area.get().time_of_day);
        if already_reached_tod.contains(possible_tod) {
            // this area is already fully accessible, no need to check it further
            continue;
        }

        // try to sleep
        if !already_reached_tod.is_empty() && area.get().can_sleep {
            inventory.reachable_areas_day.insert(*area);
            inventory.reachable_areas_night.insert(*area);
            did_change = true;
            // we now have reached everything possible, no need to check more
            continue;
        }

        // check all entrances to this area
        for entrance in area.get().map_entrances.iter() {
            for exit in placement
                .rev_connections
                .get(&entrance)
                .map(|e| e.as_slice())
                .unwrap_or_default()
                .iter()
                .chain(rev_connections.get(&entrance))
                .copied()
            {
                // we check this area for both ToD, to account for forced ToD changes
                // if the area we're currently checking has a forced ToD, we don't care what ToD
                // the exit is taken
                if !possible_tod.is_all() {
                    // ToD is forced
                    // check if this area can be entered, ToD of the previous area doesn't matter
                    if requirements.check(Into::into(exit), &inventory, options, TimeOfDay::all()) {
                        inventory.insert_area_tod(*area, possible_tod);
                        did_change = true;
                    }
                } else {
                    // ToD is not forced
                    // check for both day and night if the exit can be taken
                    for tod in [TimeOfDay::Day, TimeOfDay::Night] {
                        // only check logic if the tod isn't already reached
                        if !already_reached_tod.contains(tod) {
                            if requirements.check(exit.into(), &inventory, options, tod) {
                                // We might be able to save another area collection iteration by trying
                                // to sleep immediately
                                if area.get().can_sleep {
                                    inventory.insert_area_tod(*area, TimeOfDay::all());
                                } else {
                                    inventory.insert_area_tod(*area, tod);
                                }
                                did_change = true;
                            }
                        }
                    }
                }
            }
        }

        for exit_area in area.get().logic_entrances.iter() {
            // ToD is not forced
            // check for both day and night if the exit can be taken
            for tod in [TimeOfDay::Day, TimeOfDay::Night] {
                // only check logic if the tod isn't already reached
                if !already_reached_tod.contains(tod) {
                    if requirements.check(
                        RequirementKey::LogicExit {
                            from: *exit_area,
                            to: *area,
                        },
                        &inventory,
                        options,
                        tod,
                    ) {
                        if area.get().can_sleep {
                            inventory.insert_area_tod(*area, TimeOfDay::all());
                        } else {
                            inventory.insert_area_tod(*area, tod);
                        }
                        did_change = true;
                    }
                }
            }
        }
    }
    did_change
}

pub fn collect_spheres(
    requirements: &Requirements<'_>,
    placement: &Placement,
    options: &Options,
) -> Vec<Vec<(Location, Item)>> {
    let mut inventory = placement.get_initial_inventory();
    let mut collected_locations = LocationBitset::new();

    let mut spheres = Vec::new();
    let mut overall_did_change = true;
    while overall_did_change {
        overall_did_change = false;
        let mut cur_sphere = Vec::new();
        // fully explore areas
        while explore_areas(
            requirements,
            placement,
            options,
            &mut inventory,
            None,
            &HashMap::new(),
        ) {
            overall_did_change = true;
        }
        // fully collect events
        while collect_events(requirements, options, &mut inventory) {
            overall_did_change = true;
        }
        // all locations/item reachable now are the current sphere
        for (&location, &item) in &placement.locations {
            // only consider new, reachable locations
            if !collected_locations.has(location)
                && requirements.check(location.into(), &inventory, options, TimeOfDay::all())
            {
                collected_locations.insert(location);
                cur_sphere.push((location, item));
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
