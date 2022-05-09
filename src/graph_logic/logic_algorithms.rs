use std::collections::{HashMap, HashSet};

use rand::prelude::*;

use crate::graph_logic::logic_structs::AllowedToD;

use super::logic_structs::{
    collect_items, Area, AreaKey, CheckKey, Entrance, Exit, Inventory, LocalizedAreaKey, LogicKey,
    LogicKeyMapper, PassagewayKey, Placement, TimeOfDay,
};

#[derive(Debug, Clone)]
pub enum FillError {
    NoValidExitLeft(Entrance),
    NoValidLocationLeft(LogicKey),
}

pub fn place_entrances_decoupled<R: Rng>(
    rng: &mut R,
    areas: &HashMap<AreaKey, Area>,
    placement: &mut Placement,
    inventory: &Inventory,
    entrances_to_place: &mut Vec<Entrance>,
    exits_to_place: &mut Vec<Exit>,
    logic_key_mapper: &LogicKeyMapper,
) -> Result<(), FillError> {
    assert_eq!(entrances_to_place.len(), exits_to_place.len());
    entrances_to_place.shuffle(rng);
    exits_to_place.shuffle(rng);
    // assumed fill, assume all entrances are reachable at the start
    // cache all entrances and their supported ToD here, to avoid area lookups later
    let mut unconnected_entrances_with_tod = HashMap::new();
    for entrance in entrances_to_place.iter() {
        let area = areas.get(&entrance.area).unwrap();
        unconnected_entrances_with_tod.insert(entrance.clone(), area.allowed_tod.clone());
    }
    // what entrance we want to place now
    while let Some(entrance_to_place) = entrances_to_place.pop() {
        let mut exit_iter = exits_to_place.iter().enumerate();
        let mut try_count = 0;

        let mut current_inventory = inventory.clone();
        // TODO: here we assume that every area that can have both day/night where you can't sleep
        // needs to be accessible at both times, this isn't necessarily true (waterfall cave, goddess statue)
        let mut needs_both_tod = unconnected_entrances_with_tod
            .remove(&entrance_to_place)
            .unwrap()
            == AllowedToD::Both;
        if areas.get(&entrance_to_place.area).unwrap().can_sleep {
            needs_both_tod = false;
        }
        needs_both_tod = false;
        // assume all unplaced entrances
        for (entrance, allowed_tod) in unconnected_entrances_with_tod.iter() {
            for tod in allowed_tod.all_allowed() {
                current_inventory.add_area_tod(&entrance.area, tod);
            }
        }
        do_exploration(areas, &placement, &mut current_inventory);
        let selected_exit_index = 'exit_loop: loop {
            try_count += 1;
            let (exit_index, exit) = exit_iter
                .next()
                .ok_or_else(|| FillError::NoValidExitLeft(entrance_to_place.clone()))?;
            let (exit_area_key, exit_psgw) = exit.to_area_psgw();
            let exit_area = areas.get(&exit_area_key).unwrap();
            // check if exit is reachable
            if needs_both_tod {
                for tod in TimeOfDay::ALL {
                    if !(current_inventory.check_area_tod(&exit_area_key, &tod)
                        && exit_area
                            .exits
                            .get(&exit_psgw)
                            .unwrap()
                            .check_requirement_met(&current_inventory, &tod))
                    {
                        continue 'exit_loop;
                    }
                }
            } else {
                for tod in current_inventory
                    .areas
                    .get(&exit_area_key)
                    .unwrap_or(&AllowedToD::None)
                    .all_allowed()
                {
                    if current_inventory.check_area_tod(&exit_area_key, &tod)
                        && exit_area
                            .exits
                            .get(&exit_psgw)
                            .unwrap()
                            .check_requirement_met(&current_inventory, &tod)
                    {
                        break 'exit_loop exit_index;
                    }
                }
            }
        };
        // remove exit from list to be placed
        // fast remove, order is random anyways
        let placed_exit = exits_to_place.swap_remove(selected_exit_index);
        // println!("connecting <{}> to <{}>", placed_exit.dbg_to_string(logic_key_mapper), entrance_to_place.dbg_to_string(logic_key_mapper));
        placement.connected_areas.insert(placed_exit, entrance_to_place);
    }
    Ok(())
}

