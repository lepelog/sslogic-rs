use crate::generated::{Event, Item, Location, Region, Stage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dungeon {
    Skyivew,
    EarthTemple,
    LanayruMiningFacility,
    AncientCistern,
    Sandship,
    FireSanctuary,
    // SkyKeep,
}

impl Dungeon {
    pub const POTENTIALLY_REQUIRED: &'static [Dungeon; 6] = &[
        Dungeon::Skyivew,
        Dungeon::EarthTemple,
        Dungeon::LanayruMiningFacility,
        Dungeon::AncientCistern,
        Dungeon::Sandship,
        Dungeon::FireSanctuary,
    ];

    pub fn get_beaten_event(&self) -> Event {
        match self {
            Dungeon::Skyivew => Event::CanBeatSkyview,
            Dungeon::EarthTemple => Event::CanBeatEarthTemple,
            Dungeon::LanayruMiningFacility => Event::CanBeatLanayruMiningFacility,
            Dungeon::AncientCistern => Event::CanBeatAncientCistern,
            Dungeon::Sandship => Event::CanBeatSandship,
            Dungeon::FireSanctuary => Event::CanBeatFireSanctuary,
        }
    }

    pub fn get_region(&self) -> Region {
        match self {
            Dungeon::Skyivew => Region::Skyview,
            Dungeon::EarthTemple => Region::EarthTemple,
            Dungeon::LanayruMiningFacility => Region::LanayruMiningFacility,
            Dungeon::AncientCistern => Region::AncientCistern,
            Dungeon::Sandship => Region::Sandship,
            Dungeon::FireSanctuary => Region::FireSanctuary,
        }
    }
}

