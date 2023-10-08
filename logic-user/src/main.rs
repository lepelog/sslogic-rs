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
    Requirements, Stage, TimeOfDay, AreaBitset,
};
use plando::{PlacementEntryErrorHandling, PlandoEntries};
use rand::{
    random,
    rngs::OsRng,
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use rand_pcg::Pcg64;
use util::sample_stable;

use crate::{
    hints::calc_sots_path,
    item_pools::{add_item_pool, PROGRESS_ITEMS},
    plando::{do_plando, LocationOrStart, PlandoEntry},
};

mod error;
mod hints;
mod item_pools;
mod plando;
mod util;

const POTENTIALLY_REQUIRED_DUNGEONS: [Region; 6] = [
    Region::Skyview,
    Region::EarthTemple,
    Region::LanayruMiningFacility,
    Region::AncientCistern,
    Region::Sandship,
    Region::FireSanctuary,
];

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
    // TODO: consider these
    progress_locations: &HashSet<Location>,
    progress_items: &HashSet<Item>,
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

// TODO: plando
pub fn randomize_required_dungeons<R: Rng>(rng: &mut R, count: u8) -> Vec<Region> {
    sample_stable(rng, &POTENTIALLY_REQUIRED_DUNGEONS, count.into())
        .cloned()
        .collect()
}

fn get_plando_entries(options: &Options, required_dungeons: &Vec<Region>) -> PlandoEntries {
    let mut entries = PlandoEntries::default();
    if options.beedles_shop_vanilla {
        let VANILLA_SHOP = [(Item::ProgressiveBugNet, Location::Beedle_50RupeeItem)];
        for (item, loc) in VANILLA_SHOP {
            entries.add_fixed("Vanilla Shop", item, loc, true);
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

    entries.push(PlandoEntry::Flex {
        name: "startsword",
        count: sword_count,
        items: repeat((Item::ProgressiveSword, 1))
            .take(sword_count)
            .collect(),
        locations: repeat((LocationOrStart::Start, 1))
            .take(sword_count)
            .collect(),
        is_plando: false,
    });

    let tab_count = options.starting_tablet_count.into();

    entries.push(PlandoEntry::Flex {
        name: "start tab",
        count: tab_count,
        items: vec![
            (Item::EmeraldTablet, 1),
            (Item::RubyTablet, 1),
            (Item::AmberTablet, 1),
        ],
        locations: repeat((LocationOrStart::Start, 1))
            .take(tab_count)
            .collect(),
        is_plando: false,
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
        entries.add_fixed("crystal", Item::GratitudeCrystal, crystal_check, false);
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

    entries.push(PlandoEntry::Flex {
        name: "heh",
        count: 1,
        items: vec![(Item::ProgressiveSword, 1)],
        locations: vec![(Location::Thunderhead_IsleOfSongsDinsPower.into(), 1)],
        is_plando: true,
    });

    entries.push(PlandoEntry::Flex {
        name: "End Sword",
        count: 5,
        items: repeat((Item::ProgressiveSword, 1)).take(6).collect(),
        locations: DUNGEON_END_CHECKS
            .into_iter()
            .map(|c| (c.into(), 1))
            .collect(),
        is_plando: false,
    });

    entries
}

fn do_randomize<R: Rng>(
    rng: &mut R,
    requirements: &Requirements<'_>,
    options: &Options,
) -> Result<Placement, PlacementError> {
    let required_dungeons = randomize_required_dungeons(rng, options.required_dungeon_count);
    let mut placement = Placement::new();
    placement.initial_events.insert(Event::SealedGroundsStatue);
    placement.initial_events.insert(Event::EldinEntranceStatue);
    placement
        .initial_events
        .insert(Event::LanayruMineEntryStatue);
    let start_entrance = *Entrance::ALL.choose(rng).unwrap();
    // let start_entrance = Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_HubW;
    let start_tod = start_entrance
        .area()
        .possible_tod()
        .iter()
        .choose(rng)
        .unwrap();
    debug!("starting at {:?} at {:?}", start_entrance, start_tod);
    placement.initial_entrance = Some((start_entrance, start_tod));
    for exit in Exit::ALL {
        placement.connect(exit.vanilla_entrance(), *exit);
    }
    let mut locations: HashSet<Location> = Location::ALL.iter().cloned().collect();
    let mut items: HashMap<Item, u8> = PROGRESS_ITEMS.iter().cloned().collect();

    let mut entries = get_plando_entries(options, &required_dungeons);

    let progress_locations = locations.clone();
    let progress_items = items.keys().cloned().collect();

    do_plando(
        rng,
        &mut entries,
        &requirements,
        &mut placement,
        &options,
        &mut locations,
        &mut items,
        &mut HashSet::new(),
        &mut HashSet::new(),
        &progress_locations,
        &progress_items,
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
        &progress_locations,
        &progress_items,
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
    let mut options = random_options(&mut rng);

    options.starting_sword = StartingSword::GoddessSword;

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
    calc_sots_path(&requirements, &placement, &options);
    // for (place, item) in placement.locations.iter() {
    //     if *item == Item::ProgressiveSword
    //         && !DUNGEON_END_CHECKS.contains(place)
    //         && *place != Location::Thunderhead_IsleOfSongsDinsPower
    //     {
    //         println!("OH NO: {:?}", place);
    //     }
    // }
}
