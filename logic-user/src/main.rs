use std::{collections::{HashMap, HashSet}, fs::File};

use env_logger::Target;
use error::PlacementError;
use log::{info, debug};
use logic_core::{
    collect_spheres, get_requirements, BitSetCompatible, Entrance, Event, Exit, Explorer, Item,
    Location, Options, Placement, Requirements, Stage, TimeOfDay, Region,
};
use rand::{random, rngs::OsRng, seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg64;

use crate::{
    item_pools::{add_item_pool, PROGRESS_ITEMS},
    plando::{do_plando, LocationOrStart, PlandoEntry},
};

mod item_pools;
mod plando;
mod error;

pub fn assumed_fill<R: Rng>(
    rng: &mut R,
    requirements: &Requirements<'_>,
    placement: &mut Placement,
    options: &Options,
    locations: &mut Vec<Location>,
    items: &mut Vec<Item>,
) -> Result<(), PlacementError> {
    locations.shuffle(rng);
    items.shuffle(rng);
    'outer: while let Some(item) = items.pop() {
        let mut explorer = Explorer::create(&requirements, placement, options);
        for unplaced_item in items.iter() {
            explorer.insert_item(*unplaced_item);
        }
        let mut loc_idx = 0;
        while let Some(location) = locations.get(loc_idx) {
            // println!("checking {item:?} at {location:?}");
            // println!("{:?}", explorer.inventory);
            if explorer.can_reach(*location) {
                debug!("placing {item:?} at {location:?}");
                placement.set_location(*location, item);
                locations.swap_remove(loc_idx);
                continue 'outer;
            }
            loc_idx += 1;
        }
        debug!("cannot place {item:?}, no locations reachable");
        return Err(PlacementError::NoLocation);
    }
    Ok(())
}

pub fn junk_fill<R: Rng>(
    rng: &mut R,
    placement: &mut Placement,
    locations: &mut Vec<Location>,
    junk_pool: &[Item],
) {
    while let Some(location) = locations.pop() {
        let junk_item = *junk_pool.choose(rng).unwrap();
        placement.set_location(location, junk_item);
    }
}

pub fn flat_count_map<I: Copy>(counts: &HashMap<I, u8>) -> Vec<I> {
    let mut result = Vec::new();
    for (item, count) in counts {
        for _ in 0..*count {
            result.push(*item);
        }
    }
    result
}

fn get_plando_entries(options: &Options) -> Vec<PlandoEntry> {
    let mut entries = Vec::new();
    entries.push(PlandoEntry {
        name: "Vanilla Shop",
        count: 1,
        items: vec![(Item::ProgressiveBugNet, 1)],
        locations: vec![(Location::Beedle_50RupeeItem.into(), 1)],
    });

    let crystal_checks = [
        Location::Sky_CrystalInsideLumpyPumpkin,
        Location::Sky_CrystalOutsideLumpyPumpkin,
        Location::Sky_CrystalOnBeedlesShip,
        Location::KnightAcademy_CrystalInKnightAcademyPlant,
        Location::KnightAcademy_CrystalInLinksRoom,
        Location::KnightAcademy_CrystalInZeldasRoom,
        Location::KnightAcademy_CrystalInSparringHall,
        Location::SkyloftVillage_CrystalNearPumpkinPatch,
        Location::CentralSkyloft_CrystalAfterWaterfallCave,
        Location::CentralSkyloft_CrystalBetweenWoodenPlanks,
        Location::CentralSkyloft_CrystalInLoftwingPrison,
        Location::CentralSkyloft_CrystalInOrielleAndParrowsHouse,
        Location::CentralSkyloft_CrystalOnLightTower,
        Location::CentralSkyloft_CrystalOnWaterfallIsland,
        Location::CentralSkyloft_CrystalOnWestCliff,
    ];

    for crystal_check in crystal_checks {
        entries.push(PlandoEntry { name: "crystal", count: 1, items: vec![(Item::GratitudeCrystal, 1)], locations: vec![(crystal_check.into(), 1)] });
    }

    for (region, item, count) in &[
        (Region::Skyview, Item::SkyviewSmallKey, 2),
        (Region::Skyview, Item::SkyviewBossKey, 1),
        (Region::EarthTemple, Item::EarthTempleBossKey, 1),
        (Region::LanayruMiningFacility, Item::LanayruMiningFacilitySmallKey, 1),
        (Region::LanayruMiningFacility, Item::LanayruMiningFacilityBossKey, 1),
        (Region::AncientCistern, Item::AncientCisternSmallKey, 2),
        (Region::AncientCistern, Item::AncientCisternBossKey, 1),
        (Region::Sandship, Item::SandshipSmallKey, 2),
        (Region::Sandship, Item::SandshipBossKey, 1),
        (Region::FireSanctuary, Item::FireSanctuarySmallKey, 3),
        (Region::FireSanctuary, Item::FireSanctuaryBossKey, 1),
        (Region::SkyKeep, Item::SkyKeepSmallKey, 1),
        (Region::LanayruCaves, Item::LanayruCavesSmallKey, 1),
    ] {
        let dungeon_locations = Location::ALL
            .iter()
            .filter(|loc| loc.area().stage().region() == *region)
            .map(|loc| (LocationOrStart::Location(*loc), 1))
            .collect();
        entries.push(PlandoEntry {
            name: "dungeon",
            count: *count,
            items: vec![(*item, 1)],
            locations: dungeon_locations,
        });
    }

    entries.push(PlandoEntry {
        name: "stuff",
        count: 6,
        items: vec![
            (Item::ProgressiveBeetle, 1),
            (Item::BombBag, 1),
            (Item::GustBellows, 1),
            (Item::Whip, 1),
            (Item::ProgressiveBow, 1),
            (Item::ProgressiveMitts, 1),
        ],
        locations: vec![
            (Location::Skyview_Beetle.into(), 1),
            (Location::EarthTemple_BombBag.into(), 1),
            (Location::LanayruMiningFacility_GustBellows.into(), 1),
            (Location::AncientCistern_Whip.into(), 1),
            (Location::Sandship_Bow.into(), 1),
            (Location::FireSanctuary_MogmaMitts.into(), 1),
        ],
    });

    entries.push(PlandoEntry {
        name: "End Sword",
        count: 5,
        items: vec![(Item::ProgressiveSword, 1)],
        locations: vec![
            (Location::Skyview_RubyTablet.into(), 1),
            (Location::EarthTemple_AmberTablet.into(), 1),
            (Location::LanayruMiningFacility_GoddessHarp.into(), 1),
            (Location::AncientCistern_FaroresFlame.into(), 1),
            (Location::Sandship_NayrusFlame.into(), 1),
            (Location::FireSanctuary_DinsFlame.into(), 1),
        ],
    });

    entries
}