pub fn place_entrances_coupled<R: Rng>(
    rng: &mut R,
    areas: &HashMap<AreaKey, Area>,
    placement: &mut Placement,
    inventory: &Inventory,
    // each entrance represents both exit and entrance from one side
    entrances_to_place: &mut Vec<Entrance>,
    logic_key_mapper: &LogicKeyMapper,
) -> Result<(), FillError> {
    entrances_to_place.shuffle(rng);
    // assumed fill, assume all entrances are reachable at the start
    // cache all entrances and their supported ToD here, to avoid area lookups later
    let mut unconnected_entrances_with_tod = HashMap::new();
    for entrance in entrances_to_place.iter() {
        let area = areas.get(&entrance.area).unwrap();
        unconnected_entrances_with_tod.insert(entrance.clone(), area.allowed_tod.clone());
    }
    while let Some(side_1_entrance) = entrances_to_place.pop() {
        let side_1_exit = side_1_entrance.to_this_side_exit();
        assert!(unconnected_entrances_with_tod
            .remove(&side_1_entrance)
            .is_some());
        let mut side_2_entrances_iter = entrances_to_place.iter().enumerate();
        let selected_index = loop {
            let (side_2_entrance_index, side_2_entrance) = side_2_entrances_iter
                .next()
                .ok_or_else(|| FillError::NoValidExitLeft(side_1_entrance.clone()))?;
            let side_2_exit = side_2_entrance.to_this_side_exit();
            let mut current_inventory = inventory.clone();
            for (entrance, allowed_tod) in unconnected_entrances_with_tod.iter() {
                if entrance != side_2_entrance {
                    for tod in allowed_tod.all_allowed() {
                        current_inventory.add_area_tod(&entrance.area, tod);
                    }
                }
            }
            // temporarily connect entrances, will be reverted if this doesn't work out
            placement
                .connected_areas
                .insert(side_1_exit.clone(), side_2_entrance.clone());
            placement
                .connected_areas
                .insert(side_2_exit.clone(), side_1_entrance.clone());
            do_exploration(areas, placement, &mut current_inventory);
            if get_first_unreachable_loc(areas, &current_inventory).is_some() {
                placement.connected_areas.remove(&side_1_exit);
                placement.connected_areas.remove(&side_2_exit);
                continue;
            } else {
                break side_2_entrance_index;
            }
        };
        entrances_to_place.swap_remove(selected_index);
    }
    Ok(())
}

pub fn fill_assumed<R: Rng>(
    rng: &mut R,
    areas: &HashMap<AreaKey, Area>,
    placement: &mut Placement,
    inventory: &Inventory,
    locations_to_fill: &mut Vec<CheckKey>,
    items_to_place: &mut Vec<LogicKey>,
) {
    locations_to_fill.shuffle(rng);
    items_to_place.shuffle(rng);
    while let Some(item_to_place) = items_to_place.pop() {
        let mut current_inventory = inventory.clone();
        // assume we have all items
        for item in items_to_place.iter() {
            current_inventory.collect_item(item.clone());
        }
        // do exporation
        // also collects all placed items
        do_exploration(areas, placement, &mut current_inventory);

        // look for the first reachable location in our shuffled list
        let mut location_iter = locations_to_fill.iter().enumerate();
        let found_idx = 'out: loop {
            let (check_idx, CheckKey { area_key, check }) = location_iter.next().unwrap();
            let area = areas.get(area_key).unwrap();

            // check each time of day
            for tod in TimeOfDay::ALL {
                if area.allowed_tod.allows(&tod) && current_inventory.check_area_tod(area_key, &tod)
                {
                    let check_req = area.locations.get(check).unwrap();
                    if check_req.check_requirement_met(&current_inventory, &tod) {
                        break 'out check_idx;
                    }
                }
            }
        };
        let check = locations_to_fill.swap_remove(found_idx);
        placement.filled_checks.insert(check, item_to_place);
    }
}