pub struct DungeonRestrictionInfo {
    pub map: (Item, Location),
    pub small_key: Option<(Item, &'static [Location])>,
    pub boss_key: Option<(Item, Location)>,
    pub restricted_stages: &'static [Stage],
    pub unrestricted_stages: &'static [Stage],
}

pub const DUNGEON_RESTRICTION_INFO: &[DungeonRestrictionInfo] = &[
    DungeonRestrictionInfo {
        map: (Item::SkyviewMap, Location::SkyviewChestOnTreeBranch),
        small_key: Some((
            Item::SkyviewSmallKey,
            &[
                Location::SkyviewChestBehindTwoEyes,
                Location::SkyviewChestBehindThreeEyes,
            ],
        )),
        boss_key: Some((Item::SkyviewBossKey, Location::SkyviewBossKeyChest)),
        restricted_stages: &[Stage::SkyviewTemple],
        unrestricted_stages: &[Stage::SkyviewBoss, Stage::SkyviewSpring],
    },
    DungeonRestrictionInfo {
        map: (Item::EarthTempleMap, Location::EarthTempleChestInWestRoom),
        small_key: None,
        boss_key: Some((Item::EarthTempleBossKey, Location::EarthTempleBossKeyChest)),
        restricted_stages: &[Stage::EarthTemple],
        unrestricted_stages: &[Stage::EarthTempleBoss, Stage::EarthTempleSpring],
    },
    DungeonRestrictionInfo {
        map: (
            Item::LanayruMiningFacilityMap,
            Location::LanayruMiningFacilityChestInFirstWestRoom,
        ),
        small_key: Some((
            Item::LanayruMiningFacilitySmallKey,
            &[Location::LanayruMiningFacilityFirstChestInHubRoom],
        )),
        boss_key: Some((
            Item::LanayruMiningFacilityBossKey,
            Location::LanayruMiningFacilityBossKeyChest,
        )),
        restricted_stages: &[Stage::LanayruMiningFacilityA, Stage::LanayruMiningFacilityB],
        unrestricted_stages: &[
            Stage::LanayruMiningFacilityBoss,
            Stage::LanayruMiningFacilityToToT,
        ],
    },
    DungeonRestrictionInfo {
        map: (
            Item::AncientCisternMap,
            Location::AncientCisternChestAfterWhipHooks,
        ),
        small_key: Some((
            Item::AncientCisternSmallKey,
            &[
                Location::AncientCisternChestInEastPart,
                Location::AncientCisternBokoblin,
            ],
        )),
        boss_key: Some((
            Item::AncientCisternBossKey,
            Location::AncientCisternBossKeyChest,
        )),
        restricted_stages: &[Stage::AncientCistern],
        unrestricted_stages: &[Stage::AncientCisternBoss, Stage::AncientCisternCandleRoom],
    },
    DungeonRestrictionInfo {
        map: (
            Item::SandshipMap,
            Location::SandshipChestBefore4DoorCorridor,
        ),
        small_key: Some((
            Item::SandshipSmallKey,
            &[
                Location::SandshipChestBehindCombinationLock,
                Location::SandshipRobotInBrigsReward,
            ],
        )),
        boss_key: Some((Item::SandshipBossKey, Location::SandshipBossKeyChest)),
        restricted_stages: &[Stage::Sandship],
        unrestricted_stages: &[Stage::SandshipBoss],
    },
    DungeonRestrictionInfo {
        map: (
            Item::FireSanctuaryMap,
            Location::FireSanctuaryChestAfterSecondTrappedMogma,
        ),
        small_key: Some((
            Item::FireSanctuarySmallKey,
            &[
                Location::FireSanctuaryChestInFirstRoom,
                Location::FireSanctuaryChestNearFirstTrappedMogma,
                Location::FireSanctuaryChestAfterBombableWall,
            ],
        )),
        boss_key: Some((
            Item::FireSanctuaryBossKey,
            Location::FireSanctuaryBossKeyChest,
        )),
        restricted_stages: &[Stage::FireSanctuaryA, Stage::FireSanctuaryB],
        unrestricted_stages: &[Stage::FireSanctuaryBoss, Stage::FireSanctuaryFlameRoom],
    },
    DungeonRestrictionInfo {
        map: (Item::SkyKeepMap, Location::SkyKeepFirstChest),
        small_key: Some((
            Item::SkyKeepSmallKey,
            &[Location::SkyKeepChestAfterDreadfuse],
        )),
        boss_key: None,
        restricted_stages: &[Stage::SkyKeepEntry],
        unrestricted_stages: &[],
    },
];

pub const PROGRESS_ITEMS: &[(Item, u8)] = &[
    (Item::BombBag, 1),
    (Item::GustBellows, 1),
    (Item::Whip, 1),
    (Item::Clawshots, 1),
    (Item::WaterScale, 1),
    (Item::FireshieldEarrings, 1),
    (Item::SeaChart, 1),
    (Item::EmeraldTablet, 1),
    (Item::RubyTablet, 1),
    (Item::AmberTablet, 1),
    (Item::StoneOfTrials, 1),
    (Item::BabyRattle, 1),
    (Item::CawlinsLetter, 1),
    (Item::HornedColossusBeetle, 1),
    (Item::GoddessHarp, 1),
    (Item::BalladOfTheGoddess, 1),
    (Item::FaroresCourage, 1),
    (Item::NayrusWisdom, 1),
    (Item::DinsPower, 1),
    (Item::FaronSongOfTheHeroPart, 1),
    (Item::EldinSongOfTheHeroPart, 1),
    (Item::LanayruSongOfTheHeroPart, 1),
    (Item::LifeTreeFruit, 1),
    (Item::TriforceOfCourage, 1),
    (Item::TriforceOfPower, 1),
    (Item::TriforceOfWisdom, 1),
    (Item::GratitudeCrystalPack, 13),
    (Item::GratitudeCrystal, 15),
    (Item::ProgressiveSword, 6),
    (Item::ProgressiveMitts, 2),
    (Item::ProgressiveSlingshot, 2),
    (Item::ProgressiveBeetle, 4),
    (Item::ProgressiveBow, 3),
    (Item::ProgressiveBugNet, 2),
    (Item::ProgressivePouch, 5),
    (Item::KeyPiece, 5),
    (Item::EmptyBottle, 5),
    (Item::ProgressiveWallet, 4),
    (Item::ExtraWallet, 3),
    (Item::SpiralCharge, 1),
    // dungeon
    (Item::SkyviewSmallKey, 2),
    (Item::SkyviewBossKey, 1),
    (Item::EarthTempleBossKey, 1),
    (Item::LanayruMiningFacilitySmallKey, 1),
    (Item::LanayruMiningFacilityBossKey, 1),
    (Item::AncientCisternSmallKey, 2),
    (Item::AncientCisternBossKey, 1),
    (Item::SandshipSmallKey, 2),
    (Item::SandshipBossKey, 1),
    (Item::FireSanctuarySmallKey, 3),
    (Item::FireSanctuaryBossKey, 1),
    (Item::SkyKeepSmallKey, 1),
    (Item::LanayruCavesSmallKey, 1),
];

pub const NONPROGRESS_ITEMS: &[(Item, u8)] = &[
    (Item::WoodenShield, 1),
    (Item::HylianShield, 1),
    (Item::CursedMedal, 1),
    (Item::TreasureMedal, 1),
    (Item::PotionMedal, 1),
    (Item::BugMedal, 1),
    (Item::HeartMedal, 2),
    (Item::RupeeMedal, 2),
    (Item::LifeMedal, 2),
    (Item::SmallSeedSatchel, 1),
    (Item::SmallBombBag, 1),
    (Item::SmallQuiver, 1),
    (Item::HeartContainer, 6),
    (Item::HeartPiece, 24),
    // dungeon
    (Item::SkyviewMap, 2),
    (Item::EarthTempleMap, 1),
    (Item::LanayruMiningFacilityMap, 1),
    (Item::AncientCisternMap, 2),
    (Item::SandshipMap, 2),
    (Item::FireSanctuaryMap, 3),
    (Item::SkyKeepMap, 1),
];

pub const CONSUMABLE_ITEMS: &[(Item, u8)] = &[
    (Item::GreenRupee, 3),
    (Item::BlueRupee, 11),
    // (Item::RedRupee, 42),
    (Item::SilverRupee, 22),
    (Item::GoldRupee, 11),
    (Item::SemiRareTreasure, 10),
    (Item::GoldenSkull, 1),
    (Item::RareTreasure, 12),
    (Item::EvilCrystal, 2),
    (Item::EldinOre, 2),
    (Item::GoddessPlume, 1),
    (Item::DuskRelic, 1),
    (Item::Tumbleweed, 1),
    (Item::X5Bombs, 1),
];
