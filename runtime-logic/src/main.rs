use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use anyhow::Context;
use explorer::Placement;
use loader::{load_logic, read_item_file};
use options::Options;
use rand::{rngs::OsRng, seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg64;
use requirements::{RequirementExpression, Requirements};
use structure::{DoubleDoor, ItemId, LocationKind, LogicContext, RequirementKey};

use crate::{
    bitset::BitSet,
    explorer::Explorer,
    options::{RequiredDungeon, SmallKeyMode},
    plando::place_items_plando,
    structure::{
        AreaId, ConnectionShuffleType, ContextLoadable, EntranceId, EventId, ExitId, LocationId, TimeOfDay
    },
};

mod bitset;
mod explorer;
mod loader;
mod options;
mod plando;
mod requirements;
mod structure;
mod util;
mod generated;
mod dumper;

pub fn assumed_fill<R: Rng>(
    rng: &mut R,
    requirements: &Requirements<'_>,
    placement: &mut Placement,
    options: &Options,
    locations: &mut Vec<LocationId>,
    items: &mut Vec<ItemId>,
    ctx: &LogicContext,
) -> Result<(), ()> {
    locations.shuffle(rng);
    items.shuffle(rng);
    'outer: while let Some(item) = items.pop() {
        let mut explorer = Explorer::create(&requirements, placement, options, ctx);
        for unplaced_item in items.iter() {
            explorer.insert_item(*unplaced_item);
        }
        let mut loc_idx = 0;
        while let Some(location) = locations.get(loc_idx) {
            // println!("checking {item:?} at {location:?}");
            // println!("{:?}", explorer.inventory);
            if explorer.can_reach(*location) {
                // println!("placing {} at {}", item.name(ctx), location.name(ctx));
                placement.set_location(*location, item);
                locations.swap_remove(loc_idx);
                continue 'outer;
            }
            loc_idx += 1;
        }
        println!("cannot place {}, no locations reachable", item.name(ctx));
        println!(
            "remaining locations: {}",
            ctx.display_iter(locations.iter().cloned())
        );
        return Err(());
    }
    Ok(())
}

pub fn junk_fill<R: Rng>(
    rng: &mut R,
    placement: &mut Placement,
    locations: &mut Vec<LocationId>,
    items: &mut Vec<ItemId>,
) {
    locations.shuffle(rng);
    items.shuffle(rng);
    loop {
        if items.is_empty() || locations.is_empty() {
            return;
        }
        let item = items.pop().unwrap();
        let location = locations.pop().unwrap();
        placement.set_location(location, item);
    }
}

