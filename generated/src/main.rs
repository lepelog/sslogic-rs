use std::{env::args, fmt::Write};

use generated::get_logic;

use rand::{prelude::*, rngs::OsRng};

use crate::assumed_fill::run_with_seed;

pub mod assumed_fill;
pub mod constants;
pub mod explorer;
pub mod generated;
pub mod individual_world;
pub mod logic_static;
pub mod plando;
pub mod util;

pub fn main() {
    let world_count = args().nth(1).and_then(|arg| arg.parse().ok()).unwrap_or(1);

    let seed: u64 = OsRng.gen();
    println!("seed: {seed}");
    let requirements = get_logic();
    match run_with_seed(seed, &requirements, world_count) {
        Ok(worlds) => {
            let mut out = String::new();
            for (world_id, world) in worlds.iter().enumerate() {
                if worlds.len() != 1 {
                    writeln!(&mut out, "WORLD {world_id}").unwrap();
                }
                world.write_placement(&mut out).unwrap();
            }
            print!("{out}");
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
