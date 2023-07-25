use std::{
    collections::{HashMap, HashSet},
    fs::File,
    iter::repeat,
};

use env_logger::Target;
use error::PlacementError;
use log::{debug, info};
use logic_core::{
    collect_spheres, get_requirements,
    options::{LmfStartState, Options, RandomizeEntrances, StartingSword, TriforceShuffle},
    BitSetCompatible, Entrance, Event, Exit, Explorer, Item, Location, Placement, Region,
    Requirements, Stage, TimeOfDay,
};
use plando::PlandoEntries;
use rand::{random, rngs::OsRng, seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg64;

use crate::{
    item_pools::{add_item_pool, PROGRESS_ITEMS},
    plando::{do_plando, LocationOrStart, PlandoEntry},
};

mod error;
mod item_pools;
mod plando;

const DUNGEON_END_CHECKS: [Location; 6] = [
    Location::Skyview_RubyTablet,
    Location::EarthTemple_AmberTablet,
    Location::LanayruMiningFacility_GoddessHarp,
    Location::AncientCistern_FaroresFlame,
    Location::Sandship_NayrusFlame,
    Location::FireSanctuary_DinsFlame,
];

pub fn random_options<R: Rng>(rng: &mut R) -> Options {
    Options {
        starting_tablet_count: rng.gen_range(0..=3),
        open_thunderhead: rng.gen(),
        starting_sword: *[
            StartingSword::Swordless,
            StartingSword::PracticeSword,
            StartingSword::GoddessSword,
            StartingSword::GoddessLongsword,
            StartingSword::GoddessWhiteSword,
            StartingSword::MasterSword,
            // StartingSword::TrueMasterSword,
        ]
        .choose(rng)
        .unwrap(),
        required_dungeon_count: rng.gen_range(0..6),
        imp2_skip: rng.gen(),
        empty_unrequired_dungeons: rng.gen(),
        triforce_required: rng.gen(),
        triforce_shuffle: *[
            TriforceShuffle::Vanilla,
            TriforceShuffle::SkyKeep,
            TriforceShuffle::Anywhere,
        ]
        .choose(rng)
        .unwrap(),
        randomize_entrances: *[
            RandomizeEntrances::None,
            RandomizeEntrances::AllDungeons,
            RandomizeEntrances::AllDungeonsSkyKeep,
            RandomizeEntrances::RequiredDungeonsSeparately,
        ]
        .choose(rng)
        .unwrap(),
        lmf_start_state: *[
            LmfStartState::Nodes,
            LmfStartState::MainNode,
            LmfStartState::Open,
        ]
        .choose(rng)
        .unwrap(),
        beedles_shop_vanilla: rng.gen(),
    }
}

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

fn get_plando_entries(options: &Options) -> PlandoEntries {
    let mut entries = PlandoEntries::default();
    if options.beedles_shop_vanilla {
        let VANILLA_SHOP = [(Item::ProgressiveBugNet, Location::Beedle_50RupeeItem)];
        for (item, loc) in VANILLA_SHOP {
            entries.add_fixed("Vanilla Shop", item, loc);
        }
    }

    let sword_count = match options.starting_sword {
        StartingSword::Swordless => 0,
        StartingSword::PracticeSword => 1,
        StartingSword::GoddessSword => 2,
        StartingSword::GoddessLongsword => 3,
        StartingSword::GoddessWhiteSword => 4,
        StartingSword::MasterSword => 5,
        StartingSword::TrueMasterSword => 6,
    };

    entries.push(PlandoEntry {
        name: "startsword",
        min_count: sword_count,
        max_count: sword_count,
        items: repeat((Item::ProgressiveSword, 1))
            .take(sword_count)
            .collect(),
        locations: repeat((LocationOrStart::Start, 1))
            .take(sword_count)
            .collect(),
    });

    let tab_count = options.starting_tablet_count.into();

    entries.push(PlandoEntry {
        name: "start tab",
        min_count: tab_count,
        max_count: tab_count,
        items: vec![
            (Item::EmeraldTablet, 1),
            (Item::RubyTablet, 1),
            (Item::AmberTablet, 1),
        ],
        locations: repeat((LocationOrStart::Start, 1))
            .take(tab_count)
            .collect(),
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
        entries.add_fixed("crystal", Item::GratitudeCrystal, crystal_check);
    }

    for (region, item, count) in &[
        (Region::Skyview, Item::SkyviewSmallKey, 2),
        (Region::Skyview, Item::SkyviewBossKey, 1),
        (Region::EarthTemple, Item::EarthTempleBossKey, 1),
        (
            Region::LanayruMiningFacility,
            Item::LanayruMiningFacilitySmallKey,
            1,
        ),
        (
            Region::LanayruMiningFacility,
            Item::LanayruMiningFacilityBossKey,
            1,
        ),
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
            .filter(|loc| !DUNGEON_END_CHECKS.contains(loc))
            .cloned();
        entries.add_area_restricted("dungeon", item.clone(), dungeon_locations, *count);
    }

    // entries.push(PlandoEntry {
    //     name: "stuff",
    //     min_count: 6,
    //     max_count: 6,
    //     items: vec![
    //         (Item::ProgressiveBeetle, 1),
    //         (Item::BombBag, 1),
    //         (Item::GustBellows, 1),
    //         (Item::Whip, 1),
    //         (Item::ProgressiveBow, 1),
    //         (Item::ProgressiveMitts, 1),
    //     ],
    //     locations: vec![
    //         (Location::Skyview_Beetle.into(), 1),
    //         (Location::EarthTemple_BombBag.into(), 1),
    //         (Location::LanayruMiningFacility_GustBellows.into(), 1),
    //         (Location::AncientCistern_Whip.into(), 1),
    //         (Location::Sandship_Bow.into(), 1),
    //         (Location::FireSanctuary_MogmaMitts.into(), 1),
    //     ],
    // });

    // entries.push(PlandoEntry {
    //     name: "stuff2",
    //     min_count: 2,
    //     max_count: 2,
    //     items: vec![(Item::ProgressiveBow, 1), (Item::ProgressiveBow, 1)],
    //     locations: vec![
    //         (Location::KnightAcademy_FledgesGift.into(), 1),
    //         (Location::KnightAcademy_OwlansGift.into(), 1),
    //     ],
    // });

    entries.push(PlandoEntry {
        name: "heh",
        min_count: 1,
        max_count: 1,
        items: vec![(Item::ProgressiveSword, 1)],
        locations: vec![(Location::Thunderhead_IsleOfSongsDinsPower.into(), 1)],
    });

    entries.push(PlandoEntry {
        name: "End Sword",
        min_count: 0,
        max_count: 6,
        items: repeat((Item::ProgressiveSword, 1)).take(6).collect(),
        locations: DUNGEON_END_CHECKS
            .into_iter()
            .map(|c| (c.into(), 1))
            .collect(),
    });

    entries
}

fn do_randomize<R: Rng>(
    rng: &mut R,
    requirements: &Requirements<'_>,
    options: &Options,
) -> Result<Placement, PlacementError> {
    let mut placement = Placement::new();
    placement.initial_events.insert(Event::SealedGroundsStatue);
    placement.initial_events.insert(Event::EldinEntranceStatue);
    placement
        .initial_events
        .insert(Event::LanayruMineEntryStatue);
    let start_entrance = *Entrance::ALL.choose(rng).unwrap();
    // let start_entrance = Entrance::KnightAcademy_From_Skyloft_Chimney;
    debug!("starting at {:?}", start_entrance);
    placement.initial_entrance = Some((start_entrance, TimeOfDay::Day));
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
    let options = random_options(&mut rng);

    println!("{:?}", options);

    let mut try_num = 0;
    let placement = loop {
        if try_num > 1000 {
            log::error!("could not find placement at all!");
            return;
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
    println!("failures: {try_num}");
    for (place, item) in placement.locations.iter() {
        if *item == Item::ProgressiveSword
            && !DUNGEON_END_CHECKS.contains(place)
            && *place != Location::Thunderhead_IsleOfSongsDinsPower
        {
            println!("OH NO: {:?}", place);
        }
    }
}