pub const PROGRESS_ITEMS: &[(&str, u8)] = &[
    ("Bomb Bag", 1),
    ("Gust Bellows", 1),
    ("Whip", 1),
    ("Clawshots", 1),
    ("Water Scale", 1),
    ("Fireshield Earrings", 1),
    ("Sea Chart", 1),
    ("Emerald Tablet", 1),
    ("Ruby Tablet", 1),
    ("Amber Tablet", 1),
    ("Stone of Trials", 1),
    ("Baby Rattle", 1),
    ("Cawlin's Letter", 1),
    ("Horned Colossus Beetle", 1),
    ("Goddess Harp", 1),
    ("Ballad of the Goddess", 1),
    ("Farore's Courage", 1),
    ("Nayru's Wisdom", 1),
    ("Din's Power", 1),
    ("Faron Song of the Hero Part", 1),
    ("Eldin Song of the Hero Part", 1),
    ("Lanayru Song of the Hero Part", 1),
    ("Life Tree Fruit", 1),
    ("Triforce of Courage", 1),
    ("Triforce of Power", 1),
    ("Triforce of Wisdom", 1),
    ("Gratitude Crystal Pack", 13),
    ("Gratitude Crystal", 15),
    ("Progressive Sword", 6),
    ("Progressive Mitts", 2),
    ("Progressive Slingshot", 2),
    ("Progressive Beetle", 4),
    ("Progressive Bow", 3),
    ("Progressive Bug Net", 2),
    ("Progressive Pouch", 5),
    ("Key Piece", 5),
    ("Empty Bottle", 5),
    ("Progressive Wallet", 4),
    ("Extra Wallet", 3),
    ("Spiral Charge", 1),
    // dungeon
    ("Skyview Small Key", 2),
    ("Skyview Boss Key", 1),
    ("Earth Temple Boss Key", 1),
    ("Lanayru Mining Facility Small Key", 1),
    ("Lanayru Mining Facility Boss Key", 1),
    ("Ancient Cistern Small Key", 2),
    ("Ancient Cistern Boss Key", 1),
    ("Sandship Small Key", 2),
    ("Sandship Boss Key", 1),
    ("Fire Sanctuary Small Key", 3),
    ("Fire Sanctuary Boss Key", 1),
    ("Sky Keep Small Key", 1),
    ("LanayruCaves Small Key", 1),
];
fn collect_progress_items(ctx: &LogicContext, items: &mut Vec<ItemId>) {
    items.extend(PROGRESS_ITEMS.iter().flat_map(|(item_name, count)| {
        let item_id: ItemId = ctx.find(&item_name).expect(&item_name);
        repeat(item_id).take(*count as usize)
    }));
}
pub const NONPROGRESS_ITEMS: &[(&str, u8)] = &[
    ("Wooden Shield", 1),
    ("Hylian Shield", 1),
    ("Cursed Medal", 1),
    ("Treasure Medal", 1),
    ("Potion Medal", 1),
    ("Small Seed Satchel", 1),
    ("Small Bomb Bag", 1),
    ("Small Quiver", 1),
    ("Heart Medal", 2),
    ("Rupee Medal", 2),
    ("Heart Piece", 24),
    ("Heart Container", 6),
    ("Life Medal", 2),
    ("Bug Medal", 1),
    ("Gold Rupee", 10),
    ("Semi Rare Treasure", 10),
    ("Golden Skull", 1),
    ("Rare Treasure", 12),
    ("Evil Crystal", 2),
    ("Eldin Ore", 2),
    ("Goddess Plume", 1),
    ("Dusk Relic", 1),
    ("Tumbleweed", 1),
    ("5 Bombs", 1),
    ("Skyview Map", 1),
    ("Earth Temple Map", 1),
    ("Lanayru Mining Facility Map", 1),
    ("Ancient Cistern Map", 1),
    ("Sandship Map", 1),
    ("Fire Sanctuary Map", 1),
];

fn collect_nonprogress_items(ctx: &LogicContext, items: &mut Vec<ItemId>) {
    items.extend(NONPROGRESS_ITEMS.iter().flat_map(|(item_name, count)| {
        let item_id: ItemId = ctx.find(&item_name).expect(&item_name);
        repeat(item_id).take(*count as usize)
    }));
}

fn calculate_useless_items(
    item_pool: &HashMap<ItemId, u8>,
    ctx: &LogicContext,
    placement: &Placement,
    requirements: &Requirements<'_>,
    options: &Options,
    nonprogress_areas: &HashSet<AreaId>,
    nonprogress_locations: &HashSet<LocationId>,
) -> HashSet<ItemId> {
    let mut explorer = Explorer::create(requirements, placement, options, ctx);
    // for (item, count) in item_pool.iter() {
    //     explorer.insert_items(*item, *count);
    // }
    explorer.max_explore();
    let mut potentially_useless: HashSet<ItemId> = item_pool.keys().cloned().collect();
    let mut do_remove = |key| {
        requirements
            .get_requirement(key)
            .unwrap()
            .remove_used_items(&explorer.inventory, options, &mut potentially_useless)
    };
    for area in ctx
        .areas
        .iter()
        .filter(|a| !nonprogress_areas.contains(&a.id))
    {
        for location in area
            .locations
            .iter()
            .filter(|loc| !nonprogress_locations.contains(loc))
        {
            do_remove(RequirementKey::Location(*location));
        }
        for logic_exit in area
            .logic_exits
            .iter()
            .filter(|a| !nonprogress_areas.contains(a))
        {
            do_remove(RequirementKey::LogicExit {
                from: area.id,
                to: *logic_exit,
            });
        }
        for map_exit_id in area.map_exits.iter() {
            if let Some(entrace_id) = placement.connections.get(map_exit_id) {
                let entrance = entrace_id.ctx(ctx);
                if nonprogress_areas.contains(&entrance.area) {
                    continue;
                }
            }
            do_remove(RequirementKey::Exit(*map_exit_id));
        }
    }
    potentially_useless
    // if all locations are reachable without this item, make it nonprogress
}

