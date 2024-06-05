use rand::prelude::*;
use rand_pcg::Pcg64;
use snafu::{ResultExt, Snafu};

use crate::{
    explorer::{MultiworldExplorer, Placement},
    generated::{Item, Location, Options},
    individual_world::{generate_single_world, SingleWorld},
    logic_static::Requirements,
    plando,
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Cannot place item {} w{world} at {impossible_locations}", item.get().name))]
    NoLocationLeft {
        impossible_locations: String,
        world: usize,
        item: Item,
    },
}

#[derive(Debug, Snafu)]
pub enum CombinedError {
    #[snafu(display("w{world}: {source}"))]
    Plando { source: plando::Error, world: usize },
    #[snafu(display("{stage}: {source}"))]
    Fill { source: Error, stage: &'static str },
}

pub struct InputWorld<'a> {
    options: &'a Options,
    placement: &'a mut Placement,
    locations: &'a mut Vec<Location>,
    requirements: &'a Requirements<'a>,
    items: Vec<Item>,
}

pub fn assumed_fill_worlds<R: Rng>(
    rng: &mut R,
    mut worlds: Vec<InputWorld<'_>>,
) -> Result<(), Error> {
    let mut worlds_locations: Vec<_> = worlds
        .iter()
        .enumerate()
        .flat_map(|(world_id, world)| world.locations.iter().map(move |loc| (world_id, *loc)))
        .collect();
    worlds_locations.shuffle(rng);
    let mut worlds_items: Vec<_> = worlds
        .iter()
        .enumerate()
        .flat_map(|(world_id, world)| world.items.iter().map(move |item| (world_id, *item)))
        .collect();
    worlds_items.shuffle(rng);
    'outer: while let Some((item_world, item)) = worlds_items.pop() {
        let mut explorer = MultiworldExplorer::new();
        for (worldid, world) in worlds.iter_mut().enumerate() {
            explorer.add_world(
                worldid,
                None,
                world.placement,
                world.requirements,
                world.options,
            );
        }
        for (unplaced_item_world, unplaced_item) in worlds_items.iter() {
            explorer.insert_item(*unplaced_item_world, *unplaced_item);
        }
        let mut loc_idx = 0;
        while let Some((location_world, location)) = worlds_locations.get(loc_idx) {
            // println!("checking {item:?} at {location:?}");
            // println!("{:?}", explorer.inventory);
            if explorer.can_reach(*location_world, location) {
                // println!("placing {:?} at {:?}", item, location);
                worlds[*location_world]
                    .placement
                    .set_location(*location, item_world, item.into());
                worlds_locations.swap_remove(loc_idx);
                continue 'outer;
            }
            loc_idx += 1;
        }
        return Err(Error::NoLocationLeft {
            impossible_locations: format!("{:?}", worlds_locations),
            world: item_world,
            item,
        });
    }
    Ok(())
}

pub fn run_with_seed<'a>(
    seed: u64,
    requirements: &'a Requirements<'static>,
    world_count: usize,
) -> Result<Vec<SingleWorld<'a>>, CombinedError> {
    let mut rng = Pcg64::seed_from_u64(seed);

    let mut worlds = Vec::new();
    for world in 0..world_count {
        let single_world = generate_single_world(&mut rng, Options::default(), requirements, world)
            .context(PlandoSnafu { world })?;
        worlds.push(single_world);
    }
    let input_worlds: Vec<_> = worlds
        .iter_mut()
        .map(|world| InputWorld {
            items: world.items.clone(),
            locations: &mut world.locations,
            options: &world.options,
            placement: &mut world.placement,
            requirements: &world.world_requirements,
        })
        .collect();
    assumed_fill_worlds(&mut rng, input_worlds).context(FillSnafu { stage: "progress" })?;
    Ok(worlds)
}
