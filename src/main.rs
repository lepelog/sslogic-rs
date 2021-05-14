mod items;
mod logic_files;
mod options;
mod packedbits;
mod inventory;

extern crate serde;
extern crate serde_yaml;
extern crate base64;

use std::convert::TryFrom;
use std::collections::HashSet;

use crate::items::*;
use crate::logic_files::LogicFiles;
use packedbits::{PackedBitsWriter, PackedBitsReader};
use crate::options::RandomizerOptions;
use crate::inventory::Inventory;

fn main4() {
    // println!("{:?}", Item::try_from("Gust Bellows"));
    let logic_file = LogicFiles::read_logic().unwrap();
    let mut hs = HashSet::new();
    for (loc, req) in logic_file.requirement_map.iter() {
        // println!("{}:\n  {:?}", loc, req);
        req.get_all_base_reqs(&mut hs);
    }
    for undef_macro in hs.iter().filter(|m| !logic_file.requirement_map.contains_key(*m)) {
        println!("unknown macro {}", undef_macro);
    }
    // println!("{:?}", hs);
}

fn main2() {
    println!("{:?}", RandomizerOptions::parse_option_file().unwrap().get_options());
}

fn main() {
    let logic_file = LogicFiles::read_logic().unwrap();
    let options = RandomizerOptions::parse_option_file().unwrap();
    let mut inventory = Inventory::new();
    inventory.collect(Item::Clawshots);
    for loc in logic_file.location_names.iter() {
        let reachable = logic_file.requirement_map.get(loc).unwrap().check_requirement_met(&options, &inventory, &logic_file.requirement_map);
        println!("{}: {}", loc, reachable);
    }
}