fn rando_entrances_decoupled<R: Rng>(
    rng: &mut R,
    entrances: &mut Vec<EntranceId>,
    exits: &mut Vec<ExitId>,
    placement: &mut Placement,
    options: &Options,
    requirements: &Requirements<'_>,
    link_doors: bool,
    ctx: &LogicContext,
) {
    // remove_if would be nice here
    let mut right_door_exits = Vec::new();
    if link_doors {
        // remove all right entrances, they are never considered again
        entrances.retain(|e| !e.ctx(ctx).door_connection.is_right_door());
        // save all right exits, connect them to the same entrance as their left one
        let mut i = 0;
        while i < exits.len() {
            if exits[i].ctx(ctx).door_connection.is_right_door() {
                right_door_exits.push(exits.swap_remove(i))
            } else {
                i += 1;
            }
        }
    }
    entrances.shuffle(rng);
    exits.shuffle(rng);
    let progress_items: Vec<_> = PROGRESS_ITEMS
        .iter()
        .map(|(item_name, count)| (ItemId::find(ctx, &item_name).unwrap(), *count))
        .collect();
    while let Some(entrance) = entrances.pop() {
        // assume all unplaced entrances
        let mut explorer = Explorer::create(requirements, placement, options, ctx);
        for unplaced_entrance in entrances.iter() {
            let area = unplaced_entrance.ctx(ctx).area;
            explorer.insert_area_tod(area, area.ctx(ctx).time_of_day);
        }
        // assume all items
        for (item, count) in progress_items.iter() {
            explorer.insert_items(*item, *count);
        }
        // find a reachable exit
        let mut exit_iter = exits.iter().enumerate();
        loop {
            let (exit_index, exit_id) = exit_iter
                .next()
                .with_context(|| {
                    format!(
                        "remaining: {}\n{}\n{}",
                        ctx.display_iter(exits.iter().cloned()),
                        entrance.name(ctx),
                        ctx.display_iter(entrances.iter().cloned())
                    )
                })
                .unwrap();
            if explorer.can_reach_requirement(RequirementKey::Exit(*exit_id)) {
                // check if everything is still reachable
                let mut tmp_explorer = explorer.clone();
                tmp_explorer.set_connection(*exit_id, entrance);
                if tmp_explorer.get_unreachable().is_some() {
                    continue;
                }
                println!(
                    "connecting {} with {}",
                    exit_id.name(ctx),
                    entrance.name(ctx)
                );
                placement.connect(entrance, *exit_id);
                exits.swap_remove(exit_index);
                break;
            }
        }
    }

    if link_doors {
        for right_exit_id in right_door_exits.iter() {
            let right_exit = right_exit_id.ctx(ctx);
            let left_exit_id = right_exit.door_connection.get_left_neighbor().unwrap();
            let entrance_id = placement.disconnect_exit(left_exit_id).unwrap();
            let entrance = entrance_id.ctx(ctx);
            if entrance.door_connection.is_no() {
                // connect both exits to the one entrance
                placement.connect(entrance_id, right_exit.id);
                placement.connect(entrance_id, left_exit_id);
            } else {
                let left_entrance = entrance;
                let right_entrance_id = left_entrance.door_connection.get_right_neighbor().unwrap();
                placement.connect(left_entrance.id, left_exit_id);
                placement.connect(right_entrance_id, right_exit.id);
            }
        }
    }
}

