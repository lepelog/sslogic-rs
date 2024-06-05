use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use rand::prelude::*;

use crate::{
    constants::{
        Dungeon, CONSUMABLE_ITEMS, DUNGEON_RESTRICTION_INFO, NONPROGRESS_ITEMS, PROGRESS_ITEMS,
    },
    explorer::Placement,
    generated::{
        Area, BossKeyMode, Entrance, Event, Exit, GotDungeonRequirement, GotStart, Item, Location,
        MapMode, Options, Region, ShopMode, SmallKeyMode, Stage, StartingSword,
    },
    logic_static::{
        BitSetCompatible, Inventory, RequirementExpression, RequirementKey, Requirements, TimeOfDay,
    },
    plando::{
        self, place_items_plando, LocationOrStart, PlandoEntry, WeightedItem, WeightedLocation,
    },
    util::sample_stable,
};

pub struct ItemMeta {
    pub hintable: bool,
    pub progress_loc: bool,
}

pub struct SingleWorld<'a> {
    pub world_id: usize,
    pub placement: Placement,
    pub world_requirements: Requirements<'a>,
    pub banned_areas: HashSet<Area>,
    pub progress_locations: HashSet<Location>,
    pub items: Vec<Item>,
    pub item_meta: HashMap<Item, ItemMeta>,
    pub options: Options,
    pub locations: Vec<Location>,
}

impl<'a> SingleWorld<'a> {
    pub fn write_placement(&self, out: &mut impl std::fmt::Write) -> std::fmt::Result {
        writeln!(out, "Startitems")?;
        writeln!(out, "==========")?;
        let mut start_items: Vec<Item> = self
            .placement
            .initial_items
            .iter()
            .flat_map(|(item, count)| repeat(*item).take(*count))
            .collect();
        start_items.sort_unstable();
        for start_item in &start_items {
            writeln!(out, "{}", start_item.get().name)?;
        }
        let max_loc_len = Region::ALL
            .iter()
            .flat_map(|r| r.areas())
            .flat_map(|a| a.get().locations)
            .map(|l| l.get().name.len())
            .max()
            .unwrap_or_default()
            + 2;
        for region in Region::ALL {
            writeln!(out)?;
            writeln!(out, "{}", region.name())?;
            writeln!(out, "{:=<1$}", "", region.name().len())?;
            for loc in region.areas().iter().flat_map(|a| a.get().locations) {
                if let Some((world, item)) = self.placement.get_item_at(*loc) {
                    let item_name = item.as_item().map_or("*Empty*", |item| item.get().name);
                    if world == self.world_id {
                        writeln!(
                            out,
                            "{:<maxlen$} {}",
                            loc.get().name,
                            item_name,
                            maxlen = max_loc_len
                        )?;
                    } else {
                        writeln!(
                            out,
                            "{:<maxlen$} {} (for World {})",
                            loc.get().name,
                            item_name,
                            world,
                            maxlen = max_loc_len
                        )?;
                    }
                }
            }
        }
        Ok(())
    }
}

// randomizes a single world, as long as it doesn't need to interact with other worlds