pub fn do_exploration(
    areas: &HashMap<AreaKey, Area>,
    placement: &Placement,
    inventory: &mut Inventory,
) {
    let mut is_done = false;
    let mut visited_locations = HashSet::new();
    while !is_done {
        is_done = true;
        // explore areas
        let mut new_areas = HashSet::new();
        for (area_key, allowed_tod) in inventory.areas.iter() {
            let area = areas.get(area_key).unwrap();
            // try to sleep
            if area.can_sleep {
                let day_area = area_key.area_localize(&TimeOfDay::Day);
                let night_area = area_key.area_localize(&TimeOfDay::Night);
                if !inventory.check_area_tod(area_key, &TimeOfDay::Day)
                    && !new_areas.contains(&day_area)
                {
                    new_areas.insert(day_area);
                    is_done = false;
                }
                if !inventory.check_area_tod(area_key, &TimeOfDay::Night)
                    && !new_areas.contains(&night_area)
                {
                    new_areas.insert(night_area);
                    is_done = false;
                }
            }
            // explore exits
            for (exit, exit_req) in area.exits.iter() {
                // what area this actually leads to
                if let Some(entrance) = placement
                    .connected_areas
                    .get(&area_key.area_exit_to_psgw(exit))
                {
                    let other_area_key = &entrance.area;
                    let other_area = areas.get(other_area_key).unwrap();
                    for possible_tod in allowed_tod.all_allowed() {
                        // calculate the actual ToD this area will have
                        // some areas force a specific ToD (due to the surface not being able to handle night)
                        let actual_tod = possible_tod.with_forced(&other_area.allowed_tod);
                        // if this area is already collected, no need to check the requirement again
                        let area_end = other_area_key.area_localize(&actual_tod);
                        if !inventory.check_area_tod(other_area_key, &actual_tod)
                            && !new_areas.contains(&area_end)
                        {
                            // check if this area is reachable
                            if exit_req.check_requirement_met(inventory, possible_tod) {
                                new_areas.insert(area_end);
                                is_done = false;
                            }
                        }
                    }
                }
            }
        }
        for new_area in new_areas {
            inventory.add_area_tod(&new_area.area_key, &new_area.local);
        }
        // collect items/events
        for (area_key, allowed_tod) in inventory.areas.iter() {
            // explore locations
            let area = areas.get(area_key).unwrap();
            for (check_name, check_req) in area.locations.iter() {
                let check_key = CheckKey {
                    area_key: area_key.clone(),
                    check: check_name.clone(),
                };
                // if this location was already visited, don't collect it again
                if !visited_locations.contains(&check_key) {
                    for possible_tod in allowed_tod.all_allowed() {
                        // if this location is reachable, collect it
                        if check_req.check_requirement_met(inventory, possible_tod) {
                            visited_locations.insert(check_key.clone());
                            if let Some(item) = placement.filled_checks.get(&check_key) {
                                // TODO: inventory was a nice idea, but we want to borrow this as
                                // mutable while areas are borrowed as well
                                collect_items(&mut inventory.items_events_counts, item.clone());
                                is_done = false;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn do_playthrough(
    areas: &HashMap<AreaKey, Area>,
    start_inventory: &Inventory,
    placement: &Placement,
    logic_key_mapper: &LogicKeyMapper,
) -> Vec<Vec<(String, String)>> {
    let mut spheres = Vec::new();
    let mut visited_locations = HashSet::with_capacity(600);
    let mut inventory = start_inventory.clone();
    let mut is_done = false;
    while !is_done {
        let mut current_sphere = Vec::new();
        is_done = true;
        // explore areas
        let mut new_areas = HashSet::new();
        for (area_key, allowed_tod) in inventory.areas.iter() {
            let area = areas.get(area_key).unwrap();
            // try to sleep
            if area.can_sleep {
                let day_area = area_key.area_localize(&TimeOfDay::Day);
                let night_area = area_key.area_localize(&TimeOfDay::Night);
                if !inventory.check_area_tod(area_key, &TimeOfDay::Day)
                    && !new_areas.contains(&day_area)
                {
                    current_sphere.push(("Sleep".to_string(), format!("{}, Day", area_key.name(logic_key_mapper))));
                    new_areas.insert(day_area);
                    is_done = false;
                }
                if !inventory.check_area_tod(area_key, &TimeOfDay::Night)
                    && !new_areas.contains(&night_area)
                {
                    current_sphere.push(("Sleep".to_string(), format!("{}, Night", area_key.name(logic_key_mapper))));
                    new_areas.insert(night_area);
                    is_done = false;
                }
            }
            // explore exits
            for (exit, exit_req) in area.exits.iter() {
                // what area this actually leads to
                if let Some(entrance) = placement
                    .connected_areas
                    .get(&area_key.area_exit_to_psgw(exit))
                {
                    let other_area_key = &entrance.area;
                    let other_area = areas.get(other_area_key).unwrap();
                    for possible_tod in allowed_tod.all_allowed() {
                        // calculate the actual ToD this area will have
                        // some areas force a specific ToD (due to the surface not being able to handle night)
                        let actual_tod = possible_tod.with_forced(&other_area.allowed_tod);
                        // if this area is already collected, no need to check the requirement again
                        let area_end = other_area_key.area_localize(&actual_tod);
                        if !inventory.check_area_tod(other_area_key, &actual_tod)
                            && !new_areas.contains(&area_end)
                        {
                            // check if this area is reachable
                            if exit_req.check_requirement_met(&inventory, possible_tod) {
                                current_sphere.push((area_key.area_exit_to_psgw(exit).dbg_to_string(logic_key_mapper), entrance.dbg_to_string(logic_key_mapper)));
                                new_areas.insert(area_end);
                                is_done = false;
                            }
                        }
                    }
                }
            }
        }
        let mut new_items = Vec::new();
        // collect items/events
        for (area_key, allowed_tod) in inventory.areas.iter() {
            // explore locations
            let area = areas.get(area_key).unwrap();
            for (check_name, check_req) in area.locations.iter() {
                let check_key = CheckKey {
                    area_key: area_key.clone(),
                    check: check_name.clone(),
                };
                // if this location was already visited, don't collect it again
                if !visited_locations.contains(&check_key) {
                    for possible_tod in allowed_tod.all_allowed() {
                        // if this location is reachable, collect it
                        if check_req.check_requirement_met(&inventory, possible_tod) {
                            visited_locations.insert(check_key.clone());
                            if let Some(item) = placement.filled_checks.get(&check_key) {
                                current_sphere.push((check_key.dbg_to_string(logic_key_mapper), item.name(logic_key_mapper).to_string()));
                                new_items.push(item.clone());
                                is_done = false;
                            }
                        }
                    }
                }
            }
        }
        for new_area in new_areas {
            inventory.add_area_tod(&new_area.area_key, &new_area.local);
        }
        for item in new_items {
            inventory.collect_item(item);
        }
        spheres.push(current_sphere);
    }
    spheres
}

pub fn get_first_unreachable_loc(
    areas: &HashMap<AreaKey, Area>,
    inventory: &Inventory,
) -> Option<CheckKey> {
    for (area_key, area) in areas.iter() {
        'locations: for (check, loc_req) in area.locations.iter() {
            for tod in TimeOfDay::ALL.iter() {
                if inventory.check_area_tod(area_key, tod)
                    && loc_req.check_requirement_met(inventory, tod)
                {
                    continue 'locations;
                }
            }
            return Some(area_key.area_with_check(check));
        }
    }
    None
}

pub fn get_progress_item_list(logic_key_mapper: &LogicKeyMapper) -> Vec<LogicKey> {
    let items = [
        "Slingshot",
        "Bomb Bag",
        "Gust Bellows",
        "Whip",
        "Bow",
        "Bug Net",
        "Water Scale",
        "Fireshield Earrings",
        "Clawshots",
        "Stone of Trials",
        "Sea Chart",
        "Emerald Tablet",
        "Ruby Tablet",
        "Amber Tablet",
        "Baby Rattle",
        "Cawlin's Letter",
        "Horned Colossus Beetle",
        "Goddess Harp",
        "Ballad of the Goddess",
        "Farore's Courage",
        "Nayru's Wisdom",
        "Din's Power",
        "Faron Song of the Hero Part",
        "Eldin Song of the Hero Part",
        "Lanayru Song of the Hero Part",
        "Spiral Charge", // "Revitalizing Potion" // causes problems in events, as it's treated like you buy a potion
    ];
    let copied_items = [
        ("Gratitude Crystal Pack", 13),
        ("Gratitude Crystal", 15),
        ("Progressive Sword", 6),
        ("Progressive Mitts", 2),
        ("Progressive Beetle", 2),
        ("Progressive Pouch", 5),
        ("Key Piece", 5),
        ("Empty Bottle", 5),
        ("Progressive Wallet", 4),
        ("Extra Wallet", 3),
        ("LanayruCaves Small Key", 1),
        ("SV Boss Key", 1),
        ("SV Small Key", 2),
        ("ET Boss Key", 1),
        ("LMF Boss Key", 1),
        ("LMF Small Key", 1),
        ("AC Boss Key", 1),
        ("AC Small Key", 2),
        ("SS Boss Key", 1),
        ("SS Small Key", 2),
        ("FS Boss Key", 1),
        ("FS Small Key", 3),
        ("SK Small Key", 1),
    ];
    let mut result = Vec::new();
    for item in items {
        result.push(
            logic_key_mapper
                .convert_to_num_assuming_present(item)
                .unwrap(),
        );
    }
    for (item, count) in copied_items {
        let key = logic_key_mapper
            .convert_to_num_assuming_present(item)
            .unwrap();
        for _ in 0..count {
            result.push(key.clone());
        }
    }
    result
}

pub fn get_exits(areas: &HashMap<AreaKey, Area>) -> Vec<Exit> {
    let mut exits = Vec::new();
    for (area_key, area) in areas.iter() {
        for exit in area.exits.keys() {
            exits.push(area_key.area_exit_to_psgw(exit));
        }
    }
    exits
}
pub fn get_entrance_exits(areas: &HashMap<AreaKey, Area>) -> (Vec<Entrance>, Vec<Exit>) {
    let mut entrances = Vec::new();
    let mut exits = Vec::new();
    for (area_key, area) in areas.iter() {
        for exit_key in area.exits.keys() {
            let exit = area_key.area_exit_to_psgw(exit_key);
            // the other side of this exit, in the other area
            entrances.push(exit.to_entrance());
            exits.push(exit);
        }
    }
    (entrances, exits)
}