fn print_spolier_log(
    placement: &Placement,
    required_dungeons: &[RequiredDungeon],
    ctx: &LogicContext,
) {
    println!("REQUIRED DUNGEONS: {:?}", required_dungeons);
    println!("START: {}", placement.initial_entrance.unwrap().0.name(ctx));
    println!("ENTRANCE CONNECTIONS");
    println!("====================");
    let mut sorted_entrance_connections: Vec<_> = placement
        .connections
        .iter()
        .map(|(exit_id, entrance_id)| (exit_id.name(ctx), entrance_id.name(ctx)))
        .collect();
    sorted_entrance_connections.sort_unstable();
    for (exit, entrance) in sorted_entrance_connections.iter() {
        println!("{exit:<53} leads to {entrance}");
    }
    println!("ITEMS");
    println!("=====");
    let mut sorted_locations: Vec<_> = placement
        .locations
        .iter()
        .map(|(loc_id, item_id)| (&loc_id.ctx(ctx).display_name, item_id.name(ctx)))
        .collect();
    sorted_locations.sort_unstable();
    let max_len = sorted_locations
        .iter()
        .map(|(loc, _)| loc.len())
        .max()
        .unwrap()
        + 1;
    for (loc, item) in sorted_locations.iter() {
        println!("{loc:<73} {item}");
    }
}

fn to_count_map<'a>(iter: impl Iterator<Item = &'a ItemId>) -> HashMap<ItemId, u8> {
    let mut out: HashMap<ItemId, u8> = HashMap::new();

    for item in iter {
        let entry = out.entry(*item).or_default();
        *entry = entry.saturating_add(1);
    }

    out
}

fn from_count_map(map: &HashMap<ItemId, u8>) -> Vec<ItemId> {
    let mut out: Vec<ItemId> = map
        .iter()
        .flat_map(|(item, count)| repeat(*item).take(*count as usize))
        .collect();
    out.sort_unstable();
    out
}

fn handle_entrance_randomizer_full<R: Rng>(rng: &mut R, placement: &mut Placement, options: &Options, requirements: &Requirements<'_>, ctx: &LogicContext) {
    let mut entrances = ctx
        .entrance
        .iter()
        .filter(|e| e.connection_shuffle_type == ConnectionShuffleType::Other)
        .map(|e| e.id)
        .collect();
    let mut exits = ctx
        .exits
        .iter()
        .filter(|e| e.connection_shuffle_type == ConnectionShuffleType::Other)
        .map(|e| e.id)
        .collect();

    for exit in ctx
        .exits
        .iter()
        .filter(|e| e.connection_shuffle_type == ConnectionShuffleType::Never)
    {
        let entrance_id = exit.vanilla_entrance.unwrap();
        placement.connect(entrance_id, exit.id);
    }

    // do entrance randomizer
    rando_entrances_decoupled(
        rng,
        &mut entrances,
        &mut exits,
        placement,
        &options,
        &requirements,
        true,
        &ctx,
    );
}

fn get_required_dungeons<'a>(dungeons: &[RequiredDungeon], requirements: &'a Requirements<'a>, ctx: &LogicContext) -> RequirementExpression<'a> {
    let mut reqs = Vec::new();
    for dungeon in dungeons {
        let end = match dungeon {
            RequiredDungeon::Skyview => "Skyview - Ruby Tablet",
            RequiredDungeon::EarthTemple => "Earth Temple - Amber Tablet",
            RequiredDungeon::LanayruMiningFacility => "Lanayru Mining Facility - Goddess Harp",
            RequiredDungeon::AncientCistern => "Ancient Cistern - Farore's Flame",
            RequiredDungeon::Sandship => "Sandship - Nayru's Flame",
            RequiredDungeon::FireSanctuary => "Fire Sanctuary - Din's Flame", 
        };
        
        reqs.push(requirements.get_requirement(ctx.find::<LocationId>(end).unwrap().into()).unwrap().clone());
    }
    RequirementExpression::And(reqs)
}

fn get_triforces_requirement(options: &Options, ctx: &LogicContext) -> RequirementExpression<'static> {
    let mut reqs = Vec::new();
    if options.triforce_required {
        reqs.push(RequirementExpression::Item(ctx.find("Triforce of Courage").unwrap(), 1));
        reqs.push(RequirementExpression::Item(ctx.find("Triforce of Power").unwrap(), 1));
        reqs.push(RequirementExpression::Item(ctx.find("Triforce of Wisdom").unwrap(), 1));
    }
    RequirementExpression::And(reqs)
}