pub fn generate_single_world<'a, R: Rng>(
    rng: &mut R,
    options: Options,
    base_requirements: &'a Requirements<'static>,
    world_index: usize,
) -> Result<SingleWorld<'a>, plando::Error> {
    let required_dungeons: Vec<_> = sample_stable(
        rng,
        Dungeon::POTENTIALLY_REQUIRED,
        options.required_dungeon_count,
    )
    .collect();
    let mut world_requirements = base_requirements.create_layer();
    // TODO: only if option requires it
    world_requirements.set_requirement(
        Event::TriforcesCollected.into(),
        RequirementExpression::And(vec![
            RequirementExpression::Item(Item::TriforceOfCourage, 1),
            RequirementExpression::Item(Item::TriforceOfPower, 1),
            RequirementExpression::Item(Item::TriforceOfWisdom, 1),
        ]),
    );
    let req_dungeons_reqs = required_dungeons
        .iter()
        .map(|dungeon| RequirementExpression::Event(dungeon.get_beaten_event()))
        .collect();
    world_requirements.set_requirement(
        Event::BeatRequiredDungeons.into(),
        RequirementExpression::And(req_dungeons_reqs),
    );

    // figure out item pool
    let mut item_pool = HashMap::new();

    // TODO rename, probably
    let mut item_meta = HashMap::new();
    let mut useless_items = HashSet::new();
    for (item, count) in PROGRESS_ITEMS {
        item_pool.insert(*item, *count);
        item_meta.insert(
            *item,
            ItemMeta {
                hintable: true,
                progress_loc: true,
            },
        );
        useless_items.insert(*item);
    }
    for (item, count) in NONPROGRESS_ITEMS.iter().chain(CONSUMABLE_ITEMS.iter()) {
        item_pool.insert(*item, *count);
        item_meta.insert(
            *item,
            ItemMeta {
                hintable: false,
                progress_loc: false,
            },
        );
    }

    let mut placement = Placement::new();

    // TODO: can be variable
    placement.initial_events.insert(Event::SealedGroundsStatue);
    placement.initial_events.insert(Event::EldinEntranceStatue);
    placement
        .initial_events
        .insert(Event::LanayruMineEntryStatue);

    // TODO: entrance randomizer
    placement.initial_entrance = Some((
        Entrance::KnightAcademy_From_Skyloft_Lower_Left,
        TimeOfDay::Day,
    ));
    for exit in Exit::ALL {
        if let Some(entrance) = exit.get().vanilla_entrance {
            placement.connect(entrance, *exit);
        }
    }

    let banned_areas: HashSet<Area> = Dungeon::POTENTIALLY_REQUIRED
        .iter()
        .filter(|dungeon| options.empty_unrequired_dungeons && !required_dungeons.contains(dungeon))
        .flat_map(|dungeon| dungeon.get_region().areas())
        .copied()
        .collect();

    let empty_inventory = Inventory::default();
    let mut progress_locations = HashSet::new();

    for area in Area::ALL {
        if banned_areas.contains(area) {
            continue;
        }
        let area_def = area.get();
        // check locations
        for loc in area_def.locations {
            world_requirements
                .get_requirement(loc.into())
                .unwrap()
                .remove_used_items(&empty_inventory, &options, &mut useless_items);
            progress_locations.insert(*loc);
        }
        // TODO: events could be bound to areas as well
        for exit_area in area_def.logic_exits {
            world_requirements
                .get_requirement(RequirementKey::LogicExit {
                    from: *area,
                    to: *exit_area,
                })
                .unwrap()
                .remove_used_items(&empty_inventory, &options, &mut useless_items);
        }
        for exit in area_def.map_exits {
            if let Some(entrance) = placement.get_connected_entrance(*exit) {
                if !banned_areas.contains(&entrance.get().area) {
                    world_requirements
                        .get_requirement(exit.into())
                        .unwrap()
                        .remove_used_items(&empty_inventory, &options, &mut useless_items);
                }
            }
        }
    }

    for useless_item in useless_items.iter() {
        item_meta.insert(
            *useless_item,
            ItemMeta {
                hintable: false,
                progress_loc: false,
            },
        );
    }

    // generate plando entries
    let entries = plando_entries_for_options(&options);
    let mut locations = Location::ALL.iter().cloned().collect();
    let progress_items = item_meta
        .iter()
        .filter_map(|(item, meta)| (meta.progress_loc).then_some(*item))
        .collect();
    place_items_plando(
        rng,
        entries,
        &world_requirements,
        &mut placement,
        &options,
        &mut locations,
        &mut item_pool,
        &progress_locations,
        &progress_items,
        world_index,
    )?;

    let mut locations: Vec<Location> = locations.into_iter().collect();
    locations.sort_unstable();

    let mut items: Vec<Item> = item_pool
        .into_iter()
        .flat_map(|(item, count)| repeat(item).take(count as usize))
        .collect();
    items.sort_unstable();

    Ok(SingleWorld {
        world_id: world_index,
        banned_areas,
        placement,
        item_meta,
        items,
        progress_locations,
        world_requirements,
        options,
        locations,
    })
}

