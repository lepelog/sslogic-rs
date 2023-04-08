use std::collections::{HashMap, HashSet};

use crate::options::RandomizerOptions;
use rand::prelude::*;

use super::{
    logic_algorithms::{
        do_playthrough, fill_assumed, get_progress_item_list, place_entrances_coupled,
        place_entrances_decoupled, FillError,
    },
    logic_expression::LogicElement,
    logic_loader::{self, load_check_defs, load_passageway_defs, specialize_for_options},
    logic_structs::{
        Area, AreaKey, BasePlacement, Inventory, LogicKeyMapper, Placement, RecursivePlacement,
        TimeOfDay,
    },
};

#[derive(Debug)]
pub enum RandomizerError {
    InvalidConfig,
}

pub fn get_loc_mut<'a, 'b>(
    areas: &'a mut HashMap<AreaKey, Area>,
    mapper: &LogicKeyMapper,
    stage: &'b str,
    area: &'b str,
    loc: &'b str,
) -> Result<&'a mut LogicElement, &'b str> {
    let map_lk = |s| mapper.convert_to_num_assuming_present(s);
    areas
        .get_mut(&map_lk(stage)?.stage_with_area(&map_lk(area)?))
        .ok_or_else(|| area)?
        .locations
        .get_mut(&map_lk(loc)?)
        .ok_or_else(|| loc)
}

pub fn select_from_list_stable<'a, T: ?Sized, R: Rng>(
    rng: &mut R,
    lst: &[&'a T],
    count: usize,
) -> Vec<&'a T> {
    let mut indices = (0..lst.len()).choose_multiple(rng, count);
    indices.sort();
    indices.iter().map(|idx| lst[*idx]).collect()
}