fn main() {
    let seed = OsRng.gen();
    println!("{seed}");
    let mut rng = Pcg64::seed_from_u64(seed);
    let (ctx, base_requirements) = load_logic().unwrap();
    let mut options = Options::random(&mut rng);
    options.small_key_mode = SmallKeyMode::OwnDungeonRestricted;
    println!("{:?}", options);
    // figure out the item pool
    // we first only care about the progress items

    // choose required dungeons
    let required_dungeons = options.choose_req_dungeons(&mut rng);
    let mut requirements = base_requirements.create_layer();
    requirements.set_requirement(ctx.find::<EventId>("Beat Required Dungeons").unwrap().into(), get_required_dungeons(&required_dungeons, &base_requirements, &ctx));
    requirements.set_requirement(ctx.find::<EventId>("Sword Requirement Met").unwrap().into(), RequirementExpression::Item(ctx.find("Progressive Sword").unwrap(), options.got_sword_requirement));
    requirements.set_requirement(ctx.find::<EventId>("Triforces Collected").unwrap().into(), get_triforces_requirement(&options, &ctx));
    // TODO: game end requirements

    // mark progress/nonprogress areas
    let mut nonprogress_areas: HashSet<AreaId> = HashSet::new();
    let mut nonprogress_locations: HashSet<LocationId> = HashSet::new();
    for unrequired_dungeon in RequiredDungeon::ALL
        .iter()
        .filter(|d| !required_dungeons.contains(&d))
    {
        let region = unrequired_dungeon.to_region(&ctx).ctx(&ctx);
        for area in &region.areas {
            nonprogress_areas.insert(*area);
            nonprogress_locations.extend(area.ctx(&ctx).locations.iter().cloned());
        }
    }

    // let start_entrance: EntranceId = ctx.find("Skyloft from Knight Academy (Lower Doors Left Door)").unwrap();
    let start_entrance = ctx.entrance.choose(&mut rng).unwrap();
    let start_tod = if start_entrance.area.ctx(&ctx).time_of_day == TimeOfDay::Night {
        TimeOfDay::Night
    } else {
        TimeOfDay::Day
    };

    println!("start at {}", start_entrance.display_name);

    let mut initial_events = BitSet::new(ctx.events.len());
    let sealed_grounds_statue = ctx.find("Sealed Grounds Statue").unwrap();
    initial_events.insert(sealed_grounds_statue);
    initial_events.insert(ctx.find("Eldin Entrance Statue").unwrap());
    initial_events.insert(ctx.find("Lanayru Mine Entry Statue").unwrap());
    let mut placement = Placement {
        initial_items: HashMap::new(),
        initial_events,
        initial_entrance: Some((start_entrance.id, start_tod)),
        locations: HashMap::new(),
        connections: HashMap::new(),
        rev_connections: HashMap::new(),
    };

    // handle_entrance_randomizer_full(&mut rng, &mut placement, &options, &requirements, &ctx);
    // vanilla for now
    for exit in &ctx
        .exits
    {
        let entrance_id = exit.vanilla_entrance.unwrap();
        placement.connect(entrance_id, exit.id);
    }

    // mark items that are progress
    // mark items that should be in progress areas despite them being useless
    let mut location_pool: Vec<LocationId> = ctx
        .locations
        .iter()
        .filter_map(|loc| match &loc.kind {
            LocationKind::Check { .. } => Some(loc.id),
            _ => None,
        })
        .collect();

    pub const PROGRESS_ITEMS: &[(&str, u8)] = &[
        ("Bomb Bag", 1),
        ("Gust Bellows", 1),
        ("Whip", 1),
        ("Clawshots", 1),
        ("Water Scale", 1),
        ("Fireshield Earrings", 1),
        ("Sea Chart", 1),
        ("Emerald Tablet", 1),
        ("Ruby Tablet", 1),
        ("Amber Tablet", 1),
        ("Stone of Trials", 1),
        ("Baby Rattle", 1),
        ("Cawlin's Letter", 1),
        ("Horned Colossus Beetle", 1),
        ("Goddess Harp", 1),
        ("Ballad of the Goddess", 1),
        ("Farore's Courage", 1),
        ("Nayru's Wisdom", 1),
        ("Din's Power", 1),
        ("Faron Song of the Hero Part", 1),
        ("Eldin Song of the Hero Part", 1),
        ("Lanayru Song of the Hero Part", 1),
        ("Life Tree Fruit", 1),
        ("Triforce of Courage", 1),
        ("Triforce of Power", 1),
        ("Triforce of Wisdom", 1),
        ("Gratitude Crystal Pack", 13),
        ("Gratitude Crystal", 15),
        ("Progressive Sword", 6),
        ("Progressive Mitts", 2),
        ("Progressive Slingshot", 2),
        ("Progressive Beetle", 4),
        ("Progressive Bow", 3),
        ("Progressive Bug Net", 2),
        ("Progressive Pouch", 5),
        ("Key Piece", 5),
        ("Empty Bottle", 5),
        ("Progressive Wallet", 4),
        ("Extra Wallet", 3),
        ("Spiral Charge", 1),
        // dungeon
        ("Skyview Small Key", 2),
        ("Skyview Boss Key", 1),
        ("Earth Temple Boss Key", 1),
        ("Lanayru Mining Facility Small Key", 1),
        ("Lanayru Mining Facility Boss Key", 1),
        ("Ancient Cistern Small Key", 2),
        ("Ancient Cistern Boss Key", 1),
        ("Sandship Small Key", 2),
        ("Sandship Boss Key", 1),
        ("Fire Sanctuary Small Key", 3),
        ("Fire Sanctuary Boss Key", 1),
        ("Sky Keep Small Key", 1),
        ("LanayruCaves Small Key", 1),
    ];

    let mut item_pool: Vec<ItemId> = PROGRESS_ITEMS
        .iter()
        .flat_map(|(item_name, count)| {
            let item_id: ItemId = ctx.find(&item_name).expect(&item_name);
            repeat(item_id).take(*count as usize)
        })
        .collect();

    let useless = calculate_useless_items(
        &to_count_map(item_pool.iter()),
        &ctx,
        &placement,
        &requirements,
        &options,
        &nonprogress_areas,
        &nonprogress_locations,
    );

    println!("useless: {}", ctx.display_iter(useless.iter().copied()));

    // for entrance in &ctx.entrance {
    //     println!("{}", entrance.display_name);
    // }

    let plando_entries = options.generate_plando_entries(&ctx);

    let progress_locations = location_pool
        .iter()
        .filter(|loc| !nonprogress_locations.contains(loc))
        .cloned()
        .collect();
    let progress_items = item_pool
        .iter()
        .filter(|item| !useless.contains(item))
        .cloned()
        .collect();

    // TODO: we don't want to assume fill non progress items
    collect_nonprogress_items(&ctx, &mut item_pool);

    let mut location_set = location_pool.into_iter().collect();
    let mut items = to_count_map(item_pool.iter());

    place_items_plando(
        &mut rng,
        plando_entries,
        &requirements,
        &mut placement,
        &options,
        &mut location_set,
        &mut items,
        &progress_locations,
        &progress_items,
        &ctx,
    )
    .unwrap();

    let mut location_pool: Vec<LocationId> = location_set.into_iter().collect();
    // keys are unique to unstable is fine (and faster)
    location_pool.sort_unstable();

    let mut item_pool = from_count_map(&items);

    assumed_fill(
        &mut rng,
        &requirements,
        &mut placement,
        &options,
        &mut location_pool,
        &mut item_pool,
        &ctx,
    )
    .unwrap();

    let rupoor_item = ItemId::find(&ctx, "Rupoor").unwrap();

    while item_pool.len() < location_pool.len() {
        item_pool.push(rupoor_item);
    }

    // junk fill
    junk_fill(&mut rng, &mut placement, &mut location_pool, &mut item_pool);

    print_spolier_log(&placement, &required_dungeons, &ctx);

    // println!("{};{}", location_pool.len(), item_pool.len());
}