pub fn plando_entries_for_options(options: &Options) -> Vec<PlandoEntry> {
    let mut entries = Vec::new();
    fn nonforced_vanilla(entries: &mut Vec<PlandoEntry>, item: Item, loc: Location) {
        entries.push(PlandoEntry::Flex {
            items: vec![WeightedItem {
                item: item.into(),
                weight: 1,
            }],
            locations: vec![WeightedLocation {
                location: loc.into(),
                weight: 1,
            }],
            count: 1,
            is_plando: false,
        });
    }
    fn stage_restricted<'a>(
        entries: &mut Vec<PlandoEntry>,
        item: Item,
        stages: impl Iterator<Item = &'a Stage>,
        count: usize,
    ) {
        let locations = stages
            .flat_map(|stage| stage.get().areas)
            .flat_map(|area| area.get().locations)
            .map(|loc| WeightedLocation {
                location: (*loc).into(),
                weight: 1,
            })
            .collect();
        entries.push(PlandoEntry::Flex {
            items: vec![
                WeightedItem {
                    item: item.into(),
                    weight: 1
                };
                count
            ],
            locations,
            count,
            is_plando: false,
        });
    }
    fn startitem(entries: &mut Vec<PlandoEntry>, item: Item) {
        entries.push(PlandoEntry::Flex {
            items: vec![WeightedItem {
                item: item.into(),
                weight: 1,
            }],
            locations: vec![WeightedLocation {
                location: LocationOrStart::Start,
                weight: 1,
            }],
            count: 1,
            is_plando: false,
        });
    }
    match options.small_key_mode {
        SmallKeyMode::Anywhere => (),
        SmallKeyMode::Vanilla => {
            for entry in DUNGEON_RESTRICTION_INFO.iter() {
                if let Some((item, locations)) = entry.small_key {
                    for loc in locations {
                        nonforced_vanilla(&mut entries, item, *loc);
                    }
                }
            }
        }
        SmallKeyMode::LanayruCavesKeyOnly | SmallKeyMode::OwnDungeonRestricted => {
            for entry in DUNGEON_RESTRICTION_INFO.iter() {
                if let Some((item, locations)) = entry.small_key {
                    stage_restricted(
                        &mut entries,
                        item,
                        entry.restricted_stages.iter(),
                        locations.len(),
                    );
                }
            }
            if options.small_key_mode != SmallKeyMode::LanayruCavesKeyOnly {
                stage_restricted(
                    &mut entries,
                    Item::LanayruCavesSmallKey,
                    [Stage::LanayruCaves].iter(),
                    1,
                );
            }
        }
    }
    match options.boss_key_mode {
        BossKeyMode::Vanilla => {
            for entry in DUNGEON_RESTRICTION_INFO.iter() {
                if let Some((item, location)) = entry.boss_key {
                    nonforced_vanilla(&mut entries, item, location);
                }
            }
        }
        BossKeyMode::OwnDungeon => {
            for entry in DUNGEON_RESTRICTION_INFO.iter() {
                if let Some((item, _)) = entry.boss_key {
                    stage_restricted(&mut entries, item, entry.restricted_stages.iter(), 1);
                }
            }
        }
        BossKeyMode::Anywhere => (),
    }
    match options.map_mode {
        MapMode::Removed => todo!("remove from item pool"),
        MapMode::Vanilla => {
            for entry in DUNGEON_RESTRICTION_INFO.iter() {
                let (item, location) = entry.map;
                nonforced_vanilla(&mut entries, item, location);
            }
        }
        MapMode::OwnDungeonRestricted => {
            for entry in DUNGEON_RESTRICTION_INFO.iter() {
                stage_restricted(&mut entries, entry.map.0, entry.restricted_stages.iter(), 1);
            }
        }
        MapMode::OwnDungeonUnrestricted => {
            for entry in DUNGEON_RESTRICTION_INFO.iter() {
                stage_restricted(
                    &mut entries,
                    entry.map.0,
                    entry
                        .restricted_stages
                        .iter()
                        .chain(entry.unrestricted_stages),
                    1,
                );
            }
        }
        MapMode::Anywhere => (),
    }
    match options.got_dungeon_requirement {
        GotDungeonRequirement::Required => (),
        GotDungeonRequirement::Unrequired => unimplemented!(),
    }
    match options.got_start {
        GotStart::Lowered => (),
        GotStart::Raised => unimplemented!(),
    }
    match options.shop_mode {
        ShopMode::Vanilla => {
            nonforced_vanilla(
                &mut entries,
                Item::ProgressiveBugNet,
                Location::Beedle50RupeeItem,
            );
            nonforced_vanilla(&mut entries, Item::BugMedal, Location::Beedle1000RupeeItem);
            nonforced_vanilla(
                &mut entries,
                Item::ExtraWallet,
                Location::BeedleFirst100RupeeItem,
            );
            nonforced_vanilla(
                &mut entries,
                Item::ExtraWallet,
                Location::BeedleSecond100RupeeItem,
            );
            nonforced_vanilla(
                &mut entries,
                Item::ExtraWallet,
                Location::BeedleThird100RupeeItem,
            );
            nonforced_vanilla(&mut entries, Item::LifeMedal, Location::Beedle800RupeeItem);
            nonforced_vanilla(
                &mut entries,
                Item::HeartMedal,
                Location::Beedle1600RupeeItem,
            );
            nonforced_vanilla(
                &mut entries,
                Item::ProgressivePouch,
                Location::Beedle300RupeeItem,
            );
            nonforced_vanilla(
                &mut entries,
                Item::ProgressivePouch,
                Location::Beedle600RupeeItem,
            );
            nonforced_vanilla(
                &mut entries,
                Item::ProgressivePouch,
                Location::Beedle1200RupeeItem,
            );
        }
        ShopMode::Randomized | ShopMode::AlwaysJunk => (),
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
    for _ in 0..sword_count {
        startitem(&mut entries, Item::ProgressiveSword);
    }
    if options.start_with_pouch {
        startitem(&mut entries, Item::ProgressivePouch);
    }
    entries.push(PlandoEntry::Flex {
        items: vec![
            WeightedItem {
                item: Item::EmeraldTablet.into(),
                weight: 1,
            },
            WeightedItem {
                item: Item::RubyTablet.into(),
                weight: 1,
            },
            WeightedItem {
                item: Item::AmberTablet.into(),
                weight: 1,
            },
        ],
        locations: vec![
            WeightedLocation {
                location: LocationOrStart::Start,
                weight: 1,
            },
            WeightedLocation {
                location: LocationOrStart::Start,
                weight: 1,
            },
            WeightedLocation {
                location: LocationOrStart::Start,
                weight: 1,
            },
        ],
        count: options.starting_tablet_count,
        is_plando: false,
    });
    entries
}
