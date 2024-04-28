use std::{fs::File, io::Write};

use dumper::dump;
use loader::load_logic;

// mod bitset;
mod dumper;
mod loader;
mod requirements;
mod structure;

fn main() {
    let (ctx, base_requirements, options) = load_logic().unwrap();
    let dumped = dump(&ctx, &base_requirements, &options).unwrap();
    // println!("{}", &dumped);
    let mut outf = File::create("../generated/src/generated.rs").unwrap();
    outf.write_all(dumped.as_bytes()).unwrap();
}
