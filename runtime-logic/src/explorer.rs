use std::collections::{hash_map::Entry, HashMap, HashSet};

use rust_dense_bitset::DenseBitSetExtended;

use crate::{
    bitset::{BitSet, ItemCollection},
    requirements::Requirements,
    structure::{
        AreaId, ContextLoadable, EntranceId, EventId, ExitId, ItemId, LocationId, LogicContext,
        RequirementKey, TimeOfDay,
    },
    Options,
};

#[derive(Clone, Default)]
pub struct Inventory {
    reachable_areas_day: BitSet<AreaId>,
    reachable_areas_night: BitSet<AreaId>,
    items: ItemCollection,
    events: BitSet<EventId>,
}

impl Inventory {
    pub fn get_area_tod(&self, area: AreaId) -> TimeOfDay {
        let mut tod = TimeOfDay::empty();
        if self.reachable_areas_day.has(area) {
            tod |= TimeOfDay::Day;
        }
        if self.reachable_areas_night.has(area) {
            tod |= TimeOfDay::Night;
        }
        tod
    }
    pub fn insert_area_tod(&mut self, area: AreaId, tod: TimeOfDay) {
        if tod.contains(TimeOfDay::Day) {
            self.reachable_areas_day.insert(area);
        }
        if tod.contains(TimeOfDay::Night) {
            self.reachable_areas_night.insert(area);
        }
    }
    pub fn item_count(&self, item: ItemId) -> u8 {
        self.items.check(item)
    }
    pub fn insert_item(&mut self, item: ItemId) {
        self.items.collect(item);
    }
    pub fn insert_items(&mut self, item: ItemId, count: u8) {
        self.items.collect_multiple(item, count);
    }
    pub fn has_event(&self, event: EventId) -> bool {
        self.events.has(event)
    }
    pub fn insert_event(&mut self, event: EventId) {
        self.events.insert(event);
    }
}

#[derive(Clone)]
pub struct Placement {
    pub initial_items: HashMap<ItemId, usize>,
    pub initial_events: BitSet<EventId>,
    pub initial_entrance: Option<(EntranceId, TimeOfDay)>,
    pub locations: HashMap<LocationId, ItemId>,
    pub connections: HashMap<ExitId, EntranceId>,
    pub rev_connections: HashMap<EntranceId, Vec<ExitId>>,
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
    pub fn collect_initial_item(&mut self, item: ItemId) {
        *self.initial_items.entry(item).or_default() += 1;
    }

    pub fn get_item_at(&self, location: LocationId) -> Option<ItemId> {
        self.locations.get(&location).copied()
    }

    pub fn get_connected_entrance(&self, exit: ExitId) -> Option<EntranceId> {
        self.connections.get(&exit).copied()
    }

    pub fn connect(&mut self, entrance: EntranceId, exit: ExitId) {
        let prev = self.connections.insert(exit, entrance);
        // TODO error instead
        assert!(prev.is_none());
        self.rev_connections.entry(entrance).or_default().push(exit);
    }

    pub fn disconnect_exit(&mut self, exit: ExitId) -> Option<EntranceId> {
        if let Some(entrance_id) = self.connections.remove(&exit) {
            self.rev_connections
                .entry(entrance_id)
                .and_modify(|exits| exits.retain(|e| *e != exit));
            Some(entrance_id)
        } else {
            None
        }
    }

    pub fn set_location(&mut self, location: LocationId, item: ItemId) {
        let prev = self.locations.insert(location, item);
        // TODO: maybe should be an error instead
        assert!(prev.is_none());
    }

    pub fn get_initial_inventory(&self, ctx: &LogicContext) -> Inventory {
        let mut inventory = Inventory::default();
        inventory.events.clone_from(&self.initial_events);
        if let Some((entrance, tod)) = self.initial_entrance {
            inventory.insert_area_tod(entrance.ctx(ctx).area, tod);
        }
        for (item, count) in &self.initial_items {
            // TODO: clamp instead of unwrap
            inventory.insert_items(*item, (*count).try_into().unwrap());
        }
        inventory
    }
}

#[derive(Clone)]
pub struct Explorer<'a, 'ctx> {
    pub inventory: Inventory,
    pub collected_locations: BitSet<LocationId>,
    rev_connections: HashMap<EntranceId, ExitId>,
    item_placement: HashMap<LocationId, ItemId>,
    banned_areas: Option<&'a HashSet<AreaId>>,
    requirements: &'a Requirements<'a>,
    placement: &'a Placement,
    options: &'a Options,
    ctx: &'ctx LogicContext,
}

impl<'a, 'ctx> Explorer<'a, 'ctx> {
    pub fn create(
        requirements: &'a Requirements<'a>,
        placement: &'a Placement,
        options: &'a Options,
        ctx: &'ctx LogicContext,
    ) -> Self {
        Explorer {
            inventory: placement.get_initial_inventory(ctx),
            collected_locations: Default::default(),
            banned_areas: None,
            rev_connections: HashMap::new(),
            item_placement: HashMap::new(),
            requirements,
            placement,
            options,
            ctx,
        }
    }