pub fn do_randomize<R: Rng>(rng: &mut R, opts: &RandomizerOptions) -> Result<(), RandomizerError> {
    // https://hackmd.okfn.de/-_0hfflsQEGh0kNh6AXtqA?view#

    let mut mapper = LogicKeyMapper::default();

    let (mut areas, filled_checks) = logic_loader::do_parse(&mut mapper).unwrap();

    let mut placement = BasePlacement::new(HashMap::new(), filled_checks);

    // we should now have all the logic keys, remove the mut
    let mapper = mapper;

    let mp = |s| mapper.convert_to_num_assuming_present(s);
    let mpu = |s| mapper.convert_to_num_assuming_present(s).unwrap();

    specialize_for_options(&mut areas, opts);

    let check_defs = load_check_defs(&mapper).unwrap();

    // randomize required dungeons
    let req_dungeon_count = opts.get_option_uint("required-dungeon-count").unwrap();
    let potentially_required_dungeons = &[
        "Skyview",
        "Earth Temple",
        "Lanayru Mining Facility",
        "Ancient Cistern",
        "Sandship",
        "Fire Sanctuary",
    ];
    let mut required_dungeons =
        select_from_list_stable(rng, potentially_required_dungeons, req_dungeon_count);

    // update required dungeon event
    let req = get_loc_mut(
        &mut areas,
        &mapper,
        "Sealed Temple",
        "Main",
        "Beat Required Dungeons",
    )
    .unwrap();
    let mut reqs = Vec::new();
    for dungeon in required_dungeons.iter() {
        let dungeon_req = format!("Can Beat {}", dungeon);
        let dungeon_key = mapper
            .convert_to_num_assuming_present(&dungeon_req)
            .unwrap();
        reqs.push(LogicElement::LogicKeyRequirement(dungeon_key));
    }

    *req = LogicElement::AndExpression(reqs);

    // randomize start tablets
    let start_tablet_count = opts.get_option_uint("starting-tablet-count").unwrap();
    let potential_start_tablets = &["Emerald Tablet", "Ruby Tablet", "Amber Tablet"];
    let mut start_tablets =
        select_from_list_stable(rng, potential_start_tablets, start_tablet_count);

    let mut start_items = Vec::new();

    for tablet in start_tablets.iter() {
        start_items.push(mpu(tablet));
    }

    // pouch start
    if opts.get_option_enabled("start-with-pouch").unwrap() {
        start_items.push(mpu("Progressive Pouch"));
    }

    // start swords
    let startsword_count = match opts
        .get_option_choice_str("starting-sword")
        .unwrap()
        .as_str()
    {
        "Swordless" => 0,
        "Practice Sword" => 1,
        "Goddess Sword" => 2,
        "Goddess Longsword" => 3,
        "Goddess White Sword" => 4,
        "Master Sword" => 5,
        "True Master Sword" => 6,
        _ => panic!("invalid start sword"),
    };

    for _ in 0..startsword_count {
        start_items.push(mpu("Progressive Sword"));
    }

    let mut progress_items = get_progress_item_list(&mapper);

    // remove items we start with
    for start_item in start_items.iter() {
        if let Some(pos) = progress_items.iter().position(|itm| itm == start_item) {
            progress_items.swap_remove(pos);
        }
    }

    // prepare entrance pool

    // we iterate through all exits and determine if they should
    // be vanilla or not
    // since we are iterating through maps the result is non deterministic

    let mut exits_to_shuffle = Vec::new();
    let mut entrances_to_shuffle = Vec::new();

    let skyloft_stage = mpu("Skyloft");

    let passageway_defs = load_passageway_defs(&mapper).unwrap();

    let mut psgw_set = HashSet::with_capacity(passageway_defs.len());

    for psgw_def in passageway_defs.iter() {
        if psgw_def.pswg_type.is_none() && !matches!(psgw_def.is_right_door, Some(true)) {
            psgw_set.insert(psgw_def.to_stage_exit());
        }
    }

    for (area_key, area) in areas.iter() {
        for passageway in area.exits.keys() {
            let exit = area_key.area_exit_to_psgw(passageway);
            // if it's in the definitions, it's shuffled
            if !psgw_set.contains(&exit.to_stage_exit()) {
                // if !(area_key.stage == academy_stage || passageway.other_area.stage == academy_stage) {
                // if true {
                placement
                    .connected_areas
                    .insert(exit.clone(), exit.to_entrance());
            } else {
                entrances_to_shuffle.push(exit.to_entrance());
                exits_to_shuffle.push(exit);
            }
        }
    }

    exits_to_shuffle.sort();
    entrances_to_shuffle.sort();

    let mut start_inventory = Inventory::default();
    for item in start_items.iter() {
        start_inventory.collect_item(item.clone());
    }

    // build logic from skyloft
    start_inventory.add_area_tod(
        &mapper
            .convert_to_area_assuming_present("Skyloft", "Central Outside")
            .unwrap(),
        &TimeOfDay::Day,
    );

    // assume vanilla starting statues
    for item in &[
        "Sealed Grounds Statue",
        "Eldin Entrance Statue",
        "Lanayru Mine Entry Statue",
    ] {
        start_inventory.collect_item(mpu(item));
    }

    let mut full_inventory = start_inventory.clone();

    for item in progress_items.iter() {
        full_inventory.collect_item(item.clone());
    }

    // do place entrance
    if false {
        placement = loop {
            let mut placement_copy = placement.clone();
            let mut cloned_entrances_to_place = entrances_to_shuffle.clone();
            let mut cloned_exits_to_place = exits_to_shuffle.clone();
            if let Err(FillError::NoValidExitLeft(e)) = place_entrances_decoupled(
                rng,
                &areas,
                &mut placement_copy,
                &full_inventory,
                &mut cloned_entrances_to_place,
                &mut cloned_exits_to_place,
                &mapper,
            ) {
                println!("entrance: {}", e.dbg_to_string(&mapper));
                for exit in cloned_exits_to_place {
                    println!("exit: {}", exit.dbg_to_string(&mapper));
                }
            } else {
                break placement_copy;
            }
        };
    } else {
        let mut double_side_exits = Vec::new();
        for entrance in entrances_to_shuffle {
            if psgw_set.contains(&entrance.to_exit().to_stage_exit()) {
                double_side_exits.push(entrance);
            } else {
                placement
                    .connected_areas
                    .insert(entrance.to_exit(), entrance);
            }
        }
        println!(
            "{}",
            mapper.dbg_areas(double_side_exits.iter().map(|e| e.area.clone()))
        );
        placement = loop {
            let mut placement_copy = placement.clone();
            let mut cloned_double_side_exits = double_side_exits.clone();
            if let Err(FillError::NoValidExitLeft(e)) = place_entrances_coupled(
                rng,
                &areas,
                &mut placement_copy,
                &full_inventory,
                &mut cloned_double_side_exits,
                &mapper,
            ) {
                println!("entrance: {}", e.dbg_to_string(&mapper));
                for exit in cloned_double_side_exits {
                    println!("exit: {}", exit.dbg_to_string(&mapper));
                }
            } else {
                break placement_copy;
            }
        };
    }

    let mut locations_to_fill = Vec::new();

    let check_defs = load_check_defs(&mapper).unwrap();

    for (area_key, area) in areas.iter() {
        for loc in area.locations.keys() {
            if check_defs.contains_key(loc) {
                locations_to_fill.push(area_key.area_with_check(loc));
            }
        }
    }

    locations_to_fill.sort();
    progress_items.sort();

    let mut placement_overlay = RecursivePlacement::with_parent(&placement);

    loop {
        placement_overlay.clear();
        let mut to_fill_copy = locations_to_fill.clone();
        let mut items_copy = progress_items.clone();
        if let Err(item) = fill_assumed(
            rng,
            &areas,
            &mut placement_overlay,
            &start_inventory,
            &mut to_fill_copy,
            &mut items_copy,
        ) {
            println!(
                "couldn't place {} ",
                mapper.convert_to_string(&item).unwrap()
            );
        } else {
            locations_to_fill = to_fill_copy;
            let tmp = placement_overlay.into_inner();
            placement.consume_other(tmp);
            break;
        }
    }

    for (check, item) in placement.filled_checks.iter() {
        println!("{:<70}: {}", check.check.name(&mapper), item.name(&mapper));
    }

    let playthrough = do_playthrough(&areas, &start_inventory, &placement, &mapper);

    for (idx, sphere) in playthrough.iter().enumerate() {
        println!("\n\nSphere {}:", idx);
        for (a, b) in sphere.iter() {
            println!("{:<70}: {}", a, b);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::options::RandomizerOptions;

    use super::do_randomize;

    use rand::prelude::*;
    use rand_pcg::Pcg64;

    #[test]
    pub fn test_randomize() {
        let mut rng = Pcg64::seed_from_u64(123);
        let mut opts = RandomizerOptions::parse_option_file().unwrap();
        opts.set_option_uint("required-dungeon-count", 3).unwrap();
        do_randomize(&mut rng, &opts).unwrap();
    }
}
