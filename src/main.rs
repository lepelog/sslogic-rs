pub mod graph_logic;
pub mod options;
pub mod packedbits;

extern crate base64;
extern crate rand;
extern crate rand_pcg;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use std::collections::{HashMap, HashSet};

use graph_logic::logic_loader::{do_parse, load_passageway_defs};
use rand::prelude::*;
use rand_pcg::Pcg64;

use crate::{
    graph_logic::{
        logic_algorithms::{
            do_exploration, get_entrance_exits, get_exits, get_first_unreachable_loc,
            get_progress_item_list, place_entrances_coupled, FillError,
        },
        logic_expression::LogicElement,
        logic_loader::specialize_for_options,
        logic_structs::{
            Entrance, Exit, Inventory, LogicKey, LogicKeyMapper, PassagewayKey, Placement,
            TimeOfDay,
        },
    },
    options::RandomizerOptions,
};

pub fn main() {
    shuffle_test();
}

pub fn shuffle_test() {
    // find exits without entrance
    // let mut logic_key_mapper = LogicKeyMapper::default();
    // let (mut areas, mut item_placement) = do_parse(&mut logic_key_mapper).unwrap();
    // let mut entrance_connections: HashMap<Exit, Entrance> = HashMap::new();
    // let exits = get_exits(&areas);

    // let mut seen_passageways = HashSet::new();

    // let passageway_defs = load_passageway_defs().unwrap();
    // let mut psgw_set = HashSet::new();
    // for p in passageway_defs.iter() {
    //     if p.pswg_type.is_some() {
    //         continue;
    //     }
    //     // ignore one set of doors for now
    //     if p.door.as_ref().map_or(false, |d| d == "Right") {
    //         continue;
    //     }
    //     let from_key = logic_key_mapper
    //         .convert_to_num_assuming_present(&p.stage)
    //         .unwrap();
    //     let to_key = logic_key_mapper
    //         .convert_to_num_assuming_present(&p.to_stage)
    //         .unwrap();
    //     let disambig = p
    //         .disambiguation
    //         .as_ref()
    //         .map(|d| logic_key_mapper.convert_to_num_assuming_present(d).unwrap());
    //     psgw_set.insert((from_key, to_key, disambig));
    // }

    // for exit in exits.iter() {
    //     let from_stage = areas.get(&exit.area).unwrap().stage.clone();
    //     let to_stage = areas.get(&exit.to_area).unwrap().stage.clone();
    //     if from_stage == to_stage {
    //         continue;
    //     }
    //     let psgw = (from_stage, to_stage, exit.disambiguation.clone());
    //     if psgw_set.contains(&psgw) && seen_passageways.contains(&psgw) {
    //         println!(
    //             "duplicate passageway: {}, {}",
    //             psgw.0.name(&logic_key_mapper),
    //             psgw.1.name(&logic_key_mapper)
    //         );
    //     } else {
    //         seen_passageways.insert(psgw);
    //     }
    // }

    // let exit_map: HashSet<_> = exits.iter().cloned().collect();

    // let mut entrances_to_shuffle = Vec::new();

    // for exit in exits.iter() {
    //     let stages_pswg = exit.to_stages(&areas);
    //     if stages_pswg.0 == stages_pswg.1 {
    //         // inside a stage, always vanilla
    //         entrance_connections.insert(exit.clone(), exit.to_entrance());
    //     } else {
    //         // ensure this is a 2 way entrance
    //         let opposite_way = exit.reverse();
    //         if psgw_set.contains(&stages_pswg) && exit_map.contains(&opposite_way) {
    //             // different stage, needs to be randomized
    //             entrances_to_shuffle.push(exit.to_entrance());
    //         } else {
    //             // one way, not randomized
    //             entrance_connections.insert(exit.clone(), exit.to_entrance());
    //         }
    //     }
    // }

    // let mut rng = Pcg64::seed_from_u64(0);

    // // due to areas being a hash map, this is essentially random
    // // otherwise
    // // logic keys are stable though
    // entrances_to_shuffle.sort();

    // let mut placement = Placement {
    //     connected_areas: entrance_connections,
    //     filled_checks: item_placement,
    // };

    // let mut inventory = Inventory {
    //     areas: HashMap::new(),
    //     items_events_counts: HashMap::new(),
    // };

    // // start search from central skyloft
    // let skyloft_central = logic_key_mapper
    //     .convert_to_area_assuming_present("Skyloft", "Central Outside")
    //     .unwrap();

    // inventory.add_area_tod(&skyloft_central, &TimeOfDay::Day);

    // // assume all items
    // for item_key in get_progress_item_list(&logic_key_mapper) {
    //     inventory.collect_item(item_key);
    // }

    // // assume vanilla starting statues
    // for item in &[
    //     "Sealed Grounds Statue",
    //     "Eldin Entrance Statue",
    //     "Lanayru Mine Entry Statue",
    // ] {
    //     let item_key = logic_key_mapper
    //         .convert_to_num_assuming_present(item)
    //         .unwrap();
    //     inventory.collect_item(item_key);
    // }

    // // set required dungeons to 0
    // let sealed_temple_main = logic_key_mapper
    //     .convert_to_area_assuming_present("Sealed Temple", "Main")
    //     .unwrap();
    // let beat_req_dungeons = logic_key_mapper
    //     .convert_to_num_assuming_present("Beat Required Dungeons")
    //     .unwrap();

    // *areas
    //     .get_mut(&sealed_temple_main)
    //     .unwrap()
    //     .locations
    //     .get_mut(&beat_req_dungeons)
    //     .unwrap() = LogicElement::FixedValue(true);

    // let options = RandomizerOptions::parse_option_file().unwrap();

    // specialize_for_options(&mut areas, &options);

    // do_exploration(&areas, &placement, &mut inventory);

    // let new_placement = loop {
    //     let mut new_placement = placement.clone();
    //     let mut entrances = entrances_to_shuffle.clone();
    //     if let Err(FillError::NoValidExitLeft(entr)) = place_entrances_coupled(
    //         &mut rng,
    //         &areas,
    //         &mut new_placement,
    //         &inventory,
    //         &mut entrances,
    //         &logic_key_mapper,
    //     ) {
    //         println!("error placing {}", entr.dbg_to_string(&logic_key_mapper));
    //     } else {
    //         break new_placement;
    //     }
    // };

    // for (exit, entrance) in new_placement.connected_areas.iter() {
    //     // quadratic complexity let's go
    //     if entrances_to_shuffle.contains(&entrance) {
    //         println!(
    //             "{:<70}: {}",
    //             entrance.dbg_to_string(&logic_key_mapper),
    //             exit.dbg_to_string(&logic_key_mapper)
    //         );
    //     }
    // }

    // println!()
}
