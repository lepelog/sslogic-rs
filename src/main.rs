mod items;
mod logic_files;

extern crate serde;
extern crate serde_yaml;
extern crate linked_hash_map;

use std::convert::TryFrom;
use std::collections::HashSet;

use crate::items::*;
use crate::logic_files::LogicFiles;

fn main() {
    // println!("{:?}", Item::try_from("Gust Bellows"));
    let logic_file = LogicFiles::read_logic().unwrap();
    let mut hs = HashSet::new();
    for (loc, req) in logic_file.requirement_map.iter() {
        // println!("{}:\n  {:?}", loc, req);
        req.get_all_base_reqs(&mut hs);
    }
    println!("{:?}", hs);
}