fn do_randomize<R: Rng>(rng: &mut R, requirements: &Requirements<'_>, options: &Options) -> Result<Placement, PlacementError> {
    let mut placement = Placement::new();
    placement.initial_events.insert(Event::SealedGroundsStatue);
    placement.initial_events.insert(Event::EldinEntranceStatue);
    placement
        .initial_events
        .insert(Event::LanayruMineEntryStatue);
    let start_entrance = *Entrance::ALL.choose(rng).unwrap();
    // let start_entrance = Entrance::KnightAcademy_From_Skyloft_Chimney;
    debug!("starting at {:?}", start_entrance);
    placement.initial_entrance = Some((start_entrance, TimeOfDay::Night));
    for exit in Exit::ALL {
        placement.connect(exit.vanilla_entrance(), *exit);
    }
    let mut locations: HashSet<Location> = Location::ALL.iter().cloned().collect();
    let mut items: HashMap<Item, u8> = PROGRESS_ITEMS.iter().cloned().collect();

    let mut entries = get_plando_entries(options);

    do_plando(
        rng,
        &mut entries,
        &requirements,
        &mut placement,
        &options,
        &mut locations,
        &mut items,
    )?;
    let mut locations: Vec<_> = locations.iter().cloned().collect();
    locations.sort_unstable_by_key(|l| *l as usize);
    let mut items = flat_count_map(&items);
    items.sort_unstable_by_key(|i| *i as usize);
    // add_item_pool(PROGRESS_ITEMS, &mut items);
    assumed_fill(
        rng,
        &requirements,
        &mut placement,
        &options,
        &mut locations,
        &mut items,
    )?;
    junk_fill(
        rng,
        &mut placement,
        &mut locations,
        &[Item::BlueRupee, Item::RedRupee, Item::GoldRupee],
    );
    Ok(placement)
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .parse_default_env()
        .target(Target::Pipe(Box::new(File::create("out.log").unwrap())))
        .init();
    let seed = OsRng.gen();
    // let seed = 7025465005869385372;
    info!("seed: {seed}");
    let mut rng = Pcg64::seed_from_u64(seed);
    let requirements = Requirements::new_from_map(get_requirements());
    let options = Options::new();
    
    let mut try_num = 0;
    let placement = loop {
        if try_num > 100 {
            log::error!("could not find placement at all!");
        }
        debug!("Try {try_num}");
        try_num += 1;
        match do_randomize(&mut rng, &requirements, &options) {
            Ok(placement) => break placement,
            Err(_e) => continue,
        }
    };

    let spheres = collect_spheres(&requirements, &placement, &options);
    for (num, sphere) in spheres.iter().enumerate() {
        println!("Sphere {num}:");
        for (loc, item) in sphere {
            println!(
                " {} - {}: {}",
                loc.area().stage().region().name(),
                loc.name(),
                item.name()
            );
        }
    }
}
