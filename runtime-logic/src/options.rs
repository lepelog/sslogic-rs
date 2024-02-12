use std::iter::repeat;

use rand::{seq::SliceRandom, Rng};

use crate::{
    plando::{PlandoEntry, WeightedItem, WeightedLocation},
    structure::{ContextLoadable, ItemId, LocationId, LogicContext, Region, RegionId, StageId},
    util::sample_stable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriforceShuffle {
    Vanilla,
    SkyKeep,
    Anywhere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RupoorMode {
    Off,
    Added,
    Mayhem,
    Insanity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenLmf {
    Nodes,
    MainNode,
    Open,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapMode {
    Removed,
    Vanilla,
    OwnDungeonRestricted,
    OwnDungeonUnrestricted,
    Anywhere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmallKeyMode {
    Vanilla,
    OwnDungeonRestricted,
    OwnDungeonUnrestricted,
    LanayruCavesKeyOnly,
    Anywhere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossKeyMode {
    Vanilla,
    OwnDungeon,
    Anywhere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwordDungeonReward {
    None,
    HeartContainer,
    FinalCheck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenLakeFloria {
    Vanilla,
    TalkToYerbal,
    Open,
}

#[derive(Debug)]
pub struct Options {
    // Logic
    pub start_tablet_count: u8,
    pub open_thunderhead: bool,
    pub starting_swords: u8,
    pub required_dungeons: u8,
    pub empty_unrequired_dungeons: bool,
    pub triforce_required: bool,
    pub triforce_shuffle: TriforceShuffle,
    pub shopsanity: bool,
    pub rupoor_mode: RupoorMode,
    pub gondo_upgrades: bool,
    pub got_sword_requirement: u8, // count of progressive swords
    pub open_lmf: OpenLmf,
    pub map_mode: MapMode,
    pub small_key_mode: SmallKeyMode,
    pub boss_key_mode: BossKeyMode,
    pub sword_dungeon_reward: SwordDungeonReward,
    pub open_et: bool,
    pub open_lake_floria: OpenLakeFloria,
    pub random_start_item: bool,

    // Game
    pub skip_imp2: bool,
    pub skip_horde: bool,
    pub skip_g3: bool,
    pub skip_demise: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            start_tablet_count: 3,
            open_thunderhead: true,
            starting_swords: 2,
            required_dungeons: 2,
            empty_unrequired_dungeons: true,
            triforce_required: false,
            triforce_shuffle: TriforceShuffle::Vanilla,
            shopsanity: true,
            rupoor_mode: RupoorMode::Off,
            gondo_upgrades: false,
            got_sword_requirement: 5,
            open_lmf: OpenLmf::Nodes,
            map_mode: MapMode::OwnDungeonUnrestricted,
            small_key_mode: SmallKeyMode::OwnDungeonRestricted,
            boss_key_mode: BossKeyMode::OwnDungeon,
            sword_dungeon_reward: SwordDungeonReward::None,
            open_et: false,
            open_lake_floria: OpenLakeFloria::Vanilla,
            random_start_item: false,
            skip_imp2: true,
            skip_horde: true,
            skip_g3: true,
            skip_demise: true,
        }
    }
}

impl Options {
    pub fn choose_req_dungeons<R: Rng>(&self, rng: &mut R) -> Vec<RequiredDungeon> {
        sample_stable(rng, RequiredDungeon::ALL, self.required_dungeons as usize)
            .cloned()
            .collect()
    }

    pub fn random<R: Rng>(rng: &mut R) -> Self {
        Self {
            start_tablet_count: rng.gen_range(0..=3),
            open_thunderhead: rng.gen(),
            starting_swords: rng.gen_range(0..6),
            required_dungeons: rng.gen_range(0..=6),
            empty_unrequired_dungeons: rng.gen(),
            triforce_required: rng.gen(),
            triforce_shuffle: *[
                TriforceShuffle::Anywhere,
                TriforceShuffle::SkyKeep,
                TriforceShuffle::Vanilla,
            ]
            .choose(rng)
            .unwrap(),
            shopsanity: rng.gen(),
            rupoor_mode: RupoorMode::Added,
            gondo_upgrades: rng.gen(),
            got_sword_requirement: rng.gen_range(2..=6),
            open_lmf: OpenLmf::MainNode,
            map_mode: *[
                MapMode::Anywhere,
                MapMode::OwnDungeonRestricted,
                MapMode::OwnDungeonUnrestricted,
                MapMode::Vanilla,
            ]
            .choose(rng)
            .unwrap(),
            small_key_mode: *[
                SmallKeyMode::Anywhere,
                SmallKeyMode::OwnDungeonRestricted,
                SmallKeyMode::OwnDungeonUnrestricted,
                SmallKeyMode::Vanilla,
            ]
            .choose(rng)
            .unwrap(),
            boss_key_mode: *[
                BossKeyMode::Anywhere,
                BossKeyMode::OwnDungeon,
                BossKeyMode::Vanilla,
            ]
            .choose(rng)
            .unwrap(),
            sword_dungeon_reward: SwordDungeonReward::None,
            open_et: rng.gen(),
            open_lake_floria: OpenLakeFloria::Vanilla,
            random_start_item: false,
            skip_imp2: rng.gen(),
            skip_horde: rng.gen(),
            skip_g3: rng.gen(),
            skip_demise: rng.gen(),
        }
    }

    pub fn generate_plando_entries(&self, ctx: &LogicContext) -> Vec<PlandoEntry> {
        let mut entries = Vec::new();
        for crystal_loc in [
            "Central Skyloft - Crystal between Wooden Planks",
            "Central Skyloft - Crystal on West Cliff",
            "Central Skyloft - Crystal on Light Tower",
            "Central Skyloft - Crystal on Waterfall Island",
            "Central Skyloft - Crystal after Waterfall Cave",
            "Central Skyloft - Crystal in Loftwing Prison",
            "Central Skyloft - Crystal in Orielle and Parrow's House",
            "Knight Academy - Crystal in Link's Room",
            "Knight Academy - Crystal in Knight Academy Plant",
            "Knight Academy - Crystal in Zelda's Room",
            "Knight Academy - Crystal in Sparring Hall",
            "Skyloft Village - Crystal near Pumpkin Patch",
            "Sky - Crystal on Beedle's Ship",
            "Sky - Crystal outside Lumpy Pumpkin",
            "Sky - Crystal inside Lumpy Pumpkin",
        ] {
            entries.push(PlandoEntry::Fixed { item: ctx.find("Gratitude Crystal").unwrap(), location: ctx.find(crystal_loc).unwrap(), vacant_fallback: false });
        }
        if self.start_tablet_count > 0 {
            entries.push(PlandoEntry::Flex {
                count: self.start_tablet_count.into(),
                is_plando: false,
                items: vec![
                    WeightedItem::simple(ctx.find("Emerald Tablet").unwrap()),
                    WeightedItem::simple(ctx.find("Ruby Tablet").unwrap()),
                    WeightedItem::simple(ctx.find("Amber Tablet").unwrap()),
                ],
                locations: vec![
                    WeightedLocation::start(),
                    WeightedLocation::start(),
                    WeightedLocation::start(),
                ],
            })
        }
        if self.starting_swords > 0 {
            let sword: ItemId = ctx.find("Progressive Sword").unwrap();
            entries.push(PlandoEntry::Flex {
                items: repeat(WeightedItem::simple(sword))
                    .take(self.starting_swords.into())
                    .collect(),
                locations: repeat(WeightedLocation::start())
                    .take(self.starting_swords.into())
                    .collect(),
                count: self.starting_swords.into(),
                is_plando: false,
            });
        }
        // match self.triforce_shuffle {
        //     TriforceShuffle::Anywhere => (),
        //     TriforceShuffle::Vanilla => {
        //         entries.push(PlandoEntry::Fixed {
        //             item: ctx.find("Triforce of Courage").unwrap(),
        //             location: ctx.find("Sky Keep - Triforce of Courage").unwrap(),
        //             vacant_fallback: false,
        //         });
        //         entries.push(PlandoEntry::Fixed {
        //             item: ctx.find("Triforce of Power").unwrap(),
        //             location: ctx.find("Sky Keep - Triforce of Power").unwrap(),
        //             vacant_fallback: false,
        //         });
        //         entries.push(PlandoEntry::Fixed {
        //             item: ctx.find("Triforce of Wisdom").unwrap(),
        //             location: ctx.find("Sky Keep - Triforce of Wisdom").unwrap(),
        //             vacant_fallback: false,
        //         });
        //     }
        //     TriforceShuffle::SkyKeep => {
        //         let locations = ctx
        //             .find::<RegionId>("Sky Keep")
        //             .unwrap()
        //             .ctx(ctx)
        //             .areas
        //             .iter()
        //             .flat_map(|area| {
        //                 area.ctx(ctx)
        //                     .locations
        //                     .iter()
        //                     .cloned()
        //                     .map(WeightedLocation::simple)
        //             })
        //             .collect();
        //         let items = vec![
        //             WeightedItem::simple(ctx.find("Triforce of Courage").unwrap()),
        //             WeightedItem::simple(ctx.find("Triforce of Power").unwrap()),
        //             WeightedItem::simple(ctx.find("Triforce of Wisdom").unwrap()),
        //         ];
        //         entries.push(PlandoEntry::Flex {
        //             items,
        //             locations,
        //             count: 3,
        //             is_plando: false,
        //         });
        //     }
        // }
        if !self.shopsanity {
            entries.push(PlandoEntry::Fixed {
                item: ctx.find("Progressive Bug Net").unwrap(),
                location: ctx.find("Beedle - 50 Rupee Item").unwrap(),
                vacant_fallback: false,
            });
            entries.push(PlandoEntry::Fixed {
                item: ctx.find("Bug Medal").unwrap(),
                location: ctx.find("Beedle - 1000 Rupee Item").unwrap(),
                vacant_fallback: false,
            });
            entries.push(PlandoEntry::Fixed {
                item: ctx.find("Extra Wallet").unwrap(),
                location: ctx.find("Beedle - First 100 Rupee Item").unwrap(),
                vacant_fallback: false,
            });
            entries.push(PlandoEntry::Fixed {
                item: ctx.find("Extra Wallet").unwrap(),
                location: ctx.find("Beedle - Second 100 Rupee Item").unwrap(),
                vacant_fallback: false,
            });
            entries.push(PlandoEntry::Fixed {
                item: ctx.find("Extra Wallet").unwrap(),
                location: ctx.find("Beedle - Third 100 Rupee Item").unwrap(),
                vacant_fallback: false,
            });
            entries.push(PlandoEntry::Fixed {
                item: ctx.find("Progressive Pouch").unwrap(),
                location: ctx.find("Beedle - 300 Rupee Item").unwrap(),
                vacant_fallback: false,
            });
            entries.push(PlandoEntry::Fixed {
                item: ctx.find("Progressive Pouch").unwrap(),
                location: ctx.find("Beedle - 600 Rupee Item").unwrap(),
                vacant_fallback: false,
            });
            entries.push(PlandoEntry::Fixed {
                item: ctx.find("Progressive Pouch").unwrap(),
                location: ctx.find("Beedle - 1200 Rupee Item").unwrap(),
                vacant_fallback: false,
            });
            entries.push(PlandoEntry::Fixed {
                item: ctx.find("Life Medal").unwrap(),
                location: ctx.find("Beedle - 800 Rupee Item").unwrap(),
                vacant_fallback: false,
            });
            entries.push(PlandoEntry::Fixed {
                item: ctx.find("Heart Piece").unwrap(),
                location: ctx.find("Beedle - 1600 Rupee Item").unwrap(),
                vacant_fallback: false,
            });
        }
        let dungeon_item_info = [DungeonItemInfo {
            region: ctx.find::<RegionId>("Skyview").unwrap().ctx(ctx),
            restricted_stages: [
                ctx.find("Skyview Boss").unwrap(),
                ctx.find("Skyview Spring").unwrap(),
            ],
            map_item: ctx.find("Skyview Map").unwrap(),
            vanilla_map_location: "Skyview - Chest on Tree Branch",
            small_key_item: ctx.find("Skyview Small Key").unwrap(),
            vanilla_small_key_locations: &[
                "Skyview - Chest behind Two Eyes",
                "Skyview - Chest behind Three Eyes",
            ],
            boss_key_item: ctx.find("Skyview Boss Key").unwrap(),
            vanilla_boss_key_location: "Skyview - Boss Key Chest",
        }];
        let r = |name| {
            StageId::find(ctx, name).unwrap()
        };
        let l = |name| {
            LocationId::find(ctx, name).unwrap()
        };
        let map_info: [(&str, &[StageId], &str, &str); 7] = [
            ("Skyview", &[r("Skyview Boss"), r("Skyview Spring")], "Skyview Map", "Skyview - Chest on Tree Branch"),
            ("Earth Temple", &[r("Earth Temple Boss"), r("Earth Temple Spring")], "Earth Temple Map", "Earth Temple - Chest in West Room"),
            ("Lanayru Mining Facility", &[r("Lanayru Mining Facility Boss"), r("Lanayru Mining Facility to ToT")], "Lanayru Mining Facility Map", "Lanayru Mining Facility - Chest in First West Room"),
            ("Ancient Cistern", &[r("Ancient Cistern Boss"), r("Ancient Cistern Candle Room")], "Ancient Cistern Map", "Ancient Cistern - Chest after Whip Hooks"),
            ("Sandship", &[r("Sandship Boss"), r("Sandship Spring")], "Sandship Map", "Sandship - Chest before 4-Door Corridor"),
            ("Fire Sanctuary", &[r("Fire Sanctuary Boss"), r("Fire Sanctuary Spring")], "Fire Sanctuary Map", "Fire Sanctuary - Chest after Second Trapped Mogma"),
            ("Sky Keep", &[], "Sky Keep Map", "Sky Keep - First Chest"),
        ];
        match self.map_mode {
            MapMode::Vanilla => {
                for (_, _, item, vanilla) in map_info {
                    entries.push(PlandoEntry::Fixed {
                        item: ctx.find(item).unwrap(),
                        location: ctx.find(vanilla).unwrap(),
                        vacant_fallback: false,
                    });
                }
            }
            MapMode::Anywhere => (),
            MapMode::Removed => (),
            MapMode::OwnDungeonRestricted | MapMode::OwnDungeonUnrestricted => {
                let is_unrestricted = matches!(self.map_mode, MapMode::OwnDungeonUnrestricted);
                for (region_name, restricted_stages, item, _) in map_info {
                    let region = RegionId::find(ctx, region_name).unwrap();
                    let locations = region.ctx(ctx)
                        .areas
                        .iter()
                        .filter(|a| {
                            is_unrestricted || !restricted_stages.contains(&a.ctx(ctx).stage)
                        })
                        .flat_map(|a| {
                            a.ctx(ctx)
                                .locations
                                .iter()
                                .map(|l| WeightedLocation::simple(*l))
                        })
                        .collect();
                    let items = vec![WeightedItem::simple(ctx.find(item).unwrap())];
                    entries.push(PlandoEntry::Flex {
                        items,
                        locations,
                        count: 1,
                        is_plando: false,
                    });
                }
            }
        }
        let small_key_info: [(&str, &[StageId], &str, &[&str]); 7] = [
            ("Skyview", &[r("Skyview Boss"), r("Skyview Spring")], "Skyview Small Key", &["Skyview - Chest behind Two Eyes", "Skyview - Chest behind Three Eyes"]),
            ("Lanayru Mining Facility", &[r("Lanayru Mining Facility Boss"), r("Lanayru Mining Facility to ToT")], "Lanayru Mining Facility Small Key", &["Lanayru Mining Facility - First Chest in Hub Room"]),
            ("Ancient Cistern", &[r("Ancient Cistern Boss"), r("Ancient Cistern Candle Room")], "Ancient Cistern Small Key", &["Ancient Cistern - Chest in East Part", "Ancient Cistern - Bokoblin"]),
            ("Sandship", &[r("Sandship Boss"), r("Sandship Spring")], "Sandship Small Key", &["Sandship - Chest behind Combination Lock", "Sandship - Robot in Brig's Reward"]),
            ("Fire Sanctuary", &[r("Fire Sanctuary Boss"), r("Fire Sanctuary Spring")], "Fire Sanctuary Small Key", &["Fire Sanctuary - Chest in First Room", "Fire Sanctuary - Chest near First Trapped Mogma", "Fire Sanctuary - Chest after Bombable Wall"]),
            ("Sky Keep", &[], "Sky Keep Small Key", &["Sky Keep - Chest after Dreadfuse"]),
            ("Lanayru Caves", &[], "Lanayru Caves Small Key", &["Lanayru Caves - Chest"]),
        ];
        entries
    }
}

struct DungeonItemInfo<'a> {
    region: &'a Region,
    restricted_stages: [StageId; 2],
    map_item: ItemId,
    vanilla_map_location: &'static str,
    small_key_item: ItemId,
    vanilla_small_key_locations: &'static [&'static str],
    boss_key_item: ItemId,
    vanilla_boss_key_location: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequiredDungeon {
    Skyview,
    EarthTemple,
    LanayruMiningFacility,
    AncientCistern,
    Sandship,
    FireSanctuary,
}

impl RequiredDungeon {
    pub const ALL: &[RequiredDungeon; 6] = &[
        Self::Skyview,
        Self::EarthTemple,
        Self::LanayruMiningFacility,
        Self::AncientCistern,
        Self::Sandship,
        Self::FireSanctuary,
    ];

    pub fn to_region(&self, ctx: &LogicContext) -> RegionId {
        let name = match self {
            RequiredDungeon::Skyview => "Skyview",
            RequiredDungeon::EarthTemple => "Earth Temple",
            RequiredDungeon::LanayruMiningFacility => "Lanayru Mining Facility",
            RequiredDungeon::AncientCistern => "Ancient Cistern",
            RequiredDungeon::Sandship => "Sandship",
            RequiredDungeon::FireSanctuary => "Fire Sanctuary",
        };
        ctx.find(name).unwrap()
    }
}
