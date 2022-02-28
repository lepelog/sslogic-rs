use std::collections::{HashMap, HashSet};

use rand::prelude::*;

use super::logic_structs::{
    collect_items, Area, CheckKey, Inventory, LocalizedAreaKey, LogicKey,
    PassagewayKey, Placement, TimeOfDay, LogicKeyMapper, Entrance, Exit,
};

#[derive(Debug, Clone)]
pub enum FillError {
    NoValidExitLeft(Entrance),
    NoValidLocationLeft(LogicKey),
}

pub fn place_entrances_two_way<R: Rng>(
    rng: &mut R,
    areas: &HashMap<LogicKey, Area>,
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
    let mut unconnected_entrances_with_tod = HashMap::new();
    for entrance in entrances_to_place.iter() {
        let area = areas.get(&entrance.area).unwrap();
        unconnected_entrances_with_tod.insert(entrance.clone(), area.allowed_tod.clone());
    }
    // what entrance we want to place now
    while let Some(entrance_to_place) = entrances_to_place.pop() {
        // since we are placing 2 way, we now have to try all the exits
        let mut exit_iter = exits_to_place.iter().enumerate();
        let mut try_count = 0;
        let selected_exit_index = loop {
            let mut current_inventory = inventory.clone();
            try_count += 1;
            let (exit_index, exit_to_place) = exit_iter
                .next()
                .ok_or_else(|| FillError::NoValidExitLeft(entrance_to_place.clone()))?;
            // remove both entrances and connect both sides
            // make sure to reverse it in case of failure!!!!!
            let exit_to_place_entrance = exit_to_place.to_this_side_entrance();
            let entrance_to_place_exit = entrance_to_place.to_this_side_exit();
            // assume all entrances, exept the ones we just connected
            for (entrance, allowed_tod) in unconnected_entrances_with_tod.iter() {
                if entrance != &entrance_to_place && entrance != &exit_to_place_entrance {
                    for tod in allowed_tod.all_allowed() {
                        current_inventory.add_area_tod(&entrance.area, tod);
                    }
                }
            }
            placement.connected_areas.insert(exit_to_place.clone(), entrance_to_place.clone());
            placement.connected_areas.insert(entrance_to_place_exit.clone(), exit_to_place_entrance.clone());
            do_exploration(areas, &placement, &mut current_inventory);
            if get_first_unreachable_loc(areas, &current_inventory).is_some() {
                // we created unreachable locations, so this connection won't work
                placement.connected_areas.remove(exit_to_place);
                placement.connected_areas.remove(&entrance_to_place_exit);
                continue;
            }
            // this connection works!
            // remove the assumed entrances we just placed (the placement was done before)
            unconnected_entrances_with_tod.remove(&entrance_to_place);
            unconnected_entrances_with_tod.remove(&exit_to_place_entrance);
            break exit_index;
        };
        // remove exit from list to be placed
        // fast remove, order is random anyways
        let placed_exit = exits_to_place.swap_remove(selected_exit_index);
        println!("connecting <{}> to <{}>", placed_exit.dbg_to_string(logic_key_mapper), entrance_to_place.dbg_to_string(logic_key_mapper));
    }
    Ok(())
}

pub fn fill_assumed<R: Rng>(
    rng: &mut R,
    areas: &HashMap<LogicKey, Area>,
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
            for tod in [TimeOfDay::Day, TimeOfDay::Night].into_iter() {
                if area.allowed_tod.allows(&tod)
                    && current_inventory.check_area_tod(area_key, &tod)
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
    areas: &HashMap<LogicKey, Area>,
    placement: &Placement,
    inventory: &mut Inventory,
) {
    let mut is_done = false;
    let mut visited_locations = HashSet::new();
    while !is_done {
        is_done = true;
        // explore areas
        let mut new_areas = HashSet::new();
        for (area_key, allowed_tod) in inventory.areas.iter()
        {
            let area = areas.get(area_key).unwrap();
            // try to sleep
            if area.can_sleep {
                let day_area = area_key.area_localize(&TimeOfDay::Day);
                let night_area = area_key.area_localize(&TimeOfDay::Night);
                if !inventory.check_area_tod(area_key, &TimeOfDay::Day) && !new_areas.contains(&day_area) {
                    new_areas.insert(day_area);
                    is_done = false;
                }
                if !inventory.check_area_tod(area_key, &TimeOfDay::Night) && !new_areas.contains(&night_area) {
                    new_areas.insert(night_area);
                    is_done = false;
                }
            }
            // explore exits
            for (exit, exit_req) in area.exits.iter() {
                // what area this actually leads to
                if let Some(entrance) = placement.connected_areas.get(&area_key.area_exit_to_psgw(exit)) {
                    let other_area_key = &entrance.area;
                    let other_area = areas.get(other_area_key).unwrap();
                    for possible_tod in allowed_tod.all_allowed() {
                        // calculate the actual ToD this area will have
                        // some areas force a specific ToD (due to the surface not being able to handle night)
                        let actual_tod = possible_tod.with_forced(&other_area.allowed_tod);
                        // if this area is already collected, no need to check the requirement again
                        let area_end = other_area_key.area_localize(&actual_tod);
                        if !inventory.check_area_tod(other_area_key, &actual_tod) && !new_areas.contains(&area_end) {
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

pub fn get_first_unreachable_loc(areas: &HashMap<LogicKey, Area>, inventory: &Inventory) -> Option<CheckKey> {
    for (area_key, area) in areas.iter() {
        'locations: for (check, loc_req) in area.locations.iter() {
            for tod in TimeOfDay::ALL.iter() {
                if inventory.check_area_tod(area_key, tod) && loc_req.check_requirement_met(inventory, tod) {
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
        "Spiral Charge"
        // "Revitalizing Potion" // causes problems in events, as it's treated like you buy a potion
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
        result.push(logic_key_mapper.convert_to_num_assuming_present(item).unwrap());
    }
    for (item, count) in copied_items {
        let key = logic_key_mapper.convert_to_num_assuming_present(item).unwrap();
        for _ in 0..count {
            result.push(key.clone());
        }
    }
    result
}

pub fn get_exits(areas: &HashMap<LogicKey, Area>) -> Vec<Exit> {
    let mut exits = Vec::new();
    for (area_key, area) in areas.iter() {
        for exit in area.exits.keys() {
            exits.push(area_key.area_exit_to_psgw(exit));
        }
    }
    exits
}
pub fn get_entrance_exits(
    areas: &HashMap<LogicKey, Area>,
) -> (Vec<Entrance>, Vec<Exit>) {
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