    pub fn set_banned_areas(&'a mut self, banned_areas: &'a HashSet<AreaId>) {
        self.banned_areas = Some(banned_areas);
    }

    pub fn insert_item(&mut self, item: ItemId) {
        self.inventory.insert_item(item);
    }

    pub fn insert_items(&mut self, item: ItemId, count: u8) {
        self.inventory.insert_items(item, count);
    }

    pub fn insert_area_tod(&mut self, area: AreaId, tod: TimeOfDay) {
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
            self.ctx,
        )
    }

    fn collect_events(&mut self) -> bool {
        collect_events(
            self.requirements,
            self.options,
            &mut self.inventory,
            self.ctx,
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
                    RequirementKey::Location(*location), // TODO: use into
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

    pub fn can_reach(&mut self, location: LocationId) -> bool {
        self.can_reach_requirement(RequirementKey::Location(location))
    }

    pub fn can_reach_area(&mut self, area: AreaId) -> TimeOfDay {
        let full_tod = area.ctx(self.ctx).time_of_day;
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

    pub fn get_unreachable(&mut self) -> Option<LocationId> {
        for location in self.ctx.locations.iter() {
            if !self.can_reach(location.id) {
                return Some(location.id);
            }
        }
        None
    }

    pub fn set_connection(&mut self, exit: ExitId, entrance: EntranceId) {
        self.rev_connections.insert(entrance, exit);
    }

    pub fn set_location(&mut self, location: LocationId, item: ItemId) {
        self.item_placement.insert(location, item);
    }
}

#[derive(Clone)]
pub struct SphereExplorer<'a> {
    pub inventory: Inventory,
    pub collected_locations: BitSet<LocationId>,
    spheres: Vec<Vec<(LocationId, ItemId)>>,
    requirements: &'a Requirements<'a>,
    placement: &'a Placement,
    options: &'a Options,
}

pub fn collect_events(
    requirements: &Requirements<'_>,
    options: &Options,
    inventory: &mut Inventory,
    ctx: &LogicContext,
) -> bool {
    let mut did_change = false;
    'outer: for event in &ctx.events {
        if !inventory.has_event(event.id) {
            // TODO: previously events had multiple requirements, not actually needed?
            if requirements.check(event.id.into(), &inventory, options, TimeOfDay::all()) {
                inventory.insert_event(event.id);
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
    banned_areas: Option<&HashSet<AreaId>>,
    rev_connections: &HashMap<EntranceId, ExitId>,
    ctx: &LogicContext,
) -> bool {
    let mut did_change = false;
    for area in &ctx.areas {
        if let Some(banned) = banned_areas {
            if banned.contains(&area.id) {
                continue;
            }
        }
        // println!("checking {} - {}", area.stage.name(ctx), area.name);
        let already_reached_tod = inventory.get_area_tod(area.id);
        let possible_tod = area.time_of_day;
        if already_reached_tod.contains(possible_tod) {
            // this area is already fully accessible, no need to check it further
            continue;
        }

        // try to sleep
        if !already_reached_tod.is_empty() && area.can_sleep {
            inventory.reachable_areas_day.insert(area.id);
            inventory.reachable_areas_night.insert(area.id);
            did_change = true;
            // we now have reached everything possible, no need to check more
            continue;
        }

        // check all entrances to this area
        for entrance in &area.map_entrances {
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
                        inventory.insert_area_tod(area.id, possible_tod);
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
                                if area.can_sleep {
                                    inventory.insert_area_tod(area.id, TimeOfDay::all());
                                } else {
                                    inventory.insert_area_tod(area.id, tod);
                                }
                                did_change = true;
                            }
                        }
                    }
                }
            }
        }

        for exit_area in &area.logic_entrances {
            // ToD is not forced
            // check for both day and night if the exit can be taken
            for tod in [TimeOfDay::Day, TimeOfDay::Night] {
                // only check logic if the tod isn't already reached
                if !already_reached_tod.contains(tod) {
                    if requirements.check(
                        RequirementKey::LogicExit {
                            from: *exit_area,
                            to: area.id,
                        },
                        &inventory,
                        options,
                        tod,
                    ) {
                        if area.can_sleep {
                            inventory.insert_area_tod(area.id, TimeOfDay::all());
                        } else {
                            inventory.insert_area_tod(area.id, tod);
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
    ctx: &LogicContext,
) -> Vec<Vec<(LocationId, ItemId)>> {
    let mut inventory = placement.get_initial_inventory(ctx);
    let mut collected_locations = BitSet::<LocationId>::new(ctx.locations.len());

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
            ctx,
        ) {
            overall_did_change = true;
        }
        // fully collect events
        while collect_events(requirements, options, &mut inventory, ctx) {
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

#[derive(Clone, Copy)]
enum LocOrStart {
    Location(LocationId),
    StartItem,
}

impl LocOrStart {
    pub fn is_start_or_else(&self, f: impl FnOnce(LocationId) -> bool) -> bool {
        match self {
            LocOrStart::StartItem => true,
            LocOrStart::Location(loc) => f(*loc),
        }
    }
}
