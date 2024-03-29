#![allow(non_camel_case_types)]
use crate::logic_static::{BitSetCompatible, TimeOfDay};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Region {
    AncientCistern,
    Batreaux,
    Beedle,
    CentralSkyloft,
    EarthTemple,
    EldinSilentRealm,
    EldinVolcano,
    FaronSilentRealm,
    FaronWoods,
    FireSanctuary,
    KnightAcademy,
    LakeFloria,
    LanayruCaves,
    LanayruDesert,
    LanayruMines,
    LanayruMiningFacility,
    LanayruSandSea,
    LanayruSilentRealm,
    MogmaTurf,
    Sandship,
    SealedGrounds,
    Sky,
    SkyKeep,
    SkyloftSilentRealm,
    SkyloftVillage,
    Skyview,
    Thunderhead,
    VolcanoSummit,
}
impl Into<usize> for Region {
    fn into(self) -> usize {
        self as usize
    }
}
impl BitSetCompatible for Region {
    const ALL: &'static [Region] = &[
        Region::AncientCistern,
        Region::Batreaux,
        Region::Beedle,
        Region::CentralSkyloft,
        Region::EarthTemple,
        Region::EldinSilentRealm,
        Region::EldinVolcano,
        Region::FaronSilentRealm,
        Region::FaronWoods,
        Region::FireSanctuary,
        Region::KnightAcademy,
        Region::LakeFloria,
        Region::LanayruCaves,
        Region::LanayruDesert,
        Region::LanayruMines,
        Region::LanayruMiningFacility,
        Region::LanayruSandSea,
        Region::LanayruSilentRealm,
        Region::MogmaTurf,
        Region::Sandship,
        Region::SealedGrounds,
        Region::Sky,
        Region::SkyKeep,
        Region::SkyloftSilentRealm,
        Region::SkyloftVillage,
        Region::Skyview,
        Region::Thunderhead,
        Region::VolcanoSummit,
    ];
}
impl Region {
    pub fn name(&self) -> &'static str {
        match self {
            Region::AncientCistern => "Ancient Cistern",
            Region::Batreaux => "Batreaux",
            Region::Beedle => "Beedle",
            Region::CentralSkyloft => "Central Skyloft",
            Region::EarthTemple => "Earth Temple",
            Region::EldinSilentRealm => "Eldin Silent Realm",
            Region::EldinVolcano => "Eldin Volcano",
            Region::FaronSilentRealm => "Faron Silent Realm",
            Region::FaronWoods => "Faron Woods",
            Region::FireSanctuary => "Fire Sanctuary",
            Region::KnightAcademy => "Knight Academy",
            Region::LakeFloria => "Lake Floria",
            Region::LanayruCaves => "Lanayru Caves",
            Region::LanayruDesert => "Lanayru Desert",
            Region::LanayruMines => "Lanayru Mines",
            Region::LanayruMiningFacility => "Lanayru Mining Facility",
            Region::LanayruSandSea => "Lanayru Sand Sea",
            Region::LanayruSilentRealm => "Lanayru Silent Realm",
            Region::MogmaTurf => "Mogma Turf",
            Region::Sandship => "Sandship",
            Region::SealedGrounds => "Sealed Grounds",
            Region::Sky => "Sky",
            Region::SkyKeep => "Sky Keep",
            Region::SkyloftSilentRealm => "Skyloft Silent Realm",
            Region::SkyloftVillage => "Skyloft Village",
            Region::Skyview => "Skyview",
            Region::Thunderhead => "Thunderhead",
            Region::VolcanoSummit => "Volcano Summit",
        }
    }
}
impl Region {
    pub fn stages(&self) -> &'static [Stage] {
        match self {
            Region::AncientCistern => &[
                Stage::AncientCistern,
                Stage::AncientCisternBoss,
                Stage::AncientCisternCandleRoom,
            ],
            Region::Batreaux => &[Stage::BatreauxHouse],
            Region::Beedle => &[Stage::BeedlesShop],
            Region::CentralSkyloft => &[
                Stage::Bazaar,
                Stage::ParrowAndOriellesHouse,
                Stage::PeatricesHouse,
                Stage::PipersHouse,
                Stage::Skyloft,
                Stage::WaterfallCave,
                Stage::WrynasHouse,
            ],
            Region::EarthTemple => &[
                Stage::EarthTemple,
                Stage::EarthTempleBoss,
                Stage::EarthTempleSpring,
            ],
            Region::EldinSilentRealm => &[Stage::EldinSilentRealm],
            Region::EldinVolcano => &[Stage::EldinVolcano, Stage::ThrillDiggerCave],
            Region::FaronSilentRealm => &[Stage::FaronSilentRealm],
            Region::FaronWoods => &[Stage::DeepWoods, Stage::FaronWoods, Stage::GreatTree],
            Region::FireSanctuary => &[
                Stage::FireSanctuaryA,
                Stage::FireSanctuaryB,
                Stage::FireSanctuaryBoss,
                Stage::FireSanctuaryFlameRoom,
            ],
            Region::KnightAcademy => &[
                Stage::InsideGoddessStatue,
                Stage::KnightAcademy,
                Stage::Skyloft,
                Stage::SparringHall,
            ],
            Region::LakeFloria => &[
                Stage::FaroresLair,
                Stage::FloriaWaterfall,
                Stage::LakeFloria,
            ],
            Region::LanayruCaves => &[Stage::LanayruCaves],
            Region::LanayruDesert => &[
                Stage::FireNode,
                Stage::LanayruDesert,
                Stage::LightningNode,
                Stage::TempleOfTime,
            ],
            Region::LanayruMines => &[Stage::LanayruMines],
            Region::LanayruMiningFacility => &[
                Stage::LanayruMiningFacilityA,
                Stage::LanayruMiningFacilityB,
                Stage::LanayruMiningFacilityBoss,
                Stage::LanayruMiningFacilityToToT,
            ],
            Region::LanayruSandSea => &[
                Stage::InsidePiratesStronghold,
                Stage::OutsidePiratesStronghold,
                Stage::SandSea,
                Stage::SandSeaDocks,
                Stage::Shipyard,
                Stage::ShipyardConstructionBay,
                Stage::SkippersRetreat,
                Stage::SkippersShack,
            ],
            Region::LanayruSilentRealm => &[Stage::LanayruSilentRealm],
            Region::MogmaTurf => &[Stage::MogmaTurf],
            Region::Sandship => &[Stage::Sandship, Stage::SandshipBoss],
            Region::SealedGrounds => &[
                Stage::BehindTheTemple,
                Stage::HyliasTemple,
                Stage::SealedGroundsSpiral,
                Stage::SealedTemple,
            ],
            Region::Sky => &[Stage::InsideBambooIsland, Stage::LumpyPumpkin, Stage::Sky],
            Region::SkyKeep => &[Stage::SkyKeepEntry],
            Region::SkyloftSilentRealm => &[Stage::SkyloftSilentRealm],
            Region::SkyloftVillage => &[
                Stage::BertiesHouse,
                Stage::GondosHouse,
                Stage::MallarasHouse,
                Stage::RupinsHouse,
                Stage::Skyloft,
                Stage::SparrotsHouse,
            ],
            Region::Skyview => &[
                Stage::SkyviewBoss,
                Stage::SkyviewSpring,
                Stage::SkyviewTemple,
            ],
            Region::Thunderhead => &[Stage::InsideThunderhead, Stage::IsleOfSongs],
            Region::VolcanoSummit => &[
                Stage::InsideVolcanoSummit,
                Stage::OutsideFireSanctuary,
                Stage::VolcanoSummitWaterfall,
            ],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stage {
    AncientCistern,
    AncientCisternBoss,
    AncientCisternCandleRoom,
    BatreauxHouse,
    BeedlesShop,
    Bazaar,
    ParrowAndOriellesHouse,
    PeatricesHouse,
    PipersHouse,
    Skyloft,
    WaterfallCave,
    WrynasHouse,
    EarthTemple,
    EarthTempleBoss,
    EarthTempleSpring,
    EldinSilentRealm,
    EldinVolcano,
    ThrillDiggerCave,
    FaronSilentRealm,
    DeepWoods,
    FaronWoods,
    GreatTree,
    FireSanctuaryA,
    FireSanctuaryB,
    FireSanctuaryBoss,
    FireSanctuaryFlameRoom,
    InsideGoddessStatue,
    KnightAcademy,
    SparringHall,
    FaroresLair,
    FloriaWaterfall,
    LakeFloria,
    LanayruCaves,
    FireNode,
    LanayruDesert,
    LightningNode,
    TempleOfTime,
    LanayruMines,
    LanayruMiningFacilityA,
    LanayruMiningFacilityB,
    LanayruMiningFacilityBoss,
    LanayruMiningFacilityToToT,
    InsidePiratesStronghold,
    OutsidePiratesStronghold,
    SandSea,
    SandSeaDocks,
    Shipyard,
    ShipyardConstructionBay,
    SkippersRetreat,
    SkippersShack,
    LanayruSilentRealm,
    MogmaTurf,
    Sandship,
    SandshipBoss,
    BehindTheTemple,
    HyliasTemple,
    SealedGroundsSpiral,
    SealedTemple,
    InsideBambooIsland,
    LumpyPumpkin,
    Sky,
    SkyKeepEntry,
    SkyloftSilentRealm,
    BertiesHouse,
    GondosHouse,
    MallarasHouse,
    RupinsHouse,
    SparrotsHouse,
    SkyviewBoss,
    SkyviewSpring,
    SkyviewTemple,
    InsideThunderhead,
    IsleOfSongs,
    InsideVolcanoSummit,
    OutsideFireSanctuary,
    VolcanoSummitWaterfall,
}
impl Into<usize> for Stage {
    fn into(self) -> usize {
        self as usize
    }
}
impl BitSetCompatible for Stage {
    const ALL: &'static [Stage] = &[
        Stage::AncientCistern,
        Stage::AncientCisternBoss,
        Stage::AncientCisternCandleRoom,
        Stage::BatreauxHouse,
        Stage::BeedlesShop,
        Stage::Bazaar,
        Stage::ParrowAndOriellesHouse,
        Stage::PeatricesHouse,
        Stage::PipersHouse,
        Stage::Skyloft,
        Stage::WaterfallCave,
        Stage::WrynasHouse,
        Stage::EarthTemple,
        Stage::EarthTempleBoss,
        Stage::EarthTempleSpring,
        Stage::EldinSilentRealm,
        Stage::EldinVolcano,
        Stage::ThrillDiggerCave,
        Stage::FaronSilentRealm,
        Stage::DeepWoods,
        Stage::FaronWoods,
        Stage::GreatTree,
        Stage::FireSanctuaryA,
        Stage::FireSanctuaryB,
        Stage::FireSanctuaryBoss,
        Stage::FireSanctuaryFlameRoom,
        Stage::InsideGoddessStatue,
        Stage::KnightAcademy,
        Stage::SparringHall,
        Stage::FaroresLair,
        Stage::FloriaWaterfall,
        Stage::LakeFloria,
        Stage::LanayruCaves,
        Stage::FireNode,
        Stage::LanayruDesert,
        Stage::LightningNode,
        Stage::TempleOfTime,
        Stage::LanayruMines,
        Stage::LanayruMiningFacilityA,
        Stage::LanayruMiningFacilityB,
        Stage::LanayruMiningFacilityBoss,
        Stage::LanayruMiningFacilityToToT,
        Stage::InsidePiratesStronghold,
        Stage::OutsidePiratesStronghold,
        Stage::SandSea,
        Stage::SandSeaDocks,
        Stage::Shipyard,
        Stage::ShipyardConstructionBay,
        Stage::SkippersRetreat,
        Stage::SkippersShack,
        Stage::LanayruSilentRealm,
        Stage::MogmaTurf,
        Stage::Sandship,
        Stage::SandshipBoss,
        Stage::BehindTheTemple,
        Stage::HyliasTemple,
        Stage::SealedGroundsSpiral,
        Stage::SealedTemple,
        Stage::InsideBambooIsland,
        Stage::LumpyPumpkin,
        Stage::Sky,
        Stage::SkyKeepEntry,
        Stage::SkyloftSilentRealm,
        Stage::BertiesHouse,
        Stage::GondosHouse,
        Stage::MallarasHouse,
        Stage::RupinsHouse,
        Stage::SparrotsHouse,
        Stage::SkyviewBoss,
        Stage::SkyviewSpring,
        Stage::SkyviewTemple,
        Stage::InsideThunderhead,
        Stage::IsleOfSongs,
        Stage::InsideVolcanoSummit,
        Stage::OutsideFireSanctuary,
        Stage::VolcanoSummitWaterfall,
    ];
}
impl Stage {
    pub fn name(&self) -> &'static str {
        match self {
            Stage::AncientCistern => "Ancient Cistern",
            Stage::AncientCisternBoss => "Ancient Cistern Boss",
            Stage::AncientCisternCandleRoom => "Ancient Cistern Candle Room",
            Stage::BatreauxHouse => "Batreaux' House",
            Stage::BeedlesShop => "Beedle's Shop",
            Stage::Bazaar => "Bazaar",
            Stage::ParrowAndOriellesHouse => "Parrow and Orielle's House",
            Stage::PeatricesHouse => "Peatrice's House",
            Stage::PipersHouse => "Piper's House",
            Stage::Skyloft => "Skyloft",
            Stage::WaterfallCave => "Waterfall Cave",
            Stage::WrynasHouse => "Wryna's House",
            Stage::EarthTemple => "Earth Temple",
            Stage::EarthTempleBoss => "Earth Temple Boss",
            Stage::EarthTempleSpring => "Earth Temple Spring",
            Stage::EldinSilentRealm => "Eldin Silent Realm",
            Stage::EldinVolcano => "Eldin Volcano",
            Stage::ThrillDiggerCave => "Thrill Digger Cave",
            Stage::FaronSilentRealm => "Faron Silent Realm",
            Stage::DeepWoods => "Deep Woods",
            Stage::FaronWoods => "Faron Woods",
            Stage::GreatTree => "Great Tree",
            Stage::FireSanctuaryA => "Fire Sanctuary A",
            Stage::FireSanctuaryB => "Fire Sanctuary B",
            Stage::FireSanctuaryBoss => "Fire Sanctuary Boss",
            Stage::FireSanctuaryFlameRoom => "Fire Sanctuary Flame Room",
            Stage::InsideGoddessStatue => "Inside Goddess Statue",
            Stage::KnightAcademy => "Knight Academy",
            Stage::SparringHall => "Sparring Hall",
            Stage::FaroresLair => "Farore's Lair",
            Stage::FloriaWaterfall => "Floria Waterfall",
            Stage::LakeFloria => "Lake Floria",
            Stage::LanayruCaves => "Lanayru Caves",
            Stage::FireNode => "Fire Node",
            Stage::LanayruDesert => "Lanayru Desert",
            Stage::LightningNode => "Lightning Node",
            Stage::TempleOfTime => "Temple of Time",
            Stage::LanayruMines => "Lanayru Mines",
            Stage::LanayruMiningFacilityA => "Lanayru Mining Facility A",
            Stage::LanayruMiningFacilityB => "Lanayru Mining Facility B",
            Stage::LanayruMiningFacilityBoss => "Lanayru Mining Facility Boss",
            Stage::LanayruMiningFacilityToToT => "Lanayru Mining Facility to ToT",
            Stage::InsidePiratesStronghold => "Inside Pirate's Stronghold",
            Stage::OutsidePiratesStronghold => "Outside Pirate's Stronghold",
            Stage::SandSea => "Sand Sea",
            Stage::SandSeaDocks => "Sand Sea Docks",
            Stage::Shipyard => "Shipyard",
            Stage::ShipyardConstructionBay => "Shipyard Construction Bay",
            Stage::SkippersRetreat => "Skipper's Retreat",
            Stage::SkippersShack => "Skipper's Shack",
            Stage::LanayruSilentRealm => "Lanayru Silent Realm",
            Stage::MogmaTurf => "Mogma Turf",
            Stage::Sandship => "Sandship",
            Stage::SandshipBoss => "Sandship Boss",
            Stage::BehindTheTemple => "Behind the Temple",
            Stage::HyliasTemple => "Hylia's Temple",
            Stage::SealedGroundsSpiral => "Sealed Grounds Spiral",
            Stage::SealedTemple => "Sealed Temple",
            Stage::InsideBambooIsland => "Inside Bamboo Island",
            Stage::LumpyPumpkin => "Lumpy Pumpkin",
            Stage::Sky => "Sky",
            Stage::SkyKeepEntry => "Sky Keep Entry",
            Stage::SkyloftSilentRealm => "Skyloft Silent Realm",
            Stage::BertiesHouse => "Bertie's House",
            Stage::GondosHouse => "Gondo's House",
            Stage::MallarasHouse => "Mallara's House",
            Stage::RupinsHouse => "Rupin's House",
            Stage::SparrotsHouse => "Sparrot's House",
            Stage::SkyviewBoss => "Skyview Boss",
            Stage::SkyviewSpring => "Skyview Spring",
            Stage::SkyviewTemple => "Skyview Temple",
            Stage::InsideThunderhead => "Inside Thunderhead",
            Stage::IsleOfSongs => "Isle of Songs",
            Stage::InsideVolcanoSummit => "Inside Volcano Summit",
            Stage::OutsideFireSanctuary => "Outside Fire Sanctuary",
            Stage::VolcanoSummitWaterfall => "Volcano Summit Waterfall",
        }
    }
}
impl Stage {
    pub fn region(&self) -> Region {
        match self {
            Stage::AncientCistern => Region::AncientCistern,
            Stage::AncientCisternBoss => Region::AncientCistern,
            Stage::AncientCisternCandleRoom => Region::AncientCistern,
            Stage::BatreauxHouse => Region::Batreaux,
            Stage::BeedlesShop => Region::Beedle,
            Stage::Bazaar => Region::CentralSkyloft,
            Stage::ParrowAndOriellesHouse => Region::CentralSkyloft,
            Stage::PeatricesHouse => Region::CentralSkyloft,
            Stage::PipersHouse => Region::CentralSkyloft,
            Stage::Skyloft => Region::CentralSkyloft,
            Stage::WaterfallCave => Region::CentralSkyloft,
            Stage::WrynasHouse => Region::CentralSkyloft,
            Stage::EarthTemple => Region::EarthTemple,
            Stage::EarthTempleBoss => Region::EarthTemple,
            Stage::EarthTempleSpring => Region::EarthTemple,
            Stage::EldinSilentRealm => Region::EldinSilentRealm,
            Stage::EldinVolcano => Region::EldinVolcano,
            Stage::ThrillDiggerCave => Region::EldinVolcano,
            Stage::FaronSilentRealm => Region::FaronSilentRealm,
            Stage::DeepWoods => Region::FaronWoods,
            Stage::FaronWoods => Region::FaronWoods,
            Stage::GreatTree => Region::FaronWoods,
            Stage::FireSanctuaryA => Region::FireSanctuary,
            Stage::FireSanctuaryB => Region::FireSanctuary,
            Stage::FireSanctuaryBoss => Region::FireSanctuary,
            Stage::FireSanctuaryFlameRoom => Region::FireSanctuary,
            Stage::InsideGoddessStatue => Region::KnightAcademy,
            Stage::KnightAcademy => Region::KnightAcademy,
            Stage::SparringHall => Region::KnightAcademy,
            Stage::FaroresLair => Region::LakeFloria,
            Stage::FloriaWaterfall => Region::LakeFloria,
            Stage::LakeFloria => Region::LakeFloria,
            Stage::LanayruCaves => Region::LanayruCaves,
            Stage::FireNode => Region::LanayruDesert,
            Stage::LanayruDesert => Region::LanayruDesert,
            Stage::LightningNode => Region::LanayruDesert,
            Stage::TempleOfTime => Region::LanayruDesert,
            Stage::LanayruMines => Region::LanayruMines,
            Stage::LanayruMiningFacilityA => Region::LanayruMiningFacility,
            Stage::LanayruMiningFacilityB => Region::LanayruMiningFacility,
            Stage::LanayruMiningFacilityBoss => Region::LanayruMiningFacility,
            Stage::LanayruMiningFacilityToToT => Region::LanayruMiningFacility,
            Stage::InsidePiratesStronghold => Region::LanayruSandSea,
            Stage::OutsidePiratesStronghold => Region::LanayruSandSea,
            Stage::SandSea => Region::LanayruSandSea,
            Stage::SandSeaDocks => Region::LanayruSandSea,
            Stage::Shipyard => Region::LanayruSandSea,
            Stage::ShipyardConstructionBay => Region::LanayruSandSea,
            Stage::SkippersRetreat => Region::LanayruSandSea,
            Stage::SkippersShack => Region::LanayruSandSea,
            Stage::LanayruSilentRealm => Region::LanayruSilentRealm,
            Stage::MogmaTurf => Region::MogmaTurf,
            Stage::Sandship => Region::Sandship,
            Stage::SandshipBoss => Region::Sandship,
            Stage::BehindTheTemple => Region::SealedGrounds,
            Stage::HyliasTemple => Region::SealedGrounds,
            Stage::SealedGroundsSpiral => Region::SealedGrounds,
            Stage::SealedTemple => Region::SealedGrounds,
            Stage::InsideBambooIsland => Region::Sky,
            Stage::LumpyPumpkin => Region::Sky,
            Stage::Sky => Region::Sky,
            Stage::SkyKeepEntry => Region::SkyKeep,
            Stage::SkyloftSilentRealm => Region::SkyloftSilentRealm,
            Stage::BertiesHouse => Region::SkyloftVillage,
            Stage::GondosHouse => Region::SkyloftVillage,
            Stage::MallarasHouse => Region::SkyloftVillage,
            Stage::RupinsHouse => Region::SkyloftVillage,
            Stage::SparrotsHouse => Region::SkyloftVillage,
            Stage::SkyviewBoss => Region::Skyview,
            Stage::SkyviewSpring => Region::Skyview,
            Stage::SkyviewTemple => Region::Skyview,
            Stage::InsideThunderhead => Region::Thunderhead,
            Stage::IsleOfSongs => Region::Thunderhead,
            Stage::InsideVolcanoSummit => Region::VolcanoSummit,
            Stage::OutsideFireSanctuary => Region::VolcanoSummit,
            Stage::VolcanoSummitWaterfall => Region::VolcanoSummit,
        }
    }
    pub fn areas(&self) -> &'static [Area] {
        match self {
            Stage::AncientCistern => &[
                Area::AncientCistern_AfterAcGutters,
                Area::AncientCistern_AfterWhipHooks,
                Area::AncientCistern_BeforeBokoKeyDoor,
                Area::AncientCistern_BeforeBossDoor,
                Area::AncientCistern_BehindWaterfall,
                Area::AncientCistern_BossKeyChestArea,
                Area::AncientCistern_MainBasement,
                Area::AncientCistern_MainHub,
                Area::AncientCistern_MainRoomVines,
                Area::AncientCistern_SpiderThread,
                Area::AncientCistern_WhipChestRoom,
            ],
            Stage::AncientCisternBoss => &[Area::AncientCisternBoss_Main],
            Stage::AncientCisternCandleRoom => &[Area::AncientCisternCandleRoom_Main],
            Stage::BatreauxHouse => &[Area::BatreauxHouse_Main],
            Stage::BeedlesShop => &[Area::BeedlesShop_Main],
            Stage::Bazaar => &[Area::Bazaar_Main],
            Stage::ParrowAndOriellesHouse => &[Area::ParrowAndOriellesHouse_Main],
            Stage::PeatricesHouse => &[Area::PeatricesHouse_Main],
            Stage::PipersHouse => &[Area::PipersHouse_Main],
            Stage::Skyloft => &[
                Area::Skyloft_CentralOutside,
                Area::Skyloft_PastWaterfallCave,
                Area::Skyloft_ToSkyKeep,
                Area::Skyloft_WaterfallCaveCrystals,
            ],
            Stage::WaterfallCave => &[Area::WaterfallCave_Main],
            Stage::WrynasHouse => &[Area::WrynasHouse_Main],
            Stage::EarthTemple => &[
                Area::EarthTemple_AfterBallRolling,
                Area::EarthTemple_BallRolling,
                Area::EarthTemple_BossDoorArea,
                Area::EarthTemple_Entrance,
            ],
            Stage::EarthTempleBoss => &[Area::EarthTempleBoss_Main],
            Stage::EarthTempleSpring => &[Area::EarthTempleSpring_Main],
            Stage::EldinSilentRealm => &[Area::EldinSilentRealm_Trial],
            Stage::EldinVolcano => &[
                Area::EldinVolcano_FirstRoom,
                Area::EldinVolcano_HotCaveArea,
                Area::EldinVolcano_NearThrillDigger,
                Area::EldinVolcano_OutsideEt,
                Area::EldinVolcano_PastMogmaTurf,
                Area::EldinVolcano_PastSlide,
                Area::EldinVolcano_PreMogmaTurf,
                Area::EldinVolcano_SandSlide,
                Area::EldinVolcano_VolcanoAscent,
            ],
            Stage::ThrillDiggerCave => &[Area::ThrillDiggerCave_Main],
            Stage::FaronSilentRealm => &[Area::FaronSilentRealm_Trial],
            Stage::DeepWoods => &[Area::DeepWoods_Entry, Area::DeepWoods_PastBeehive],
            Stage::FaronWoods => &[
                Area::FaronWoods_ClawshotTargetBranch,
                Area::FaronWoods_Entry,
                Area::FaronWoods_GreatTreePlatforms,
                Area::FaronWoods_GreatTreeTop,
                Area::FaronWoods_Main,
            ],
            Stage::GreatTree => &[
                Area::GreatTree_Entry,
                Area::GreatTree_Lower,
                Area::GreatTree_Middle,
                Area::GreatTree_PastPlatforms,
                Area::GreatTree_Upper,
            ],
            Stage::FireSanctuaryA => &[
                Area::FireSanctuaryA_Entry,
                Area::FireSanctuaryA_InFrontOfBossDoor,
                Area::FireSanctuaryA_PastFirstWaterPlant,
                Area::FireSanctuaryA_PrePlatsArea,
                Area::FireSanctuaryA_UpperStaircaseRoom,
            ],
            Stage::FireSanctuaryB => &[
                Area::FireSanctuaryB_AfterDoubleMagmanosFight,
                Area::FireSanctuaryB_FirstOutsideSection,
                Area::FireSanctuaryB_LastTrappedMogmaArea,
                Area::FireSanctuaryB_PastSecondRoomWithWaterFruit,
                Area::FireSanctuaryB_UnderDoubleMagmanosFight,
                Area::FireSanctuaryB_WaterFruitRoom,
            ],
            Stage::FireSanctuaryBoss => &[Area::FireSanctuaryBoss_Main],
            Stage::FireSanctuaryFlameRoom => &[Area::FireSanctuaryFlameRoom_Main],
            Stage::InsideGoddessStatue => &[Area::InsideGoddessStatue_Main],
            Stage::KnightAcademy => &[
                Area::KnightAcademy_AboveZeldasRoom,
                Area::KnightAcademy_Main,
            ],
            Stage::SparringHall => &[Area::SparringHall_Main],
            Stage::FaroresLair => &[Area::FaroresLair_Main],
            Stage::FloriaWaterfall => &[Area::FloriaWaterfall_Main],
            Stage::LakeFloria => &[
                Area::LakeFloria_Entry,
                Area::LakeFloria_StatueSpot,
                Area::LakeFloria_ToFaroresLair,
            ],
            Stage::LanayruCaves => &[Area::LanayruCaves_Main, Area::LanayruCaves_ToSandSea],
            Stage::FireNode => &[Area::FireNode_End, Area::FireNode_Main],
            Stage::LanayruDesert => &[
                Area::LanayruDesert_HookBeetleArea,
                Area::LanayruDesert_PastToT,
                Area::LanayruDesert_SandOasis,
            ],
            Stage::LightningNode => &[Area::LightningNode_Main],
            Stage::TempleOfTime => &[
                Area::TempleOfTime_AfterLmf,
                Area::TempleOfTime_End,
                Area::TempleOfTime_NearCube,
                Area::TempleOfTime_NearGossipStone,
                Area::TempleOfTime_Start,
            ],
            Stage::LanayruMines => &[
                Area::LanayruMines_FirstHalf,
                Area::LanayruMines_ToCaves,
                Area::LanayruMines_ToDesert,
            ],
            Stage::LanayruMiningFacilityA => &[
                Area::LanayruMiningFacilityA_Entry,
                Area::LanayruMiningFacilityA_FirstKeyLockedRoom,
                Area::LanayruMiningFacilityA_FirstWestRoom,
                Area::LanayruMiningFacilityA_GustBellowsRoom,
                Area::LanayruMiningFacilityA_MapRoom,
                Area::LanayruMiningFacilityA_SecondRoom,
            ],
            Stage::LanayruMiningFacilityB => &[
                Area::LanayruMiningFacilityB_AfterLmfBkRoom,
                Area::LanayruMiningFacilityB_HubRoom,
                Area::LanayruMiningFacilityB_InsideLmfBkRoom,
                Area::LanayruMiningFacilityB_NearBossDoor,
                Area::LanayruMiningFacilityB_NearFirstHubRoomChest,
                Area::LanayruMiningFacilityB_WestHub,
            ],
            Stage::LanayruMiningFacilityBoss => &[Area::LanayruMiningFacilityBoss_Main],
            Stage::LanayruMiningFacilityToToT => &[
                Area::LanayruMiningFacilityToToT_BossDoor,
                Area::LanayruMiningFacilityToToT_ToTExit,
            ],
            Stage::InsidePiratesStronghold => &[Area::InsidePiratesStronghold_Main],
            Stage::OutsidePiratesStronghold => &[
                Area::OutsidePiratesStronghold_InsideSharkhead,
                Area::OutsidePiratesStronghold_Main,
            ],
            Stage::SandSea => &[Area::SandSea_Main],
            Stage::SandSeaDocks => &[Area::SandSeaDocks_Main, Area::SandSeaDocks_ToCaves],
            Stage::Shipyard => &[Area::Shipyard_AfterMinecartRide, Area::Shipyard_Main],
            Stage::ShipyardConstructionBay => &[
                Area::ShipyardConstructionBay_Lower,
                Area::ShipyardConstructionBay_Upper,
            ],
            Stage::SkippersRetreat => &[
                Area::SkippersRetreat_NextToShack,
                Area::SkippersRetreat_PastDekuBaba,
                Area::SkippersRetreat_PastMoblin,
                Area::SkippersRetreat_Start,
            ],
            Stage::SkippersShack => &[Area::SkippersShack_Main],
            Stage::LanayruSilentRealm => &[Area::LanayruSilentRealm_Trial],
            Stage::MogmaTurf => &[Area::MogmaTurf_Main],
            Stage::Sandship => &[
                Area::Sandship_Deck,
                Area::Sandship_PastSpume,
                Area::Sandship_SandshipBrig,
            ],
            Stage::SandshipBoss => &[Area::SandshipBoss_Main],
            Stage::BehindTheTemple => &[Area::BehindTheTemple_Main],
            Stage::HyliasTemple => &[Area::HyliasTemple_Main],
            Stage::SealedGroundsSpiral => &[
                Area::SealedGroundsSpiral_Lower,
                Area::SealedGroundsSpiral_Upper,
            ],
            Stage::SealedTemple => &[Area::SealedTemple_Main],
            Stage::InsideBambooIsland => &[Area::InsideBambooIsland_Main],
            Stage::LumpyPumpkin => &[Area::LumpyPumpkin_Main],
            Stage::Sky => &[
                Area::Sky_BeedleIslandCage,
                Area::Sky_BeedlesSkyHome,
                Area::Sky_Field,
                Area::Sky_OutsideLumpyPumpkin,
            ],
            Stage::SkyKeepEntry => &[Area::SkyKeepEntry_Main],
            Stage::SkyloftSilentRealm => &[Area::SkyloftSilentRealm_Trial],
            Stage::BertiesHouse => &[Area::BertiesHouse_Main],
            Stage::GondosHouse => &[Area::GondosHouse_Main],
            Stage::MallarasHouse => &[Area::MallarasHouse_Main],
            Stage::RupinsHouse => &[Area::RupinsHouse_Main],
            Stage::SparrotsHouse => &[Area::SparrotsHouse_Main],
            Stage::SkyviewBoss => &[Area::SkyviewBoss_Main],
            Stage::SkyviewSpring => &[Area::SkyviewSpring_Main],
            Stage::SkyviewTemple => &[
                Area::SkyviewTemple_BossDoorArea,
                Area::SkyviewTemple_Entry,
                Area::SkyviewTemple_FirstHub,
                Area::SkyviewTemple_MainHub,
            ],
            Stage::InsideThunderhead => &[Area::InsideThunderhead_Main],
            Stage::IsleOfSongs => &[Area::IsleOfSongs_Main],
            Stage::InsideVolcanoSummit => &[Area::InsideVolcanoSummit_Main],
            Stage::OutsideFireSanctuary => &[
                Area::OutsideFireSanctuary_Middle,
                Area::OutsideFireSanctuary_ToFireSanctuary,
                Area::OutsideFireSanctuary_ToInsideSummit,
            ],
            Stage::VolcanoSummitWaterfall => &[Area::VolcanoSummitWaterfall_Main],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Area {
    AncientCistern_AfterAcGutters,
    AncientCistern_AfterWhipHooks,
    AncientCistern_BeforeBokoKeyDoor,
    AncientCistern_BeforeBossDoor,
    AncientCistern_BehindWaterfall,
    AncientCistern_BossKeyChestArea,
    AncientCistern_MainBasement,
    AncientCistern_MainHub,
    AncientCistern_MainRoomVines,
    AncientCistern_SpiderThread,
    AncientCistern_WhipChestRoom,
    AncientCisternBoss_Main,
    AncientCisternCandleRoom_Main,
    BatreauxHouse_Main,
    BeedlesShop_Main,
    Bazaar_Main,
    ParrowAndOriellesHouse_Main,
    PeatricesHouse_Main,
    PipersHouse_Main,
    Skyloft_CentralOutside,
    Skyloft_PastWaterfallCave,
    Skyloft_ToSkyKeep,
    Skyloft_WaterfallCaveCrystals,
    WaterfallCave_Main,
    WrynasHouse_Main,
    EarthTemple_AfterBallRolling,
    EarthTemple_BallRolling,
    EarthTemple_BossDoorArea,
    EarthTemple_Entrance,
    EarthTempleBoss_Main,
    EarthTempleSpring_Main,
    EldinSilentRealm_Trial,
    EldinVolcano_FirstRoom,
    EldinVolcano_HotCaveArea,
    EldinVolcano_NearThrillDigger,
    EldinVolcano_OutsideEt,
    EldinVolcano_PastMogmaTurf,
    EldinVolcano_PastSlide,
    EldinVolcano_PreMogmaTurf,
    EldinVolcano_SandSlide,
    EldinVolcano_VolcanoAscent,
    ThrillDiggerCave_Main,
    FaronSilentRealm_Trial,
    DeepWoods_Entry,
    DeepWoods_PastBeehive,
    FaronWoods_ClawshotTargetBranch,
    FaronWoods_Entry,
    FaronWoods_GreatTreePlatforms,
    FaronWoods_GreatTreeTop,
    FaronWoods_Main,
    GreatTree_Entry,
    GreatTree_Lower,
    GreatTree_Middle,
    GreatTree_PastPlatforms,
    GreatTree_Upper,
    FireSanctuaryA_Entry,
    FireSanctuaryA_InFrontOfBossDoor,
    FireSanctuaryA_PastFirstWaterPlant,
    FireSanctuaryA_PrePlatsArea,
    FireSanctuaryA_UpperStaircaseRoom,
    FireSanctuaryB_AfterDoubleMagmanosFight,
    FireSanctuaryB_FirstOutsideSection,
    FireSanctuaryB_LastTrappedMogmaArea,
    FireSanctuaryB_PastSecondRoomWithWaterFruit,
    FireSanctuaryB_UnderDoubleMagmanosFight,
    FireSanctuaryB_WaterFruitRoom,
    FireSanctuaryBoss_Main,
    FireSanctuaryFlameRoom_Main,
    InsideGoddessStatue_Main,
    KnightAcademy_AboveZeldasRoom,
    KnightAcademy_Main,
    Skyloft_OutsideGoddessStatue,
    SparringHall_Main,
    FaroresLair_Main,
    FloriaWaterfall_Main,
    LakeFloria_Entry,
    LakeFloria_StatueSpot,
    LakeFloria_ToFaroresLair,
    LanayruCaves_Main,
    LanayruCaves_ToSandSea,
    FireNode_End,
    FireNode_Main,
    LanayruDesert_HookBeetleArea,
    LanayruDesert_PastToT,
    LanayruDesert_SandOasis,
    LightningNode_Main,
    TempleOfTime_AfterLmf,
    TempleOfTime_End,
    TempleOfTime_NearCube,
    TempleOfTime_NearGossipStone,
    TempleOfTime_Start,
    LanayruMines_FirstHalf,
    LanayruMines_ToCaves,
    LanayruMines_ToDesert,
    LanayruMiningFacilityA_Entry,
    LanayruMiningFacilityA_FirstKeyLockedRoom,
    LanayruMiningFacilityA_FirstWestRoom,
    LanayruMiningFacilityA_GustBellowsRoom,
    LanayruMiningFacilityA_MapRoom,
    LanayruMiningFacilityA_SecondRoom,
    LanayruMiningFacilityB_AfterLmfBkRoom,
    LanayruMiningFacilityB_HubRoom,
    LanayruMiningFacilityB_InsideLmfBkRoom,
    LanayruMiningFacilityB_NearBossDoor,
    LanayruMiningFacilityB_NearFirstHubRoomChest,
    LanayruMiningFacilityB_WestHub,
    LanayruMiningFacilityBoss_Main,
    LanayruMiningFacilityToToT_BossDoor,
    LanayruMiningFacilityToToT_ToTExit,
    InsidePiratesStronghold_Main,
    OutsidePiratesStronghold_InsideSharkhead,
    OutsidePiratesStronghold_Main,
    SandSea_Main,
    SandSeaDocks_Main,
    SandSeaDocks_ToCaves,
    Shipyard_AfterMinecartRide,
    Shipyard_Main,
    ShipyardConstructionBay_Lower,
    ShipyardConstructionBay_Upper,
    SkippersRetreat_NextToShack,
    SkippersRetreat_PastDekuBaba,
    SkippersRetreat_PastMoblin,
    SkippersRetreat_Start,
    SkippersShack_Main,
    LanayruSilentRealm_Trial,
    MogmaTurf_Main,
    Sandship_Deck,
    Sandship_PastSpume,
    Sandship_SandshipBrig,
    SandshipBoss_Main,
    BehindTheTemple_Main,
    HyliasTemple_Main,
    SealedGroundsSpiral_Lower,
    SealedGroundsSpiral_Upper,
    SealedTemple_Main,
    InsideBambooIsland_Main,
    LumpyPumpkin_Main,
    Sky_BeedleIslandCage,
    Sky_BeedlesSkyHome,
    Sky_Field,
    Sky_OutsideLumpyPumpkin,
    SkyKeepEntry_Main,
    SkyloftSilentRealm_Trial,
    BertiesHouse_Main,
    GondosHouse_Main,
    MallarasHouse_Main,
    RupinsHouse_Main,
    Skyloft_OutsideSkyloftVillage,
    SparrotsHouse_Main,
    SkyviewBoss_Main,
    SkyviewSpring_Main,
    SkyviewTemple_BossDoorArea,
    SkyviewTemple_Entry,
    SkyviewTemple_FirstHub,
    SkyviewTemple_MainHub,
    InsideThunderhead_Main,
    IsleOfSongs_Main,
    InsideVolcanoSummit_Main,
    OutsideFireSanctuary_Middle,
    OutsideFireSanctuary_ToFireSanctuary,
    OutsideFireSanctuary_ToInsideSummit,
    VolcanoSummitWaterfall_Main,
}
impl Into<usize> for Area {
    fn into(self) -> usize {
        self as usize
    }
}
impl BitSetCompatible for Area {
    const ALL: &'static [Area] = &[
        Area::AncientCistern_AfterAcGutters,
        Area::AncientCistern_AfterWhipHooks,
        Area::AncientCistern_BeforeBokoKeyDoor,
        Area::AncientCistern_BeforeBossDoor,
        Area::AncientCistern_BehindWaterfall,
        Area::AncientCistern_BossKeyChestArea,
        Area::AncientCistern_MainBasement,
        Area::AncientCistern_MainHub,
        Area::AncientCistern_MainRoomVines,
        Area::AncientCistern_SpiderThread,
        Area::AncientCistern_WhipChestRoom,
        Area::AncientCisternBoss_Main,
        Area::AncientCisternCandleRoom_Main,
        Area::BatreauxHouse_Main,
        Area::BeedlesShop_Main,
        Area::Bazaar_Main,
        Area::ParrowAndOriellesHouse_Main,
        Area::PeatricesHouse_Main,
        Area::PipersHouse_Main,
        Area::Skyloft_CentralOutside,
        Area::Skyloft_PastWaterfallCave,
        Area::Skyloft_ToSkyKeep,
        Area::Skyloft_WaterfallCaveCrystals,
        Area::WaterfallCave_Main,
        Area::WrynasHouse_Main,
        Area::EarthTemple_AfterBallRolling,
        Area::EarthTemple_BallRolling,
        Area::EarthTemple_BossDoorArea,
        Area::EarthTemple_Entrance,
        Area::EarthTempleBoss_Main,
        Area::EarthTempleSpring_Main,
        Area::EldinSilentRealm_Trial,
        Area::EldinVolcano_FirstRoom,
        Area::EldinVolcano_HotCaveArea,
        Area::EldinVolcano_NearThrillDigger,
        Area::EldinVolcano_OutsideEt,
        Area::EldinVolcano_PastMogmaTurf,
        Area::EldinVolcano_PastSlide,
        Area::EldinVolcano_PreMogmaTurf,
        Area::EldinVolcano_SandSlide,
        Area::EldinVolcano_VolcanoAscent,
        Area::ThrillDiggerCave_Main,
        Area::FaronSilentRealm_Trial,
        Area::DeepWoods_Entry,
        Area::DeepWoods_PastBeehive,
        Area::FaronWoods_ClawshotTargetBranch,
        Area::FaronWoods_Entry,
        Area::FaronWoods_GreatTreePlatforms,
        Area::FaronWoods_GreatTreeTop,
        Area::FaronWoods_Main,
        Area::GreatTree_Entry,
        Area::GreatTree_Lower,
        Area::GreatTree_Middle,
        Area::GreatTree_PastPlatforms,
        Area::GreatTree_Upper,
        Area::FireSanctuaryA_Entry,
        Area::FireSanctuaryA_InFrontOfBossDoor,
        Area::FireSanctuaryA_PastFirstWaterPlant,
        Area::FireSanctuaryA_PrePlatsArea,
        Area::FireSanctuaryA_UpperStaircaseRoom,
        Area::FireSanctuaryB_AfterDoubleMagmanosFight,
        Area::FireSanctuaryB_FirstOutsideSection,
        Area::FireSanctuaryB_LastTrappedMogmaArea,
        Area::FireSanctuaryB_PastSecondRoomWithWaterFruit,
        Area::FireSanctuaryB_UnderDoubleMagmanosFight,
        Area::FireSanctuaryB_WaterFruitRoom,
        Area::FireSanctuaryBoss_Main,
        Area::FireSanctuaryFlameRoom_Main,
        Area::InsideGoddessStatue_Main,
        Area::KnightAcademy_AboveZeldasRoom,
        Area::KnightAcademy_Main,
        Area::Skyloft_OutsideGoddessStatue,
        Area::SparringHall_Main,
        Area::FaroresLair_Main,
        Area::FloriaWaterfall_Main,
        Area::LakeFloria_Entry,
        Area::LakeFloria_StatueSpot,
        Area::LakeFloria_ToFaroresLair,
        Area::LanayruCaves_Main,
        Area::LanayruCaves_ToSandSea,
        Area::FireNode_End,
        Area::FireNode_Main,
        Area::LanayruDesert_HookBeetleArea,
        Area::LanayruDesert_PastToT,
        Area::LanayruDesert_SandOasis,
        Area::LightningNode_Main,
        Area::TempleOfTime_AfterLmf,
        Area::TempleOfTime_End,
        Area::TempleOfTime_NearCube,
        Area::TempleOfTime_NearGossipStone,
        Area::TempleOfTime_Start,
        Area::LanayruMines_FirstHalf,
        Area::LanayruMines_ToCaves,
        Area::LanayruMines_ToDesert,
        Area::LanayruMiningFacilityA_Entry,
        Area::LanayruMiningFacilityA_FirstKeyLockedRoom,
        Area::LanayruMiningFacilityA_FirstWestRoom,
        Area::LanayruMiningFacilityA_GustBellowsRoom,
        Area::LanayruMiningFacilityA_MapRoom,
        Area::LanayruMiningFacilityA_SecondRoom,
        Area::LanayruMiningFacilityB_AfterLmfBkRoom,
        Area::LanayruMiningFacilityB_HubRoom,
        Area::LanayruMiningFacilityB_InsideLmfBkRoom,
        Area::LanayruMiningFacilityB_NearBossDoor,
        Area::LanayruMiningFacilityB_NearFirstHubRoomChest,
        Area::LanayruMiningFacilityB_WestHub,
        Area::LanayruMiningFacilityBoss_Main,
        Area::LanayruMiningFacilityToToT_BossDoor,
        Area::LanayruMiningFacilityToToT_ToTExit,
        Area::InsidePiratesStronghold_Main,
        Area::OutsidePiratesStronghold_InsideSharkhead,
        Area::OutsidePiratesStronghold_Main,
        Area::SandSea_Main,
        Area::SandSeaDocks_Main,
        Area::SandSeaDocks_ToCaves,
        Area::Shipyard_AfterMinecartRide,
        Area::Shipyard_Main,
        Area::ShipyardConstructionBay_Lower,
        Area::ShipyardConstructionBay_Upper,
        Area::SkippersRetreat_NextToShack,
        Area::SkippersRetreat_PastDekuBaba,
        Area::SkippersRetreat_PastMoblin,
        Area::SkippersRetreat_Start,
        Area::SkippersShack_Main,
        Area::LanayruSilentRealm_Trial,
        Area::MogmaTurf_Main,
        Area::Sandship_Deck,
        Area::Sandship_PastSpume,
        Area::Sandship_SandshipBrig,
        Area::SandshipBoss_Main,
        Area::BehindTheTemple_Main,
        Area::HyliasTemple_Main,
        Area::SealedGroundsSpiral_Lower,
        Area::SealedGroundsSpiral_Upper,
        Area::SealedTemple_Main,
        Area::InsideBambooIsland_Main,
        Area::LumpyPumpkin_Main,
        Area::Sky_BeedleIslandCage,
        Area::Sky_BeedlesSkyHome,
        Area::Sky_Field,
        Area::Sky_OutsideLumpyPumpkin,
        Area::SkyKeepEntry_Main,
        Area::SkyloftSilentRealm_Trial,
        Area::BertiesHouse_Main,
        Area::GondosHouse_Main,
        Area::MallarasHouse_Main,
        Area::RupinsHouse_Main,
        Area::Skyloft_OutsideSkyloftVillage,
        Area::SparrotsHouse_Main,
        Area::SkyviewBoss_Main,
        Area::SkyviewSpring_Main,
        Area::SkyviewTemple_BossDoorArea,
        Area::SkyviewTemple_Entry,
        Area::SkyviewTemple_FirstHub,
        Area::SkyviewTemple_MainHub,
        Area::InsideThunderhead_Main,
        Area::IsleOfSongs_Main,
        Area::InsideVolcanoSummit_Main,
        Area::OutsideFireSanctuary_Middle,
        Area::OutsideFireSanctuary_ToFireSanctuary,
        Area::OutsideFireSanctuary_ToInsideSummit,
        Area::VolcanoSummitWaterfall_Main,
    ];
}
impl Area {
    pub fn name(&self) -> &'static str {
        match self {
            Area::AncientCistern_AfterAcGutters => "After AC Gutters",
            Area::AncientCistern_AfterWhipHooks => "After Whip Hooks",
            Area::AncientCistern_BeforeBokoKeyDoor => "Before Boko Key Door",
            Area::AncientCistern_BeforeBossDoor => "Before Boss Door",
            Area::AncientCistern_BehindWaterfall => "Behind Waterfall",
            Area::AncientCistern_BossKeyChestArea => "Boss Key Chest Area",
            Area::AncientCistern_MainBasement => "Main Basement",
            Area::AncientCistern_MainHub => "Main Hub",
            Area::AncientCistern_MainRoomVines => "Main Room Vines",
            Area::AncientCistern_SpiderThread => "Spider Thread",
            Area::AncientCistern_WhipChestRoom => "Whip Chest Room",
            Area::AncientCisternBoss_Main => "Main",
            Area::AncientCisternCandleRoom_Main => "Main",
            Area::BatreauxHouse_Main => "Main",
            Area::BeedlesShop_Main => "Main",
            Area::Bazaar_Main => "Main",
            Area::ParrowAndOriellesHouse_Main => "Main",
            Area::PeatricesHouse_Main => "Main",
            Area::PipersHouse_Main => "Main",
            Area::Skyloft_CentralOutside => "Central Outside",
            Area::Skyloft_PastWaterfallCave => "Past Waterfall Cave",
            Area::Skyloft_ToSkyKeep => "To Sky Keep",
            Area::Skyloft_WaterfallCaveCrystals => "Waterfall Cave Crystals",
            Area::WaterfallCave_Main => "Main",
            Area::WrynasHouse_Main => "Main",
            Area::EarthTemple_AfterBallRolling => "After Ball Rolling",
            Area::EarthTemple_BallRolling => "Ball Rolling",
            Area::EarthTemple_BossDoorArea => "Boss Door Area",
            Area::EarthTemple_Entrance => "Entrance",
            Area::EarthTempleBoss_Main => "Main",
            Area::EarthTempleSpring_Main => "Main",
            Area::EldinSilentRealm_Trial => "Trial",
            Area::EldinVolcano_FirstRoom => "First Room",
            Area::EldinVolcano_HotCaveArea => "Hot Cave Area",
            Area::EldinVolcano_NearThrillDigger => "Near Thrill Digger",
            Area::EldinVolcano_OutsideEt => "Outside ET",
            Area::EldinVolcano_PastMogmaTurf => "Past Mogma Turf",
            Area::EldinVolcano_PastSlide => "Past Slide",
            Area::EldinVolcano_PreMogmaTurf => "Pre Mogma Turf",
            Area::EldinVolcano_SandSlide => "Sand Slide",
            Area::EldinVolcano_VolcanoAscent => "Volcano Ascent",
            Area::ThrillDiggerCave_Main => "Main",
            Area::FaronSilentRealm_Trial => "Trial",
            Area::DeepWoods_Entry => "Entry",
            Area::DeepWoods_PastBeehive => "Past Beehive",
            Area::FaronWoods_ClawshotTargetBranch => "Clawshot Target Branch",
            Area::FaronWoods_Entry => "Entry",
            Area::FaronWoods_GreatTreePlatforms => "Great Tree Platforms",
            Area::FaronWoods_GreatTreeTop => "Great Tree Top",
            Area::FaronWoods_Main => "Main",
            Area::GreatTree_Entry => "Entry",
            Area::GreatTree_Lower => "Lower",
            Area::GreatTree_Middle => "Middle",
            Area::GreatTree_PastPlatforms => "Past Platforms",
            Area::GreatTree_Upper => "Upper",
            Area::FireSanctuaryA_Entry => "Entry",
            Area::FireSanctuaryA_InFrontOfBossDoor => "In Front of Boss Door",
            Area::FireSanctuaryA_PastFirstWaterPlant => "Past First Water Plant",
            Area::FireSanctuaryA_PrePlatsArea => "Pre Plats Area",
            Area::FireSanctuaryA_UpperStaircaseRoom => "Upper Staircase Room",
            Area::FireSanctuaryB_AfterDoubleMagmanosFight => "After Double Magmanos Fight",
            Area::FireSanctuaryB_FirstOutsideSection => "First Outside Section",
            Area::FireSanctuaryB_LastTrappedMogmaArea => "Last Trapped Mogma Area",
            Area::FireSanctuaryB_PastSecondRoomWithWaterFruit => {
                "Past Second Room with Water Fruit"
            }
            Area::FireSanctuaryB_UnderDoubleMagmanosFight => "Under Double Magmanos Fight",
            Area::FireSanctuaryB_WaterFruitRoom => "Water Fruit Room",
            Area::FireSanctuaryBoss_Main => "Main",
            Area::FireSanctuaryFlameRoom_Main => "Main",
            Area::InsideGoddessStatue_Main => "Main",
            Area::KnightAcademy_AboveZeldasRoom => "Above Zelda's Room",
            Area::KnightAcademy_Main => "Main",
            Area::Skyloft_OutsideGoddessStatue => "Outside Goddess Statue",
            Area::SparringHall_Main => "Main",
            Area::FaroresLair_Main => "Main",
            Area::FloriaWaterfall_Main => "Main",
            Area::LakeFloria_Entry => "Entry",
            Area::LakeFloria_StatueSpot => "Statue Spot",
            Area::LakeFloria_ToFaroresLair => "To Farore's Lair",
            Area::LanayruCaves_Main => "Main",
            Area::LanayruCaves_ToSandSea => "To Sand Sea",
            Area::FireNode_End => "End",
            Area::FireNode_Main => "Main",
            Area::LanayruDesert_HookBeetleArea => "Hook Beetle Area",
            Area::LanayruDesert_PastToT => "Past ToT",
            Area::LanayruDesert_SandOasis => "Sand Oasis",
            Area::LightningNode_Main => "Main",
            Area::TempleOfTime_AfterLmf => "After LMF",
            Area::TempleOfTime_End => "End",
            Area::TempleOfTime_NearCube => "Near Cube",
            Area::TempleOfTime_NearGossipStone => "Near Gossip Stone",
            Area::TempleOfTime_Start => "Start",
            Area::LanayruMines_FirstHalf => "First Half",
            Area::LanayruMines_ToCaves => "To Caves",
            Area::LanayruMines_ToDesert => "To Desert",
            Area::LanayruMiningFacilityA_Entry => "Entry",
            Area::LanayruMiningFacilityA_FirstKeyLockedRoom => "First Key Locked Room",
            Area::LanayruMiningFacilityA_FirstWestRoom => "First West Room",
            Area::LanayruMiningFacilityA_GustBellowsRoom => "Gust Bellows Room",
            Area::LanayruMiningFacilityA_MapRoom => "Map Room",
            Area::LanayruMiningFacilityA_SecondRoom => "Second Room",
            Area::LanayruMiningFacilityB_AfterLmfBkRoom => "After LMF BK Room",
            Area::LanayruMiningFacilityB_HubRoom => "Hub Room",
            Area::LanayruMiningFacilityB_InsideLmfBkRoom => "Inside LMF BK Room",
            Area::LanayruMiningFacilityB_NearBossDoor => "Near Boss Door",
            Area::LanayruMiningFacilityB_NearFirstHubRoomChest => "Near First Hub Room Chest",
            Area::LanayruMiningFacilityB_WestHub => "West Hub",
            Area::LanayruMiningFacilityBoss_Main => "Main",
            Area::LanayruMiningFacilityToToT_BossDoor => "Boss Door",
            Area::LanayruMiningFacilityToToT_ToTExit => "ToT Exit",
            Area::InsidePiratesStronghold_Main => "Main",
            Area::OutsidePiratesStronghold_InsideSharkhead => "Inside Sharkhead",
            Area::OutsidePiratesStronghold_Main => "Main",
            Area::SandSea_Main => "Main",
            Area::SandSeaDocks_Main => "Main",
            Area::SandSeaDocks_ToCaves => "To Caves",
            Area::Shipyard_AfterMinecartRide => "After Minecart Ride",
            Area::Shipyard_Main => "Main",
            Area::ShipyardConstructionBay_Lower => "Lower",
            Area::ShipyardConstructionBay_Upper => "Upper",
            Area::SkippersRetreat_NextToShack => "Next to Shack",
            Area::SkippersRetreat_PastDekuBaba => "Past Deku Baba",
            Area::SkippersRetreat_PastMoblin => "Past Moblin",
            Area::SkippersRetreat_Start => "Start",
            Area::SkippersShack_Main => "Main",
            Area::LanayruSilentRealm_Trial => "Trial",
            Area::MogmaTurf_Main => "Main",
            Area::Sandship_Deck => "Deck",
            Area::Sandship_PastSpume => "Past Spume",
            Area::Sandship_SandshipBrig => "Sandship Brig",
            Area::SandshipBoss_Main => "Main",
            Area::BehindTheTemple_Main => "Main",
            Area::HyliasTemple_Main => "Main",
            Area::SealedGroundsSpiral_Lower => "Lower",
            Area::SealedGroundsSpiral_Upper => "Upper",
            Area::SealedTemple_Main => "Main",
            Area::InsideBambooIsland_Main => "Main",
            Area::LumpyPumpkin_Main => "Main",
            Area::Sky_BeedleIslandCage => "Beedle Island Cage",
            Area::Sky_BeedlesSkyHome => "Beedle's Sky Home",
            Area::Sky_Field => "Field",
            Area::Sky_OutsideLumpyPumpkin => "Outside Lumpy Pumpkin",
            Area::SkyKeepEntry_Main => "Main",
            Area::SkyloftSilentRealm_Trial => "Trial",
            Area::BertiesHouse_Main => "Main",
            Area::GondosHouse_Main => "Main",
            Area::MallarasHouse_Main => "Main",
            Area::RupinsHouse_Main => "Main",
            Area::Skyloft_OutsideSkyloftVillage => "Outside Skyloft Village",
            Area::SparrotsHouse_Main => "Main",
            Area::SkyviewBoss_Main => "Main",
            Area::SkyviewSpring_Main => "Main",
            Area::SkyviewTemple_BossDoorArea => "Boss Door Area",
            Area::SkyviewTemple_Entry => "Entry",
            Area::SkyviewTemple_FirstHub => "First Hub",
            Area::SkyviewTemple_MainHub => "Main Hub",
            Area::InsideThunderhead_Main => "Main",
            Area::IsleOfSongs_Main => "Main",
            Area::InsideVolcanoSummit_Main => "Main",
            Area::OutsideFireSanctuary_Middle => "Middle",
            Area::OutsideFireSanctuary_ToFireSanctuary => "To Fire Sanctuary",
            Area::OutsideFireSanctuary_ToInsideSummit => "To Inside Summit",
            Area::VolcanoSummitWaterfall_Main => "Main",
        }
    }
}
impl Area {
    pub fn stage(&self) -> Stage {
        match self {
            Area::AncientCistern_AfterAcGutters => Stage::AncientCistern,
            Area::AncientCistern_AfterWhipHooks => Stage::AncientCistern,
            Area::AncientCistern_BeforeBokoKeyDoor => Stage::AncientCistern,
            Area::AncientCistern_BeforeBossDoor => Stage::AncientCistern,
            Area::AncientCistern_BehindWaterfall => Stage::AncientCistern,
            Area::AncientCistern_BossKeyChestArea => Stage::AncientCistern,
            Area::AncientCistern_MainBasement => Stage::AncientCistern,
            Area::AncientCistern_MainHub => Stage::AncientCistern,
            Area::AncientCistern_MainRoomVines => Stage::AncientCistern,
            Area::AncientCistern_SpiderThread => Stage::AncientCistern,
            Area::AncientCistern_WhipChestRoom => Stage::AncientCistern,
            Area::AncientCisternBoss_Main => Stage::AncientCisternBoss,
            Area::AncientCisternCandleRoom_Main => Stage::AncientCisternCandleRoom,
            Area::BatreauxHouse_Main => Stage::BatreauxHouse,
            Area::BeedlesShop_Main => Stage::BeedlesShop,
            Area::Bazaar_Main => Stage::Bazaar,
            Area::ParrowAndOriellesHouse_Main => Stage::ParrowAndOriellesHouse,
            Area::PeatricesHouse_Main => Stage::PeatricesHouse,
            Area::PipersHouse_Main => Stage::PipersHouse,
            Area::Skyloft_CentralOutside => Stage::Skyloft,
            Area::Skyloft_PastWaterfallCave => Stage::Skyloft,
            Area::Skyloft_ToSkyKeep => Stage::Skyloft,
            Area::Skyloft_WaterfallCaveCrystals => Stage::Skyloft,
            Area::WaterfallCave_Main => Stage::WaterfallCave,
            Area::WrynasHouse_Main => Stage::WrynasHouse,
            Area::EarthTemple_AfterBallRolling => Stage::EarthTemple,
            Area::EarthTemple_BallRolling => Stage::EarthTemple,
            Area::EarthTemple_BossDoorArea => Stage::EarthTemple,
            Area::EarthTemple_Entrance => Stage::EarthTemple,
            Area::EarthTempleBoss_Main => Stage::EarthTempleBoss,
            Area::EarthTempleSpring_Main => Stage::EarthTempleSpring,
            Area::EldinSilentRealm_Trial => Stage::EldinSilentRealm,
            Area::EldinVolcano_FirstRoom => Stage::EldinVolcano,
            Area::EldinVolcano_HotCaveArea => Stage::EldinVolcano,
            Area::EldinVolcano_NearThrillDigger => Stage::EldinVolcano,
            Area::EldinVolcano_OutsideEt => Stage::EldinVolcano,
            Area::EldinVolcano_PastMogmaTurf => Stage::EldinVolcano,
            Area::EldinVolcano_PastSlide => Stage::EldinVolcano,
            Area::EldinVolcano_PreMogmaTurf => Stage::EldinVolcano,
            Area::EldinVolcano_SandSlide => Stage::EldinVolcano,
            Area::EldinVolcano_VolcanoAscent => Stage::EldinVolcano,
            Area::ThrillDiggerCave_Main => Stage::ThrillDiggerCave,
            Area::FaronSilentRealm_Trial => Stage::FaronSilentRealm,
            Area::DeepWoods_Entry => Stage::DeepWoods,
            Area::DeepWoods_PastBeehive => Stage::DeepWoods,
            Area::FaronWoods_ClawshotTargetBranch => Stage::FaronWoods,
            Area::FaronWoods_Entry => Stage::FaronWoods,
            Area::FaronWoods_GreatTreePlatforms => Stage::FaronWoods,
            Area::FaronWoods_GreatTreeTop => Stage::FaronWoods,
            Area::FaronWoods_Main => Stage::FaronWoods,
            Area::GreatTree_Entry => Stage::GreatTree,
            Area::GreatTree_Lower => Stage::GreatTree,
            Area::GreatTree_Middle => Stage::GreatTree,
            Area::GreatTree_PastPlatforms => Stage::GreatTree,
            Area::GreatTree_Upper => Stage::GreatTree,
            Area::FireSanctuaryA_Entry => Stage::FireSanctuaryA,
            Area::FireSanctuaryA_InFrontOfBossDoor => Stage::FireSanctuaryA,
            Area::FireSanctuaryA_PastFirstWaterPlant => Stage::FireSanctuaryA,
            Area::FireSanctuaryA_PrePlatsArea => Stage::FireSanctuaryA,
            Area::FireSanctuaryA_UpperStaircaseRoom => Stage::FireSanctuaryA,
            Area::FireSanctuaryB_AfterDoubleMagmanosFight => Stage::FireSanctuaryB,
            Area::FireSanctuaryB_FirstOutsideSection => Stage::FireSanctuaryB,
            Area::FireSanctuaryB_LastTrappedMogmaArea => Stage::FireSanctuaryB,
            Area::FireSanctuaryB_PastSecondRoomWithWaterFruit => Stage::FireSanctuaryB,
            Area::FireSanctuaryB_UnderDoubleMagmanosFight => Stage::FireSanctuaryB,
            Area::FireSanctuaryB_WaterFruitRoom => Stage::FireSanctuaryB,
            Area::FireSanctuaryBoss_Main => Stage::FireSanctuaryBoss,
            Area::FireSanctuaryFlameRoom_Main => Stage::FireSanctuaryFlameRoom,
            Area::InsideGoddessStatue_Main => Stage::InsideGoddessStatue,
            Area::KnightAcademy_AboveZeldasRoom => Stage::KnightAcademy,
            Area::KnightAcademy_Main => Stage::KnightAcademy,
            Area::Skyloft_OutsideGoddessStatue => Stage::Skyloft,
            Area::SparringHall_Main => Stage::SparringHall,
            Area::FaroresLair_Main => Stage::FaroresLair,
            Area::FloriaWaterfall_Main => Stage::FloriaWaterfall,
            Area::LakeFloria_Entry => Stage::LakeFloria,
            Area::LakeFloria_StatueSpot => Stage::LakeFloria,
            Area::LakeFloria_ToFaroresLair => Stage::LakeFloria,
            Area::LanayruCaves_Main => Stage::LanayruCaves,
            Area::LanayruCaves_ToSandSea => Stage::LanayruCaves,
            Area::FireNode_End => Stage::FireNode,
            Area::FireNode_Main => Stage::FireNode,
            Area::LanayruDesert_HookBeetleArea => Stage::LanayruDesert,
            Area::LanayruDesert_PastToT => Stage::LanayruDesert,
            Area::LanayruDesert_SandOasis => Stage::LanayruDesert,
            Area::LightningNode_Main => Stage::LightningNode,
            Area::TempleOfTime_AfterLmf => Stage::TempleOfTime,
            Area::TempleOfTime_End => Stage::TempleOfTime,
            Area::TempleOfTime_NearCube => Stage::TempleOfTime,
            Area::TempleOfTime_NearGossipStone => Stage::TempleOfTime,
            Area::TempleOfTime_Start => Stage::TempleOfTime,
            Area::LanayruMines_FirstHalf => Stage::LanayruMines,
            Area::LanayruMines_ToCaves => Stage::LanayruMines,
            Area::LanayruMines_ToDesert => Stage::LanayruMines,
            Area::LanayruMiningFacilityA_Entry => Stage::LanayruMiningFacilityA,
            Area::LanayruMiningFacilityA_FirstKeyLockedRoom => Stage::LanayruMiningFacilityA,
            Area::LanayruMiningFacilityA_FirstWestRoom => Stage::LanayruMiningFacilityA,
            Area::LanayruMiningFacilityA_GustBellowsRoom => Stage::LanayruMiningFacilityA,
            Area::LanayruMiningFacilityA_MapRoom => Stage::LanayruMiningFacilityA,
            Area::LanayruMiningFacilityA_SecondRoom => Stage::LanayruMiningFacilityA,
            Area::LanayruMiningFacilityB_AfterLmfBkRoom => Stage::LanayruMiningFacilityB,
            Area::LanayruMiningFacilityB_HubRoom => Stage::LanayruMiningFacilityB,
            Area::LanayruMiningFacilityB_InsideLmfBkRoom => Stage::LanayruMiningFacilityB,
            Area::LanayruMiningFacilityB_NearBossDoor => Stage::LanayruMiningFacilityB,
            Area::LanayruMiningFacilityB_NearFirstHubRoomChest => Stage::LanayruMiningFacilityB,
            Area::LanayruMiningFacilityB_WestHub => Stage::LanayruMiningFacilityB,
            Area::LanayruMiningFacilityBoss_Main => Stage::LanayruMiningFacilityBoss,
            Area::LanayruMiningFacilityToToT_BossDoor => Stage::LanayruMiningFacilityToToT,
            Area::LanayruMiningFacilityToToT_ToTExit => Stage::LanayruMiningFacilityToToT,
            Area::InsidePiratesStronghold_Main => Stage::InsidePiratesStronghold,
            Area::OutsidePiratesStronghold_InsideSharkhead => Stage::OutsidePiratesStronghold,
            Area::OutsidePiratesStronghold_Main => Stage::OutsidePiratesStronghold,
            Area::SandSea_Main => Stage::SandSea,
            Area::SandSeaDocks_Main => Stage::SandSeaDocks,
            Area::SandSeaDocks_ToCaves => Stage::SandSeaDocks,
            Area::Shipyard_AfterMinecartRide => Stage::Shipyard,
            Area::Shipyard_Main => Stage::Shipyard,
            Area::ShipyardConstructionBay_Lower => Stage::ShipyardConstructionBay,
            Area::ShipyardConstructionBay_Upper => Stage::ShipyardConstructionBay,
            Area::SkippersRetreat_NextToShack => Stage::SkippersRetreat,
            Area::SkippersRetreat_PastDekuBaba => Stage::SkippersRetreat,
            Area::SkippersRetreat_PastMoblin => Stage::SkippersRetreat,
            Area::SkippersRetreat_Start => Stage::SkippersRetreat,
            Area::SkippersShack_Main => Stage::SkippersShack,
            Area::LanayruSilentRealm_Trial => Stage::LanayruSilentRealm,
            Area::MogmaTurf_Main => Stage::MogmaTurf,
            Area::Sandship_Deck => Stage::Sandship,
            Area::Sandship_PastSpume => Stage::Sandship,
            Area::Sandship_SandshipBrig => Stage::Sandship,
            Area::SandshipBoss_Main => Stage::SandshipBoss,
            Area::BehindTheTemple_Main => Stage::BehindTheTemple,
            Area::HyliasTemple_Main => Stage::HyliasTemple,
            Area::SealedGroundsSpiral_Lower => Stage::SealedGroundsSpiral,
            Area::SealedGroundsSpiral_Upper => Stage::SealedGroundsSpiral,
            Area::SealedTemple_Main => Stage::SealedTemple,
            Area::InsideBambooIsland_Main => Stage::InsideBambooIsland,
            Area::LumpyPumpkin_Main => Stage::LumpyPumpkin,
            Area::Sky_BeedleIslandCage => Stage::Sky,
            Area::Sky_BeedlesSkyHome => Stage::Sky,
            Area::Sky_Field => Stage::Sky,
            Area::Sky_OutsideLumpyPumpkin => Stage::Sky,
            Area::SkyKeepEntry_Main => Stage::SkyKeepEntry,
            Area::SkyloftSilentRealm_Trial => Stage::SkyloftSilentRealm,
            Area::BertiesHouse_Main => Stage::BertiesHouse,
            Area::GondosHouse_Main => Stage::GondosHouse,
            Area::MallarasHouse_Main => Stage::MallarasHouse,
            Area::RupinsHouse_Main => Stage::RupinsHouse,
            Area::Skyloft_OutsideSkyloftVillage => Stage::Skyloft,
            Area::SparrotsHouse_Main => Stage::SparrotsHouse,
            Area::SkyviewBoss_Main => Stage::SkyviewBoss,
            Area::SkyviewSpring_Main => Stage::SkyviewSpring,
            Area::SkyviewTemple_BossDoorArea => Stage::SkyviewTemple,
            Area::SkyviewTemple_Entry => Stage::SkyviewTemple,
            Area::SkyviewTemple_FirstHub => Stage::SkyviewTemple,
            Area::SkyviewTemple_MainHub => Stage::SkyviewTemple,
            Area::InsideThunderhead_Main => Stage::InsideThunderhead,
            Area::IsleOfSongs_Main => Stage::IsleOfSongs,
            Area::InsideVolcanoSummit_Main => Stage::InsideVolcanoSummit,
            Area::OutsideFireSanctuary_Middle => Stage::OutsideFireSanctuary,
            Area::OutsideFireSanctuary_ToFireSanctuary => Stage::OutsideFireSanctuary,
            Area::OutsideFireSanctuary_ToInsideSummit => Stage::OutsideFireSanctuary,
            Area::VolcanoSummitWaterfall_Main => Stage::VolcanoSummitWaterfall,
        }
    }
    pub fn possible_tod(&self) -> TimeOfDay {
        match self {
            Area::AncientCistern_AfterAcGutters => TimeOfDay::Day,
            Area::AncientCistern_AfterWhipHooks => TimeOfDay::Day,
            Area::AncientCistern_BeforeBokoKeyDoor => TimeOfDay::Day,
            Area::AncientCistern_BeforeBossDoor => TimeOfDay::Day,
            Area::AncientCistern_BehindWaterfall => TimeOfDay::Day,
            Area::AncientCistern_BossKeyChestArea => TimeOfDay::Day,
            Area::AncientCistern_MainBasement => TimeOfDay::Day,
            Area::AncientCistern_MainHub => TimeOfDay::Day,
            Area::AncientCistern_MainRoomVines => TimeOfDay::Day,
            Area::AncientCistern_SpiderThread => TimeOfDay::Day,
            Area::AncientCistern_WhipChestRoom => TimeOfDay::Day,
            Area::AncientCisternBoss_Main => TimeOfDay::Day,
            Area::AncientCisternCandleRoom_Main => TimeOfDay::Day,
            Area::BatreauxHouse_Main => TimeOfDay::Both,
            Area::BeedlesShop_Main => TimeOfDay::Both,
            Area::Bazaar_Main => TimeOfDay::Day,
            Area::ParrowAndOriellesHouse_Main => TimeOfDay::Both,
            Area::PeatricesHouse_Main => TimeOfDay::Both,
            Area::PipersHouse_Main => TimeOfDay::Both,
            Area::Skyloft_CentralOutside => TimeOfDay::Both,
            Area::Skyloft_PastWaterfallCave => TimeOfDay::Both,
            Area::Skyloft_ToSkyKeep => TimeOfDay::Both,
            Area::Skyloft_WaterfallCaveCrystals => TimeOfDay::Both,
            Area::WaterfallCave_Main => TimeOfDay::Both,
            Area::WrynasHouse_Main => TimeOfDay::Both,
            Area::EarthTemple_AfterBallRolling => TimeOfDay::Day,
            Area::EarthTemple_BallRolling => TimeOfDay::Day,
            Area::EarthTemple_BossDoorArea => TimeOfDay::Day,
            Area::EarthTemple_Entrance => TimeOfDay::Day,
            Area::EarthTempleBoss_Main => TimeOfDay::Day,
            Area::EarthTempleSpring_Main => TimeOfDay::Day,
            Area::EldinSilentRealm_Trial => TimeOfDay::Both,
            Area::EldinVolcano_FirstRoom => TimeOfDay::Day,
            Area::EldinVolcano_HotCaveArea => TimeOfDay::Day,
            Area::EldinVolcano_NearThrillDigger => TimeOfDay::Day,
            Area::EldinVolcano_OutsideEt => TimeOfDay::Day,
            Area::EldinVolcano_PastMogmaTurf => TimeOfDay::Day,
            Area::EldinVolcano_PastSlide => TimeOfDay::Day,
            Area::EldinVolcano_PreMogmaTurf => TimeOfDay::Day,
            Area::EldinVolcano_SandSlide => TimeOfDay::Day,
            Area::EldinVolcano_VolcanoAscent => TimeOfDay::Day,
            Area::ThrillDiggerCave_Main => TimeOfDay::Day,
            Area::FaronSilentRealm_Trial => TimeOfDay::Both,
            Area::DeepWoods_Entry => TimeOfDay::Day,
            Area::DeepWoods_PastBeehive => TimeOfDay::Day,
            Area::FaronWoods_ClawshotTargetBranch => TimeOfDay::Day,
            Area::FaronWoods_Entry => TimeOfDay::Day,
            Area::FaronWoods_GreatTreePlatforms => TimeOfDay::Day,
            Area::FaronWoods_GreatTreeTop => TimeOfDay::Day,
            Area::FaronWoods_Main => TimeOfDay::Day,
            Area::GreatTree_Entry => TimeOfDay::Day,
            Area::GreatTree_Lower => TimeOfDay::Day,
            Area::GreatTree_Middle => TimeOfDay::Day,
            Area::GreatTree_PastPlatforms => TimeOfDay::Day,
            Area::GreatTree_Upper => TimeOfDay::Day,
            Area::FireSanctuaryA_Entry => TimeOfDay::Day,
            Area::FireSanctuaryA_InFrontOfBossDoor => TimeOfDay::Day,
            Area::FireSanctuaryA_PastFirstWaterPlant => TimeOfDay::Day,
            Area::FireSanctuaryA_PrePlatsArea => TimeOfDay::Day,
            Area::FireSanctuaryA_UpperStaircaseRoom => TimeOfDay::Day,
            Area::FireSanctuaryB_AfterDoubleMagmanosFight => TimeOfDay::Day,
            Area::FireSanctuaryB_FirstOutsideSection => TimeOfDay::Day,
            Area::FireSanctuaryB_LastTrappedMogmaArea => TimeOfDay::Day,
            Area::FireSanctuaryB_PastSecondRoomWithWaterFruit => TimeOfDay::Day,
            Area::FireSanctuaryB_UnderDoubleMagmanosFight => TimeOfDay::Day,
            Area::FireSanctuaryB_WaterFruitRoom => TimeOfDay::Day,
            Area::FireSanctuaryBoss_Main => TimeOfDay::Day,
            Area::FireSanctuaryFlameRoom_Main => TimeOfDay::Day,
            Area::InsideGoddessStatue_Main => TimeOfDay::Both,
            Area::KnightAcademy_AboveZeldasRoom => TimeOfDay::Both,
            Area::KnightAcademy_Main => TimeOfDay::Both,
            Area::Skyloft_OutsideGoddessStatue => TimeOfDay::Both,
            Area::SparringHall_Main => TimeOfDay::Both,
            Area::FaroresLair_Main => TimeOfDay::Day,
            Area::FloriaWaterfall_Main => TimeOfDay::Day,
            Area::LakeFloria_Entry => TimeOfDay::Day,
            Area::LakeFloria_StatueSpot => TimeOfDay::Day,
            Area::LakeFloria_ToFaroresLair => TimeOfDay::Day,
            Area::LanayruCaves_Main => TimeOfDay::Day,
            Area::LanayruCaves_ToSandSea => TimeOfDay::Day,
            Area::FireNode_End => TimeOfDay::Day,
            Area::FireNode_Main => TimeOfDay::Day,
            Area::LanayruDesert_HookBeetleArea => TimeOfDay::Day,
            Area::LanayruDesert_PastToT => TimeOfDay::Day,
            Area::LanayruDesert_SandOasis => TimeOfDay::Day,
            Area::LightningNode_Main => TimeOfDay::Day,
            Area::TempleOfTime_AfterLmf => TimeOfDay::Day,
            Area::TempleOfTime_End => TimeOfDay::Day,
            Area::TempleOfTime_NearCube => TimeOfDay::Day,
            Area::TempleOfTime_NearGossipStone => TimeOfDay::Day,
            Area::TempleOfTime_Start => TimeOfDay::Day,
            Area::LanayruMines_FirstHalf => TimeOfDay::Day,
            Area::LanayruMines_ToCaves => TimeOfDay::Day,
            Area::LanayruMines_ToDesert => TimeOfDay::Day,
            Area::LanayruMiningFacilityA_Entry => TimeOfDay::Day,
            Area::LanayruMiningFacilityA_FirstKeyLockedRoom => TimeOfDay::Day,
            Area::LanayruMiningFacilityA_FirstWestRoom => TimeOfDay::Day,
            Area::LanayruMiningFacilityA_GustBellowsRoom => TimeOfDay::Day,
            Area::LanayruMiningFacilityA_MapRoom => TimeOfDay::Day,
            Area::LanayruMiningFacilityA_SecondRoom => TimeOfDay::Day,
            Area::LanayruMiningFacilityB_AfterLmfBkRoom => TimeOfDay::Day,
            Area::LanayruMiningFacilityB_HubRoom => TimeOfDay::Day,
            Area::LanayruMiningFacilityB_InsideLmfBkRoom => TimeOfDay::Day,
            Area::LanayruMiningFacilityB_NearBossDoor => TimeOfDay::Day,
            Area::LanayruMiningFacilityB_NearFirstHubRoomChest => TimeOfDay::Day,
            Area::LanayruMiningFacilityB_WestHub => TimeOfDay::Day,
            Area::LanayruMiningFacilityBoss_Main => TimeOfDay::Day,
            Area::LanayruMiningFacilityToToT_BossDoor => TimeOfDay::Day,
            Area::LanayruMiningFacilityToToT_ToTExit => TimeOfDay::Day,
            Area::InsidePiratesStronghold_Main => TimeOfDay::Day,
            Area::OutsidePiratesStronghold_InsideSharkhead => TimeOfDay::Day,
            Area::OutsidePiratesStronghold_Main => TimeOfDay::Day,
            Area::SandSea_Main => TimeOfDay::Day,
            Area::SandSeaDocks_Main => TimeOfDay::Day,
            Area::SandSeaDocks_ToCaves => TimeOfDay::Day,
            Area::Shipyard_AfterMinecartRide => TimeOfDay::Day,
            Area::Shipyard_Main => TimeOfDay::Day,
            Area::ShipyardConstructionBay_Lower => TimeOfDay::Day,
            Area::ShipyardConstructionBay_Upper => TimeOfDay::Day,
            Area::SkippersRetreat_NextToShack => TimeOfDay::Day,
            Area::SkippersRetreat_PastDekuBaba => TimeOfDay::Day,
            Area::SkippersRetreat_PastMoblin => TimeOfDay::Day,
            Area::SkippersRetreat_Start => TimeOfDay::Day,
            Area::SkippersShack_Main => TimeOfDay::Day,
            Area::LanayruSilentRealm_Trial => TimeOfDay::Day,
            Area::MogmaTurf_Main => TimeOfDay::Day,
            Area::Sandship_Deck => TimeOfDay::Day,
            Area::Sandship_PastSpume => TimeOfDay::Day,
            Area::Sandship_SandshipBrig => TimeOfDay::Day,
            Area::SandshipBoss_Main => TimeOfDay::Day,
            Area::BehindTheTemple_Main => TimeOfDay::Day,
            Area::HyliasTemple_Main => TimeOfDay::Day,
            Area::SealedGroundsSpiral_Lower => TimeOfDay::Day,
            Area::SealedGroundsSpiral_Upper => TimeOfDay::Day,
            Area::SealedTemple_Main => TimeOfDay::Day,
            Area::InsideBambooIsland_Main => TimeOfDay::Day,
            Area::LumpyPumpkin_Main => TimeOfDay::Both,
            Area::Sky_BeedleIslandCage => TimeOfDay::Both,
            Area::Sky_BeedlesSkyHome => TimeOfDay::Night,
            Area::Sky_Field => TimeOfDay::Day,
            Area::Sky_OutsideLumpyPumpkin => TimeOfDay::Both,
            Area::SkyKeepEntry_Main => TimeOfDay::Day,
            Area::SkyloftSilentRealm_Trial => TimeOfDay::Day,
            Area::BertiesHouse_Main => TimeOfDay::Both,
            Area::GondosHouse_Main => TimeOfDay::Both,
            Area::MallarasHouse_Main => TimeOfDay::Both,
            Area::RupinsHouse_Main => TimeOfDay::Both,
            Area::Skyloft_OutsideSkyloftVillage => TimeOfDay::Both,
            Area::SparrotsHouse_Main => TimeOfDay::Both,
            Area::SkyviewBoss_Main => TimeOfDay::Day,
            Area::SkyviewSpring_Main => TimeOfDay::Day,
            Area::SkyviewTemple_BossDoorArea => TimeOfDay::Day,
            Area::SkyviewTemple_Entry => TimeOfDay::Day,
            Area::SkyviewTemple_FirstHub => TimeOfDay::Day,
            Area::SkyviewTemple_MainHub => TimeOfDay::Day,
            Area::InsideThunderhead_Main => TimeOfDay::Day,
            Area::IsleOfSongs_Main => TimeOfDay::Day,
            Area::InsideVolcanoSummit_Main => TimeOfDay::Day,
            Area::OutsideFireSanctuary_Middle => TimeOfDay::Day,
            Area::OutsideFireSanctuary_ToFireSanctuary => TimeOfDay::Day,
            Area::OutsideFireSanctuary_ToInsideSummit => TimeOfDay::Day,
            Area::VolcanoSummitWaterfall_Main => TimeOfDay::Day,
        }
    }
    pub fn can_sleep(&self) -> bool {
        match self {
            Area::AncientCistern_AfterAcGutters => false,
            Area::AncientCistern_AfterWhipHooks => false,
            Area::AncientCistern_BeforeBokoKeyDoor => false,
            Area::AncientCistern_BeforeBossDoor => false,
            Area::AncientCistern_BehindWaterfall => false,
            Area::AncientCistern_BossKeyChestArea => false,
            Area::AncientCistern_MainBasement => false,
            Area::AncientCistern_MainHub => false,
            Area::AncientCistern_MainRoomVines => false,
            Area::AncientCistern_SpiderThread => false,
            Area::AncientCistern_WhipChestRoom => false,
            Area::AncientCisternBoss_Main => false,
            Area::AncientCisternCandleRoom_Main => false,
            Area::BatreauxHouse_Main => false,
            Area::BeedlesShop_Main => true,
            Area::Bazaar_Main => false,
            Area::ParrowAndOriellesHouse_Main => true,
            Area::PeatricesHouse_Main => true,
            Area::PipersHouse_Main => true,
            Area::Skyloft_CentralOutside => false,
            Area::Skyloft_PastWaterfallCave => false,
            Area::Skyloft_ToSkyKeep => false,
            Area::Skyloft_WaterfallCaveCrystals => false,
            Area::WaterfallCave_Main => false,
            Area::WrynasHouse_Main => true,
            Area::EarthTemple_AfterBallRolling => false,
            Area::EarthTemple_BallRolling => false,
            Area::EarthTemple_BossDoorArea => false,
            Area::EarthTemple_Entrance => false,
            Area::EarthTempleBoss_Main => false,
            Area::EarthTempleSpring_Main => false,
            Area::EldinSilentRealm_Trial => false,
            Area::EldinVolcano_FirstRoom => false,
            Area::EldinVolcano_HotCaveArea => false,
            Area::EldinVolcano_NearThrillDigger => false,
            Area::EldinVolcano_OutsideEt => false,
            Area::EldinVolcano_PastMogmaTurf => false,
            Area::EldinVolcano_PastSlide => false,
            Area::EldinVolcano_PreMogmaTurf => false,
            Area::EldinVolcano_SandSlide => false,
            Area::EldinVolcano_VolcanoAscent => false,
            Area::ThrillDiggerCave_Main => false,
            Area::FaronSilentRealm_Trial => false,
            Area::DeepWoods_Entry => false,
            Area::DeepWoods_PastBeehive => false,
            Area::FaronWoods_ClawshotTargetBranch => false,
            Area::FaronWoods_Entry => false,
            Area::FaronWoods_GreatTreePlatforms => false,
            Area::FaronWoods_GreatTreeTop => false,
            Area::FaronWoods_Main => false,
            Area::GreatTree_Entry => false,
            Area::GreatTree_Lower => false,
            Area::GreatTree_Middle => false,
            Area::GreatTree_PastPlatforms => false,
            Area::GreatTree_Upper => false,
            Area::FireSanctuaryA_Entry => false,
            Area::FireSanctuaryA_InFrontOfBossDoor => false,
            Area::FireSanctuaryA_PastFirstWaterPlant => false,
            Area::FireSanctuaryA_PrePlatsArea => false,
            Area::FireSanctuaryA_UpperStaircaseRoom => false,
            Area::FireSanctuaryB_AfterDoubleMagmanosFight => false,
            Area::FireSanctuaryB_FirstOutsideSection => false,
            Area::FireSanctuaryB_LastTrappedMogmaArea => false,
            Area::FireSanctuaryB_PastSecondRoomWithWaterFruit => false,
            Area::FireSanctuaryB_UnderDoubleMagmanosFight => false,
            Area::FireSanctuaryB_WaterFruitRoom => false,
            Area::FireSanctuaryBoss_Main => false,
            Area::FireSanctuaryFlameRoom_Main => false,
            Area::InsideGoddessStatue_Main => false,
            Area::KnightAcademy_AboveZeldasRoom => false,
            Area::KnightAcademy_Main => true,
            Area::Skyloft_OutsideGoddessStatue => false,
            Area::SparringHall_Main => false,
            Area::FaroresLair_Main => false,
            Area::FloriaWaterfall_Main => false,
            Area::LakeFloria_Entry => false,
            Area::LakeFloria_StatueSpot => false,
            Area::LakeFloria_ToFaroresLair => false,
            Area::LanayruCaves_Main => false,
            Area::LanayruCaves_ToSandSea => false,
            Area::FireNode_End => false,
            Area::FireNode_Main => false,
            Area::LanayruDesert_HookBeetleArea => false,
            Area::LanayruDesert_PastToT => false,
            Area::LanayruDesert_SandOasis => false,
            Area::LightningNode_Main => false,
            Area::TempleOfTime_AfterLmf => false,
            Area::TempleOfTime_End => false,
            Area::TempleOfTime_NearCube => false,
            Area::TempleOfTime_NearGossipStone => false,
            Area::TempleOfTime_Start => false,
            Area::LanayruMines_FirstHalf => false,
            Area::LanayruMines_ToCaves => false,
            Area::LanayruMines_ToDesert => false,
            Area::LanayruMiningFacilityA_Entry => false,
            Area::LanayruMiningFacilityA_FirstKeyLockedRoom => false,
            Area::LanayruMiningFacilityA_FirstWestRoom => false,
            Area::LanayruMiningFacilityA_GustBellowsRoom => false,
            Area::LanayruMiningFacilityA_MapRoom => false,
            Area::LanayruMiningFacilityA_SecondRoom => false,
            Area::LanayruMiningFacilityB_AfterLmfBkRoom => false,
            Area::LanayruMiningFacilityB_HubRoom => false,
            Area::LanayruMiningFacilityB_InsideLmfBkRoom => false,
            Area::LanayruMiningFacilityB_NearBossDoor => false,
            Area::LanayruMiningFacilityB_NearFirstHubRoomChest => false,
            Area::LanayruMiningFacilityB_WestHub => false,
            Area::LanayruMiningFacilityBoss_Main => false,
            Area::LanayruMiningFacilityToToT_BossDoor => false,
            Area::LanayruMiningFacilityToToT_ToTExit => false,
            Area::InsidePiratesStronghold_Main => false,
            Area::OutsidePiratesStronghold_InsideSharkhead => false,
            Area::OutsidePiratesStronghold_Main => false,
            Area::SandSea_Main => false,
            Area::SandSeaDocks_Main => false,
            Area::SandSeaDocks_ToCaves => false,
            Area::Shipyard_AfterMinecartRide => false,
            Area::Shipyard_Main => false,
            Area::ShipyardConstructionBay_Lower => false,
            Area::ShipyardConstructionBay_Upper => false,
            Area::SkippersRetreat_NextToShack => false,
            Area::SkippersRetreat_PastDekuBaba => false,
            Area::SkippersRetreat_PastMoblin => false,
            Area::SkippersRetreat_Start => false,
            Area::SkippersShack_Main => false,
            Area::LanayruSilentRealm_Trial => false,
            Area::MogmaTurf_Main => false,
            Area::Sandship_Deck => false,
            Area::Sandship_PastSpume => false,
            Area::Sandship_SandshipBrig => false,
            Area::SandshipBoss_Main => false,
            Area::BehindTheTemple_Main => false,
            Area::HyliasTemple_Main => false,
            Area::SealedGroundsSpiral_Lower => false,
            Area::SealedGroundsSpiral_Upper => false,
            Area::SealedTemple_Main => false,
            Area::InsideBambooIsland_Main => false,
            Area::LumpyPumpkin_Main => true,
            Area::Sky_BeedleIslandCage => false,
            Area::Sky_BeedlesSkyHome => false,
            Area::Sky_Field => false,
            Area::Sky_OutsideLumpyPumpkin => false,
            Area::SkyKeepEntry_Main => false,
            Area::SkyloftSilentRealm_Trial => false,
            Area::BertiesHouse_Main => true,
            Area::GondosHouse_Main => true,
            Area::MallarasHouse_Main => true,
            Area::RupinsHouse_Main => true,
            Area::Skyloft_OutsideSkyloftVillage => false,
            Area::SparrotsHouse_Main => true,
            Area::SkyviewBoss_Main => false,
            Area::SkyviewSpring_Main => false,
            Area::SkyviewTemple_BossDoorArea => false,
            Area::SkyviewTemple_Entry => false,
            Area::SkyviewTemple_FirstHub => false,
            Area::SkyviewTemple_MainHub => false,
            Area::InsideThunderhead_Main => false,
            Area::IsleOfSongs_Main => false,
            Area::InsideVolcanoSummit_Main => false,
            Area::OutsideFireSanctuary_Middle => false,
            Area::OutsideFireSanctuary_ToFireSanctuary => false,
            Area::OutsideFireSanctuary_ToInsideSummit => false,
            Area::VolcanoSummitWaterfall_Main => false,
        }
    }
    pub fn exits(&self) -> &'static [Exit] {
        match self {
            Area::AncientCistern_AfterAcGutters => &[],
            Area::AncientCistern_AfterWhipHooks => &[],
            Area::AncientCistern_BeforeBokoKeyDoor => &[],
            Area::AncientCistern_BeforeBossDoor => &[Exit::AncientCistern_To_AncientCisternBoss],
            Area::AncientCistern_BehindWaterfall => &[],
            Area::AncientCistern_BossKeyChestArea => &[],
            Area::AncientCistern_MainBasement => &[],
            Area::AncientCistern_MainHub => &[Exit::AncientCistern_To_FloriaWaterfall],
            Area::AncientCistern_MainRoomVines => &[],
            Area::AncientCistern_SpiderThread => &[],
            Area::AncientCistern_WhipChestRoom => &[],
            Area::AncientCisternBoss_Main => {
                &[Exit::AncientCisternBoss_To_AncientCisternCandleRoom]
            }
            Area::AncientCisternCandleRoom_Main => &[],
            Area::BatreauxHouse_Main => &[Exit::BatreauxHouse_To_Skyloft],
            Area::BeedlesShop_Main => &[
                Exit::BeedlesShop_To_Sky_Night,
                Exit::BeedlesShop_To_Skyloft_Day,
            ],
            Area::Bazaar_Main => &[
                Exit::Bazaar_To_Skyloft_North,
                Exit::Bazaar_To_Skyloft_South,
                Exit::Bazaar_To_Skyloft_West,
            ],
            Area::ParrowAndOriellesHouse_Main => &[Exit::ParrowAndOriellesHouse_To_Skyloft],
            Area::PeatricesHouse_Main => &[Exit::PeatricesHouse_To_Skyloft],
            Area::PipersHouse_Main => &[Exit::PipersHouse_To_Skyloft],
            Area::Skyloft_CentralOutside => &[
                Exit::Skyloft_To_Bazaar_North,
                Exit::Skyloft_To_Bazaar_South,
                Exit::Skyloft_To_Bazaar_West,
                Exit::Skyloft_To_BeedlesShop_Day,
                Exit::Skyloft_To_ParrowAndOriellesHouse,
                Exit::Skyloft_To_PeatricesHouse,
                Exit::Skyloft_To_PipersHouse,
                Exit::Skyloft_To_Sky,
                Exit::Skyloft_To_SkyloftSilentRealm,
                Exit::Skyloft_To_WaterfallCave_Upper,
                Exit::Skyloft_To_WrynasHouse,
            ],
            Area::Skyloft_PastWaterfallCave => &[
                Exit::Skyloft_To_Sky_PastWaterfallCave,
                Exit::Skyloft_To_WaterfallCave_Lower,
            ],
            Area::Skyloft_ToSkyKeep => &[Exit::Skyloft_To_SkyKeepEntry],
            Area::Skyloft_WaterfallCaveCrystals => &[],
            Area::WaterfallCave_Main => &[
                Exit::WaterfallCave_To_Skyloft_Upper,
                Exit::WaterfallCave_To_Skyloft_Lower,
            ],
            Area::WrynasHouse_Main => &[Exit::WrynasHouse_To_Skyloft],
            Area::EarthTemple_AfterBallRolling => &[],
            Area::EarthTemple_BallRolling => &[],
            Area::EarthTemple_BossDoorArea => &[Exit::EarthTemple_To_EarthTempleBoss],
            Area::EarthTemple_Entrance => &[Exit::EarthTemple_To_EldinVolcano],
            Area::EarthTempleBoss_Main => &[Exit::EarthTempleBoss_To_EarthTempleSpring],
            Area::EarthTempleSpring_Main => &[Exit::EarthTempleSpring_To_EldinVolcano],
            Area::EldinSilentRealm_Trial => &[Exit::EldinSilentRealm_To_EldinVolcano],
            Area::EldinVolcano_FirstRoom => &[Exit::EldinVolcano_To_Sky_EldinEntranceStatue],
            Area::EldinVolcano_HotCaveArea => &[Exit::EldinVolcano_To_InsideVolcanoSummit],
            Area::EldinVolcano_NearThrillDigger => &[Exit::EldinVolcano_To_ThrillDiggerCave],
            Area::EldinVolcano_OutsideEt => &[
                Exit::EldinVolcano_To_EarthTemple,
                Exit::EldinVolcano_To_Sky_TempleEntranceStatue,
            ],
            Area::EldinVolcano_PastMogmaTurf => &[],
            Area::EldinVolcano_PastSlide => &[],
            Area::EldinVolcano_PreMogmaTurf => &[
                Exit::EldinVolcano_To_MogmaTurf_Skydive,
                Exit::EldinVolcano_To_Sky_VolcanoEastStatue,
            ],
            Area::EldinVolcano_SandSlide => &[],
            Area::EldinVolcano_VolcanoAscent => &[
                Exit::EldinVolcano_To_EldinSilentRealm,
                Exit::EldinVolcano_To_Sky_VolcanoAscentStatue,
            ],
            Area::ThrillDiggerCave_Main => &[Exit::ThrillDiggerCave_To_EldinVolcano],
            Area::FaronSilentRealm_Trial => &[Exit::FaronSilentRealm_To_FaronWoods],
            Area::DeepWoods_Entry => &[Exit::DeepWoods_To_FaronWoods],
            Area::DeepWoods_PastBeehive => &[
                Exit::DeepWoods_To_Sky_DeepWoodsStatue,
                Exit::DeepWoods_To_Sky_ForestTempleStatue,
                Exit::DeepWoods_To_SkyviewTemple,
            ],
            Area::FaronWoods_ClawshotTargetBranch => &[],
            Area::FaronWoods_Entry => &[
                Exit::FaronWoods_To_BehindTheTemple,
                Exit::FaronWoods_To_Sky_FaronWoodsEntryStatue,
            ],
            Area::FaronWoods_GreatTreePlatforms => &[
                Exit::FaronWoods_To_GreatTree_LowerPlatform,
                Exit::FaronWoods_To_GreatTree_UpperPlatform,
            ],
            Area::FaronWoods_GreatTreeTop => &[
                Exit::FaronWoods_To_GreatTree_Top,
                Exit::FaronWoods_To_Sky_GreatTreeStatue,
            ],
            Area::FaronWoods_Main => &[
                Exit::FaronWoods_To_DeepWoods,
                Exit::FaronWoods_To_FaronSilentRealm,
                Exit::FaronWoods_To_GreatTree_Tunnel,
                Exit::FaronWoods_To_LakeFloria,
                Exit::FaronWoods_To_Sky_InTheWoodsStatue,
                Exit::FaronWoods_To_Sky_ViewingPlatformStatue,
            ],
            Area::GreatTree_Entry => &[Exit::GreatTree_To_FaronWoods_Tunnel],
            Area::GreatTree_Lower => &[],
            Area::GreatTree_Middle => &[],
            Area::GreatTree_PastPlatforms => &[Exit::GreatTree_To_FaronWoods_LowerPlatform],
            Area::GreatTree_Upper => &[
                Exit::GreatTree_To_FaronWoods_UpperPlatform,
                Exit::GreatTree_To_FaronWoods_Top,
            ],
            Area::FireSanctuaryA_Entry => &[Exit::FireSanctuaryA_To_OutsideFireSanctuary],
            Area::FireSanctuaryA_InFrontOfBossDoor => &[Exit::FireSanctuaryA_To_FireSanctuaryBoss],
            Area::FireSanctuaryA_PastFirstWaterPlant => &[Exit::FireSanctuaryA_To_FireSanctuaryB],
            Area::FireSanctuaryA_PrePlatsArea => &[],
            Area::FireSanctuaryA_UpperStaircaseRoom => &[],
            Area::FireSanctuaryB_AfterDoubleMagmanosFight => &[],
            Area::FireSanctuaryB_FirstOutsideSection => &[],
            Area::FireSanctuaryB_LastTrappedMogmaArea => &[],
            Area::FireSanctuaryB_PastSecondRoomWithWaterFruit => &[],
            Area::FireSanctuaryB_UnderDoubleMagmanosFight => {
                &[Exit::FireSanctuaryB_To_FireSanctuaryA]
            }
            Area::FireSanctuaryB_WaterFruitRoom => &[],
            Area::FireSanctuaryBoss_Main => &[Exit::FireSanctuaryBoss_To_FireSanctuaryFlameRoom],
            Area::FireSanctuaryFlameRoom_Main => &[],
            Area::InsideGoddessStatue_Main => &[Exit::InsideGoddessStatue_To_Skyloft],
            Area::KnightAcademy_AboveZeldasRoom => &[],
            Area::KnightAcademy_Main => &[
                Exit::KnightAcademy_To_Skyloft_LowerDoors,
                Exit::KnightAcademy_To_Skyloft_UpperDoors,
            ],
            Area::Skyloft_OutsideGoddessStatue => &[
                Exit::Skyloft_To_InsideGoddessStatue,
                Exit::Skyloft_To_KnightAcademy_Chimney,
                Exit::Skyloft_To_KnightAcademy_LowerDoors,
                Exit::Skyloft_To_KnightAcademy_UpperDoors,
                Exit::Skyloft_To_SparringHall,
            ],
            Area::SparringHall_Main => &[Exit::SparringHall_To_Skyloft],
            Area::FaroresLair_Main => &[
                Exit::FaroresLair_To_FloriaWaterfall,
                Exit::FaroresLair_To_LakeFloria,
            ],
            Area::FloriaWaterfall_Main => &[
                Exit::FloriaWaterfall_To_AncientCistern,
                Exit::FloriaWaterfall_To_FaronWoods,
                Exit::FloriaWaterfall_To_FaroresLair,
                Exit::FloriaWaterfall_To_Sky_FloriaWaterfallStatue,
            ],
            Area::LakeFloria_Entry => &[],
            Area::LakeFloria_StatueSpot => &[Exit::LakeFloria_To_Sky_LakeFloriaStatue],
            Area::LakeFloria_ToFaroresLair => &[Exit::LakeFloria_To_FaroresLair],
            Area::LanayruCaves_Main => &[
                Exit::LanayruCaves_To_LanayruDesert,
                Exit::LanayruCaves_To_LanayruMines,
            ],
            Area::LanayruCaves_ToSandSea => &[Exit::LanayruCaves_To_SandSeaDocks],
            Area::FireNode_End => &[],
            Area::FireNode_Main => &[Exit::FireNode_To_LanayruDesert],
            Area::LanayruDesert_HookBeetleArea => &[
                Exit::LanayruDesert_To_LanayruMines,
                Exit::LanayruDesert_To_Sky_DesertEntranceStatue,
            ],
            Area::LanayruDesert_PastToT => &[
                Exit::LanayruDesert_To_FireNode,
                Exit::LanayruDesert_To_LanayruMiningFacilityA,
                Exit::LanayruDesert_To_LanayruSilentRealm,
                Exit::LanayruDesert_To_LightningNode,
                Exit::LanayruDesert_To_Sky_NorthDesertStatue,
                Exit::LanayruDesert_To_Sky_StoneCacheStatue,
                Exit::LanayruDesert_To_TempleOfTime_End,
            ],
            Area::LanayruDesert_SandOasis => &[
                Exit::LanayruDesert_To_LanayruCaves,
                Exit::LanayruDesert_To_Sky_WestDesertStatue,
                Exit::LanayruDesert_To_TempleOfTime_Start,
            ],
            Area::LightningNode_Main => &[Exit::LightningNode_To_LanayruDesert],
            Area::TempleOfTime_AfterLmf => &[],
            Area::TempleOfTime_End => &[Exit::TempleOfTime_To_LanayruDesert_End],
            Area::TempleOfTime_NearCube => &[],
            Area::TempleOfTime_NearGossipStone => &[],
            Area::TempleOfTime_Start => &[Exit::TempleOfTime_To_LanayruDesert_Start],
            Area::LanayruMines_FirstHalf => &[Exit::LanayruMines_To_Sky_LanayruMineEntryStatue],
            Area::LanayruMines_ToCaves => &[Exit::LanayruMines_To_LanayruCaves],
            Area::LanayruMines_ToDesert => &[Exit::LanayruMines_To_LanayruDesert],
            Area::LanayruMiningFacilityA_Entry => &[Exit::LanayruMiningFacilityA_To_LanayruDesert],
            Area::LanayruMiningFacilityA_FirstKeyLockedRoom => &[],
            Area::LanayruMiningFacilityA_FirstWestRoom => &[],
            Area::LanayruMiningFacilityA_GustBellowsRoom => {
                &[Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub2]
            }
            Area::LanayruMiningFacilityA_MapRoom => {
                &[Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_HubW]
            }
            Area::LanayruMiningFacilityA_SecondRoom => {
                &[Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub]
            }
            Area::LanayruMiningFacilityB_AfterLmfBkRoom => &[],
            Area::LanayruMiningFacilityB_HubRoom => &[],
            Area::LanayruMiningFacilityB_InsideLmfBkRoom => &[],
            Area::LanayruMiningFacilityB_NearBossDoor => {
                &[Exit::LanayruMiningFacilityB_To_LanayruMiningFacilityBoss]
            }
            Area::LanayruMiningFacilityB_NearFirstHubRoomChest => &[],
            Area::LanayruMiningFacilityB_WestHub => &[],
            Area::LanayruMiningFacilityBoss_Main => {
                &[Exit::LanayruMiningFacilityBoss_To_LanayruMiningFacilityToToT]
            }
            Area::LanayruMiningFacilityToToT_BossDoor => &[],
            Area::LanayruMiningFacilityToToT_ToTExit => {
                &[Exit::LanayruMiningFacilityToToT_To_TempleOfTime]
            }
            Area::InsidePiratesStronghold_Main => &[
                Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_End,
                Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_Beginning,
            ],
            Area::OutsidePiratesStronghold_InsideSharkhead => {
                &[Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_End]
            }
            Area::OutsidePiratesStronghold_Main => &[
                Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_Beginning,
                Exit::OutsidePiratesStronghold_To_SandSea,
            ],
            Area::SandSea_Main => &[
                Exit::SandSea_To_OutsidePiratesStronghold,
                Exit::SandSea_To_SandSeaDocks,
                Exit::SandSea_To_Sandship,
                Exit::SandSea_To_Shipyard,
                Exit::SandSea_To_SkippersRetreat,
            ],
            Area::SandSeaDocks_Main => &[
                Exit::SandSeaDocks_To_SandSea,
                Exit::SandSeaDocks_To_Sky_AncientHarbor,
            ],
            Area::SandSeaDocks_ToCaves => &[Exit::SandSeaDocks_To_LanayruCaves],
            Area::Shipyard_AfterMinecartRide => &[Exit::Shipyard_To_ShipyardConstructionBay_Upper],
            Area::Shipyard_Main => &[
                Exit::Shipyard_To_SandSea,
                Exit::Shipyard_To_ShipyardConstructionBay_Lower,
            ],
            Area::ShipyardConstructionBay_Lower => {
                &[Exit::ShipyardConstructionBay_To_Shipyard_Lower]
            }
            Area::ShipyardConstructionBay_Upper => {
                &[Exit::ShipyardConstructionBay_To_Shipyard_Upper]
            }
            Area::SkippersRetreat_NextToShack => &[Exit::SkippersRetreat_To_SkippersShack],
            Area::SkippersRetreat_PastDekuBaba => &[],
            Area::SkippersRetreat_PastMoblin => &[],
            Area::SkippersRetreat_Start => &[Exit::SkippersRetreat_To_SandSea],
            Area::SkippersShack_Main => &[Exit::SkippersShack_To_SkippersRetreat],
            Area::LanayruSilentRealm_Trial => &[Exit::LanayruSilentRealm_To_LanayruDesert],
            Area::MogmaTurf_Main => &[
                Exit::MogmaTurf_To_EldinVolcano_EndVent,
                Exit::MogmaTurf_To_EldinVolcano_StartVent,
            ],
            Area::Sandship_Deck => &[Exit::Sandship_To_SandSea],
            Area::Sandship_PastSpume => &[Exit::Sandship_To_SandshipBoss],
            Area::Sandship_SandshipBrig => &[],
            Area::SandshipBoss_Main => &[],
            Area::BehindTheTemple_Main => &[
                Exit::BehindTheTemple_To_FaronWoods,
                Exit::BehindTheTemple_To_SealedGroundsSpiral,
                Exit::BehindTheTemple_To_SealedTemple,
                Exit::BehindTheTemple_To_Sky_BehindTheTempleStatue,
            ],
            Area::HyliasTemple_Main => &[],
            Area::SealedGroundsSpiral_Lower => &[Exit::SealedGroundsSpiral_To_SealedTemple],
            Area::SealedGroundsSpiral_Upper => {
                &[Exit::SealedGroundsSpiral_To_Sky_SealedGroundsStatue]
            }
            Area::SealedTemple_Main => &[
                Exit::SealedTemple_To_BehindTheTemple,
                Exit::SealedTemple_To_HyliasTemple,
                Exit::SealedTemple_To_SealedGroundsSpiral,
            ],
            Area::InsideBambooIsland_Main => &[Exit::InsideBambooIsland_To_Sky],
            Area::LumpyPumpkin_Main => &[
                Exit::LumpyPumpkin_To_Sky_North,
                Exit::LumpyPumpkin_To_Sky_South,
            ],
            Area::Sky_BeedleIslandCage => &[],
            Area::Sky_BeedlesSkyHome => &[Exit::Sky_To_BeedlesShop_Night],
            Area::Sky_Field => &[
                Exit::Sky_To_BehindTheTemple_BehindTheTempleStatue,
                Exit::Sky_To_DeepWoods_DeepWoodsStatue,
                Exit::Sky_To_DeepWoods_ForestTempleStatue,
                Exit::Sky_To_EldinVolcano_EldinEntranceStatue,
                Exit::Sky_To_EldinVolcano_TempleEntranceStatue,
                Exit::Sky_To_EldinVolcano_VolcanoEastStatue,
                Exit::Sky_To_EldinVolcano_VolcanoAscentStatue,
                Exit::Sky_To_FaronWoods_FaronWoodsEntryStatue,
                Exit::Sky_To_FaronWoods_GreatTreeStatue,
                Exit::Sky_To_FaronWoods_InTheWoodsStatue,
                Exit::Sky_To_FaronWoods_ViewingPlatformStatue,
                Exit::Sky_To_FloriaWaterfall_FloriaWaterfallStatue,
                Exit::Sky_To_InsideBambooIsland,
                Exit::Sky_To_InsideThunderhead,
                Exit::Sky_To_LakeFloria_LakeFloriaStatue,
                Exit::Sky_To_LanayruDesert_DesertEntranceStatue,
                Exit::Sky_To_LanayruDesert_NorthDesertStatue,
                Exit::Sky_To_LanayruDesert_StoneCacheStatue,
                Exit::Sky_To_LanayruDesert_WestDesertStatue,
                Exit::Sky_To_LanayruMines_LanayruMineEntryStatue,
                Exit::Sky_To_OutsideFireSanctuary_InsideTheVolcanoStatue,
                Exit::Sky_To_SealedGroundsSpiral_SealedGroundsStatue,
                Exit::Sky_To_Skyloft,
            ],
            Area::Sky_OutsideLumpyPumpkin => &[
                Exit::Sky_To_LumpyPumpkin_North,
                Exit::Sky_To_LumpyPumpkin_South,
            ],
            Area::SkyKeepEntry_Main => &[Exit::SkyKeepEntry_To_Skyloft],
            Area::SkyloftSilentRealm_Trial => &[Exit::SkyloftSilentRealm_To_Skyloft],
            Area::BertiesHouse_Main => &[Exit::BertiesHouse_To_Skyloft],
            Area::GondosHouse_Main => &[Exit::GondosHouse_To_Skyloft],
            Area::MallarasHouse_Main => &[Exit::MallarasHouse_To_Skyloft],
            Area::RupinsHouse_Main => &[Exit::RupinsHouse_To_Skyloft],
            Area::Skyloft_OutsideSkyloftVillage => &[
                Exit::Skyloft_To_BatreauxHouse,
                Exit::Skyloft_To_BertiesHouse,
                Exit::Skyloft_To_GondosHouse,
                Exit::Skyloft_To_MallarasHouse,
                Exit::Skyloft_To_RupinsHouse,
                Exit::Skyloft_To_SparrotsHouse,
            ],
            Area::SparrotsHouse_Main => &[Exit::SparrotsHouse_To_Skyloft],
            Area::SkyviewBoss_Main => &[
                Exit::SkyviewBoss_To_SkyviewSpring,
                Exit::SkyviewBoss_To_SkyviewTemple,
            ],
            Area::SkyviewSpring_Main => &[
                Exit::SkyviewSpring_To_DeepWoods,
                Exit::SkyviewSpring_To_SkyviewBoss,
            ],
            Area::SkyviewTemple_BossDoorArea => &[Exit::SkyviewTemple_To_SkyviewBoss],
            Area::SkyviewTemple_Entry => &[Exit::SkyviewTemple_To_DeepWoods],
            Area::SkyviewTemple_FirstHub => &[],
            Area::SkyviewTemple_MainHub => &[],
            Area::InsideThunderhead_Main => &[
                Exit::InsideThunderhead_To_IsleOfSongs,
                Exit::InsideThunderhead_To_Sky,
            ],
            Area::IsleOfSongs_Main => &[Exit::IsleOfSongs_To_InsideThunderhead],
            Area::InsideVolcanoSummit_Main => &[
                Exit::InsideVolcanoSummit_To_EldinVolcano,
                Exit::InsideVolcanoSummit_To_OutsideFireSanctuary,
                Exit::InsideVolcanoSummit_To_VolcanoSummitWaterfall,
            ],
            Area::OutsideFireSanctuary_Middle => &[],
            Area::OutsideFireSanctuary_ToFireSanctuary => &[
                Exit::OutsideFireSanctuary_To_FireSanctuaryA,
                Exit::OutsideFireSanctuary_To_Sky_InsideTheVolcanoStatue,
            ],
            Area::OutsideFireSanctuary_ToInsideSummit => {
                &[Exit::OutsideFireSanctuary_To_InsideVolcanoSummit]
            }
            Area::VolcanoSummitWaterfall_Main => {
                &[Exit::VolcanoSummitWaterfall_To_InsideVolcanoSummit]
            }
        }
    }
    pub fn entrances(&self) -> &'static [Entrance] {
        match self {
            Area::AncientCistern_AfterAcGutters => &[],
            Area::AncientCistern_AfterWhipHooks => &[],
            Area::AncientCistern_BeforeBokoKeyDoor => &[],
            Area::AncientCistern_BeforeBossDoor => &[],
            Area::AncientCistern_BehindWaterfall => &[],
            Area::AncientCistern_BossKeyChestArea => &[],
            Area::AncientCistern_MainBasement => &[],
            Area::AncientCistern_MainHub => &[Entrance::AncientCistern_From_FloriaWaterfall],
            Area::AncientCistern_MainRoomVines => &[],
            Area::AncientCistern_SpiderThread => &[],
            Area::AncientCistern_WhipChestRoom => &[],
            Area::AncientCisternBoss_Main => &[Entrance::AncientCisternBoss_From_AncientCistern],
            Area::AncientCisternCandleRoom_Main => {
                &[Entrance::AncientCisternCandleRoom_From_AncientCisternBoss]
            }
            Area::BatreauxHouse_Main => &[Entrance::BatreauxHouse_From_Skyloft],
            Area::BeedlesShop_Main => &[
                Entrance::BeedlesShop_From_Sky_Night,
                Entrance::BeedlesShop_From_Skyloft_Day,
            ],
            Area::Bazaar_Main => &[
                Entrance::Bazaar_From_Skyloft_North,
                Entrance::Bazaar_From_Skyloft_South,
                Entrance::Bazaar_From_Skyloft_West,
            ],
            Area::ParrowAndOriellesHouse_Main => &[Entrance::ParrowAndOriellesHouse_From_Skyloft],
            Area::PeatricesHouse_Main => &[Entrance::PeatricesHouse_From_Skyloft],
            Area::PipersHouse_Main => &[Entrance::PipersHouse_From_Skyloft],
            Area::Skyloft_CentralOutside => &[
                Entrance::Skyloft_From_BatreauxHouse,
                Entrance::Skyloft_From_Bazaar_North,
                Entrance::Skyloft_From_Bazaar_South,
                Entrance::Skyloft_From_Bazaar_West,
                Entrance::Skyloft_From_BeedlesShop_Day,
                Entrance::Skyloft_From_BertiesHouse,
                Entrance::Skyloft_From_GondosHouse,
                Entrance::Skyloft_From_MallarasHouse,
                Entrance::Skyloft_From_ParrowAndOriellesHouse,
                Entrance::Skyloft_From_PeatricesHouse,
                Entrance::Skyloft_From_PipersHouse,
                Entrance::Skyloft_From_RupinsHouse,
                Entrance::Skyloft_From_Sky,
                Entrance::Skyloft_From_SkyloftSilentRealm,
                Entrance::Skyloft_From_SparrotsHouse,
                Entrance::Skyloft_From_WaterfallCave_Upper,
                Entrance::Skyloft_From_WrynasHouse,
            ],
            Area::Skyloft_PastWaterfallCave => &[Entrance::Skyloft_From_WaterfallCave_Lower],
            Area::Skyloft_ToSkyKeep => &[Entrance::Skyloft_From_SkyKeepEntry],
            Area::Skyloft_WaterfallCaveCrystals => &[],
            Area::WaterfallCave_Main => &[
                Entrance::WaterfallCave_From_Skyloft_Lower,
                Entrance::WaterfallCave_From_Skyloft_Upper,
            ],
            Area::WrynasHouse_Main => &[Entrance::WrynasHouse_From_Skyloft],
            Area::EarthTemple_AfterBallRolling => &[],
            Area::EarthTemple_BallRolling => &[],
            Area::EarthTemple_BossDoorArea => &[],
            Area::EarthTemple_Entrance => &[Entrance::EarthTemple_From_EldinVolcano],
            Area::EarthTempleBoss_Main => &[Entrance::EarthTempleBoss_From_EarthTemple],
            Area::EarthTempleSpring_Main => &[Entrance::EarthTempleSpring_From_EarthTempleBoss],
            Area::EldinSilentRealm_Trial => &[Entrance::EldinSilentRealm_From_EldinVolcano],
            Area::EldinVolcano_FirstRoom => &[Entrance::EldinVolcano_From_Sky_EldinEntranceStatue],
            Area::EldinVolcano_HotCaveArea => &[Entrance::EldinVolcano_From_InsideVolcanoSummit],
            Area::EldinVolcano_NearThrillDigger => &[Entrance::EldinVolcano_From_ThrillDiggerCave],
            Area::EldinVolcano_OutsideEt => &[
                Entrance::EldinVolcano_From_EarthTemple,
                Entrance::EldinVolcano_From_EarthTempleSpring,
                Entrance::EldinVolcano_From_Sky_TempleEntranceStatue,
            ],
            Area::EldinVolcano_PastMogmaTurf => &[Entrance::EldinVolcano_From_MogmaTurf_EndVent],
            Area::EldinVolcano_PastSlide => &[],
            Area::EldinVolcano_PreMogmaTurf => &[
                Entrance::EldinVolcano_From_MogmaTurf_StartVent,
                Entrance::EldinVolcano_From_Sky_VolcanoEastStatue,
            ],
            Area::EldinVolcano_SandSlide => &[],
            Area::EldinVolcano_VolcanoAscent => &[
                Entrance::EldinVolcano_From_EldinSilentRealm,
                Entrance::EldinVolcano_From_Sky_VolcanoAscentStatue,
            ],
            Area::ThrillDiggerCave_Main => &[Entrance::ThrillDiggerCave_From_EldinVolcano],
            Area::FaronSilentRealm_Trial => &[Entrance::FaronSilentRealm_From_FaronWoods],
            Area::DeepWoods_Entry => &[Entrance::DeepWoods_From_FaronWoods],
            Area::DeepWoods_PastBeehive => &[
                Entrance::DeepWoods_From_Sky_DeepWoodsStatue,
                Entrance::DeepWoods_From_Sky_ForestTempleStatue,
                Entrance::DeepWoods_From_SkyviewSpring,
                Entrance::DeepWoods_From_SkyviewTemple,
            ],
            Area::FaronWoods_ClawshotTargetBranch => &[],
            Area::FaronWoods_Entry => &[
                Entrance::FaronWoods_From_BehindTheTemple,
                Entrance::FaronWoods_From_Sky_FaronWoodsEntryStatue,
            ],
            Area::FaronWoods_GreatTreePlatforms => &[
                Entrance::FaronWoods_From_GreatTree_LowerPlatform,
                Entrance::FaronWoods_From_GreatTree_UpperPlatform,
            ],
            Area::FaronWoods_GreatTreeTop => &[
                Entrance::FaronWoods_From_GreatTree_Top,
                Entrance::FaronWoods_From_Sky_GreatTreeStatue,
            ],
            Area::FaronWoods_Main => &[
                Entrance::FaronWoods_From_DeepWoods,
                Entrance::FaronWoods_From_FaronSilentRealm,
                Entrance::FaronWoods_From_FloriaWaterfall,
                Entrance::FaronWoods_From_GreatTree_Tunnel,
                Entrance::FaronWoods_From_Sky_InTheWoodsStatue,
                Entrance::FaronWoods_From_Sky_ViewingPlatformStatue,
            ],
            Area::GreatTree_Entry => &[Entrance::GreatTree_From_FaronWoods_Tunnel],
            Area::GreatTree_Lower => &[Entrance::GreatTree_From_FaronWoods_LowerPlatform],
            Area::GreatTree_Middle => &[],
            Area::GreatTree_PastPlatforms => &[],
            Area::GreatTree_Upper => &[
                Entrance::GreatTree_From_FaronWoods_Top,
                Entrance::GreatTree_From_FaronWoods_UpperPlatform,
            ],
            Area::FireSanctuaryA_Entry => &[Entrance::FireSanctuaryA_From_OutsideFireSanctuary],
            Area::FireSanctuaryA_InFrontOfBossDoor => &[],
            Area::FireSanctuaryA_PastFirstWaterPlant => &[],
            Area::FireSanctuaryA_PrePlatsArea => &[Entrance::FireSanctuaryA_From_FireSanctuaryB],
            Area::FireSanctuaryA_UpperStaircaseRoom => &[],
            Area::FireSanctuaryB_AfterDoubleMagmanosFight => &[],
            Area::FireSanctuaryB_FirstOutsideSection => {
                &[Entrance::FireSanctuaryB_From_FireSanctuaryA]
            }
            Area::FireSanctuaryB_LastTrappedMogmaArea => &[],
            Area::FireSanctuaryB_PastSecondRoomWithWaterFruit => &[],
            Area::FireSanctuaryB_UnderDoubleMagmanosFight => &[],
            Area::FireSanctuaryB_WaterFruitRoom => &[],
            Area::FireSanctuaryBoss_Main => &[Entrance::FireSanctuaryBoss_From_FireSanctuaryA],
            Area::FireSanctuaryFlameRoom_Main => {
                &[Entrance::FireSanctuaryFlameRoom_From_FireSanctuaryBoss]
            }
            Area::InsideGoddessStatue_Main => &[Entrance::InsideGoddessStatue_From_Skyloft],
            Area::KnightAcademy_AboveZeldasRoom => &[Entrance::KnightAcademy_From_Skyloft_Chimney],
            Area::KnightAcademy_Main => &[
                Entrance::KnightAcademy_From_Skyloft_LowerDoors,
                Entrance::KnightAcademy_From_Skyloft_UpperDoors,
            ],
            Area::Skyloft_OutsideGoddessStatue => &[
                Entrance::Skyloft_From_InsideGoddessStatue,
                Entrance::Skyloft_From_KnightAcademy_LowerDoors,
                Entrance::Skyloft_From_KnightAcademy_UpperDoors,
                Entrance::Skyloft_From_SparringHall,
            ],
            Area::SparringHall_Main => &[Entrance::SparringHall_From_Skyloft],
            Area::FaroresLair_Main => &[
                Entrance::FaroresLair_From_FloriaWaterfall,
                Entrance::FaroresLair_From_LakeFloria,
            ],
            Area::FloriaWaterfall_Main => &[
                Entrance::FloriaWaterfall_From_AncientCistern,
                Entrance::FloriaWaterfall_From_FaroresLair,
                Entrance::FloriaWaterfall_From_Sky_FloriaWaterfallStatue,
            ],
            Area::LakeFloria_Entry => &[Entrance::LakeFloria_From_FaronWoods],
            Area::LakeFloria_StatueSpot => &[Entrance::LakeFloria_From_Sky_LakeFloriaStatue],
            Area::LakeFloria_ToFaroresLair => &[Entrance::LakeFloria_From_FaroresLair],
            Area::LanayruCaves_Main => &[
                Entrance::LanayruCaves_From_LanayruDesert,
                Entrance::LanayruCaves_From_LanayruMines,
            ],
            Area::LanayruCaves_ToSandSea => &[Entrance::LanayruCaves_From_SandSeaDocks],
            Area::FireNode_End => &[],
            Area::FireNode_Main => &[Entrance::FireNode_From_LanayruDesert],
            Area::LanayruDesert_HookBeetleArea => &[
                Entrance::LanayruDesert_From_LanayruMines,
                Entrance::LanayruDesert_From_Sky_DesertEntranceStatue,
            ],
            Area::LanayruDesert_PastToT => &[
                Entrance::LanayruDesert_From_FireNode,
                Entrance::LanayruDesert_From_LanayruMiningFacilityA,
                Entrance::LanayruDesert_From_LanayruSilentRealm,
                Entrance::LanayruDesert_From_LightningNode,
                Entrance::LanayruDesert_From_Sky_NorthDesertStatue,
                Entrance::LanayruDesert_From_Sky_StoneCacheStatue,
                Entrance::LanayruDesert_From_TempleOfTime_End,
            ],
            Area::LanayruDesert_SandOasis => &[
                Entrance::LanayruDesert_From_LanayruCaves,
                Entrance::LanayruDesert_From_Sky_WestDesertStatue,
                Entrance::LanayruDesert_From_TempleOfTime_Start,
            ],
            Area::LightningNode_Main => &[Entrance::LightningNode_From_LanayruDesert],
            Area::TempleOfTime_AfterLmf => {
                &[Entrance::TempleOfTime_From_LanayruMiningFacilityToToT]
            }
            Area::TempleOfTime_End => &[Entrance::TempleOfTime_From_LanayruDesert_End],
            Area::TempleOfTime_NearCube => &[],
            Area::TempleOfTime_NearGossipStone => &[],
            Area::TempleOfTime_Start => &[Entrance::TempleOfTime_From_LanayruDesert_Start],
            Area::LanayruMines_FirstHalf => {
                &[Entrance::LanayruMines_From_Sky_LanayruMineEntryStatue]
            }
            Area::LanayruMines_ToCaves => &[Entrance::LanayruMines_From_LanayruCaves],
            Area::LanayruMines_ToDesert => &[Entrance::LanayruMines_From_LanayruDesert],
            Area::LanayruMiningFacilityA_Entry => {
                &[Entrance::LanayruMiningFacilityA_From_LanayruDesert]
            }
            Area::LanayruMiningFacilityA_FirstKeyLockedRoom => &[],
            Area::LanayruMiningFacilityA_FirstWestRoom => &[],
            Area::LanayruMiningFacilityA_GustBellowsRoom => &[],
            Area::LanayruMiningFacilityA_MapRoom => &[],
            Area::LanayruMiningFacilityA_SecondRoom => &[],
            Area::LanayruMiningFacilityB_AfterLmfBkRoom => &[],
            Area::LanayruMiningFacilityB_HubRoom => {
                &[Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub]
            }
            Area::LanayruMiningFacilityB_InsideLmfBkRoom => &[],
            Area::LanayruMiningFacilityB_NearBossDoor => &[],
            Area::LanayruMiningFacilityB_NearFirstHubRoomChest => {
                &[Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub2]
            }
            Area::LanayruMiningFacilityB_WestHub => {
                &[Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_HubW]
            }
            Area::LanayruMiningFacilityBoss_Main => {
                &[Entrance::LanayruMiningFacilityBoss_From_LanayruMiningFacilityB]
            }
            Area::LanayruMiningFacilityToToT_BossDoor => {
                &[Entrance::LanayruMiningFacilityToToT_From_LanayruMiningFacilityBoss]
            }
            Area::LanayruMiningFacilityToToT_ToTExit => &[],
            Area::InsidePiratesStronghold_Main => &[
                Entrance::InsidePiratesStronghold_From_OutsidePiratesStronghold_Beginning,
                Entrance::InsidePiratesStronghold_From_OutsidePiratesStronghold_End,
            ],
            Area::OutsidePiratesStronghold_InsideSharkhead => {
                &[Entrance::OutsidePiratesStronghold_From_InsidePiratesStronghold_End]
            }
            Area::OutsidePiratesStronghold_Main => &[
                Entrance::OutsidePiratesStronghold_From_InsidePiratesStronghold_Beginning,
                Entrance::OutsidePiratesStronghold_From_SandSea,
            ],
            Area::SandSea_Main => &[
                Entrance::SandSea_From_OutsidePiratesStronghold,
                Entrance::SandSea_From_SandSeaDocks,
                Entrance::SandSea_From_Sandship,
                Entrance::SandSea_From_Shipyard,
                Entrance::SandSea_From_SkippersRetreat,
            ],
            Area::SandSeaDocks_Main => &[Entrance::SandSeaDocks_From_SandSea],
            Area::SandSeaDocks_ToCaves => &[Entrance::SandSeaDocks_From_LanayruCaves],
            Area::Shipyard_AfterMinecartRide => {
                &[Entrance::Shipyard_From_ShipyardConstructionBay_Upper]
            }
            Area::Shipyard_Main => &[
                Entrance::Shipyard_From_SandSea,
                Entrance::Shipyard_From_ShipyardConstructionBay_Lower,
            ],
            Area::ShipyardConstructionBay_Lower => {
                &[Entrance::ShipyardConstructionBay_From_Shipyard_Lower]
            }
            Area::ShipyardConstructionBay_Upper => {
                &[Entrance::ShipyardConstructionBay_From_Shipyard_Upper]
            }
            Area::SkippersRetreat_NextToShack => &[Entrance::SkippersRetreat_From_SkippersShack],
            Area::SkippersRetreat_PastDekuBaba => &[],
            Area::SkippersRetreat_PastMoblin => &[],
            Area::SkippersRetreat_Start => &[Entrance::SkippersRetreat_From_SandSea],
            Area::SkippersShack_Main => &[Entrance::SkippersShack_From_SkippersRetreat],
            Area::LanayruSilentRealm_Trial => &[Entrance::LanayruSilentRealm_From_LanayruDesert],
            Area::MogmaTurf_Main => &[Entrance::MogmaTurf_From_EldinVolcano_Skydive],
            Area::Sandship_Deck => &[Entrance::Sandship_From_SandSea],
            Area::Sandship_PastSpume => &[],
            Area::Sandship_SandshipBrig => &[],
            Area::SandshipBoss_Main => &[Entrance::SandshipBoss_From_Sandship],
            Area::BehindTheTemple_Main => &[
                Entrance::BehindTheTemple_From_FaronWoods,
                Entrance::BehindTheTemple_From_SealedTemple,
                Entrance::BehindTheTemple_From_Sky_BehindTheTempleStatue,
            ],
            Area::HyliasTemple_Main => &[Entrance::HyliasTemple_From_SealedTemple],
            Area::SealedGroundsSpiral_Lower => &[Entrance::SealedGroundsSpiral_From_SealedTemple],
            Area::SealedGroundsSpiral_Upper => &[
                Entrance::SealedGroundsSpiral_From_BehindTheTemple,
                Entrance::SealedGroundsSpiral_From_Sky_SealedGroundsStatue,
            ],
            Area::SealedTemple_Main => &[
                Entrance::SealedTemple_From_BehindTheTemple,
                Entrance::SealedTemple_From_SealedGroundsSpiral,
            ],
            Area::InsideBambooIsland_Main => &[Entrance::InsideBambooIsland_From_Sky],
            Area::LumpyPumpkin_Main => &[
                Entrance::LumpyPumpkin_From_Sky_North,
                Entrance::LumpyPumpkin_From_Sky_South,
            ],
            Area::Sky_BeedleIslandCage => &[],
            Area::Sky_BeedlesSkyHome => &[Entrance::Sky_From_BeedlesShop_Night],
            Area::Sky_Field => &[
                Entrance::Sky_From_BehindTheTemple_BehindTheTempleStatue,
                Entrance::Sky_From_DeepWoods_DeepWoodsStatue,
                Entrance::Sky_From_DeepWoods_ForestTempleStatue,
                Entrance::Sky_From_EldinVolcano_EldinEntranceStatue,
                Entrance::Sky_From_EldinVolcano_TempleEntranceStatue,
                Entrance::Sky_From_EldinVolcano_VolcanoAscentStatue,
                Entrance::Sky_From_EldinVolcano_VolcanoEastStatue,
                Entrance::Sky_From_FaronWoods_FaronWoodsEntryStatue,
                Entrance::Sky_From_FaronWoods_GreatTreeStatue,
                Entrance::Sky_From_FaronWoods_InTheWoodsStatue,
                Entrance::Sky_From_FaronWoods_ViewingPlatformStatue,
                Entrance::Sky_From_FloriaWaterfall_FloriaWaterfallStatue,
                Entrance::Sky_From_InsideBambooIsland,
                Entrance::Sky_From_InsideThunderhead,
                Entrance::Sky_From_LakeFloria_LakeFloriaStatue,
                Entrance::Sky_From_LanayruDesert_DesertEntranceStatue,
                Entrance::Sky_From_LanayruDesert_NorthDesertStatue,
                Entrance::Sky_From_LanayruDesert_StoneCacheStatue,
                Entrance::Sky_From_LanayruDesert_WestDesertStatue,
                Entrance::Sky_From_LanayruMines_LanayruMineEntryStatue,
                Entrance::Sky_From_OutsideFireSanctuary_InsideTheVolcanoStatue,
                Entrance::Sky_From_SandSeaDocks_AncientHarbor,
                Entrance::Sky_From_SealedGroundsSpiral_SealedGroundsStatue,
                Entrance::Sky_From_Skyloft,
                Entrance::Sky_From_Skyloft_PastWaterfallCave,
            ],
            Area::Sky_OutsideLumpyPumpkin => &[
                Entrance::Sky_From_LumpyPumpkin_North,
                Entrance::Sky_From_LumpyPumpkin_South,
            ],
            Area::SkyKeepEntry_Main => &[Entrance::SkyKeepEntry_From_Skyloft],
            Area::SkyloftSilentRealm_Trial => &[Entrance::SkyloftSilentRealm_From_Skyloft],
            Area::BertiesHouse_Main => &[Entrance::BertiesHouse_From_Skyloft],
            Area::GondosHouse_Main => &[Entrance::GondosHouse_From_Skyloft],
            Area::MallarasHouse_Main => &[Entrance::MallarasHouse_From_Skyloft],
            Area::RupinsHouse_Main => &[Entrance::RupinsHouse_From_Skyloft],
            Area::Skyloft_OutsideSkyloftVillage => &[],
            Area::SparrotsHouse_Main => &[Entrance::SparrotsHouse_From_Skyloft],
            Area::SkyviewBoss_Main => &[
                Entrance::SkyviewBoss_From_SkyviewSpring,
                Entrance::SkyviewBoss_From_SkyviewTemple,
            ],
            Area::SkyviewSpring_Main => &[Entrance::SkyviewSpring_From_SkyviewBoss],
            Area::SkyviewTemple_BossDoorArea => &[Entrance::SkyviewTemple_From_SkyviewBoss],
            Area::SkyviewTemple_Entry => &[Entrance::SkyviewTemple_From_DeepWoods],
            Area::SkyviewTemple_FirstHub => &[],
            Area::SkyviewTemple_MainHub => &[],
            Area::InsideThunderhead_Main => &[
                Entrance::InsideThunderhead_From_IsleOfSongs,
                Entrance::InsideThunderhead_From_Sky,
            ],
            Area::IsleOfSongs_Main => &[Entrance::IsleOfSongs_From_InsideThunderhead],
            Area::InsideVolcanoSummit_Main => &[
                Entrance::InsideVolcanoSummit_From_EldinVolcano,
                Entrance::InsideVolcanoSummit_From_OutsideFireSanctuary,
                Entrance::InsideVolcanoSummit_From_VolcanoSummitWaterfall,
            ],
            Area::OutsideFireSanctuary_Middle => &[],
            Area::OutsideFireSanctuary_ToFireSanctuary => &[
                Entrance::OutsideFireSanctuary_From_FireSanctuaryA,
                Entrance::OutsideFireSanctuary_From_Sky_InsideTheVolcanoStatue,
            ],
            Area::OutsideFireSanctuary_ToInsideSummit => {
                &[Entrance::OutsideFireSanctuary_From_InsideVolcanoSummit]
            }
            Area::VolcanoSummitWaterfall_Main => {
                &[Entrance::VolcanoSummitWaterfall_From_InsideVolcanoSummit]
            }
        }
    }
    pub fn logic_entrances(&self) -> &'static [(Area, RequirementKey)] {
        match self {
            Area::AncientCistern_AfterAcGutters => &[(
                Area::AncientCistern_BeforeBokoKeyDoor,
                RequirementKey::AncientCistern_BeforeBokoKeyDoor_To_AfterAcGutters,
            )],
            Area::AncientCistern_AfterWhipHooks => &[
                (
                    Area::AncientCistern_AfterAcGutters,
                    RequirementKey::AncientCistern_AfterAcGutters_To_AfterWhipHooks,
                ),
                (
                    Area::AncientCistern_MainHub,
                    RequirementKey::AncientCistern_MainHub_To_AfterWhipHooks,
                ),
                (
                    Area::AncientCistern_MainRoomVines,
                    RequirementKey::AncientCistern_MainRoomVines_To_AfterWhipHooks,
                ),
            ],
            Area::AncientCistern_BeforeBokoKeyDoor => &[(
                Area::AncientCistern_BehindWaterfall,
                RequirementKey::AncientCistern_BehindWaterfall_To_BeforeBokoKeyDoor,
            )],
            Area::AncientCistern_BeforeBossDoor => &[
                (
                    Area::AncientCistern_MainBasement,
                    RequirementKey::AncientCistern_MainBasement_To_BeforeBossDoor,
                ),
                (
                    Area::AncientCistern_MainHub,
                    RequirementKey::AncientCistern_MainHub_To_BeforeBossDoor,
                ),
            ],
            Area::AncientCistern_BehindWaterfall => &[(
                Area::AncientCistern_MainHub,
                RequirementKey::AncientCistern_MainHub_To_BehindWaterfall,
            )],
            Area::AncientCistern_BossKeyChestArea => &[
                (
                    Area::AncientCistern_MainBasement,
                    RequirementKey::AncientCistern_MainBasement_To_BossKeyChestArea,
                ),
                (
                    Area::AncientCistern_SpiderThread,
                    RequirementKey::AncientCistern_SpiderThread_To_BossKeyChestArea,
                ),
            ],
            Area::AncientCistern_MainBasement => &[
                (
                    Area::AncientCistern_MainHub,
                    RequirementKey::AncientCistern_MainHub_To_MainBasement,
                ),
                (
                    Area::AncientCistern_SpiderThread,
                    RequirementKey::AncientCistern_SpiderThread_To_MainBasement,
                ),
            ],
            Area::AncientCistern_MainHub => &[],
            Area::AncientCistern_MainRoomVines => &[
                (
                    Area::AncientCistern_AfterAcGutters,
                    RequirementKey::AncientCistern_AfterAcGutters_To_MainRoomVines,
                ),
                (
                    Area::AncientCistern_AfterWhipHooks,
                    RequirementKey::AncientCistern_AfterWhipHooks_To_MainRoomVines,
                ),
            ],
            Area::AncientCistern_SpiderThread => &[
                (
                    Area::AncientCistern_MainBasement,
                    RequirementKey::AncientCistern_MainBasement_To_SpiderThread,
                ),
                (
                    Area::AncientCistern_MainHub,
                    RequirementKey::AncientCistern_MainHub_To_SpiderThread,
                ),
            ],
            Area::AncientCistern_WhipChestRoom => &[(
                Area::AncientCistern_MainHub,
                RequirementKey::AncientCistern_MainHub_To_WhipChestRoom,
            )],
            Area::AncientCisternBoss_Main => &[],
            Area::AncientCisternCandleRoom_Main => &[],
            Area::BatreauxHouse_Main => &[],
            Area::BeedlesShop_Main => &[],
            Area::Bazaar_Main => &[],
            Area::ParrowAndOriellesHouse_Main => &[],
            Area::PeatricesHouse_Main => &[],
            Area::PipersHouse_Main => &[],
            Area::Skyloft_CentralOutside => &[
                (
                    Area::Skyloft_OutsideGoddessStatue,
                    RequirementKey::Skyloft_OutsideGoddessStatue_To_CentralOutside,
                ),
                (
                    Area::Skyloft_OutsideSkyloftVillage,
                    RequirementKey::Skyloft_OutsideSkyloftVillage_To_CentralOutside,
                ),
                (
                    Area::Skyloft_ToSkyKeep,
                    RequirementKey::Skyloft_ToSkyKeep_To_CentralOutside,
                ),
            ],
            Area::Skyloft_PastWaterfallCave => &[(
                Area::Skyloft_OutsideSkyloftVillage,
                RequirementKey::Skyloft_OutsideSkyloftVillage_To_PastWaterfallCave,
            )],
            Area::Skyloft_ToSkyKeep => &[(
                Area::Skyloft_CentralOutside,
                RequirementKey::Skyloft_CentralOutside_To_ToSkyKeep,
            )],
            Area::Skyloft_WaterfallCaveCrystals => &[
                (
                    Area::Skyloft_CentralOutside,
                    RequirementKey::Skyloft_CentralOutside_To_WaterfallCaveCrystals,
                ),
                (
                    Area::Skyloft_PastWaterfallCave,
                    RequirementKey::Skyloft_PastWaterfallCave_To_WaterfallCaveCrystals,
                ),
            ],
            Area::WaterfallCave_Main => &[],
            Area::WrynasHouse_Main => &[],
            Area::EarthTemple_AfterBallRolling => &[(
                Area::EarthTemple_BallRolling,
                RequirementKey::EarthTemple_BallRolling_To_AfterBallRolling,
            )],
            Area::EarthTemple_BallRolling => &[(
                Area::EarthTemple_Entrance,
                RequirementKey::EarthTemple_Entrance_To_BallRolling,
            )],
            Area::EarthTemple_BossDoorArea => &[(
                Area::EarthTemple_AfterBallRolling,
                RequirementKey::EarthTemple_AfterBallRolling_To_BossDoorArea,
            )],
            Area::EarthTemple_Entrance => &[],
            Area::EarthTempleBoss_Main => &[],
            Area::EarthTempleSpring_Main => &[],
            Area::EldinSilentRealm_Trial => &[],
            Area::EldinVolcano_FirstRoom => &[
                (
                    Area::EldinVolcano_PreMogmaTurf,
                    RequirementKey::EldinVolcano_PreMogmaTurf_To_FirstRoom,
                ),
                (
                    Area::EldinVolcano_VolcanoAscent,
                    RequirementKey::EldinVolcano_VolcanoAscent_To_FirstRoom,
                ),
            ],
            Area::EldinVolcano_HotCaveArea => &[
                (
                    Area::EldinVolcano_OutsideEt,
                    RequirementKey::EldinVolcano_OutsideEt_To_HotCaveArea,
                ),
                (
                    Area::EldinVolcano_PastSlide,
                    RequirementKey::EldinVolcano_PastSlide_To_HotCaveArea,
                ),
            ],
            Area::EldinVolcano_NearThrillDigger => &[(
                Area::EldinVolcano_VolcanoAscent,
                RequirementKey::EldinVolcano_VolcanoAscent_To_NearThrillDigger,
            )],
            Area::EldinVolcano_OutsideEt => &[(
                Area::EldinVolcano_NearThrillDigger,
                RequirementKey::EldinVolcano_NearThrillDigger_To_OutsideEt,
            )],
            Area::EldinVolcano_PastMogmaTurf => &[(
                Area::EldinVolcano_PreMogmaTurf,
                RequirementKey::EldinVolcano_PreMogmaTurf_To_PastMogmaTurf,
            )],
            Area::EldinVolcano_PastSlide => &[
                (
                    Area::EldinVolcano_SandSlide,
                    RequirementKey::EldinVolcano_SandSlide_To_PastSlide,
                ),
                (
                    Area::EldinVolcano_VolcanoAscent,
                    RequirementKey::EldinVolcano_VolcanoAscent_To_PastSlide,
                ),
            ],
            Area::EldinVolcano_PreMogmaTurf => &[
                (
                    Area::EldinVolcano_FirstRoom,
                    RequirementKey::EldinVolcano_FirstRoom_To_PreMogmaTurf,
                ),
                (
                    Area::EldinVolcano_PastMogmaTurf,
                    RequirementKey::EldinVolcano_PastMogmaTurf_To_PreMogmaTurf,
                ),
            ],
            Area::EldinVolcano_SandSlide => &[(
                Area::EldinVolcano_HotCaveArea,
                RequirementKey::EldinVolcano_HotCaveArea_To_SandSlide,
            )],
            Area::EldinVolcano_VolcanoAscent => &[
                (
                    Area::EldinVolcano_FirstRoom,
                    RequirementKey::EldinVolcano_FirstRoom_To_VolcanoAscent,
                ),
                (
                    Area::EldinVolcano_NearThrillDigger,
                    RequirementKey::EldinVolcano_NearThrillDigger_To_VolcanoAscent,
                ),
                (
                    Area::EldinVolcano_PastMogmaTurf,
                    RequirementKey::EldinVolcano_PastMogmaTurf_To_VolcanoAscent,
                ),
                (
                    Area::EldinVolcano_PastSlide,
                    RequirementKey::EldinVolcano_PastSlide_To_VolcanoAscent,
                ),
            ],
            Area::ThrillDiggerCave_Main => &[],
            Area::FaronSilentRealm_Trial => &[],
            Area::DeepWoods_Entry => &[(
                Area::DeepWoods_PastBeehive,
                RequirementKey::DeepWoods_PastBeehive_To_Entry,
            )],
            Area::DeepWoods_PastBeehive => &[(
                Area::DeepWoods_Entry,
                RequirementKey::DeepWoods_Entry_To_PastBeehive,
            )],
            Area::FaronWoods_ClawshotTargetBranch => &[
                (
                    Area::FaronWoods_GreatTreeTop,
                    RequirementKey::FaronWoods_GreatTreeTop_To_ClawshotTargetBranch,
                ),
                (
                    Area::FaronWoods_Main,
                    RequirementKey::FaronWoods_Main_To_ClawshotTargetBranch,
                ),
            ],
            Area::FaronWoods_Entry => &[(
                Area::FaronWoods_Main,
                RequirementKey::FaronWoods_Main_To_Entry,
            )],
            Area::FaronWoods_GreatTreePlatforms => &[
                (
                    Area::FaronWoods_GreatTreeTop,
                    RequirementKey::FaronWoods_GreatTreeTop_To_GreatTreePlatforms,
                ),
                (
                    Area::FaronWoods_Main,
                    RequirementKey::FaronWoods_Main_To_GreatTreePlatforms,
                ),
            ],
            Area::FaronWoods_GreatTreeTop => &[],
            Area::FaronWoods_Main => &[(
                Area::FaronWoods_Entry,
                RequirementKey::FaronWoods_Entry_To_Main,
            )],
            Area::GreatTree_Entry => &[(
                Area::GreatTree_Lower,
                RequirementKey::GreatTree_Lower_To_Entry,
            )],
            Area::GreatTree_Lower => &[
                (
                    Area::GreatTree_Entry,
                    RequirementKey::GreatTree_Entry_To_Lower,
                ),
                (
                    Area::GreatTree_Middle,
                    RequirementKey::GreatTree_Middle_To_Lower,
                ),
                (
                    Area::GreatTree_PastPlatforms,
                    RequirementKey::GreatTree_PastPlatforms_To_Lower,
                ),
            ],
            Area::GreatTree_Middle => &[
                (
                    Area::GreatTree_Lower,
                    RequirementKey::GreatTree_Lower_To_Middle,
                ),
                (
                    Area::GreatTree_Upper,
                    RequirementKey::GreatTree_Upper_To_Middle,
                ),
            ],
            Area::GreatTree_PastPlatforms => &[
                (
                    Area::GreatTree_Lower,
                    RequirementKey::GreatTree_Lower_To_PastPlatforms,
                ),
                (
                    Area::GreatTree_Middle,
                    RequirementKey::GreatTree_Middle_To_PastPlatforms,
                ),
            ],
            Area::GreatTree_Upper => &[],
            Area::FireSanctuaryA_Entry => &[],
            Area::FireSanctuaryA_InFrontOfBossDoor => &[
                (
                    Area::FireSanctuaryA_PrePlatsArea,
                    RequirementKey::FireSanctuaryA_PrePlatsArea_To_InFrontOfBossDoor,
                ),
                (
                    Area::FireSanctuaryA_UpperStaircaseRoom,
                    RequirementKey::FireSanctuaryA_UpperStaircaseRoom_To_InFrontOfBossDoor,
                ),
            ],
            Area::FireSanctuaryA_PastFirstWaterPlant => &[(
                Area::FireSanctuaryA_Entry,
                RequirementKey::FireSanctuaryA_Entry_To_PastFirstWaterPlant,
            )],
            Area::FireSanctuaryA_PrePlatsArea => &[],
            Area::FireSanctuaryA_UpperStaircaseRoom => &[(
                Area::FireSanctuaryA_InFrontOfBossDoor,
                RequirementKey::FireSanctuaryA_InFrontOfBossDoor_To_UpperStaircaseRoom,
            )],
            Area::FireSanctuaryB_AfterDoubleMagmanosFight => &[(
                Area::FireSanctuaryB_WaterFruitRoom,
                RequirementKey::FireSanctuaryB_WaterFruitRoom_To_AfterDoubleMagmanosFight,
            )],
            Area::FireSanctuaryB_FirstOutsideSection => &[],
            Area::FireSanctuaryB_LastTrappedMogmaArea => &[(
                Area::FireSanctuaryB_UnderDoubleMagmanosFight,
                RequirementKey::FireSanctuaryB_UnderDoubleMagmanosFight_To_LastTrappedMogmaArea,
            )],
            Area::FireSanctuaryB_PastSecondRoomWithWaterFruit => &[(
                Area::FireSanctuaryB_FirstOutsideSection,
                RequirementKey::FireSanctuaryB_FirstOutsideSection_To_PastSecondRoomWithWaterFruit,
            )],
            Area::FireSanctuaryB_UnderDoubleMagmanosFight => &[(
                Area::FireSanctuaryB_AfterDoubleMagmanosFight,
                RequirementKey::FireSanctuaryB_AfterDoubleMagmanosFight_To_UnderDoubleMagmanosFight,
            )],
            Area::FireSanctuaryB_WaterFruitRoom => &[(
                Area::FireSanctuaryB_PastSecondRoomWithWaterFruit,
                RequirementKey::FireSanctuaryB_PastSecondRoomWithWaterFruit_To_WaterFruitRoom,
            )],
            Area::FireSanctuaryBoss_Main => &[],
            Area::FireSanctuaryFlameRoom_Main => &[],
            Area::InsideGoddessStatue_Main => &[],
            Area::KnightAcademy_AboveZeldasRoom => &[],
            Area::KnightAcademy_Main => &[(
                Area::KnightAcademy_AboveZeldasRoom,
                RequirementKey::KnightAcademy_AboveZeldasRoom_To_Main,
            )],
            Area::Skyloft_OutsideGoddessStatue => &[(
                Area::Skyloft_CentralOutside,
                RequirementKey::Skyloft_CentralOutside_To_OutsideGoddessStatue,
            )],
            Area::SparringHall_Main => &[],
            Area::FaroresLair_Main => &[],
            Area::FloriaWaterfall_Main => &[],
            Area::LakeFloria_Entry => &[],
            Area::LakeFloria_StatueSpot => &[
                (
                    Area::LakeFloria_Entry,
                    RequirementKey::LakeFloria_Entry_To_StatueSpot,
                ),
                (
                    Area::LakeFloria_ToFaroresLair,
                    RequirementKey::LakeFloria_ToFaroresLair_To_StatueSpot,
                ),
            ],
            Area::LakeFloria_ToFaroresLair => &[(
                Area::LakeFloria_StatueSpot,
                RequirementKey::LakeFloria_StatueSpot_To_ToFaroresLair,
            )],
            Area::LanayruCaves_Main => &[(
                Area::LanayruCaves_ToSandSea,
                RequirementKey::LanayruCaves_ToSandSea_To_Main,
            )],
            Area::LanayruCaves_ToSandSea => &[(
                Area::LanayruCaves_Main,
                RequirementKey::LanayruCaves_Main_To_ToSandSea,
            )],
            Area::FireNode_End => &[(Area::FireNode_Main, RequirementKey::FireNode_Main_To_End)],
            Area::FireNode_Main => &[],
            Area::LanayruDesert_HookBeetleArea => &[
                (
                    Area::LanayruDesert_PastToT,
                    RequirementKey::LanayruDesert_PastToT_To_HookBeetleArea,
                ),
                (
                    Area::LanayruDesert_SandOasis,
                    RequirementKey::LanayruDesert_SandOasis_To_HookBeetleArea,
                ),
            ],
            Area::LanayruDesert_PastToT => &[(
                Area::LanayruDesert_HookBeetleArea,
                RequirementKey::LanayruDesert_HookBeetleArea_To_PastToT,
            )],
            Area::LanayruDesert_SandOasis => &[(
                Area::LanayruDesert_HookBeetleArea,
                RequirementKey::LanayruDesert_HookBeetleArea_To_SandOasis,
            )],
            Area::LightningNode_Main => &[],
            Area::TempleOfTime_AfterLmf => &[],
            Area::TempleOfTime_End => &[(
                Area::TempleOfTime_NearCube,
                RequirementKey::TempleOfTime_NearCube_To_End,
            )],
            Area::TempleOfTime_NearCube => &[(
                Area::TempleOfTime_NearGossipStone,
                RequirementKey::TempleOfTime_NearGossipStone_To_NearCube,
            )],
            Area::TempleOfTime_NearGossipStone => &[
                (
                    Area::TempleOfTime_AfterLmf,
                    RequirementKey::TempleOfTime_AfterLmf_To_NearGossipStone,
                ),
                (
                    Area::TempleOfTime_Start,
                    RequirementKey::TempleOfTime_Start_To_NearGossipStone,
                ),
            ],
            Area::TempleOfTime_Start => &[(
                Area::TempleOfTime_End,
                RequirementKey::TempleOfTime_End_To_Start,
            )],
            Area::LanayruMines_FirstHalf => &[(
                Area::LanayruMines_ToCaves,
                RequirementKey::LanayruMines_ToCaves_To_FirstHalf,
            )],
            Area::LanayruMines_ToCaves => &[(
                Area::LanayruMines_FirstHalf,
                RequirementKey::LanayruMines_FirstHalf_To_ToCaves,
            )],
            Area::LanayruMines_ToDesert => &[(
                Area::LanayruMines_FirstHalf,
                RequirementKey::LanayruMines_FirstHalf_To_ToDesert,
            )],
            Area::LanayruMiningFacilityA_Entry => &[],
            Area::LanayruMiningFacilityA_FirstKeyLockedRoom => &[(
                Area::LanayruMiningFacilityA_SecondRoom,
                RequirementKey::LanayruMiningFacilityA_SecondRoom_To_FirstKeyLockedRoom,
            )],
            Area::LanayruMiningFacilityA_FirstWestRoom => &[(
                Area::LanayruMiningFacilityA_SecondRoom,
                RequirementKey::LanayruMiningFacilityA_SecondRoom_To_FirstWestRoom,
            )],
            Area::LanayruMiningFacilityA_GustBellowsRoom => &[(
                Area::LanayruMiningFacilityA_FirstKeyLockedRoom,
                RequirementKey::LanayruMiningFacilityA_FirstKeyLockedRoom_To_GustBellowsRoom,
            )],
            Area::LanayruMiningFacilityA_MapRoom => &[(
                Area::LanayruMiningFacilityA_FirstWestRoom,
                RequirementKey::LanayruMiningFacilityA_FirstWestRoom_To_MapRoom,
            )],
            Area::LanayruMiningFacilityA_SecondRoom => &[(
                Area::LanayruMiningFacilityA_Entry,
                RequirementKey::LanayruMiningFacilityA_Entry_To_SecondRoom,
            )],
            Area::LanayruMiningFacilityB_AfterLmfBkRoom => &[
                (
                    Area::LanayruMiningFacilityB_InsideLmfBkRoom,
                    RequirementKey::LanayruMiningFacilityB_InsideLmfBkRoom_To_AfterLmfBkRoom,
                ),
                (
                    Area::LanayruMiningFacilityB_NearBossDoor,
                    RequirementKey::LanayruMiningFacilityB_NearBossDoor_To_AfterLmfBkRoom,
                ),
            ],
            Area::LanayruMiningFacilityB_HubRoom => &[],
            Area::LanayruMiningFacilityB_InsideLmfBkRoom => &[
                (
                    Area::LanayruMiningFacilityB_AfterLmfBkRoom,
                    RequirementKey::LanayruMiningFacilityB_AfterLmfBkRoom_To_InsideLmfBkRoom,
                ),
                (
                    Area::LanayruMiningFacilityB_NearBossDoor,
                    RequirementKey::LanayruMiningFacilityB_NearBossDoor_To_InsideLmfBkRoom,
                ),
            ],
            Area::LanayruMiningFacilityB_NearBossDoor => &[
                (
                    Area::LanayruMiningFacilityB_AfterLmfBkRoom,
                    RequirementKey::LanayruMiningFacilityB_AfterLmfBkRoom_To_NearBossDoor,
                ),
                (
                    Area::LanayruMiningFacilityB_WestHub,
                    RequirementKey::LanayruMiningFacilityB_WestHub_To_NearBossDoor,
                ),
            ],
            Area::LanayruMiningFacilityB_NearFirstHubRoomChest => &[(
                Area::LanayruMiningFacilityB_HubRoom,
                RequirementKey::LanayruMiningFacilityB_HubRoom_To_NearFirstHubRoomChest,
            )],
            Area::LanayruMiningFacilityB_WestHub => &[],
            Area::LanayruMiningFacilityBoss_Main => &[],
            Area::LanayruMiningFacilityToToT_BossDoor => &[],
            Area::LanayruMiningFacilityToToT_ToTExit => &[(
                Area::LanayruMiningFacilityToToT_BossDoor,
                RequirementKey::LanayruMiningFacilityToToT_BossDoor_To_ToTExit,
            )],
            Area::InsidePiratesStronghold_Main => &[],
            Area::OutsidePiratesStronghold_InsideSharkhead => &[(
                Area::OutsidePiratesStronghold_Main,
                RequirementKey::OutsidePiratesStronghold_Main_To_InsideSharkhead,
            )],
            Area::OutsidePiratesStronghold_Main => &[],
            Area::SandSea_Main => &[],
            Area::SandSeaDocks_Main => &[(
                Area::SandSeaDocks_ToCaves,
                RequirementKey::SandSeaDocks_ToCaves_To_Main,
            )],
            Area::SandSeaDocks_ToCaves => &[],
            Area::Shipyard_AfterMinecartRide => &[(
                Area::Shipyard_Main,
                RequirementKey::Shipyard_Main_To_AfterMinecartRide,
            )],
            Area::Shipyard_Main => &[],
            Area::ShipyardConstructionBay_Lower => &[(
                Area::ShipyardConstructionBay_Upper,
                RequirementKey::ShipyardConstructionBay_Upper_To_Lower,
            )],
            Area::ShipyardConstructionBay_Upper => &[],
            Area::SkippersRetreat_NextToShack => &[(
                Area::SkippersRetreat_PastDekuBaba,
                RequirementKey::SkippersRetreat_PastDekuBaba_To_NextToShack,
            )],
            Area::SkippersRetreat_PastDekuBaba => &[
                (
                    Area::SkippersRetreat_NextToShack,
                    RequirementKey::SkippersRetreat_NextToShack_To_PastDekuBaba,
                ),
                (
                    Area::SkippersRetreat_PastMoblin,
                    RequirementKey::SkippersRetreat_PastMoblin_To_PastDekuBaba,
                ),
            ],
            Area::SkippersRetreat_PastMoblin => &[(
                Area::SkippersRetreat_Start,
                RequirementKey::SkippersRetreat_Start_To_PastMoblin,
            )],
            Area::SkippersRetreat_Start => &[],
            Area::SkippersShack_Main => &[],
            Area::LanayruSilentRealm_Trial => &[],
            Area::MogmaTurf_Main => &[],
            Area::Sandship_Deck => &[],
            Area::Sandship_PastSpume => &[(
                Area::Sandship_Deck,
                RequirementKey::Sandship_Deck_To_PastSpume,
            )],
            Area::Sandship_SandshipBrig => &[(
                Area::Sandship_PastSpume,
                RequirementKey::Sandship_PastSpume_To_SandshipBrig,
            )],
            Area::SandshipBoss_Main => &[],
            Area::BehindTheTemple_Main => &[],
            Area::HyliasTemple_Main => &[],
            Area::SealedGroundsSpiral_Lower => &[(
                Area::SealedGroundsSpiral_Upper,
                RequirementKey::SealedGroundsSpiral_Upper_To_Lower,
            )],
            Area::SealedGroundsSpiral_Upper => &[(
                Area::SealedGroundsSpiral_Lower,
                RequirementKey::SealedGroundsSpiral_Lower_To_Upper,
            )],
            Area::SealedTemple_Main => &[],
            Area::InsideBambooIsland_Main => &[],
            Area::LumpyPumpkin_Main => &[],
            Area::Sky_BeedleIslandCage => &[
                (
                    Area::Sky_BeedlesSkyHome,
                    RequirementKey::Sky_BeedlesSkyHome_To_BeedleIslandCage,
                ),
                (
                    Area::Sky_Field,
                    RequirementKey::Sky_Field_To_BeedleIslandCage,
                ),
            ],
            Area::Sky_BeedlesSkyHome => &[],
            Area::Sky_Field => &[(
                Area::Sky_OutsideLumpyPumpkin,
                RequirementKey::Sky_OutsideLumpyPumpkin_To_Field,
            )],
            Area::Sky_OutsideLumpyPumpkin => &[(
                Area::Sky_Field,
                RequirementKey::Sky_Field_To_OutsideLumpyPumpkin,
            )],
            Area::SkyKeepEntry_Main => &[],
            Area::SkyloftSilentRealm_Trial => &[],
            Area::BertiesHouse_Main => &[],
            Area::GondosHouse_Main => &[],
            Area::MallarasHouse_Main => &[],
            Area::RupinsHouse_Main => &[],
            Area::Skyloft_OutsideSkyloftVillage => &[(
                Area::Skyloft_CentralOutside,
                RequirementKey::Skyloft_CentralOutside_To_OutsideSkyloftVillage,
            )],
            Area::SparrotsHouse_Main => &[],
            Area::SkyviewBoss_Main => &[],
            Area::SkyviewSpring_Main => &[],
            Area::SkyviewTemple_BossDoorArea => &[(
                Area::SkyviewTemple_MainHub,
                RequirementKey::SkyviewTemple_MainHub_To_BossDoorArea,
            )],
            Area::SkyviewTemple_Entry => &[],
            Area::SkyviewTemple_FirstHub => &[(
                Area::SkyviewTemple_Entry,
                RequirementKey::SkyviewTemple_Entry_To_FirstHub,
            )],
            Area::SkyviewTemple_MainHub => &[(
                Area::SkyviewTemple_FirstHub,
                RequirementKey::SkyviewTemple_FirstHub_To_MainHub,
            )],
            Area::InsideThunderhead_Main => &[],
            Area::IsleOfSongs_Main => &[],
            Area::InsideVolcanoSummit_Main => &[],
            Area::OutsideFireSanctuary_Middle => &[(
                Area::OutsideFireSanctuary_ToInsideSummit,
                RequirementKey::OutsideFireSanctuary_ToInsideSummit_To_Middle,
            )],
            Area::OutsideFireSanctuary_ToFireSanctuary => &[(
                Area::OutsideFireSanctuary_Middle,
                RequirementKey::OutsideFireSanctuary_Middle_To_ToFireSanctuary,
            )],
            Area::OutsideFireSanctuary_ToInsideSummit => &[],
            Area::VolcanoSummitWaterfall_Main => &[],
        }
    }
    pub fn logic_exits(&self) -> &'static [(Area, RequirementKey)] {
        match self {
            Area::AncientCistern_AfterAcGutters => &[
                (
                    Area::AncientCistern_AfterWhipHooks,
                    RequirementKey::AncientCistern_AfterAcGutters_To_AfterWhipHooks,
                ),
                (
                    Area::AncientCistern_MainRoomVines,
                    RequirementKey::AncientCistern_AfterAcGutters_To_MainRoomVines,
                ),
            ],
            Area::AncientCistern_AfterWhipHooks => &[(
                Area::AncientCistern_MainRoomVines,
                RequirementKey::AncientCistern_AfterWhipHooks_To_MainRoomVines,
            )],
            Area::AncientCistern_BeforeBokoKeyDoor => &[(
                Area::AncientCistern_AfterAcGutters,
                RequirementKey::AncientCistern_BeforeBokoKeyDoor_To_AfterAcGutters,
            )],
            Area::AncientCistern_BeforeBossDoor => &[],
            Area::AncientCistern_BehindWaterfall => &[(
                Area::AncientCistern_BeforeBokoKeyDoor,
                RequirementKey::AncientCistern_BehindWaterfall_To_BeforeBokoKeyDoor,
            )],
            Area::AncientCistern_BossKeyChestArea => &[],
            Area::AncientCistern_MainBasement => &[
                (
                    Area::AncientCistern_BeforeBossDoor,
                    RequirementKey::AncientCistern_MainBasement_To_BeforeBossDoor,
                ),
                (
                    Area::AncientCistern_BossKeyChestArea,
                    RequirementKey::AncientCistern_MainBasement_To_BossKeyChestArea,
                ),
                (
                    Area::AncientCistern_SpiderThread,
                    RequirementKey::AncientCistern_MainBasement_To_SpiderThread,
                ),
            ],
            Area::AncientCistern_MainHub => &[
                (
                    Area::AncientCistern_AfterWhipHooks,
                    RequirementKey::AncientCistern_MainHub_To_AfterWhipHooks,
                ),
                (
                    Area::AncientCistern_BeforeBossDoor,
                    RequirementKey::AncientCistern_MainHub_To_BeforeBossDoor,
                ),
                (
                    Area::AncientCistern_BehindWaterfall,
                    RequirementKey::AncientCistern_MainHub_To_BehindWaterfall,
                ),
                (
                    Area::AncientCistern_MainBasement,
                    RequirementKey::AncientCistern_MainHub_To_MainBasement,
                ),
                (
                    Area::AncientCistern_SpiderThread,
                    RequirementKey::AncientCistern_MainHub_To_SpiderThread,
                ),
                (
                    Area::AncientCistern_WhipChestRoom,
                    RequirementKey::AncientCistern_MainHub_To_WhipChestRoom,
                ),
            ],
            Area::AncientCistern_MainRoomVines => &[(
                Area::AncientCistern_AfterWhipHooks,
                RequirementKey::AncientCistern_MainRoomVines_To_AfterWhipHooks,
            )],
            Area::AncientCistern_SpiderThread => &[
                (
                    Area::AncientCistern_BossKeyChestArea,
                    RequirementKey::AncientCistern_SpiderThread_To_BossKeyChestArea,
                ),
                (
                    Area::AncientCistern_MainBasement,
                    RequirementKey::AncientCistern_SpiderThread_To_MainBasement,
                ),
            ],
            Area::AncientCistern_WhipChestRoom => &[],
            Area::AncientCisternBoss_Main => &[],
            Area::AncientCisternCandleRoom_Main => &[],
            Area::BatreauxHouse_Main => &[],
            Area::BeedlesShop_Main => &[],
            Area::Bazaar_Main => &[],
            Area::ParrowAndOriellesHouse_Main => &[],
            Area::PeatricesHouse_Main => &[],
            Area::PipersHouse_Main => &[],
            Area::Skyloft_CentralOutside => &[
                (
                    Area::Skyloft_OutsideGoddessStatue,
                    RequirementKey::Skyloft_CentralOutside_To_OutsideGoddessStatue,
                ),
                (
                    Area::Skyloft_OutsideSkyloftVillage,
                    RequirementKey::Skyloft_CentralOutside_To_OutsideSkyloftVillage,
                ),
                (
                    Area::Skyloft_ToSkyKeep,
                    RequirementKey::Skyloft_CentralOutside_To_ToSkyKeep,
                ),
                (
                    Area::Skyloft_WaterfallCaveCrystals,
                    RequirementKey::Skyloft_CentralOutside_To_WaterfallCaveCrystals,
                ),
            ],
            Area::Skyloft_PastWaterfallCave => &[(
                Area::Skyloft_WaterfallCaveCrystals,
                RequirementKey::Skyloft_PastWaterfallCave_To_WaterfallCaveCrystals,
            )],
            Area::Skyloft_ToSkyKeep => &[(
                Area::Skyloft_CentralOutside,
                RequirementKey::Skyloft_ToSkyKeep_To_CentralOutside,
            )],
            Area::Skyloft_WaterfallCaveCrystals => &[],
            Area::WaterfallCave_Main => &[],
            Area::WrynasHouse_Main => &[],
            Area::EarthTemple_AfterBallRolling => &[(
                Area::EarthTemple_BossDoorArea,
                RequirementKey::EarthTemple_AfterBallRolling_To_BossDoorArea,
            )],
            Area::EarthTemple_BallRolling => &[(
                Area::EarthTemple_AfterBallRolling,
                RequirementKey::EarthTemple_BallRolling_To_AfterBallRolling,
            )],
            Area::EarthTemple_BossDoorArea => &[],
            Area::EarthTemple_Entrance => &[(
                Area::EarthTemple_BallRolling,
                RequirementKey::EarthTemple_Entrance_To_BallRolling,
            )],
            Area::EarthTempleBoss_Main => &[],
            Area::EarthTempleSpring_Main => &[],
            Area::EldinSilentRealm_Trial => &[],
            Area::EldinVolcano_FirstRoom => &[
                (
                    Area::EldinVolcano_PreMogmaTurf,
                    RequirementKey::EldinVolcano_FirstRoom_To_PreMogmaTurf,
                ),
                (
                    Area::EldinVolcano_VolcanoAscent,
                    RequirementKey::EldinVolcano_FirstRoom_To_VolcanoAscent,
                ),
            ],
            Area::EldinVolcano_HotCaveArea => &[(
                Area::EldinVolcano_SandSlide,
                RequirementKey::EldinVolcano_HotCaveArea_To_SandSlide,
            )],
            Area::EldinVolcano_NearThrillDigger => &[
                (
                    Area::EldinVolcano_OutsideEt,
                    RequirementKey::EldinVolcano_NearThrillDigger_To_OutsideEt,
                ),
                (
                    Area::EldinVolcano_VolcanoAscent,
                    RequirementKey::EldinVolcano_NearThrillDigger_To_VolcanoAscent,
                ),
            ],
            Area::EldinVolcano_OutsideEt => &[(
                Area::EldinVolcano_HotCaveArea,
                RequirementKey::EldinVolcano_OutsideEt_To_HotCaveArea,
            )],
            Area::EldinVolcano_PastMogmaTurf => &[
                (
                    Area::EldinVolcano_PreMogmaTurf,
                    RequirementKey::EldinVolcano_PastMogmaTurf_To_PreMogmaTurf,
                ),
                (
                    Area::EldinVolcano_VolcanoAscent,
                    RequirementKey::EldinVolcano_PastMogmaTurf_To_VolcanoAscent,
                ),
            ],
            Area::EldinVolcano_PastSlide => &[
                (
                    Area::EldinVolcano_HotCaveArea,
                    RequirementKey::EldinVolcano_PastSlide_To_HotCaveArea,
                ),
                (
                    Area::EldinVolcano_VolcanoAscent,
                    RequirementKey::EldinVolcano_PastSlide_To_VolcanoAscent,
                ),
            ],
            Area::EldinVolcano_PreMogmaTurf => &[
                (
                    Area::EldinVolcano_FirstRoom,
                    RequirementKey::EldinVolcano_PreMogmaTurf_To_FirstRoom,
                ),
                (
                    Area::EldinVolcano_PastMogmaTurf,
                    RequirementKey::EldinVolcano_PreMogmaTurf_To_PastMogmaTurf,
                ),
            ],
            Area::EldinVolcano_SandSlide => &[(
                Area::EldinVolcano_PastSlide,
                RequirementKey::EldinVolcano_SandSlide_To_PastSlide,
            )],
            Area::EldinVolcano_VolcanoAscent => &[
                (
                    Area::EldinVolcano_FirstRoom,
                    RequirementKey::EldinVolcano_VolcanoAscent_To_FirstRoom,
                ),
                (
                    Area::EldinVolcano_NearThrillDigger,
                    RequirementKey::EldinVolcano_VolcanoAscent_To_NearThrillDigger,
                ),
                (
                    Area::EldinVolcano_PastSlide,
                    RequirementKey::EldinVolcano_VolcanoAscent_To_PastSlide,
                ),
            ],
            Area::ThrillDiggerCave_Main => &[],
            Area::FaronSilentRealm_Trial => &[],
            Area::DeepWoods_Entry => &[(
                Area::DeepWoods_PastBeehive,
                RequirementKey::DeepWoods_Entry_To_PastBeehive,
            )],
            Area::DeepWoods_PastBeehive => &[(
                Area::DeepWoods_Entry,
                RequirementKey::DeepWoods_PastBeehive_To_Entry,
            )],
            Area::FaronWoods_ClawshotTargetBranch => &[],
            Area::FaronWoods_Entry => &[(
                Area::FaronWoods_Main,
                RequirementKey::FaronWoods_Entry_To_Main,
            )],
            Area::FaronWoods_GreatTreePlatforms => &[],
            Area::FaronWoods_GreatTreeTop => &[
                (
                    Area::FaronWoods_ClawshotTargetBranch,
                    RequirementKey::FaronWoods_GreatTreeTop_To_ClawshotTargetBranch,
                ),
                (
                    Area::FaronWoods_GreatTreePlatforms,
                    RequirementKey::FaronWoods_GreatTreeTop_To_GreatTreePlatforms,
                ),
            ],
            Area::FaronWoods_Main => &[
                (
                    Area::FaronWoods_ClawshotTargetBranch,
                    RequirementKey::FaronWoods_Main_To_ClawshotTargetBranch,
                ),
                (
                    Area::FaronWoods_Entry,
                    RequirementKey::FaronWoods_Main_To_Entry,
                ),
                (
                    Area::FaronWoods_GreatTreePlatforms,
                    RequirementKey::FaronWoods_Main_To_GreatTreePlatforms,
                ),
            ],
            Area::GreatTree_Entry => &[(
                Area::GreatTree_Lower,
                RequirementKey::GreatTree_Entry_To_Lower,
            )],
            Area::GreatTree_Lower => &[
                (
                    Area::GreatTree_Entry,
                    RequirementKey::GreatTree_Lower_To_Entry,
                ),
                (
                    Area::GreatTree_Middle,
                    RequirementKey::GreatTree_Lower_To_Middle,
                ),
                (
                    Area::GreatTree_PastPlatforms,
                    RequirementKey::GreatTree_Lower_To_PastPlatforms,
                ),
            ],
            Area::GreatTree_Middle => &[
                (
                    Area::GreatTree_Lower,
                    RequirementKey::GreatTree_Middle_To_Lower,
                ),
                (
                    Area::GreatTree_PastPlatforms,
                    RequirementKey::GreatTree_Middle_To_PastPlatforms,
                ),
            ],
            Area::GreatTree_PastPlatforms => &[(
                Area::GreatTree_Lower,
                RequirementKey::GreatTree_PastPlatforms_To_Lower,
            )],
            Area::GreatTree_Upper => &[(
                Area::GreatTree_Middle,
                RequirementKey::GreatTree_Upper_To_Middle,
            )],
            Area::FireSanctuaryA_Entry => &[(
                Area::FireSanctuaryA_PastFirstWaterPlant,
                RequirementKey::FireSanctuaryA_Entry_To_PastFirstWaterPlant,
            )],
            Area::FireSanctuaryA_InFrontOfBossDoor => &[(
                Area::FireSanctuaryA_UpperStaircaseRoom,
                RequirementKey::FireSanctuaryA_InFrontOfBossDoor_To_UpperStaircaseRoom,
            )],
            Area::FireSanctuaryA_PastFirstWaterPlant => &[],
            Area::FireSanctuaryA_PrePlatsArea => &[(
                Area::FireSanctuaryA_InFrontOfBossDoor,
                RequirementKey::FireSanctuaryA_PrePlatsArea_To_InFrontOfBossDoor,
            )],
            Area::FireSanctuaryA_UpperStaircaseRoom => &[(
                Area::FireSanctuaryA_InFrontOfBossDoor,
                RequirementKey::FireSanctuaryA_UpperStaircaseRoom_To_InFrontOfBossDoor,
            )],
            Area::FireSanctuaryB_AfterDoubleMagmanosFight => &[(
                Area::FireSanctuaryB_UnderDoubleMagmanosFight,
                RequirementKey::FireSanctuaryB_AfterDoubleMagmanosFight_To_UnderDoubleMagmanosFight,
            )],
            Area::FireSanctuaryB_FirstOutsideSection => &[(
                Area::FireSanctuaryB_PastSecondRoomWithWaterFruit,
                RequirementKey::FireSanctuaryB_FirstOutsideSection_To_PastSecondRoomWithWaterFruit,
            )],
            Area::FireSanctuaryB_LastTrappedMogmaArea => &[],
            Area::FireSanctuaryB_PastSecondRoomWithWaterFruit => &[(
                Area::FireSanctuaryB_WaterFruitRoom,
                RequirementKey::FireSanctuaryB_PastSecondRoomWithWaterFruit_To_WaterFruitRoom,
            )],
            Area::FireSanctuaryB_UnderDoubleMagmanosFight => &[(
                Area::FireSanctuaryB_LastTrappedMogmaArea,
                RequirementKey::FireSanctuaryB_UnderDoubleMagmanosFight_To_LastTrappedMogmaArea,
            )],
            Area::FireSanctuaryB_WaterFruitRoom => &[(
                Area::FireSanctuaryB_AfterDoubleMagmanosFight,
                RequirementKey::FireSanctuaryB_WaterFruitRoom_To_AfterDoubleMagmanosFight,
            )],
            Area::FireSanctuaryBoss_Main => &[],
            Area::FireSanctuaryFlameRoom_Main => &[],
            Area::InsideGoddessStatue_Main => &[],
            Area::KnightAcademy_AboveZeldasRoom => &[(
                Area::KnightAcademy_Main,
                RequirementKey::KnightAcademy_AboveZeldasRoom_To_Main,
            )],
            Area::KnightAcademy_Main => &[],
            Area::Skyloft_OutsideGoddessStatue => &[(
                Area::Skyloft_CentralOutside,
                RequirementKey::Skyloft_OutsideGoddessStatue_To_CentralOutside,
            )],
            Area::SparringHall_Main => &[],
            Area::FaroresLair_Main => &[],
            Area::FloriaWaterfall_Main => &[],
            Area::LakeFloria_Entry => &[(
                Area::LakeFloria_StatueSpot,
                RequirementKey::LakeFloria_Entry_To_StatueSpot,
            )],
            Area::LakeFloria_StatueSpot => &[(
                Area::LakeFloria_ToFaroresLair,
                RequirementKey::LakeFloria_StatueSpot_To_ToFaroresLair,
            )],
            Area::LakeFloria_ToFaroresLair => &[(
                Area::LakeFloria_StatueSpot,
                RequirementKey::LakeFloria_ToFaroresLair_To_StatueSpot,
            )],
            Area::LanayruCaves_Main => &[(
                Area::LanayruCaves_ToSandSea,
                RequirementKey::LanayruCaves_Main_To_ToSandSea,
            )],
            Area::LanayruCaves_ToSandSea => &[(
                Area::LanayruCaves_Main,
                RequirementKey::LanayruCaves_ToSandSea_To_Main,
            )],
            Area::FireNode_End => &[],
            Area::FireNode_Main => &[(Area::FireNode_End, RequirementKey::FireNode_Main_To_End)],
            Area::LanayruDesert_HookBeetleArea => &[
                (
                    Area::LanayruDesert_PastToT,
                    RequirementKey::LanayruDesert_HookBeetleArea_To_PastToT,
                ),
                (
                    Area::LanayruDesert_SandOasis,
                    RequirementKey::LanayruDesert_HookBeetleArea_To_SandOasis,
                ),
            ],
            Area::LanayruDesert_PastToT => &[(
                Area::LanayruDesert_HookBeetleArea,
                RequirementKey::LanayruDesert_PastToT_To_HookBeetleArea,
            )],
            Area::LanayruDesert_SandOasis => &[(
                Area::LanayruDesert_HookBeetleArea,
                RequirementKey::LanayruDesert_SandOasis_To_HookBeetleArea,
            )],
            Area::LightningNode_Main => &[],
            Area::TempleOfTime_AfterLmf => &[(
                Area::TempleOfTime_NearGossipStone,
                RequirementKey::TempleOfTime_AfterLmf_To_NearGossipStone,
            )],
            Area::TempleOfTime_End => &[(
                Area::TempleOfTime_Start,
                RequirementKey::TempleOfTime_End_To_Start,
            )],
            Area::TempleOfTime_NearCube => &[(
                Area::TempleOfTime_End,
                RequirementKey::TempleOfTime_NearCube_To_End,
            )],
            Area::TempleOfTime_NearGossipStone => &[(
                Area::TempleOfTime_NearCube,
                RequirementKey::TempleOfTime_NearGossipStone_To_NearCube,
            )],
            Area::TempleOfTime_Start => &[(
                Area::TempleOfTime_NearGossipStone,
                RequirementKey::TempleOfTime_Start_To_NearGossipStone,
            )],
            Area::LanayruMines_FirstHalf => &[
                (
                    Area::LanayruMines_ToCaves,
                    RequirementKey::LanayruMines_FirstHalf_To_ToCaves,
                ),
                (
                    Area::LanayruMines_ToDesert,
                    RequirementKey::LanayruMines_FirstHalf_To_ToDesert,
                ),
            ],
            Area::LanayruMines_ToCaves => &[(
                Area::LanayruMines_FirstHalf,
                RequirementKey::LanayruMines_ToCaves_To_FirstHalf,
            )],
            Area::LanayruMines_ToDesert => &[],
            Area::LanayruMiningFacilityA_Entry => &[(
                Area::LanayruMiningFacilityA_SecondRoom,
                RequirementKey::LanayruMiningFacilityA_Entry_To_SecondRoom,
            )],
            Area::LanayruMiningFacilityA_FirstKeyLockedRoom => &[(
                Area::LanayruMiningFacilityA_GustBellowsRoom,
                RequirementKey::LanayruMiningFacilityA_FirstKeyLockedRoom_To_GustBellowsRoom,
            )],
            Area::LanayruMiningFacilityA_FirstWestRoom => &[(
                Area::LanayruMiningFacilityA_MapRoom,
                RequirementKey::LanayruMiningFacilityA_FirstWestRoom_To_MapRoom,
            )],
            Area::LanayruMiningFacilityA_GustBellowsRoom => &[],
            Area::LanayruMiningFacilityA_MapRoom => &[],
            Area::LanayruMiningFacilityA_SecondRoom => &[
                (
                    Area::LanayruMiningFacilityA_FirstKeyLockedRoom,
                    RequirementKey::LanayruMiningFacilityA_SecondRoom_To_FirstKeyLockedRoom,
                ),
                (
                    Area::LanayruMiningFacilityA_FirstWestRoom,
                    RequirementKey::LanayruMiningFacilityA_SecondRoom_To_FirstWestRoom,
                ),
            ],
            Area::LanayruMiningFacilityB_AfterLmfBkRoom => &[
                (
                    Area::LanayruMiningFacilityB_InsideLmfBkRoom,
                    RequirementKey::LanayruMiningFacilityB_AfterLmfBkRoom_To_InsideLmfBkRoom,
                ),
                (
                    Area::LanayruMiningFacilityB_NearBossDoor,
                    RequirementKey::LanayruMiningFacilityB_AfterLmfBkRoom_To_NearBossDoor,
                ),
            ],
            Area::LanayruMiningFacilityB_HubRoom => &[(
                Area::LanayruMiningFacilityB_NearFirstHubRoomChest,
                RequirementKey::LanayruMiningFacilityB_HubRoom_To_NearFirstHubRoomChest,
            )],
            Area::LanayruMiningFacilityB_InsideLmfBkRoom => &[(
                Area::LanayruMiningFacilityB_AfterLmfBkRoom,
                RequirementKey::LanayruMiningFacilityB_InsideLmfBkRoom_To_AfterLmfBkRoom,
            )],
            Area::LanayruMiningFacilityB_NearBossDoor => &[
                (
                    Area::LanayruMiningFacilityB_AfterLmfBkRoom,
                    RequirementKey::LanayruMiningFacilityB_NearBossDoor_To_AfterLmfBkRoom,
                ),
                (
                    Area::LanayruMiningFacilityB_InsideLmfBkRoom,
                    RequirementKey::LanayruMiningFacilityB_NearBossDoor_To_InsideLmfBkRoom,
                ),
            ],
            Area::LanayruMiningFacilityB_NearFirstHubRoomChest => &[],
            Area::LanayruMiningFacilityB_WestHub => &[(
                Area::LanayruMiningFacilityB_NearBossDoor,
                RequirementKey::LanayruMiningFacilityB_WestHub_To_NearBossDoor,
            )],
            Area::LanayruMiningFacilityBoss_Main => &[],
            Area::LanayruMiningFacilityToToT_BossDoor => &[(
                Area::LanayruMiningFacilityToToT_ToTExit,
                RequirementKey::LanayruMiningFacilityToToT_BossDoor_To_ToTExit,
            )],
            Area::LanayruMiningFacilityToToT_ToTExit => &[],
            Area::InsidePiratesStronghold_Main => &[],
            Area::OutsidePiratesStronghold_InsideSharkhead => &[],
            Area::OutsidePiratesStronghold_Main => &[(
                Area::OutsidePiratesStronghold_InsideSharkhead,
                RequirementKey::OutsidePiratesStronghold_Main_To_InsideSharkhead,
            )],
            Area::SandSea_Main => &[],
            Area::SandSeaDocks_Main => &[],
            Area::SandSeaDocks_ToCaves => &[(
                Area::SandSeaDocks_Main,
                RequirementKey::SandSeaDocks_ToCaves_To_Main,
            )],
            Area::Shipyard_AfterMinecartRide => &[],
            Area::Shipyard_Main => &[(
                Area::Shipyard_AfterMinecartRide,
                RequirementKey::Shipyard_Main_To_AfterMinecartRide,
            )],
            Area::ShipyardConstructionBay_Lower => &[],
            Area::ShipyardConstructionBay_Upper => &[(
                Area::ShipyardConstructionBay_Lower,
                RequirementKey::ShipyardConstructionBay_Upper_To_Lower,
            )],
            Area::SkippersRetreat_NextToShack => &[(
                Area::SkippersRetreat_PastDekuBaba,
                RequirementKey::SkippersRetreat_NextToShack_To_PastDekuBaba,
            )],
            Area::SkippersRetreat_PastDekuBaba => &[(
                Area::SkippersRetreat_NextToShack,
                RequirementKey::SkippersRetreat_PastDekuBaba_To_NextToShack,
            )],
            Area::SkippersRetreat_PastMoblin => &[(
                Area::SkippersRetreat_PastDekuBaba,
                RequirementKey::SkippersRetreat_PastMoblin_To_PastDekuBaba,
            )],
            Area::SkippersRetreat_Start => &[(
                Area::SkippersRetreat_PastMoblin,
                RequirementKey::SkippersRetreat_Start_To_PastMoblin,
            )],
            Area::SkippersShack_Main => &[],
            Area::LanayruSilentRealm_Trial => &[],
            Area::MogmaTurf_Main => &[],
            Area::Sandship_Deck => &[(
                Area::Sandship_PastSpume,
                RequirementKey::Sandship_Deck_To_PastSpume,
            )],
            Area::Sandship_PastSpume => &[(
                Area::Sandship_SandshipBrig,
                RequirementKey::Sandship_PastSpume_To_SandshipBrig,
            )],
            Area::Sandship_SandshipBrig => &[],
            Area::SandshipBoss_Main => &[],
            Area::BehindTheTemple_Main => &[],
            Area::HyliasTemple_Main => &[],
            Area::SealedGroundsSpiral_Lower => &[(
                Area::SealedGroundsSpiral_Upper,
                RequirementKey::SealedGroundsSpiral_Lower_To_Upper,
            )],
            Area::SealedGroundsSpiral_Upper => &[(
                Area::SealedGroundsSpiral_Lower,
                RequirementKey::SealedGroundsSpiral_Upper_To_Lower,
            )],
            Area::SealedTemple_Main => &[],
            Area::InsideBambooIsland_Main => &[],
            Area::LumpyPumpkin_Main => &[],
            Area::Sky_BeedleIslandCage => &[],
            Area::Sky_BeedlesSkyHome => &[(
                Area::Sky_BeedleIslandCage,
                RequirementKey::Sky_BeedlesSkyHome_To_BeedleIslandCage,
            )],
            Area::Sky_Field => &[
                (
                    Area::Sky_BeedleIslandCage,
                    RequirementKey::Sky_Field_To_BeedleIslandCage,
                ),
                (
                    Area::Sky_OutsideLumpyPumpkin,
                    RequirementKey::Sky_Field_To_OutsideLumpyPumpkin,
                ),
            ],
            Area::Sky_OutsideLumpyPumpkin => &[(
                Area::Sky_Field,
                RequirementKey::Sky_OutsideLumpyPumpkin_To_Field,
            )],
            Area::SkyKeepEntry_Main => &[],
            Area::SkyloftSilentRealm_Trial => &[],
            Area::BertiesHouse_Main => &[],
            Area::GondosHouse_Main => &[],
            Area::MallarasHouse_Main => &[],
            Area::RupinsHouse_Main => &[],
            Area::Skyloft_OutsideSkyloftVillage => &[
                (
                    Area::Skyloft_CentralOutside,
                    RequirementKey::Skyloft_OutsideSkyloftVillage_To_CentralOutside,
                ),
                (
                    Area::Skyloft_PastWaterfallCave,
                    RequirementKey::Skyloft_OutsideSkyloftVillage_To_PastWaterfallCave,
                ),
            ],
            Area::SparrotsHouse_Main => &[],
            Area::SkyviewBoss_Main => &[],
            Area::SkyviewSpring_Main => &[],
            Area::SkyviewTemple_BossDoorArea => &[],
            Area::SkyviewTemple_Entry => &[(
                Area::SkyviewTemple_FirstHub,
                RequirementKey::SkyviewTemple_Entry_To_FirstHub,
            )],
            Area::SkyviewTemple_FirstHub => &[(
                Area::SkyviewTemple_MainHub,
                RequirementKey::SkyviewTemple_FirstHub_To_MainHub,
            )],
            Area::SkyviewTemple_MainHub => &[(
                Area::SkyviewTemple_BossDoorArea,
                RequirementKey::SkyviewTemple_MainHub_To_BossDoorArea,
            )],
            Area::InsideThunderhead_Main => &[],
            Area::IsleOfSongs_Main => &[],
            Area::InsideVolcanoSummit_Main => &[],
            Area::OutsideFireSanctuary_Middle => &[(
                Area::OutsideFireSanctuary_ToFireSanctuary,
                RequirementKey::OutsideFireSanctuary_Middle_To_ToFireSanctuary,
            )],
            Area::OutsideFireSanctuary_ToFireSanctuary => &[],
            Area::OutsideFireSanctuary_ToInsideSummit => &[(
                Area::OutsideFireSanctuary_Middle,
                RequirementKey::OutsideFireSanctuary_ToInsideSummit_To_Middle,
            )],
            Area::VolcanoSummitWaterfall_Main => &[],
        }
    }
    pub fn locations(&self) -> &'static [Location] {
        match self {
            Area::AncientCistern_AfterAcGutters => &[],
            Area::AncientCistern_AfterWhipHooks => &[Location::AncientCistern_ChestAfterWhipHooks],
            Area::AncientCistern_BeforeBokoKeyDoor => &[Location::AncientCistern_Bokoblin],
            Area::AncientCistern_BeforeBossDoor => &[],
            Area::AncientCistern_BehindWaterfall => {
                &[Location::AncientCistern_ChestBehindTheWaterfall]
            }
            Area::AncientCistern_BossKeyChestArea => &[Location::AncientCistern_BossKeyChest],
            Area::AncientCistern_MainBasement => &[],
            Area::AncientCistern_MainHub => &[Location::AncientCistern_ChestInEastPart],
            Area::AncientCistern_MainRoomVines => &[Location::AncientCistern_ChestNearVines],
            Area::AncientCistern_SpiderThread => &[],
            Area::AncientCistern_WhipChestRoom => &[Location::AncientCistern_Whip],
            Area::AncientCisternBoss_Main => &[Location::AncientCistern_KoloktosHeartContainer],
            Area::AncientCisternCandleRoom_Main => &[Location::AncientCistern_FaroresFlame],
            Area::BatreauxHouse_Main => &[
                Location::Batreaux_10Crystals,
                Location::Batreaux_30Crystals,
                Location::Batreaux_30CrystalsChest,
                Location::Batreaux_40Crystals,
                Location::Batreaux_5Crystals,
                Location::Batreaux_50Crystals,
                Location::Batreaux_70Crystals,
                Location::Batreaux_70CrystalsSecondReward,
                Location::Batreaux_80Crystals,
            ],
            Area::BeedlesShop_Main => &[
                Location::Beedle_1000RupeeItem,
                Location::Beedle_1200RupeeItem,
                Location::Beedle_1600RupeeItem,
                Location::Beedle_300RupeeItem,
                Location::Beedle_50RupeeItem,
                Location::Beedle_600RupeeItem,
                Location::Beedle_800RupeeItem,
                Location::Beedle_First100RupeeItem,
                Location::Beedle_Second100RupeeItem,
                Location::Beedle_Third100RupeeItem,
            ],
            Area::Bazaar_Main => &[
                Location::CentralSkyloft_BazaarGoddessChest,
                Location::CentralSkyloft_PotionLadysGift,
            ],
            Area::ParrowAndOriellesHouse_Main => {
                &[Location::CentralSkyloft_CrystalInOrielleAndParrowsHouse]
            }
            Area::PeatricesHouse_Main => &[Location::CentralSkyloft_PeaterPeatricesCrystals],
            Area::PipersHouse_Main => &[],
            Area::Skyloft_CentralOutside => &[
                Location::CentralSkyloft_CrystalBetweenWoodenPlanks,
                Location::CentralSkyloft_CrystalOnLightTower,
                Location::CentralSkyloft_CrystalOnWaterfallIsland,
                Location::CentralSkyloft_CrystalOnWestCliff,
                Location::CentralSkyloft_FloatingIslandGoddessChest,
                Location::CentralSkyloft_FloatingIslandGossipStone,
                Location::CentralSkyloft_ItemInBirdNest,
                Location::CentralSkyloft_ParrowsCrystals,
                Location::CentralSkyloft_ParrowsGift,
                Location::CentralSkyloft_ShedChest,
                Location::CentralSkyloft_ShedGoddessChest,
                Location::CentralSkyloft_WaterfallGoddessChest,
                Location::CentralSkyloft_WestCliffGoddessChest,
            ],
            Area::Skyloft_PastWaterfallCave => &[],
            Area::Skyloft_ToSkyKeep => &[],
            Area::Skyloft_WaterfallCaveCrystals => &[
                Location::CentralSkyloft_CrystalAfterWaterfallCave,
                Location::CentralSkyloft_CrystalInLoftwingPrison,
            ],
            Area::WaterfallCave_Main => &[
                Location::CentralSkyloft_WaterfallCaveFirstChest,
                Location::CentralSkyloft_WaterfallCaveSecondChest,
            ],
            Area::WrynasHouse_Main => &[Location::CentralSkyloft_WrynasCrystals],
            Area::EarthTemple_AfterBallRolling => &[Location::EarthTemple_ChestGuardedByLizalfos],
            Area::EarthTemple_BallRolling => &[
                Location::EarthTemple_BombBag,
                Location::EarthTemple_ChestLeftOfMainRoomBridge,
                Location::EarthTemple_ChestBehindBombableRock,
                Location::EarthTemple_ChestInWestRoom,
                Location::EarthTemple_LeddsGift,
            ],
            Area::EarthTemple_BossDoorArea => &[Location::EarthTemple_BossKeyChest],
            Area::EarthTemple_Entrance => &[Location::EarthTemple_VentChest],
            Area::EarthTempleBoss_Main => &[Location::EarthTemple_ScalderaHeartContainer],
            Area::EarthTempleSpring_Main => &[Location::EarthTemple_AmberTablet],
            Area::EldinSilentRealm_Trial => &[Location::EldinSilentRealm_FireshieldEarrings],
            Area::EldinVolcano_FirstRoom => {
                &[Location::EldinVolcano_ChestBehindBombableWallInFirstRoom]
            }
            Area::EldinVolcano_HotCaveArea => &[],
            Area::EldinVolcano_NearThrillDigger => &[],
            Area::EldinVolcano_OutsideEt => &[
                Location::EldinVolcano_DiggingSpotBehindBoulderOnSandySlope,
                Location::EldinVolcano_DiggingSpotBelowTower,
                Location::EldinVolcano_DiggingSpotInFrontOfEarthTemple,
                Location::EldinVolcano_GossipStoneNextToEarthTemple,
            ],
            Area::EldinVolcano_PastMogmaTurf => &[],
            Area::EldinVolcano_PastSlide => &[Location::EldinVolcano_DiggingSpotAfterDrainingLava],
            Area::EldinVolcano_PreMogmaTurf => &[
                Location::EldinVolcano_ChestAfterCrawlspace,
                Location::EldinVolcano_ChestBehindBombableWallNearCliff,
                Location::EldinVolcano_ItemOnCliff,
            ],
            Area::EldinVolcano_SandSlide => &[Location::EldinVolcano_DiggingSpotAfterVents],
            Area::EldinVolcano_VolcanoAscent => {
                &[Location::EldinVolcano_ChestBehindBombableWallNearVolcanoAscent]
            }
            Area::ThrillDiggerCave_Main => &[Location::EldinVolcano_GossipStoneInThrillDiggerCave],
            Area::FaronSilentRealm_Trial => &[Location::FaronSilentRealm_WaterScale],
            Area::DeepWoods_Entry => &[],
            Area::DeepWoods_PastBeehive => &[Location::FaronWoods_DeepWoodsChest],
            Area::FaronWoods_ClawshotTargetBranch => &[],
            Area::FaronWoods_Entry => &[],
            Area::FaronWoods_GreatTreePlatforms => &[],
            Area::FaronWoods_GreatTreeTop => &[],
            Area::FaronWoods_Main => &[
                Location::FaronWoods_ChestBehindBombableRocksNearErla,
                Location::FaronWoods_ItemBehindBombableRock,
                Location::FaronWoods_ItemOnTree,
                Location::FaronWoods_Slingshot,
            ],
            Area::GreatTree_Entry => &[],
            Area::GreatTree_Lower => &[],
            Area::GreatTree_Middle => &[Location::FaronWoods_ChestInsideGreatTree],
            Area::GreatTree_PastPlatforms => &[],
            Area::GreatTree_Upper => &[],
            Area::FireSanctuaryA_Entry => &[],
            Area::FireSanctuaryA_InFrontOfBossDoor => &[],
            Area::FireSanctuaryA_PastFirstWaterPlant => &[Location::FireSanctuary_ChestInFirstRoom],
            Area::FireSanctuaryA_PrePlatsArea => &[Location::FireSanctuary_PlatsChest],
            Area::FireSanctuaryA_UpperStaircaseRoom => &[
                Location::FireSanctuary_BossKeyChest,
                Location::FireSanctuary_ChestInStaircaseRoom,
            ],
            Area::FireSanctuaryB_AfterDoubleMagmanosFight => &[Location::FireSanctuary_MogmaMitts],
            Area::FireSanctuaryB_FirstOutsideSection => &[
                Location::FireSanctuary_ChestInSecondRoom,
                Location::FireSanctuary_ChestOnBalcony,
            ],
            Area::FireSanctuaryB_LastTrappedMogmaArea => &[
                Location::FireSanctuary_ChestAfterBombableWall,
                Location::FireSanctuary_ChestAfterSecondTrappedMogma,
            ],
            Area::FireSanctuaryB_PastSecondRoomWithWaterFruit => {
                &[Location::FireSanctuary_ChestNearFirstTrappedMogma]
            }
            Area::FireSanctuaryB_UnderDoubleMagmanosFight => &[],
            Area::FireSanctuaryB_WaterFruitRoom => &[
                Location::FireSanctuary_FirstChestInWaterFruitRoom,
                Location::FireSanctuary_SecondChestInWaterFruitRoom,
            ],
            Area::FireSanctuaryBoss_Main => &[Location::FireSanctuary_GhirahimHeartContainer],
            Area::FireSanctuaryFlameRoom_Main => &[Location::FireSanctuary_DinsFlame],
            Area::InsideGoddessStatue_Main => &[Location::KnightAcademy_ChestInGoddessStatue],
            Area::KnightAcademy_AboveZeldasRoom => &[],
            Area::KnightAcademy_Main => &[
                Location::KnightAcademy_CawlinsLetter,
                Location::KnightAcademy_CrystalInKnightAcademyPlant,
                Location::KnightAcademy_CrystalInLinksRoom,
                Location::KnightAcademy_CrystalInZeldasRoom,
                Location::KnightAcademy_FledgesCrystals,
                Location::KnightAcademy_FledgesGift,
                Location::KnightAcademy_GhostPipitsCrystals,
                Location::KnightAcademy_InZeldasCloset,
                Location::KnightAcademy_OwlansCrystals,
            ],
            Area::Skyloft_OutsideGoddessStatue => &[
                Location::KnightAcademy_ChestNearGoddessStatue,
                Location::KnightAcademy_OwlansGift,
                Location::KnightAcademy_PumpkinArchery600Points,
            ],
            Area::SparringHall_Main => &[
                Location::KnightAcademy_CrystalInSparringHall,
                Location::KnightAcademy_SparringHallChest,
            ],
            Area::FaroresLair_Main => &[
                Location::LakeFloria_DragonLairEastChest,
                Location::LakeFloria_DragonLairSouthChest,
            ],
            Area::FloriaWaterfall_Main => &[],
            Area::LakeFloria_Entry => &[],
            Area::LakeFloria_StatueSpot => &[Location::LakeFloria_LakeFloriaChest],
            Area::LakeFloria_ToFaroresLair => &[],
            Area::LanayruCaves_Main => &[
                Location::LanayruCaves_Chest,
                Location::LanayruCaves_GolosGift,
                Location::LanayruCaves_GossipStoneInCenter,
            ],
            Area::LanayruCaves_ToSandSea => &[],
            Area::FireNode_End => &[
                Location::LanayruDesert_FireNodeLeftEndingChest,
                Location::LanayruDesert_FireNodeRightEndingChest,
            ],
            Area::FireNode_Main => &[
                Location::LanayruDesert_FireNodeFirstSmallChest,
                Location::LanayruDesert_FireNodeSecondSmallChest,
                Location::LanayruDesert_FireNodeShortcutChest,
            ],
            Area::LanayruDesert_HookBeetleArea => &[
                Location::LanayruDesert_ChestNearHookBeetleFight,
                Location::LanayruDesert_ChestNearPartyWheel,
                Location::LanayruDesert_HookBeetleFight,
            ],
            Area::LanayruDesert_PastToT => &[
                Location::LanayruDesert_ChestOnPlatformNearFireNode,
                Location::LanayruDesert_ChestOnPlatformNearLightningNode,
                Location::LanayruDesert_ChestOnTopOfLanayruMiningFacility,
                Location::LanayruDesert_SecretPassagewayChest,
            ],
            Area::LanayruDesert_SandOasis => &[Location::LanayruDesert_ChestNearSandOasis],
            Area::LightningNode_Main => &[
                Location::LanayruDesert_LightningNodeFirstChest,
                Location::LanayruDesert_LightningNodeRaisedChestNearGenerator,
                Location::LanayruDesert_LightningNodeSecondChest,
            ],
            Area::TempleOfTime_AfterLmf => &[],
            Area::TempleOfTime_End => &[],
            Area::TempleOfTime_NearCube => &[],
            Area::TempleOfTime_NearGossipStone => {
                &[Location::LanayruDesert_GossipStoneInTempleOfTimeArea]
            }
            Area::TempleOfTime_Start => &[],
            Area::LanayruMines_FirstHalf => &[
                Location::LanayruMines_ChestAtTheEndOfMines,
                Location::LanayruMines_ChestBehindFirstLanding,
                Location::LanayruMines_ChestBehindStatue,
                Location::LanayruMines_ChestNearFirstTimeshiftStone,
            ],
            Area::LanayruMines_ToCaves => &[],
            Area::LanayruMines_ToDesert => &[],
            Area::LanayruMiningFacilityA_Entry => {
                &[Location::LanayruMiningFacility_ChestBehindBars]
            }
            Area::LanayruMiningFacilityA_FirstKeyLockedRoom => {
                &[Location::LanayruMiningFacility_ChestInKeyLockedRoom]
            }
            Area::LanayruMiningFacilityA_FirstWestRoom => {
                &[Location::LanayruMiningFacility_ChestInFirstWestRoom]
            }
            Area::LanayruMiningFacilityA_GustBellowsRoom => &[
                Location::LanayruMiningFacility_ChestInsideGustBellowsRoom,
                Location::LanayruMiningFacility_GustBellows,
            ],
            Area::LanayruMiningFacilityA_MapRoom => {
                &[Location::LanayruMiningFacility_ChestAfterArmosFight]
            }
            Area::LanayruMiningFacilityA_SecondRoom => &[],
            Area::LanayruMiningFacilityB_AfterLmfBkRoom => {
                &[Location::LanayruMiningFacility_ShortcutChestInMainHub]
            }
            Area::LanayruMiningFacilityB_HubRoom => &[],
            Area::LanayruMiningFacilityB_InsideLmfBkRoom => {
                &[Location::LanayruMiningFacility_BossKeyChest]
            }
            Area::LanayruMiningFacilityB_NearBossDoor => &[],
            Area::LanayruMiningFacilityB_NearFirstHubRoomChest => {
                &[Location::LanayruMiningFacility_FirstChestInHubRoom]
            }
            Area::LanayruMiningFacilityB_WestHub => &[
                Location::LanayruMiningFacility_ChestBehindFirstCrawlspace,
                Location::LanayruMiningFacility_ChestInSpikeMaze,
            ],
            Area::LanayruMiningFacilityBoss_Main => {
                &[Location::LanayruMiningFacility_MolderachHeartContainer]
            }
            Area::LanayruMiningFacilityToToT_BossDoor => &[],
            Area::LanayruMiningFacilityToToT_ToTExit => {
                &[Location::LanayruMiningFacility_GoddessHarp]
            }
            Area::InsidePiratesStronghold_Main => &[
                Location::LanayruSandSea_PirateStrongholdFirstChest,
                Location::LanayruSandSea_PirateStrongholdSecondChest,
                Location::LanayruSandSea_PirateStrongholdThirdChest,
            ],
            Area::OutsidePiratesStronghold_InsideSharkhead => &[],
            Area::OutsidePiratesStronghold_Main => &[],
            Area::SandSea_Main => &[],
            Area::SandSeaDocks_Main => &[],
            Area::SandSeaDocks_ToCaves => &[],
            Area::Shipyard_AfterMinecartRide => &[],
            Area::Shipyard_Main => &[
                Location::LanayruSandSea_GossipStoneInShipyard,
                Location::LanayruSandSea_RicketyCoasterHeartStoppingTrackIn105,
            ],
            Area::ShipyardConstructionBay_Lower => &[],
            Area::ShipyardConstructionBay_Upper => &[],
            Area::SkippersRetreat_NextToShack => {
                &[Location::LanayruSandSea_SkippersRetreatSkydiveChest]
            }
            Area::SkippersRetreat_PastDekuBaba => {
                &[Location::LanayruSandSea_SkippersRetreatChestOnTopOfCactiPillar]
            }
            Area::SkippersRetreat_PastMoblin => {
                &[Location::LanayruSandSea_SkippersRetreatChestAfterMoblin]
            }
            Area::SkippersRetreat_Start => &[],
            Area::SkippersShack_Main => &[Location::LanayruSandSea_SkippersRetreatChestInShack],
            Area::LanayruSilentRealm_Trial => &[Location::LanayruSilentRealm_Clawshots],
            Area::MogmaTurf_Main => &[
                Location::MogmaTurf_ChestBehindBombableWallAtEntrance,
                Location::MogmaTurf_ChestBehindBombableWallInFireMaze,
                Location::MogmaTurf_DiggingMittsFight,
                Location::MogmaTurf_FreeFallChest,
                Location::MogmaTurf_SandSlideChest,
            ],
            Area::Sandship_Deck => &[
                Location::Sandship_BossKeyChest,
                Location::Sandship_Bow,
                Location::Sandship_ChestAtTheStern,
                Location::Sandship_ChestBefore4DoorCorridor,
            ],
            Area::Sandship_PastSpume => &[Location::Sandship_ChestBehindCombinationLock],
            Area::Sandship_SandshipBrig => &[
                Location::Sandship_RobotInBrigsReward,
                Location::Sandship_TreasureRoomFifthChest,
                Location::Sandship_TreasureRoomFirstChest,
                Location::Sandship_TreasureRoomFourthChest,
                Location::Sandship_TreasureRoomSecondChest,
                Location::Sandship_TreasureRoomThirdChest,
            ],
            Area::SandshipBoss_Main => &[
                Location::Sandship_NayrusFlame,
                Location::Sandship_TentalusHeartContainer,
            ],
            Area::BehindTheTemple_Main => &[Location::SealedGrounds_GorkosGoddessWallReward],
            Area::HyliasTemple_Main => &[Location::SealedGrounds_ZeldasBlessing],
            Area::SealedGroundsSpiral_Lower => &[],
            Area::SealedGroundsSpiral_Upper => &[],
            Area::SealedTemple_Main => &[
                Location::SealedGrounds_ChestInsideSealedTemple,
                Location::SealedGrounds_SongFromImpa,
            ],
            Area::InsideBambooIsland_Main => &[Location::Sky_GossipStoneInsideBambooIsland],
            Area::LumpyPumpkin_Main => &[
                Location::Sky_CrystalInsideLumpyPumpkin,
                Location::Sky_LumpyPumpkinChandelier,
                Location::Sky_LumpyPumpkinHarpMinigame,
            ],
            Area::Sky_BeedleIslandCage => &[Location::Sky_BeedlesIslandCageGoddessChest],
            Area::Sky_BeedlesSkyHome => &[
                Location::Sky_BeedlesCrystals,
                Location::Sky_CrystalOnBeedlesShip,
            ],
            Area::Sky_Field => &[
                Location::Sky_BambooIslandGoddessChest,
                Location::Sky_BeedlesIslandGoddessChest,
                Location::Sky_ChestInBreakableBoulderNearFunFunIsland,
                Location::Sky_ChestInBreakableBoulderNearLumpyPumpkin,
                Location::Sky_DodohsCrystals,
                Location::Sky_FunFunIslandMinigame500Rupees,
                Location::Sky_GoddessChestInCaveOnIslandNextToBambooIsland,
                Location::Sky_GoddessChestInsideVolcanicIsland,
                Location::Sky_GoddessChestOnIslandClosestToFaronPillar,
                Location::Sky_GoddessChestOnIslandNextToBambooIsland,
                Location::Sky_GoddessChestOutsideVolcanicIsland,
                Location::Sky_GoddessChestUnderFunFunIsland,
                Location::Sky_GossipStoneInVolcanicIsland,
                Location::Sky_LumpyPumpkinGoddessChestOnTheRoof,
                Location::Sky_NortheastIslandCageGoddessChest,
                Location::Sky_NortheastIslandGoddessChestBehindBombableRocks,
                Location::Sky_OriellesCrystals,
                Location::Sky_SouthwestTripleIslandCageGoddessChest,
                Location::Sky_SouthwestTripleIslandLowerGoddessChest,
                Location::Sky_SouthwestTripleIslandUpperGoddessChest,
            ],
            Area::Sky_OutsideLumpyPumpkin => &[
                Location::Sky_CrystalOutsideLumpyPumpkin,
                Location::Sky_KinasCrystals,
                Location::Sky_LumpyPumpkinOutsideGoddessChest,
            ],
            Area::SkyKeepEntry_Main => &[
                Location::SkyKeep_ChestAfterDreadfuse,
                Location::SkyKeep_FirstChest,
            ],
            Area::SkyloftSilentRealm_Trial => &[Location::SkyloftSilentRealm_StoneOfTrials],
            Area::BertiesHouse_Main => &[Location::SkyloftVillage_BertiesCrystals],
            Area::GondosHouse_Main => &[],
            Area::MallarasHouse_Main => &[Location::SkyloftVillage_MallarasCrystals],
            Area::RupinsHouse_Main => &[],
            Area::Skyloft_OutsideSkyloftVillage => {
                &[Location::SkyloftVillage_CrystalNearPumpkinPatch]
            }
            Area::SparrotsHouse_Main => &[Location::SkyloftVillage_SparrotsCrystals],
            Area::SkyviewBoss_Main => &[Location::Skyview_GhirahimHeartContainer],
            Area::SkyviewSpring_Main => &[Location::Skyview_RubyTablet],
            Area::SkyviewTemple_BossDoorArea => &[
                Location::Skyview_BossKeyChest,
                Location::Skyview_ChestNearBossDoor,
            ],
            Area::SkyviewTemple_Entry => &[],
            Area::SkyviewTemple_FirstHub => &[
                Location::Skyview_ChestBehindTwoEyes,
                Location::Skyview_ChestOnTreeBranch,
                Location::Skyview_DiggingSpotInCrawlspace,
            ],
            Area::SkyviewTemple_MainHub => &[
                Location::Skyview_Beetle,
                Location::Skyview_ChestBehindThreeEyes,
                Location::Skyview_ItemBehindBars,
            ],
            Area::InsideThunderhead_Main => &[
                Location::Thunderhead_BugHeaven10BugsIn3Minutes,
                Location::Thunderhead_BugHeavenGoddessChest,
                Location::Thunderhead_EastIslandChest,
                Location::Thunderhead_EastIslandGoddessChest,
                Location::Thunderhead_FirstGoddessChestOnMogmaMittsIsland,
                Location::Thunderhead_GoddessChestOnTopOfIsleOfSongs,
                Location::Thunderhead_GoddessChestOutsideIsleOfSongs,
                Location::Thunderhead_SongFromLevias,
            ],
            Area::IsleOfSongs_Main => &[
                Location::Thunderhead_IsleOfSongsDinsPower,
                Location::Thunderhead_IsleOfSongsFaroresCourage,
                Location::Thunderhead_IsleOfSongsNayrusWisdom,
            ],
            Area::InsideVolcanoSummit_Main => &[
                Location::VolcanoSummit_BokoBasePouchChest,
                Location::VolcanoSummit_SmallChestInVolcanoSummit,
            ],
            Area::OutsideFireSanctuary_Middle => &[
                Location::VolcanoSummit_GossipStoneOutsideFireSanctuary,
                Location::VolcanoSummit_ItemBehindDigging,
            ],
            Area::OutsideFireSanctuary_ToFireSanctuary => &[],
            Area::OutsideFireSanctuary_ToInsideSummit => &[],
            Area::VolcanoSummitWaterfall_Main => &[
                Location::VolcanoSummit_ChestBehindBombableWallInWaterfallArea,
                Location::VolcanoSummit_GossipStoneInWaterfallArea,
            ],
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Location {
    AncientCistern_ChestAfterWhipHooks,
    AncientCistern_Bokoblin,
    AncientCistern_ChestBehindTheWaterfall,
    AncientCistern_BossKeyChest,
    AncientCistern_ChestInEastPart,
    AncientCistern_ChestNearVines,
    AncientCistern_Whip,
    AncientCistern_KoloktosHeartContainer,
    AncientCistern_FaroresFlame,
    Batreaux_10Crystals,
    Batreaux_30Crystals,
    Batreaux_30CrystalsChest,
    Batreaux_40Crystals,
    Batreaux_5Crystals,
    Batreaux_50Crystals,
    Batreaux_70Crystals,
    Batreaux_70CrystalsSecondReward,
    Batreaux_80Crystals,
    Beedle_1000RupeeItem,
    Beedle_1200RupeeItem,
    Beedle_1600RupeeItem,
    Beedle_300RupeeItem,
    Beedle_50RupeeItem,
    Beedle_600RupeeItem,
    Beedle_800RupeeItem,
    Beedle_First100RupeeItem,
    Beedle_Second100RupeeItem,
    Beedle_Third100RupeeItem,
    CentralSkyloft_BazaarGoddessChest,
    CentralSkyloft_PotionLadysGift,
    CentralSkyloft_CrystalInOrielleAndParrowsHouse,
    CentralSkyloft_PeaterPeatricesCrystals,
    CentralSkyloft_CrystalBetweenWoodenPlanks,
    CentralSkyloft_CrystalOnLightTower,
    CentralSkyloft_CrystalOnWaterfallIsland,
    CentralSkyloft_CrystalOnWestCliff,
    CentralSkyloft_FloatingIslandGoddessChest,
    CentralSkyloft_FloatingIslandGossipStone,
    CentralSkyloft_ItemInBirdNest,
    CentralSkyloft_ParrowsCrystals,
    CentralSkyloft_ParrowsGift,
    CentralSkyloft_ShedChest,
    CentralSkyloft_ShedGoddessChest,
    CentralSkyloft_WaterfallGoddessChest,
    CentralSkyloft_WestCliffGoddessChest,
    CentralSkyloft_CrystalAfterWaterfallCave,
    CentralSkyloft_CrystalInLoftwingPrison,
    CentralSkyloft_WaterfallCaveFirstChest,
    CentralSkyloft_WaterfallCaveSecondChest,
    CentralSkyloft_WrynasCrystals,
    EarthTemple_ChestGuardedByLizalfos,
    EarthTemple_BombBag,
    EarthTemple_ChestLeftOfMainRoomBridge,
    EarthTemple_ChestBehindBombableRock,
    EarthTemple_ChestInWestRoom,
    EarthTemple_LeddsGift,
    EarthTemple_BossKeyChest,
    EarthTemple_VentChest,
    EarthTemple_ScalderaHeartContainer,
    EarthTemple_AmberTablet,
    EldinSilentRealm_FireshieldEarrings,
    EldinVolcano_ChestBehindBombableWallInFirstRoom,
    EldinVolcano_DiggingSpotBehindBoulderOnSandySlope,
    EldinVolcano_DiggingSpotBelowTower,
    EldinVolcano_DiggingSpotInFrontOfEarthTemple,
    EldinVolcano_GossipStoneNextToEarthTemple,
    EldinVolcano_DiggingSpotAfterDrainingLava,
    EldinVolcano_ChestAfterCrawlspace,
    EldinVolcano_ChestBehindBombableWallNearCliff,
    EldinVolcano_ItemOnCliff,
    EldinVolcano_DiggingSpotAfterVents,
    EldinVolcano_ChestBehindBombableWallNearVolcanoAscent,
    EldinVolcano_GossipStoneInThrillDiggerCave,
    FaronSilentRealm_WaterScale,
    FaronWoods_DeepWoodsChest,
    FaronWoods_ChestBehindBombableRocksNearErla,
    FaronWoods_ItemBehindBombableRock,
    FaronWoods_ItemOnTree,
    FaronWoods_Slingshot,
    FaronWoods_ChestInsideGreatTree,
    FireSanctuary_ChestInFirstRoom,
    FireSanctuary_PlatsChest,
    FireSanctuary_BossKeyChest,
    FireSanctuary_ChestInStaircaseRoom,
    FireSanctuary_MogmaMitts,
    FireSanctuary_ChestInSecondRoom,
    FireSanctuary_ChestOnBalcony,
    FireSanctuary_ChestAfterBombableWall,
    FireSanctuary_ChestAfterSecondTrappedMogma,
    FireSanctuary_ChestNearFirstTrappedMogma,
    FireSanctuary_FirstChestInWaterFruitRoom,
    FireSanctuary_SecondChestInWaterFruitRoom,
    FireSanctuary_GhirahimHeartContainer,
    FireSanctuary_DinsFlame,
    KnightAcademy_ChestInGoddessStatue,
    KnightAcademy_CawlinsLetter,
    KnightAcademy_CrystalInKnightAcademyPlant,
    KnightAcademy_CrystalInLinksRoom,
    KnightAcademy_CrystalInZeldasRoom,
    KnightAcademy_FledgesCrystals,
    KnightAcademy_FledgesGift,
    KnightAcademy_GhostPipitsCrystals,
    KnightAcademy_InZeldasCloset,
    KnightAcademy_OwlansCrystals,
    KnightAcademy_ChestNearGoddessStatue,
    KnightAcademy_OwlansGift,
    KnightAcademy_PumpkinArchery600Points,
    KnightAcademy_CrystalInSparringHall,
    KnightAcademy_SparringHallChest,
    LakeFloria_DragonLairEastChest,
    LakeFloria_DragonLairSouthChest,
    LakeFloria_LakeFloriaChest,
    LanayruCaves_Chest,
    LanayruCaves_GolosGift,
    LanayruCaves_GossipStoneInCenter,
    LanayruDesert_FireNodeLeftEndingChest,
    LanayruDesert_FireNodeRightEndingChest,
    LanayruDesert_FireNodeFirstSmallChest,
    LanayruDesert_FireNodeSecondSmallChest,
    LanayruDesert_FireNodeShortcutChest,
    LanayruDesert_ChestNearHookBeetleFight,
    LanayruDesert_ChestNearPartyWheel,
    LanayruDesert_HookBeetleFight,
    LanayruDesert_ChestOnPlatformNearFireNode,
    LanayruDesert_ChestOnPlatformNearLightningNode,
    LanayruDesert_ChestOnTopOfLanayruMiningFacility,
    LanayruDesert_SecretPassagewayChest,
    LanayruDesert_ChestNearSandOasis,
    LanayruDesert_LightningNodeFirstChest,
    LanayruDesert_LightningNodeRaisedChestNearGenerator,
    LanayruDesert_LightningNodeSecondChest,
    LanayruDesert_GossipStoneInTempleOfTimeArea,
    LanayruMines_ChestAtTheEndOfMines,
    LanayruMines_ChestBehindFirstLanding,
    LanayruMines_ChestBehindStatue,
    LanayruMines_ChestNearFirstTimeshiftStone,
    LanayruMiningFacility_ChestBehindBars,
    LanayruMiningFacility_ChestInKeyLockedRoom,
    LanayruMiningFacility_ChestInFirstWestRoom,
    LanayruMiningFacility_ChestInsideGustBellowsRoom,
    LanayruMiningFacility_GustBellows,
    LanayruMiningFacility_ChestAfterArmosFight,
    LanayruMiningFacility_ShortcutChestInMainHub,
    LanayruMiningFacility_BossKeyChest,
    LanayruMiningFacility_FirstChestInHubRoom,
    LanayruMiningFacility_ChestBehindFirstCrawlspace,
    LanayruMiningFacility_ChestInSpikeMaze,
    LanayruMiningFacility_MolderachHeartContainer,
    LanayruMiningFacility_GoddessHarp,
    LanayruSandSea_PirateStrongholdFirstChest,
    LanayruSandSea_PirateStrongholdSecondChest,
    LanayruSandSea_PirateStrongholdThirdChest,
    LanayruSandSea_GossipStoneInShipyard,
    LanayruSandSea_RicketyCoasterHeartStoppingTrackIn105,
    LanayruSandSea_SkippersRetreatSkydiveChest,
    LanayruSandSea_SkippersRetreatChestOnTopOfCactiPillar,
    LanayruSandSea_SkippersRetreatChestAfterMoblin,
    LanayruSandSea_SkippersRetreatChestInShack,
    LanayruSilentRealm_Clawshots,
    MogmaTurf_ChestBehindBombableWallAtEntrance,
    MogmaTurf_ChestBehindBombableWallInFireMaze,
    MogmaTurf_DiggingMittsFight,
    MogmaTurf_FreeFallChest,
    MogmaTurf_SandSlideChest,
    Sandship_BossKeyChest,
    Sandship_Bow,
    Sandship_ChestAtTheStern,
    Sandship_ChestBefore4DoorCorridor,
    Sandship_ChestBehindCombinationLock,
    Sandship_RobotInBrigsReward,
    Sandship_TreasureRoomFifthChest,
    Sandship_TreasureRoomFirstChest,
    Sandship_TreasureRoomFourthChest,
    Sandship_TreasureRoomSecondChest,
    Sandship_TreasureRoomThirdChest,
    Sandship_NayrusFlame,
    Sandship_TentalusHeartContainer,
    SealedGrounds_GorkosGoddessWallReward,
    SealedGrounds_ZeldasBlessing,
    SealedGrounds_ChestInsideSealedTemple,
    SealedGrounds_SongFromImpa,
    Sky_GossipStoneInsideBambooIsland,
    Sky_CrystalInsideLumpyPumpkin,
    Sky_LumpyPumpkinChandelier,
    Sky_LumpyPumpkinHarpMinigame,
    Sky_BeedlesIslandCageGoddessChest,
    Sky_BeedlesCrystals,
    Sky_CrystalOnBeedlesShip,
    Sky_BambooIslandGoddessChest,
    Sky_BeedlesIslandGoddessChest,
    Sky_ChestInBreakableBoulderNearFunFunIsland,
    Sky_ChestInBreakableBoulderNearLumpyPumpkin,
    Sky_DodohsCrystals,
    Sky_FunFunIslandMinigame500Rupees,
    Sky_GoddessChestInCaveOnIslandNextToBambooIsland,
    Sky_GoddessChestInsideVolcanicIsland,
    Sky_GoddessChestOnIslandClosestToFaronPillar,
    Sky_GoddessChestOnIslandNextToBambooIsland,
    Sky_GoddessChestOutsideVolcanicIsland,
    Sky_GoddessChestUnderFunFunIsland,
    Sky_GossipStoneInVolcanicIsland,
    Sky_LumpyPumpkinGoddessChestOnTheRoof,
    Sky_NortheastIslandCageGoddessChest,
    Sky_NortheastIslandGoddessChestBehindBombableRocks,
    Sky_OriellesCrystals,
    Sky_SouthwestTripleIslandCageGoddessChest,
    Sky_SouthwestTripleIslandLowerGoddessChest,
    Sky_SouthwestTripleIslandUpperGoddessChest,
    Sky_CrystalOutsideLumpyPumpkin,
    Sky_KinasCrystals,
    Sky_LumpyPumpkinOutsideGoddessChest,
    SkyKeep_ChestAfterDreadfuse,
    SkyKeep_FirstChest,
    SkyloftSilentRealm_StoneOfTrials,
    SkyloftVillage_BertiesCrystals,
    SkyloftVillage_MallarasCrystals,
    SkyloftVillage_CrystalNearPumpkinPatch,
    SkyloftVillage_SparrotsCrystals,
    Skyview_GhirahimHeartContainer,
    Skyview_RubyTablet,
    Skyview_BossKeyChest,
    Skyview_ChestNearBossDoor,
    Skyview_ChestBehindTwoEyes,
    Skyview_ChestOnTreeBranch,
    Skyview_DiggingSpotInCrawlspace,
    Skyview_Beetle,
    Skyview_ChestBehindThreeEyes,
    Skyview_ItemBehindBars,
    Thunderhead_BugHeaven10BugsIn3Minutes,
    Thunderhead_BugHeavenGoddessChest,
    Thunderhead_EastIslandChest,
    Thunderhead_EastIslandGoddessChest,
    Thunderhead_FirstGoddessChestOnMogmaMittsIsland,
    Thunderhead_GoddessChestOnTopOfIsleOfSongs,
    Thunderhead_GoddessChestOutsideIsleOfSongs,
    Thunderhead_SongFromLevias,
    Thunderhead_IsleOfSongsDinsPower,
    Thunderhead_IsleOfSongsFaroresCourage,
    Thunderhead_IsleOfSongsNayrusWisdom,
    VolcanoSummit_BokoBasePouchChest,
    VolcanoSummit_SmallChestInVolcanoSummit,
    VolcanoSummit_GossipStoneOutsideFireSanctuary,
    VolcanoSummit_ItemBehindDigging,
    VolcanoSummit_ChestBehindBombableWallInWaterfallArea,
    VolcanoSummit_GossipStoneInWaterfallArea,
}
impl Into<usize> for Location {
    fn into(self) -> usize {
        self as usize
    }
}
impl BitSetCompatible for Location {
    const ALL: &'static [Location] = &[
        Location::AncientCistern_ChestAfterWhipHooks,
        Location::AncientCistern_Bokoblin,
        Location::AncientCistern_ChestBehindTheWaterfall,
        Location::AncientCistern_BossKeyChest,
        Location::AncientCistern_ChestInEastPart,
        Location::AncientCistern_ChestNearVines,
        Location::AncientCistern_Whip,
        Location::AncientCistern_KoloktosHeartContainer,
        Location::AncientCistern_FaroresFlame,
        Location::Batreaux_10Crystals,
        Location::Batreaux_30Crystals,
        Location::Batreaux_30CrystalsChest,
        Location::Batreaux_40Crystals,
        Location::Batreaux_5Crystals,
        Location::Batreaux_50Crystals,
        Location::Batreaux_70Crystals,
        Location::Batreaux_70CrystalsSecondReward,
        Location::Batreaux_80Crystals,
        Location::Beedle_1000RupeeItem,
        Location::Beedle_1200RupeeItem,
        Location::Beedle_1600RupeeItem,
        Location::Beedle_300RupeeItem,
        Location::Beedle_50RupeeItem,
        Location::Beedle_600RupeeItem,
        Location::Beedle_800RupeeItem,
        Location::Beedle_First100RupeeItem,
        Location::Beedle_Second100RupeeItem,
        Location::Beedle_Third100RupeeItem,
        Location::CentralSkyloft_BazaarGoddessChest,
        Location::CentralSkyloft_PotionLadysGift,
        Location::CentralSkyloft_CrystalInOrielleAndParrowsHouse,
        Location::CentralSkyloft_PeaterPeatricesCrystals,
        Location::CentralSkyloft_CrystalBetweenWoodenPlanks,
        Location::CentralSkyloft_CrystalOnLightTower,
        Location::CentralSkyloft_CrystalOnWaterfallIsland,
        Location::CentralSkyloft_CrystalOnWestCliff,
        Location::CentralSkyloft_FloatingIslandGoddessChest,
        Location::CentralSkyloft_FloatingIslandGossipStone,
        Location::CentralSkyloft_ItemInBirdNest,
        Location::CentralSkyloft_ParrowsCrystals,
        Location::CentralSkyloft_ParrowsGift,
        Location::CentralSkyloft_ShedChest,
        Location::CentralSkyloft_ShedGoddessChest,
        Location::CentralSkyloft_WaterfallGoddessChest,
        Location::CentralSkyloft_WestCliffGoddessChest,
        Location::CentralSkyloft_CrystalAfterWaterfallCave,
        Location::CentralSkyloft_CrystalInLoftwingPrison,
        Location::CentralSkyloft_WaterfallCaveFirstChest,
        Location::CentralSkyloft_WaterfallCaveSecondChest,
        Location::CentralSkyloft_WrynasCrystals,
        Location::EarthTemple_ChestGuardedByLizalfos,
        Location::EarthTemple_BombBag,
        Location::EarthTemple_ChestLeftOfMainRoomBridge,
        Location::EarthTemple_ChestBehindBombableRock,
        Location::EarthTemple_ChestInWestRoom,
        Location::EarthTemple_LeddsGift,
        Location::EarthTemple_BossKeyChest,
        Location::EarthTemple_VentChest,
        Location::EarthTemple_ScalderaHeartContainer,
        Location::EarthTemple_AmberTablet,
        Location::EldinSilentRealm_FireshieldEarrings,
        Location::EldinVolcano_ChestBehindBombableWallInFirstRoom,
        Location::EldinVolcano_DiggingSpotBehindBoulderOnSandySlope,
        Location::EldinVolcano_DiggingSpotBelowTower,
        Location::EldinVolcano_DiggingSpotInFrontOfEarthTemple,
        Location::EldinVolcano_GossipStoneNextToEarthTemple,
        Location::EldinVolcano_DiggingSpotAfterDrainingLava,
        Location::EldinVolcano_ChestAfterCrawlspace,
        Location::EldinVolcano_ChestBehindBombableWallNearCliff,
        Location::EldinVolcano_ItemOnCliff,
        Location::EldinVolcano_DiggingSpotAfterVents,
        Location::EldinVolcano_ChestBehindBombableWallNearVolcanoAscent,
        Location::EldinVolcano_GossipStoneInThrillDiggerCave,
        Location::FaronSilentRealm_WaterScale,
        Location::FaronWoods_DeepWoodsChest,
        Location::FaronWoods_ChestBehindBombableRocksNearErla,
        Location::FaronWoods_ItemBehindBombableRock,
        Location::FaronWoods_ItemOnTree,
        Location::FaronWoods_Slingshot,
        Location::FaronWoods_ChestInsideGreatTree,
        Location::FireSanctuary_ChestInFirstRoom,
        Location::FireSanctuary_PlatsChest,
        Location::FireSanctuary_BossKeyChest,
        Location::FireSanctuary_ChestInStaircaseRoom,
        Location::FireSanctuary_MogmaMitts,
        Location::FireSanctuary_ChestInSecondRoom,
        Location::FireSanctuary_ChestOnBalcony,
        Location::FireSanctuary_ChestAfterBombableWall,
        Location::FireSanctuary_ChestAfterSecondTrappedMogma,
        Location::FireSanctuary_ChestNearFirstTrappedMogma,
        Location::FireSanctuary_FirstChestInWaterFruitRoom,
        Location::FireSanctuary_SecondChestInWaterFruitRoom,
        Location::FireSanctuary_GhirahimHeartContainer,
        Location::FireSanctuary_DinsFlame,
        Location::KnightAcademy_ChestInGoddessStatue,
        Location::KnightAcademy_CawlinsLetter,
        Location::KnightAcademy_CrystalInKnightAcademyPlant,
        Location::KnightAcademy_CrystalInLinksRoom,
        Location::KnightAcademy_CrystalInZeldasRoom,
        Location::KnightAcademy_FledgesCrystals,
        Location::KnightAcademy_FledgesGift,
        Location::KnightAcademy_GhostPipitsCrystals,
        Location::KnightAcademy_InZeldasCloset,
        Location::KnightAcademy_OwlansCrystals,
        Location::KnightAcademy_ChestNearGoddessStatue,
        Location::KnightAcademy_OwlansGift,
        Location::KnightAcademy_PumpkinArchery600Points,
        Location::KnightAcademy_CrystalInSparringHall,
        Location::KnightAcademy_SparringHallChest,
        Location::LakeFloria_DragonLairEastChest,
        Location::LakeFloria_DragonLairSouthChest,
        Location::LakeFloria_LakeFloriaChest,
        Location::LanayruCaves_Chest,
        Location::LanayruCaves_GolosGift,
        Location::LanayruCaves_GossipStoneInCenter,
        Location::LanayruDesert_FireNodeLeftEndingChest,
        Location::LanayruDesert_FireNodeRightEndingChest,
        Location::LanayruDesert_FireNodeFirstSmallChest,
        Location::LanayruDesert_FireNodeSecondSmallChest,
        Location::LanayruDesert_FireNodeShortcutChest,
        Location::LanayruDesert_ChestNearHookBeetleFight,
        Location::LanayruDesert_ChestNearPartyWheel,
        Location::LanayruDesert_HookBeetleFight,
        Location::LanayruDesert_ChestOnPlatformNearFireNode,
        Location::LanayruDesert_ChestOnPlatformNearLightningNode,
        Location::LanayruDesert_ChestOnTopOfLanayruMiningFacility,
        Location::LanayruDesert_SecretPassagewayChest,
        Location::LanayruDesert_ChestNearSandOasis,
        Location::LanayruDesert_LightningNodeFirstChest,
        Location::LanayruDesert_LightningNodeRaisedChestNearGenerator,
        Location::LanayruDesert_LightningNodeSecondChest,
        Location::LanayruDesert_GossipStoneInTempleOfTimeArea,
        Location::LanayruMines_ChestAtTheEndOfMines,
        Location::LanayruMines_ChestBehindFirstLanding,
        Location::LanayruMines_ChestBehindStatue,
        Location::LanayruMines_ChestNearFirstTimeshiftStone,
        Location::LanayruMiningFacility_ChestBehindBars,
        Location::LanayruMiningFacility_ChestInKeyLockedRoom,
        Location::LanayruMiningFacility_ChestInFirstWestRoom,
        Location::LanayruMiningFacility_ChestInsideGustBellowsRoom,
        Location::LanayruMiningFacility_GustBellows,
        Location::LanayruMiningFacility_ChestAfterArmosFight,
        Location::LanayruMiningFacility_ShortcutChestInMainHub,
        Location::LanayruMiningFacility_BossKeyChest,
        Location::LanayruMiningFacility_FirstChestInHubRoom,
        Location::LanayruMiningFacility_ChestBehindFirstCrawlspace,
        Location::LanayruMiningFacility_ChestInSpikeMaze,
        Location::LanayruMiningFacility_MolderachHeartContainer,
        Location::LanayruMiningFacility_GoddessHarp,
        Location::LanayruSandSea_PirateStrongholdFirstChest,
        Location::LanayruSandSea_PirateStrongholdSecondChest,
        Location::LanayruSandSea_PirateStrongholdThirdChest,
        Location::LanayruSandSea_GossipStoneInShipyard,
        Location::LanayruSandSea_RicketyCoasterHeartStoppingTrackIn105,
        Location::LanayruSandSea_SkippersRetreatSkydiveChest,
        Location::LanayruSandSea_SkippersRetreatChestOnTopOfCactiPillar,
        Location::LanayruSandSea_SkippersRetreatChestAfterMoblin,
        Location::LanayruSandSea_SkippersRetreatChestInShack,
        Location::LanayruSilentRealm_Clawshots,
        Location::MogmaTurf_ChestBehindBombableWallAtEntrance,
        Location::MogmaTurf_ChestBehindBombableWallInFireMaze,
        Location::MogmaTurf_DiggingMittsFight,
        Location::MogmaTurf_FreeFallChest,
        Location::MogmaTurf_SandSlideChest,
        Location::Sandship_BossKeyChest,
        Location::Sandship_Bow,
        Location::Sandship_ChestAtTheStern,
        Location::Sandship_ChestBefore4DoorCorridor,
        Location::Sandship_ChestBehindCombinationLock,
        Location::Sandship_RobotInBrigsReward,
        Location::Sandship_TreasureRoomFifthChest,
        Location::Sandship_TreasureRoomFirstChest,
        Location::Sandship_TreasureRoomFourthChest,
        Location::Sandship_TreasureRoomSecondChest,
        Location::Sandship_TreasureRoomThirdChest,
        Location::Sandship_NayrusFlame,
        Location::Sandship_TentalusHeartContainer,
        Location::SealedGrounds_GorkosGoddessWallReward,
        Location::SealedGrounds_ZeldasBlessing,
        Location::SealedGrounds_ChestInsideSealedTemple,
        Location::SealedGrounds_SongFromImpa,
        Location::Sky_GossipStoneInsideBambooIsland,
        Location::Sky_CrystalInsideLumpyPumpkin,
        Location::Sky_LumpyPumpkinChandelier,
        Location::Sky_LumpyPumpkinHarpMinigame,
        Location::Sky_BeedlesIslandCageGoddessChest,
        Location::Sky_BeedlesCrystals,
        Location::Sky_CrystalOnBeedlesShip,
        Location::Sky_BambooIslandGoddessChest,
        Location::Sky_BeedlesIslandGoddessChest,
        Location::Sky_ChestInBreakableBoulderNearFunFunIsland,
        Location::Sky_ChestInBreakableBoulderNearLumpyPumpkin,
        Location::Sky_DodohsCrystals,
        Location::Sky_FunFunIslandMinigame500Rupees,
        Location::Sky_GoddessChestInCaveOnIslandNextToBambooIsland,
        Location::Sky_GoddessChestInsideVolcanicIsland,
        Location::Sky_GoddessChestOnIslandClosestToFaronPillar,
        Location::Sky_GoddessChestOnIslandNextToBambooIsland,
        Location::Sky_GoddessChestOutsideVolcanicIsland,
        Location::Sky_GoddessChestUnderFunFunIsland,
        Location::Sky_GossipStoneInVolcanicIsland,
        Location::Sky_LumpyPumpkinGoddessChestOnTheRoof,
        Location::Sky_NortheastIslandCageGoddessChest,
        Location::Sky_NortheastIslandGoddessChestBehindBombableRocks,
        Location::Sky_OriellesCrystals,
        Location::Sky_SouthwestTripleIslandCageGoddessChest,
        Location::Sky_SouthwestTripleIslandLowerGoddessChest,
        Location::Sky_SouthwestTripleIslandUpperGoddessChest,
        Location::Sky_CrystalOutsideLumpyPumpkin,
        Location::Sky_KinasCrystals,
        Location::Sky_LumpyPumpkinOutsideGoddessChest,
        Location::SkyKeep_ChestAfterDreadfuse,
        Location::SkyKeep_FirstChest,
        Location::SkyloftSilentRealm_StoneOfTrials,
        Location::SkyloftVillage_BertiesCrystals,
        Location::SkyloftVillage_MallarasCrystals,
        Location::SkyloftVillage_CrystalNearPumpkinPatch,
        Location::SkyloftVillage_SparrotsCrystals,
        Location::Skyview_GhirahimHeartContainer,
        Location::Skyview_RubyTablet,
        Location::Skyview_BossKeyChest,
        Location::Skyview_ChestNearBossDoor,
        Location::Skyview_ChestBehindTwoEyes,
        Location::Skyview_ChestOnTreeBranch,
        Location::Skyview_DiggingSpotInCrawlspace,
        Location::Skyview_Beetle,
        Location::Skyview_ChestBehindThreeEyes,
        Location::Skyview_ItemBehindBars,
        Location::Thunderhead_BugHeaven10BugsIn3Minutes,
        Location::Thunderhead_BugHeavenGoddessChest,
        Location::Thunderhead_EastIslandChest,
        Location::Thunderhead_EastIslandGoddessChest,
        Location::Thunderhead_FirstGoddessChestOnMogmaMittsIsland,
        Location::Thunderhead_GoddessChestOnTopOfIsleOfSongs,
        Location::Thunderhead_GoddessChestOutsideIsleOfSongs,
        Location::Thunderhead_SongFromLevias,
        Location::Thunderhead_IsleOfSongsDinsPower,
        Location::Thunderhead_IsleOfSongsFaroresCourage,
        Location::Thunderhead_IsleOfSongsNayrusWisdom,
        Location::VolcanoSummit_BokoBasePouchChest,
        Location::VolcanoSummit_SmallChestInVolcanoSummit,
        Location::VolcanoSummit_GossipStoneOutsideFireSanctuary,
        Location::VolcanoSummit_ItemBehindDigging,
        Location::VolcanoSummit_ChestBehindBombableWallInWaterfallArea,
        Location::VolcanoSummit_GossipStoneInWaterfallArea,
    ];
}
impl Location {
    pub fn name(&self) -> &'static str {
        match self {
            Location::AncientCistern_ChestAfterWhipHooks => "Chest after Whip Hooks",
            Location::AncientCistern_Bokoblin => "Bokoblin",
            Location::AncientCistern_ChestBehindTheWaterfall => "Chest behind the Waterfall",
            Location::AncientCistern_BossKeyChest => "Boss Key Chest",
            Location::AncientCistern_ChestInEastPart => "Chest in East Part",
            Location::AncientCistern_ChestNearVines => "Chest near Vines",
            Location::AncientCistern_Whip => "Whip",
            Location::AncientCistern_KoloktosHeartContainer => "Koloktos Heart Container",
            Location::AncientCistern_FaroresFlame => "Farore's Flame",
            Location::Batreaux_10Crystals => "10 Crystals",
            Location::Batreaux_30Crystals => "30 Crystals",
            Location::Batreaux_30CrystalsChest => "30 Crystals Chest",
            Location::Batreaux_40Crystals => "40 Crystals",
            Location::Batreaux_5Crystals => "5 Crystals",
            Location::Batreaux_50Crystals => "50 Crystals",
            Location::Batreaux_70Crystals => "70 Crystals",
            Location::Batreaux_70CrystalsSecondReward => "70 Crystals Second Reward",
            Location::Batreaux_80Crystals => "80 Crystals",
            Location::Beedle_1000RupeeItem => "1000 Rupee Item",
            Location::Beedle_1200RupeeItem => "1200 Rupee Item",
            Location::Beedle_1600RupeeItem => "1600 Rupee Item",
            Location::Beedle_300RupeeItem => "300 Rupee Item",
            Location::Beedle_50RupeeItem => "50 Rupee Item",
            Location::Beedle_600RupeeItem => "600 Rupee Item",
            Location::Beedle_800RupeeItem => "800 Rupee Item",
            Location::Beedle_First100RupeeItem => "First 100 Rupee Item",
            Location::Beedle_Second100RupeeItem => "Second 100 Rupee Item",
            Location::Beedle_Third100RupeeItem => "Third 100 Rupee Item",
            Location::CentralSkyloft_BazaarGoddessChest => "Bazaar Goddess Chest",
            Location::CentralSkyloft_PotionLadysGift => "Potion Lady's Gift",
            Location::CentralSkyloft_CrystalInOrielleAndParrowsHouse => {
                "Crystal in Orielle and Parrow's House"
            }
            Location::CentralSkyloft_PeaterPeatricesCrystals => "Peater/Peatrice's Crystals",
            Location::CentralSkyloft_CrystalBetweenWoodenPlanks => "Crystal between Wooden Planks",
            Location::CentralSkyloft_CrystalOnLightTower => "Crystal on Light Tower",
            Location::CentralSkyloft_CrystalOnWaterfallIsland => "Crystal on Waterfall Island",
            Location::CentralSkyloft_CrystalOnWestCliff => "Crystal on West Cliff",
            Location::CentralSkyloft_FloatingIslandGoddessChest => "Floating Island Goddess Chest",
            Location::CentralSkyloft_FloatingIslandGossipStone => "Floating Island Gossip Stone",
            Location::CentralSkyloft_ItemInBirdNest => "Item in Bird Nest",
            Location::CentralSkyloft_ParrowsCrystals => "Parrow's Crystals",
            Location::CentralSkyloft_ParrowsGift => "Parrow's Gift",
            Location::CentralSkyloft_ShedChest => "Shed Chest",
            Location::CentralSkyloft_ShedGoddessChest => "Shed Goddess Chest",
            Location::CentralSkyloft_WaterfallGoddessChest => "Waterfall Goddess Chest",
            Location::CentralSkyloft_WestCliffGoddessChest => "West Cliff Goddess Chest",
            Location::CentralSkyloft_CrystalAfterWaterfallCave => "Crystal after Waterfall Cave",
            Location::CentralSkyloft_CrystalInLoftwingPrison => "Crystal in Loftwing Prison",
            Location::CentralSkyloft_WaterfallCaveFirstChest => "Waterfall Cave First Chest",
            Location::CentralSkyloft_WaterfallCaveSecondChest => "Waterfall Cave Second Chest",
            Location::CentralSkyloft_WrynasCrystals => "Wryna's Crystals",
            Location::EarthTemple_ChestGuardedByLizalfos => "Chest Guarded by Lizalfos",
            Location::EarthTemple_BombBag => "Bomb Bag",
            Location::EarthTemple_ChestLeftOfMainRoomBridge => "Chest Left of Main Room Bridge",
            Location::EarthTemple_ChestBehindBombableRock => "Chest behind Bombable Rock",
            Location::EarthTemple_ChestInWestRoom => "Chest in West Room",
            Location::EarthTemple_LeddsGift => "Ledd's Gift",
            Location::EarthTemple_BossKeyChest => "Boss Key Chest",
            Location::EarthTemple_VentChest => "Vent Chest",
            Location::EarthTemple_ScalderaHeartContainer => "Scaldera Heart Container",
            Location::EarthTemple_AmberTablet => "Amber Tablet",
            Location::EldinSilentRealm_FireshieldEarrings => "Fireshield Earrings",
            Location::EldinVolcano_ChestBehindBombableWallInFirstRoom => {
                "Chest behind Bombable Wall in First Room"
            }
            Location::EldinVolcano_DiggingSpotBehindBoulderOnSandySlope => {
                "Digging Spot behind Boulder on Sandy Slope"
            }
            Location::EldinVolcano_DiggingSpotBelowTower => "Digging Spot below Tower",
            Location::EldinVolcano_DiggingSpotInFrontOfEarthTemple => {
                "Digging Spot in front of Earth Temple"
            }
            Location::EldinVolcano_GossipStoneNextToEarthTemple => {
                "Gossip Stone next to Earth Temple"
            }
            Location::EldinVolcano_DiggingSpotAfterDrainingLava => {
                "Digging Spot after Draining Lava"
            }
            Location::EldinVolcano_ChestAfterCrawlspace => "Chest after Crawlspace",
            Location::EldinVolcano_ChestBehindBombableWallNearCliff => {
                "Chest behind Bombable Wall near Cliff"
            }
            Location::EldinVolcano_ItemOnCliff => "Item on Cliff",
            Location::EldinVolcano_DiggingSpotAfterVents => "Digging Spot after Vents",
            Location::EldinVolcano_ChestBehindBombableWallNearVolcanoAscent => {
                "Chest behind Bombable Wall near Volcano Ascent"
            }
            Location::EldinVolcano_GossipStoneInThrillDiggerCave => {
                "Gossip Stone in Thrill Digger Cave"
            }
            Location::FaronSilentRealm_WaterScale => "Water Scale",
            Location::FaronWoods_DeepWoodsChest => "Deep Woods Chest",
            Location::FaronWoods_ChestBehindBombableRocksNearErla => {
                "Chest behind Bombable Rocks near Erla"
            }
            Location::FaronWoods_ItemBehindBombableRock => "Item behind Bombable Rock",
            Location::FaronWoods_ItemOnTree => "Item on Tree",
            Location::FaronWoods_Slingshot => "Slingshot",
            Location::FaronWoods_ChestInsideGreatTree => "Chest inside Great Tree",
            Location::FireSanctuary_ChestInFirstRoom => "Chest in First Room",
            Location::FireSanctuary_PlatsChest => "Plats' Chest",
            Location::FireSanctuary_BossKeyChest => "Boss Key Chest",
            Location::FireSanctuary_ChestInStaircaseRoom => "Chest in Staircase Room",
            Location::FireSanctuary_MogmaMitts => "Mogma Mitts",
            Location::FireSanctuary_ChestInSecondRoom => "Chest in Second Room",
            Location::FireSanctuary_ChestOnBalcony => "Chest on Balcony",
            Location::FireSanctuary_ChestAfterBombableWall => "Chest after Bombable Wall",
            Location::FireSanctuary_ChestAfterSecondTrappedMogma => {
                "Chest after Second Trapped Mogma"
            }
            Location::FireSanctuary_ChestNearFirstTrappedMogma => "Chest near First Trapped Mogma",
            Location::FireSanctuary_FirstChestInWaterFruitRoom => "First Chest in Water Fruit Room",
            Location::FireSanctuary_SecondChestInWaterFruitRoom => {
                "Second Chest in Water Fruit Room"
            }
            Location::FireSanctuary_GhirahimHeartContainer => "Ghirahim Heart Container",
            Location::FireSanctuary_DinsFlame => "Din's Flame",
            Location::KnightAcademy_ChestInGoddessStatue => "Chest in Goddess Statue",
            Location::KnightAcademy_CawlinsLetter => "Cawlin's Letter",
            Location::KnightAcademy_CrystalInKnightAcademyPlant => {
                "Crystal in Knight Academy Plant"
            }
            Location::KnightAcademy_CrystalInLinksRoom => "Crystal in Link's Room",
            Location::KnightAcademy_CrystalInZeldasRoom => "Crystal in Zelda's Room",
            Location::KnightAcademy_FledgesCrystals => "Fledge's Crystals",
            Location::KnightAcademy_FledgesGift => "Fledge's Gift",
            Location::KnightAcademy_GhostPipitsCrystals => "Ghost/Pipit's Crystals",
            Location::KnightAcademy_InZeldasCloset => "In Zelda's Closet",
            Location::KnightAcademy_OwlansCrystals => "Owlan's Crystals",
            Location::KnightAcademy_ChestNearGoddessStatue => "Chest near Goddess Statue",
            Location::KnightAcademy_OwlansGift => "Owlan's Gift",
            Location::KnightAcademy_PumpkinArchery600Points => "Pumpkin Archery - 600 Points",
            Location::KnightAcademy_CrystalInSparringHall => "Crystal in Sparring Hall",
            Location::KnightAcademy_SparringHallChest => "Sparring Hall Chest",
            Location::LakeFloria_DragonLairEastChest => "Dragon Lair East Chest",
            Location::LakeFloria_DragonLairSouthChest => "Dragon Lair South Chest",
            Location::LakeFloria_LakeFloriaChest => "Lake Floria Chest",
            Location::LanayruCaves_Chest => "Chest",
            Location::LanayruCaves_GolosGift => "Golo's Gift",
            Location::LanayruCaves_GossipStoneInCenter => "Gossip Stone in Center",
            Location::LanayruDesert_FireNodeLeftEndingChest => "Fire Node - Left Ending Chest",
            Location::LanayruDesert_FireNodeRightEndingChest => "Fire Node - Right Ending Chest",
            Location::LanayruDesert_FireNodeFirstSmallChest => "Fire Node - First Small Chest",
            Location::LanayruDesert_FireNodeSecondSmallChest => "Fire Node - Second Small Chest",
            Location::LanayruDesert_FireNodeShortcutChest => "Fire Node - Shortcut Chest",
            Location::LanayruDesert_ChestNearHookBeetleFight => "Chest near Hook Beetle Fight",
            Location::LanayruDesert_ChestNearPartyWheel => "Chest near Party Wheel",
            Location::LanayruDesert_HookBeetleFight => "Hook Beetle Fight",
            Location::LanayruDesert_ChestOnPlatformNearFireNode => {
                "Chest on Platform near Fire Node"
            }
            Location::LanayruDesert_ChestOnPlatformNearLightningNode => {
                "Chest on Platform near Lightning Node"
            }
            Location::LanayruDesert_ChestOnTopOfLanayruMiningFacility => {
                "Chest on top of Lanayru Mining Facility"
            }
            Location::LanayruDesert_SecretPassagewayChest => "Secret Passageway Chest",
            Location::LanayruDesert_ChestNearSandOasis => "Chest near Sand Oasis",
            Location::LanayruDesert_LightningNodeFirstChest => "Lightning Node - First Chest",
            Location::LanayruDesert_LightningNodeRaisedChestNearGenerator => {
                "Lightning Node - Raised Chest near Generator"
            }
            Location::LanayruDesert_LightningNodeSecondChest => "Lightning Node - Second Chest",
            Location::LanayruDesert_GossipStoneInTempleOfTimeArea => {
                "Gossip Stone in Temple of Time Area"
            }
            Location::LanayruMines_ChestAtTheEndOfMines => "Chest at the End of Mines",
            Location::LanayruMines_ChestBehindFirstLanding => "Chest behind First Landing",
            Location::LanayruMines_ChestBehindStatue => "Chest behind Statue",
            Location::LanayruMines_ChestNearFirstTimeshiftStone => {
                "Chest near First Timeshift Stone"
            }
            Location::LanayruMiningFacility_ChestBehindBars => "Chest behind Bars",
            Location::LanayruMiningFacility_ChestInKeyLockedRoom => "Chest in Key Locked Room",
            Location::LanayruMiningFacility_ChestInFirstWestRoom => "Chest in First West Room",
            Location::LanayruMiningFacility_ChestInsideGustBellowsRoom => {
                "Chest inside Gust Bellows Room"
            }
            Location::LanayruMiningFacility_GustBellows => "Gust Bellows",
            Location::LanayruMiningFacility_ChestAfterArmosFight => "Chest after Armos Fight",
            Location::LanayruMiningFacility_ShortcutChestInMainHub => "Shortcut Chest in Main Hub",
            Location::LanayruMiningFacility_BossKeyChest => "Boss Key Chest",
            Location::LanayruMiningFacility_FirstChestInHubRoom => "First Chest in Hub Room",
            Location::LanayruMiningFacility_ChestBehindFirstCrawlspace => {
                "Chest behind First Crawlspace"
            }
            Location::LanayruMiningFacility_ChestInSpikeMaze => "Chest in Spike Maze",
            Location::LanayruMiningFacility_MolderachHeartContainer => "Molderach Heart Container",
            Location::LanayruMiningFacility_GoddessHarp => "Goddess Harp",
            Location::LanayruSandSea_PirateStrongholdFirstChest => {
                "Pirate Stronghold - First Chest"
            }
            Location::LanayruSandSea_PirateStrongholdSecondChest => {
                "Pirate Stronghold - Second Chest"
            }
            Location::LanayruSandSea_PirateStrongholdThirdChest => {
                "Pirate Stronghold - Third Chest"
            }
            Location::LanayruSandSea_GossipStoneInShipyard => "Gossip Stone in Shipyard",
            Location::LanayruSandSea_RicketyCoasterHeartStoppingTrackIn105 => {
                "Rickety Coaster - Heart Stopping Track in 1'05"
            }
            Location::LanayruSandSea_SkippersRetreatSkydiveChest => {
                "Skipper's Retreat - Skydive Chest"
            }
            Location::LanayruSandSea_SkippersRetreatChestOnTopOfCactiPillar => {
                "Skipper's Retreat - Chest on top of Cacti Pillar"
            }
            Location::LanayruSandSea_SkippersRetreatChestAfterMoblin => {
                "Skipper's Retreat - Chest after Moblin"
            }
            Location::LanayruSandSea_SkippersRetreatChestInShack => {
                "Skipper's Retreat - Chest in Shack"
            }
            Location::LanayruSilentRealm_Clawshots => "Clawshots",
            Location::MogmaTurf_ChestBehindBombableWallAtEntrance => {
                "Chest behind Bombable Wall at Entrance"
            }
            Location::MogmaTurf_ChestBehindBombableWallInFireMaze => {
                "Chest behind Bombable Wall in Fire Maze"
            }
            Location::MogmaTurf_DiggingMittsFight => "Digging Mitts Fight",
            Location::MogmaTurf_FreeFallChest => "Free Fall Chest",
            Location::MogmaTurf_SandSlideChest => "Sand Slide Chest",
            Location::Sandship_BossKeyChest => "Boss Key Chest",
            Location::Sandship_Bow => "Bow",
            Location::Sandship_ChestAtTheStern => "Chest at the Stern",
            Location::Sandship_ChestBefore4DoorCorridor => "Chest before 4-Door Corridor",
            Location::Sandship_ChestBehindCombinationLock => "Chest behind Combination Lock",
            Location::Sandship_RobotInBrigsReward => "Robot in Brig's Reward",
            Location::Sandship_TreasureRoomFifthChest => "Treasure Room Fifth Chest",
            Location::Sandship_TreasureRoomFirstChest => "Treasure Room First Chest",
            Location::Sandship_TreasureRoomFourthChest => "Treasure Room Fourth Chest",
            Location::Sandship_TreasureRoomSecondChest => "Treasure Room Second Chest",
            Location::Sandship_TreasureRoomThirdChest => "Treasure Room Third Chest",
            Location::Sandship_NayrusFlame => "Nayru's Flame",
            Location::Sandship_TentalusHeartContainer => "Tentalus Heart Container",
            Location::SealedGrounds_GorkosGoddessWallReward => "Gorko's Goddess Wall Reward",
            Location::SealedGrounds_ZeldasBlessing => "Zelda's Blessing",
            Location::SealedGrounds_ChestInsideSealedTemple => "Chest inside Sealed Temple",
            Location::SealedGrounds_SongFromImpa => "Song from Impa",
            Location::Sky_GossipStoneInsideBambooIsland => "Gossip Stone inside Bamboo Island",
            Location::Sky_CrystalInsideLumpyPumpkin => "Crystal inside Lumpy Pumpkin",
            Location::Sky_LumpyPumpkinChandelier => "Lumpy Pumpkin - Chandelier",
            Location::Sky_LumpyPumpkinHarpMinigame => "Lumpy Pumpkin Harp Minigame",
            Location::Sky_BeedlesIslandCageGoddessChest => "Beedle's Island Cage Goddess Chest",
            Location::Sky_BeedlesCrystals => "Beedle's Crystals",
            Location::Sky_CrystalOnBeedlesShip => "Crystal on Beedle's Ship",
            Location::Sky_BambooIslandGoddessChest => "Bamboo Island Goddess Chest",
            Location::Sky_BeedlesIslandGoddessChest => "Beedle's Island Goddess Chest",
            Location::Sky_ChestInBreakableBoulderNearFunFunIsland => {
                "Chest in Breakable Boulder near Fun Fun Island"
            }
            Location::Sky_ChestInBreakableBoulderNearLumpyPumpkin => {
                "Chest in Breakable Boulder near Lumpy Pumpkin"
            }
            Location::Sky_DodohsCrystals => "Dodoh's Crystals",
            Location::Sky_FunFunIslandMinigame500Rupees => "Fun Fun Island Minigame - 500 Rupees",
            Location::Sky_GoddessChestInCaveOnIslandNextToBambooIsland => {
                "Goddess Chest in Cave on Island Next to Bamboo Island"
            }
            Location::Sky_GoddessChestInsideVolcanicIsland => {
                "Goddess Chest inside Volcanic Island"
            }
            Location::Sky_GoddessChestOnIslandClosestToFaronPillar => {
                "Goddess Chest on Island Closest to Faron Pillar"
            }
            Location::Sky_GoddessChestOnIslandNextToBambooIsland => {
                "Goddess Chest on Island next to Bamboo Island"
            }
            Location::Sky_GoddessChestOutsideVolcanicIsland => {
                "Goddess Chest outside Volcanic Island"
            }
            Location::Sky_GoddessChestUnderFunFunIsland => "Goddess Chest under Fun Fun Island",
            Location::Sky_GossipStoneInVolcanicIsland => "Gossip Stone in Volcanic Island",
            Location::Sky_LumpyPumpkinGoddessChestOnTheRoof => {
                "Lumpy Pumpkin - Goddess Chest on the Roof"
            }
            Location::Sky_NortheastIslandCageGoddessChest => "Northeast Island Cage Goddess Chest",
            Location::Sky_NortheastIslandGoddessChestBehindBombableRocks => {
                "Northeast Island Goddess Chest Behind Bombable Rocks"
            }
            Location::Sky_OriellesCrystals => "Orielle's Crystals",
            Location::Sky_SouthwestTripleIslandCageGoddessChest => {
                "Southwest Triple Island Cage Goddess Chest"
            }
            Location::Sky_SouthwestTripleIslandLowerGoddessChest => {
                "Southwest Triple Island Lower Goddess Chest"
            }
            Location::Sky_SouthwestTripleIslandUpperGoddessChest => {
                "Southwest Triple Island Upper Goddess Chest"
            }
            Location::Sky_CrystalOutsideLumpyPumpkin => "Crystal outside Lumpy Pumpkin",
            Location::Sky_KinasCrystals => "Kina's Crystals",
            Location::Sky_LumpyPumpkinOutsideGoddessChest => {
                "Lumpy Pumpkin - Outside Goddess Chest"
            }
            Location::SkyKeep_ChestAfterDreadfuse => "Chest after Dreadfuse",
            Location::SkyKeep_FirstChest => "First Chest",
            Location::SkyloftSilentRealm_StoneOfTrials => "Stone of Trials",
            Location::SkyloftVillage_BertiesCrystals => "Bertie's Crystals",
            Location::SkyloftVillage_MallarasCrystals => "Mallara's Crystals",
            Location::SkyloftVillage_CrystalNearPumpkinPatch => "Crystal near Pumpkin Patch",
            Location::SkyloftVillage_SparrotsCrystals => "Sparrot's Crystals",
            Location::Skyview_GhirahimHeartContainer => "Ghirahim Heart Container",
            Location::Skyview_RubyTablet => "Ruby Tablet",
            Location::Skyview_BossKeyChest => "Boss Key Chest",
            Location::Skyview_ChestNearBossDoor => "Chest near Boss Door",
            Location::Skyview_ChestBehindTwoEyes => "Chest behind Two Eyes",
            Location::Skyview_ChestOnTreeBranch => "Chest on Tree Branch",
            Location::Skyview_DiggingSpotInCrawlspace => "Digging Spot in Crawlspace",
            Location::Skyview_Beetle => "Beetle",
            Location::Skyview_ChestBehindThreeEyes => "Chest behind Three Eyes",
            Location::Skyview_ItemBehindBars => "Item behind Bars",
            Location::Thunderhead_BugHeaven10BugsIn3Minutes => "Bug Heaven - 10 Bugs in 3 Minutes",
            Location::Thunderhead_BugHeavenGoddessChest => "Bug Heaven Goddess Chest",
            Location::Thunderhead_EastIslandChest => "East Island Chest",
            Location::Thunderhead_EastIslandGoddessChest => "East Island Goddess Chest",
            Location::Thunderhead_FirstGoddessChestOnMogmaMittsIsland => {
                "First Goddess Chest on Mogma Mitts Island"
            }
            Location::Thunderhead_GoddessChestOnTopOfIsleOfSongs => {
                "Goddess Chest on top of Isle of Songs"
            }
            Location::Thunderhead_GoddessChestOutsideIsleOfSongs => {
                "Goddess Chest outside Isle of Songs"
            }
            Location::Thunderhead_SongFromLevias => "Song from Levias",
            Location::Thunderhead_IsleOfSongsDinsPower => "Isle of Songs - Din's Power",
            Location::Thunderhead_IsleOfSongsFaroresCourage => "Isle of Songs - Farore's Courage",
            Location::Thunderhead_IsleOfSongsNayrusWisdom => "Isle of Songs - Nayru's Wisdom",
            Location::VolcanoSummit_BokoBasePouchChest => "Boko Base Pouch Chest",
            Location::VolcanoSummit_SmallChestInVolcanoSummit => "Small Chest in Volcano Summit",
            Location::VolcanoSummit_GossipStoneOutsideFireSanctuary => {
                "Gossip Stone outside Fire Sanctuary"
            }
            Location::VolcanoSummit_ItemBehindDigging => "Item behind Digging",
            Location::VolcanoSummit_ChestBehindBombableWallInWaterfallArea => {
                "Chest behind Bombable Wall in Waterfall Area"
            }
            Location::VolcanoSummit_GossipStoneInWaterfallArea => "Gossip Stone in Waterfall Area",
        }
    }
}
impl Location {
    pub fn area(&self) -> Area {
        match self {
            Location::AncientCistern_ChestAfterWhipHooks => Area::AncientCistern_AfterWhipHooks,
            Location::AncientCistern_Bokoblin => Area::AncientCistern_BeforeBokoKeyDoor,
            Location::AncientCistern_ChestBehindTheWaterfall => {
                Area::AncientCistern_BehindWaterfall
            }
            Location::AncientCistern_BossKeyChest => Area::AncientCistern_BossKeyChestArea,
            Location::AncientCistern_ChestInEastPart => Area::AncientCistern_MainHub,
            Location::AncientCistern_ChestNearVines => Area::AncientCistern_MainRoomVines,
            Location::AncientCistern_Whip => Area::AncientCistern_WhipChestRoom,
            Location::AncientCistern_KoloktosHeartContainer => Area::AncientCisternBoss_Main,
            Location::AncientCistern_FaroresFlame => Area::AncientCisternCandleRoom_Main,
            Location::Batreaux_10Crystals => Area::BatreauxHouse_Main,
            Location::Batreaux_30Crystals => Area::BatreauxHouse_Main,
            Location::Batreaux_30CrystalsChest => Area::BatreauxHouse_Main,
            Location::Batreaux_40Crystals => Area::BatreauxHouse_Main,
            Location::Batreaux_5Crystals => Area::BatreauxHouse_Main,
            Location::Batreaux_50Crystals => Area::BatreauxHouse_Main,
            Location::Batreaux_70Crystals => Area::BatreauxHouse_Main,
            Location::Batreaux_70CrystalsSecondReward => Area::BatreauxHouse_Main,
            Location::Batreaux_80Crystals => Area::BatreauxHouse_Main,
            Location::Beedle_1000RupeeItem => Area::BeedlesShop_Main,
            Location::Beedle_1200RupeeItem => Area::BeedlesShop_Main,
            Location::Beedle_1600RupeeItem => Area::BeedlesShop_Main,
            Location::Beedle_300RupeeItem => Area::BeedlesShop_Main,
            Location::Beedle_50RupeeItem => Area::BeedlesShop_Main,
            Location::Beedle_600RupeeItem => Area::BeedlesShop_Main,
            Location::Beedle_800RupeeItem => Area::BeedlesShop_Main,
            Location::Beedle_First100RupeeItem => Area::BeedlesShop_Main,
            Location::Beedle_Second100RupeeItem => Area::BeedlesShop_Main,
            Location::Beedle_Third100RupeeItem => Area::BeedlesShop_Main,
            Location::CentralSkyloft_BazaarGoddessChest => Area::Bazaar_Main,
            Location::CentralSkyloft_PotionLadysGift => Area::Bazaar_Main,
            Location::CentralSkyloft_CrystalInOrielleAndParrowsHouse => {
                Area::ParrowAndOriellesHouse_Main
            }
            Location::CentralSkyloft_PeaterPeatricesCrystals => Area::PeatricesHouse_Main,
            Location::CentralSkyloft_CrystalBetweenWoodenPlanks => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_CrystalOnLightTower => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_CrystalOnWaterfallIsland => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_CrystalOnWestCliff => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_FloatingIslandGoddessChest => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_FloatingIslandGossipStone => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_ItemInBirdNest => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_ParrowsCrystals => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_ParrowsGift => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_ShedChest => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_ShedGoddessChest => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_WaterfallGoddessChest => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_WestCliffGoddessChest => Area::Skyloft_CentralOutside,
            Location::CentralSkyloft_CrystalAfterWaterfallCave => {
                Area::Skyloft_WaterfallCaveCrystals
            }
            Location::CentralSkyloft_CrystalInLoftwingPrison => Area::Skyloft_WaterfallCaveCrystals,
            Location::CentralSkyloft_WaterfallCaveFirstChest => Area::WaterfallCave_Main,
            Location::CentralSkyloft_WaterfallCaveSecondChest => Area::WaterfallCave_Main,
            Location::CentralSkyloft_WrynasCrystals => Area::WrynasHouse_Main,
            Location::EarthTemple_ChestGuardedByLizalfos => Area::EarthTemple_AfterBallRolling,
            Location::EarthTemple_BombBag => Area::EarthTemple_BallRolling,
            Location::EarthTemple_ChestLeftOfMainRoomBridge => Area::EarthTemple_BallRolling,
            Location::EarthTemple_ChestBehindBombableRock => Area::EarthTemple_BallRolling,
            Location::EarthTemple_ChestInWestRoom => Area::EarthTemple_BallRolling,
            Location::EarthTemple_LeddsGift => Area::EarthTemple_BallRolling,
            Location::EarthTemple_BossKeyChest => Area::EarthTemple_BossDoorArea,
            Location::EarthTemple_VentChest => Area::EarthTemple_Entrance,
            Location::EarthTemple_ScalderaHeartContainer => Area::EarthTempleBoss_Main,
            Location::EarthTemple_AmberTablet => Area::EarthTempleSpring_Main,
            Location::EldinSilentRealm_FireshieldEarrings => Area::EldinSilentRealm_Trial,
            Location::EldinVolcano_ChestBehindBombableWallInFirstRoom => {
                Area::EldinVolcano_FirstRoom
            }
            Location::EldinVolcano_DiggingSpotBehindBoulderOnSandySlope => {
                Area::EldinVolcano_OutsideEt
            }
            Location::EldinVolcano_DiggingSpotBelowTower => Area::EldinVolcano_OutsideEt,
            Location::EldinVolcano_DiggingSpotInFrontOfEarthTemple => Area::EldinVolcano_OutsideEt,
            Location::EldinVolcano_GossipStoneNextToEarthTemple => Area::EldinVolcano_OutsideEt,
            Location::EldinVolcano_DiggingSpotAfterDrainingLava => Area::EldinVolcano_PastSlide,
            Location::EldinVolcano_ChestAfterCrawlspace => Area::EldinVolcano_PreMogmaTurf,
            Location::EldinVolcano_ChestBehindBombableWallNearCliff => {
                Area::EldinVolcano_PreMogmaTurf
            }
            Location::EldinVolcano_ItemOnCliff => Area::EldinVolcano_PreMogmaTurf,
            Location::EldinVolcano_DiggingSpotAfterVents => Area::EldinVolcano_SandSlide,
            Location::EldinVolcano_ChestBehindBombableWallNearVolcanoAscent => {
                Area::EldinVolcano_VolcanoAscent
            }
            Location::EldinVolcano_GossipStoneInThrillDiggerCave => Area::ThrillDiggerCave_Main,
            Location::FaronSilentRealm_WaterScale => Area::FaronSilentRealm_Trial,
            Location::FaronWoods_DeepWoodsChest => Area::DeepWoods_PastBeehive,
            Location::FaronWoods_ChestBehindBombableRocksNearErla => Area::FaronWoods_Main,
            Location::FaronWoods_ItemBehindBombableRock => Area::FaronWoods_Main,
            Location::FaronWoods_ItemOnTree => Area::FaronWoods_Main,
            Location::FaronWoods_Slingshot => Area::FaronWoods_Main,
            Location::FaronWoods_ChestInsideGreatTree => Area::GreatTree_Middle,
            Location::FireSanctuary_ChestInFirstRoom => Area::FireSanctuaryA_PastFirstWaterPlant,
            Location::FireSanctuary_PlatsChest => Area::FireSanctuaryA_PrePlatsArea,
            Location::FireSanctuary_BossKeyChest => Area::FireSanctuaryA_UpperStaircaseRoom,
            Location::FireSanctuary_ChestInStaircaseRoom => Area::FireSanctuaryA_UpperStaircaseRoom,
            Location::FireSanctuary_MogmaMitts => Area::FireSanctuaryB_AfterDoubleMagmanosFight,
            Location::FireSanctuary_ChestInSecondRoom => Area::FireSanctuaryB_FirstOutsideSection,
            Location::FireSanctuary_ChestOnBalcony => Area::FireSanctuaryB_FirstOutsideSection,
            Location::FireSanctuary_ChestAfterBombableWall => {
                Area::FireSanctuaryB_LastTrappedMogmaArea
            }
            Location::FireSanctuary_ChestAfterSecondTrappedMogma => {
                Area::FireSanctuaryB_LastTrappedMogmaArea
            }
            Location::FireSanctuary_ChestNearFirstTrappedMogma => {
                Area::FireSanctuaryB_PastSecondRoomWithWaterFruit
            }
            Location::FireSanctuary_FirstChestInWaterFruitRoom => {
                Area::FireSanctuaryB_WaterFruitRoom
            }
            Location::FireSanctuary_SecondChestInWaterFruitRoom => {
                Area::FireSanctuaryB_WaterFruitRoom
            }
            Location::FireSanctuary_GhirahimHeartContainer => Area::FireSanctuaryBoss_Main,
            Location::FireSanctuary_DinsFlame => Area::FireSanctuaryFlameRoom_Main,
            Location::KnightAcademy_ChestInGoddessStatue => Area::InsideGoddessStatue_Main,
            Location::KnightAcademy_CawlinsLetter => Area::KnightAcademy_Main,
            Location::KnightAcademy_CrystalInKnightAcademyPlant => Area::KnightAcademy_Main,
            Location::KnightAcademy_CrystalInLinksRoom => Area::KnightAcademy_Main,
            Location::KnightAcademy_CrystalInZeldasRoom => Area::KnightAcademy_Main,
            Location::KnightAcademy_FledgesCrystals => Area::KnightAcademy_Main,
            Location::KnightAcademy_FledgesGift => Area::KnightAcademy_Main,
            Location::KnightAcademy_GhostPipitsCrystals => Area::KnightAcademy_Main,
            Location::KnightAcademy_InZeldasCloset => Area::KnightAcademy_Main,
            Location::KnightAcademy_OwlansCrystals => Area::KnightAcademy_Main,
            Location::KnightAcademy_ChestNearGoddessStatue => Area::Skyloft_OutsideGoddessStatue,
            Location::KnightAcademy_OwlansGift => Area::Skyloft_OutsideGoddessStatue,
            Location::KnightAcademy_PumpkinArchery600Points => Area::Skyloft_OutsideGoddessStatue,
            Location::KnightAcademy_CrystalInSparringHall => Area::SparringHall_Main,
            Location::KnightAcademy_SparringHallChest => Area::SparringHall_Main,
            Location::LakeFloria_DragonLairEastChest => Area::FaroresLair_Main,
            Location::LakeFloria_DragonLairSouthChest => Area::FaroresLair_Main,
            Location::LakeFloria_LakeFloriaChest => Area::LakeFloria_StatueSpot,
            Location::LanayruCaves_Chest => Area::LanayruCaves_Main,
            Location::LanayruCaves_GolosGift => Area::LanayruCaves_Main,
            Location::LanayruCaves_GossipStoneInCenter => Area::LanayruCaves_Main,
            Location::LanayruDesert_FireNodeLeftEndingChest => Area::FireNode_End,
            Location::LanayruDesert_FireNodeRightEndingChest => Area::FireNode_End,
            Location::LanayruDesert_FireNodeFirstSmallChest => Area::FireNode_Main,
            Location::LanayruDesert_FireNodeSecondSmallChest => Area::FireNode_Main,
            Location::LanayruDesert_FireNodeShortcutChest => Area::FireNode_Main,
            Location::LanayruDesert_ChestNearHookBeetleFight => Area::LanayruDesert_HookBeetleArea,
            Location::LanayruDesert_ChestNearPartyWheel => Area::LanayruDesert_HookBeetleArea,
            Location::LanayruDesert_HookBeetleFight => Area::LanayruDesert_HookBeetleArea,
            Location::LanayruDesert_ChestOnPlatformNearFireNode => Area::LanayruDesert_PastToT,
            Location::LanayruDesert_ChestOnPlatformNearLightningNode => Area::LanayruDesert_PastToT,
            Location::LanayruDesert_ChestOnTopOfLanayruMiningFacility => {
                Area::LanayruDesert_PastToT
            }
            Location::LanayruDesert_SecretPassagewayChest => Area::LanayruDesert_PastToT,
            Location::LanayruDesert_ChestNearSandOasis => Area::LanayruDesert_SandOasis,
            Location::LanayruDesert_LightningNodeFirstChest => Area::LightningNode_Main,
            Location::LanayruDesert_LightningNodeRaisedChestNearGenerator => {
                Area::LightningNode_Main
            }
            Location::LanayruDesert_LightningNodeSecondChest => Area::LightningNode_Main,
            Location::LanayruDesert_GossipStoneInTempleOfTimeArea => {
                Area::TempleOfTime_NearGossipStone
            }
            Location::LanayruMines_ChestAtTheEndOfMines => Area::LanayruMines_FirstHalf,
            Location::LanayruMines_ChestBehindFirstLanding => Area::LanayruMines_FirstHalf,
            Location::LanayruMines_ChestBehindStatue => Area::LanayruMines_FirstHalf,
            Location::LanayruMines_ChestNearFirstTimeshiftStone => Area::LanayruMines_FirstHalf,
            Location::LanayruMiningFacility_ChestBehindBars => Area::LanayruMiningFacilityA_Entry,
            Location::LanayruMiningFacility_ChestInKeyLockedRoom => {
                Area::LanayruMiningFacilityA_FirstKeyLockedRoom
            }
            Location::LanayruMiningFacility_ChestInFirstWestRoom => {
                Area::LanayruMiningFacilityA_FirstWestRoom
            }
            Location::LanayruMiningFacility_ChestInsideGustBellowsRoom => {
                Area::LanayruMiningFacilityA_GustBellowsRoom
            }
            Location::LanayruMiningFacility_GustBellows => {
                Area::LanayruMiningFacilityA_GustBellowsRoom
            }
            Location::LanayruMiningFacility_ChestAfterArmosFight => {
                Area::LanayruMiningFacilityA_MapRoom
            }
            Location::LanayruMiningFacility_ShortcutChestInMainHub => {
                Area::LanayruMiningFacilityB_AfterLmfBkRoom
            }
            Location::LanayruMiningFacility_BossKeyChest => {
                Area::LanayruMiningFacilityB_InsideLmfBkRoom
            }
            Location::LanayruMiningFacility_FirstChestInHubRoom => {
                Area::LanayruMiningFacilityB_NearFirstHubRoomChest
            }
            Location::LanayruMiningFacility_ChestBehindFirstCrawlspace => {
                Area::LanayruMiningFacilityB_WestHub
            }
            Location::LanayruMiningFacility_ChestInSpikeMaze => {
                Area::LanayruMiningFacilityB_WestHub
            }
            Location::LanayruMiningFacility_MolderachHeartContainer => {
                Area::LanayruMiningFacilityBoss_Main
            }
            Location::LanayruMiningFacility_GoddessHarp => Area::LanayruMiningFacilityToToT_ToTExit,
            Location::LanayruSandSea_PirateStrongholdFirstChest => {
                Area::InsidePiratesStronghold_Main
            }
            Location::LanayruSandSea_PirateStrongholdSecondChest => {
                Area::InsidePiratesStronghold_Main
            }
            Location::LanayruSandSea_PirateStrongholdThirdChest => {
                Area::InsidePiratesStronghold_Main
            }
            Location::LanayruSandSea_GossipStoneInShipyard => Area::Shipyard_Main,
            Location::LanayruSandSea_RicketyCoasterHeartStoppingTrackIn105 => Area::Shipyard_Main,
            Location::LanayruSandSea_SkippersRetreatSkydiveChest => {
                Area::SkippersRetreat_NextToShack
            }
            Location::LanayruSandSea_SkippersRetreatChestOnTopOfCactiPillar => {
                Area::SkippersRetreat_PastDekuBaba
            }
            Location::LanayruSandSea_SkippersRetreatChestAfterMoblin => {
                Area::SkippersRetreat_PastMoblin
            }
            Location::LanayruSandSea_SkippersRetreatChestInShack => Area::SkippersShack_Main,
            Location::LanayruSilentRealm_Clawshots => Area::LanayruSilentRealm_Trial,
            Location::MogmaTurf_ChestBehindBombableWallAtEntrance => Area::MogmaTurf_Main,
            Location::MogmaTurf_ChestBehindBombableWallInFireMaze => Area::MogmaTurf_Main,
            Location::MogmaTurf_DiggingMittsFight => Area::MogmaTurf_Main,
            Location::MogmaTurf_FreeFallChest => Area::MogmaTurf_Main,
            Location::MogmaTurf_SandSlideChest => Area::MogmaTurf_Main,
            Location::Sandship_BossKeyChest => Area::Sandship_Deck,
            Location::Sandship_Bow => Area::Sandship_Deck,
            Location::Sandship_ChestAtTheStern => Area::Sandship_Deck,
            Location::Sandship_ChestBefore4DoorCorridor => Area::Sandship_Deck,
            Location::Sandship_ChestBehindCombinationLock => Area::Sandship_PastSpume,
            Location::Sandship_RobotInBrigsReward => Area::Sandship_SandshipBrig,
            Location::Sandship_TreasureRoomFifthChest => Area::Sandship_SandshipBrig,
            Location::Sandship_TreasureRoomFirstChest => Area::Sandship_SandshipBrig,
            Location::Sandship_TreasureRoomFourthChest => Area::Sandship_SandshipBrig,
            Location::Sandship_TreasureRoomSecondChest => Area::Sandship_SandshipBrig,
            Location::Sandship_TreasureRoomThirdChest => Area::Sandship_SandshipBrig,
            Location::Sandship_NayrusFlame => Area::SandshipBoss_Main,
            Location::Sandship_TentalusHeartContainer => Area::SandshipBoss_Main,
            Location::SealedGrounds_GorkosGoddessWallReward => Area::BehindTheTemple_Main,
            Location::SealedGrounds_ZeldasBlessing => Area::HyliasTemple_Main,
            Location::SealedGrounds_ChestInsideSealedTemple => Area::SealedTemple_Main,
            Location::SealedGrounds_SongFromImpa => Area::SealedTemple_Main,
            Location::Sky_GossipStoneInsideBambooIsland => Area::InsideBambooIsland_Main,
            Location::Sky_CrystalInsideLumpyPumpkin => Area::LumpyPumpkin_Main,
            Location::Sky_LumpyPumpkinChandelier => Area::LumpyPumpkin_Main,
            Location::Sky_LumpyPumpkinHarpMinigame => Area::LumpyPumpkin_Main,
            Location::Sky_BeedlesIslandCageGoddessChest => Area::Sky_BeedleIslandCage,
            Location::Sky_BeedlesCrystals => Area::Sky_BeedlesSkyHome,
            Location::Sky_CrystalOnBeedlesShip => Area::Sky_BeedlesSkyHome,
            Location::Sky_BambooIslandGoddessChest => Area::Sky_Field,
            Location::Sky_BeedlesIslandGoddessChest => Area::Sky_Field,
            Location::Sky_ChestInBreakableBoulderNearFunFunIsland => Area::Sky_Field,
            Location::Sky_ChestInBreakableBoulderNearLumpyPumpkin => Area::Sky_Field,
            Location::Sky_DodohsCrystals => Area::Sky_Field,
            Location::Sky_FunFunIslandMinigame500Rupees => Area::Sky_Field,
            Location::Sky_GoddessChestInCaveOnIslandNextToBambooIsland => Area::Sky_Field,
            Location::Sky_GoddessChestInsideVolcanicIsland => Area::Sky_Field,
            Location::Sky_GoddessChestOnIslandClosestToFaronPillar => Area::Sky_Field,
            Location::Sky_GoddessChestOnIslandNextToBambooIsland => Area::Sky_Field,
            Location::Sky_GoddessChestOutsideVolcanicIsland => Area::Sky_Field,
            Location::Sky_GoddessChestUnderFunFunIsland => Area::Sky_Field,
            Location::Sky_GossipStoneInVolcanicIsland => Area::Sky_Field,
            Location::Sky_LumpyPumpkinGoddessChestOnTheRoof => Area::Sky_Field,
            Location::Sky_NortheastIslandCageGoddessChest => Area::Sky_Field,
            Location::Sky_NortheastIslandGoddessChestBehindBombableRocks => Area::Sky_Field,
            Location::Sky_OriellesCrystals => Area::Sky_Field,
            Location::Sky_SouthwestTripleIslandCageGoddessChest => Area::Sky_Field,
            Location::Sky_SouthwestTripleIslandLowerGoddessChest => Area::Sky_Field,
            Location::Sky_SouthwestTripleIslandUpperGoddessChest => Area::Sky_Field,
            Location::Sky_CrystalOutsideLumpyPumpkin => Area::Sky_OutsideLumpyPumpkin,
            Location::Sky_KinasCrystals => Area::Sky_OutsideLumpyPumpkin,
            Location::Sky_LumpyPumpkinOutsideGoddessChest => Area::Sky_OutsideLumpyPumpkin,
            Location::SkyKeep_ChestAfterDreadfuse => Area::SkyKeepEntry_Main,
            Location::SkyKeep_FirstChest => Area::SkyKeepEntry_Main,
            Location::SkyloftSilentRealm_StoneOfTrials => Area::SkyloftSilentRealm_Trial,
            Location::SkyloftVillage_BertiesCrystals => Area::BertiesHouse_Main,
            Location::SkyloftVillage_MallarasCrystals => Area::MallarasHouse_Main,
            Location::SkyloftVillage_CrystalNearPumpkinPatch => Area::Skyloft_OutsideSkyloftVillage,
            Location::SkyloftVillage_SparrotsCrystals => Area::SparrotsHouse_Main,
            Location::Skyview_GhirahimHeartContainer => Area::SkyviewBoss_Main,
            Location::Skyview_RubyTablet => Area::SkyviewSpring_Main,
            Location::Skyview_BossKeyChest => Area::SkyviewTemple_BossDoorArea,
            Location::Skyview_ChestNearBossDoor => Area::SkyviewTemple_BossDoorArea,
            Location::Skyview_ChestBehindTwoEyes => Area::SkyviewTemple_FirstHub,
            Location::Skyview_ChestOnTreeBranch => Area::SkyviewTemple_FirstHub,
            Location::Skyview_DiggingSpotInCrawlspace => Area::SkyviewTemple_FirstHub,
            Location::Skyview_Beetle => Area::SkyviewTemple_MainHub,
            Location::Skyview_ChestBehindThreeEyes => Area::SkyviewTemple_MainHub,
            Location::Skyview_ItemBehindBars => Area::SkyviewTemple_MainHub,
            Location::Thunderhead_BugHeaven10BugsIn3Minutes => Area::InsideThunderhead_Main,
            Location::Thunderhead_BugHeavenGoddessChest => Area::InsideThunderhead_Main,
            Location::Thunderhead_EastIslandChest => Area::InsideThunderhead_Main,
            Location::Thunderhead_EastIslandGoddessChest => Area::InsideThunderhead_Main,
            Location::Thunderhead_FirstGoddessChestOnMogmaMittsIsland => {
                Area::InsideThunderhead_Main
            }
            Location::Thunderhead_GoddessChestOnTopOfIsleOfSongs => Area::InsideThunderhead_Main,
            Location::Thunderhead_GoddessChestOutsideIsleOfSongs => Area::InsideThunderhead_Main,
            Location::Thunderhead_SongFromLevias => Area::InsideThunderhead_Main,
            Location::Thunderhead_IsleOfSongsDinsPower => Area::IsleOfSongs_Main,
            Location::Thunderhead_IsleOfSongsFaroresCourage => Area::IsleOfSongs_Main,
            Location::Thunderhead_IsleOfSongsNayrusWisdom => Area::IsleOfSongs_Main,
            Location::VolcanoSummit_BokoBasePouchChest => Area::InsideVolcanoSummit_Main,
            Location::VolcanoSummit_SmallChestInVolcanoSummit => Area::InsideVolcanoSummit_Main,
            Location::VolcanoSummit_GossipStoneOutsideFireSanctuary => {
                Area::OutsideFireSanctuary_Middle
            }
            Location::VolcanoSummit_ItemBehindDigging => Area::OutsideFireSanctuary_Middle,
            Location::VolcanoSummit_ChestBehindBombableWallInWaterfallArea => {
                Area::VolcanoSummitWaterfall_Main
            }
            Location::VolcanoSummit_GossipStoneInWaterfallArea => Area::VolcanoSummitWaterfall_Main,
        }
    }
    pub fn requirement_key(&self) -> RequirementKey {
        match self {
            Location::AncientCistern_ChestAfterWhipHooks => {
                RequirementKey::AncientCistern_ChestAfterWhipHooks
            }
            Location::AncientCistern_Bokoblin => RequirementKey::AncientCistern_Bokoblin,
            Location::AncientCistern_ChestBehindTheWaterfall => {
                RequirementKey::AncientCistern_ChestBehindTheWaterfall
            }
            Location::AncientCistern_BossKeyChest => RequirementKey::AncientCistern_BossKeyChest,
            Location::AncientCistern_ChestInEastPart => {
                RequirementKey::AncientCistern_ChestInEastPart
            }
            Location::AncientCistern_ChestNearVines => {
                RequirementKey::AncientCistern_ChestNearVines
            }
            Location::AncientCistern_Whip => RequirementKey::AncientCistern_Whip,
            Location::AncientCistern_KoloktosHeartContainer => {
                RequirementKey::AncientCistern_KoloktosHeartContainer
            }
            Location::AncientCistern_FaroresFlame => RequirementKey::AncientCistern_FaroresFlame,
            Location::Batreaux_10Crystals => RequirementKey::Batreaux_10Crystals,
            Location::Batreaux_30Crystals => RequirementKey::Batreaux_30Crystals,
            Location::Batreaux_30CrystalsChest => RequirementKey::Batreaux_30CrystalsChest,
            Location::Batreaux_40Crystals => RequirementKey::Batreaux_40Crystals,
            Location::Batreaux_5Crystals => RequirementKey::Batreaux_5Crystals,
            Location::Batreaux_50Crystals => RequirementKey::Batreaux_50Crystals,
            Location::Batreaux_70Crystals => RequirementKey::Batreaux_70Crystals,
            Location::Batreaux_70CrystalsSecondReward => {
                RequirementKey::Batreaux_70CrystalsSecondReward
            }
            Location::Batreaux_80Crystals => RequirementKey::Batreaux_80Crystals,
            Location::Beedle_1000RupeeItem => RequirementKey::Beedle_1000RupeeItem,
            Location::Beedle_1200RupeeItem => RequirementKey::Beedle_1200RupeeItem,
            Location::Beedle_1600RupeeItem => RequirementKey::Beedle_1600RupeeItem,
            Location::Beedle_300RupeeItem => RequirementKey::Beedle_300RupeeItem,
            Location::Beedle_50RupeeItem => RequirementKey::Beedle_50RupeeItem,
            Location::Beedle_600RupeeItem => RequirementKey::Beedle_600RupeeItem,
            Location::Beedle_800RupeeItem => RequirementKey::Beedle_800RupeeItem,
            Location::Beedle_First100RupeeItem => RequirementKey::Beedle_First100RupeeItem,
            Location::Beedle_Second100RupeeItem => RequirementKey::Beedle_Second100RupeeItem,
            Location::Beedle_Third100RupeeItem => RequirementKey::Beedle_Third100RupeeItem,
            Location::CentralSkyloft_BazaarGoddessChest => {
                RequirementKey::CentralSkyloft_BazaarGoddessChest
            }
            Location::CentralSkyloft_PotionLadysGift => {
                RequirementKey::CentralSkyloft_PotionLadysGift
            }
            Location::CentralSkyloft_CrystalInOrielleAndParrowsHouse => {
                RequirementKey::CentralSkyloft_CrystalInOrielleAndParrowsHouse
            }
            Location::CentralSkyloft_PeaterPeatricesCrystals => {
                RequirementKey::CentralSkyloft_PeaterPeatricesCrystals
            }
            Location::CentralSkyloft_CrystalBetweenWoodenPlanks => {
                RequirementKey::CentralSkyloft_CrystalBetweenWoodenPlanks
            }
            Location::CentralSkyloft_CrystalOnLightTower => {
                RequirementKey::CentralSkyloft_CrystalOnLightTower
            }
            Location::CentralSkyloft_CrystalOnWaterfallIsland => {
                RequirementKey::CentralSkyloft_CrystalOnWaterfallIsland
            }
            Location::CentralSkyloft_CrystalOnWestCliff => {
                RequirementKey::CentralSkyloft_CrystalOnWestCliff
            }
            Location::CentralSkyloft_FloatingIslandGoddessChest => {
                RequirementKey::CentralSkyloft_FloatingIslandGoddessChest
            }
            Location::CentralSkyloft_FloatingIslandGossipStone => {
                RequirementKey::CentralSkyloft_FloatingIslandGossipStone
            }
            Location::CentralSkyloft_ItemInBirdNest => {
                RequirementKey::CentralSkyloft_ItemInBirdNest
            }
            Location::CentralSkyloft_ParrowsCrystals => {
                RequirementKey::CentralSkyloft_ParrowsCrystals
            }
            Location::CentralSkyloft_ParrowsGift => RequirementKey::CentralSkyloft_ParrowsGift,
            Location::CentralSkyloft_ShedChest => RequirementKey::CentralSkyloft_ShedChest,
            Location::CentralSkyloft_ShedGoddessChest => {
                RequirementKey::CentralSkyloft_ShedGoddessChest
            }
            Location::CentralSkyloft_WaterfallGoddessChest => {
                RequirementKey::CentralSkyloft_WaterfallGoddessChest
            }
            Location::CentralSkyloft_WestCliffGoddessChest => {
                RequirementKey::CentralSkyloft_WestCliffGoddessChest
            }
            Location::CentralSkyloft_CrystalAfterWaterfallCave => {
                RequirementKey::CentralSkyloft_CrystalAfterWaterfallCave
            }
            Location::CentralSkyloft_CrystalInLoftwingPrison => {
                RequirementKey::CentralSkyloft_CrystalInLoftwingPrison
            }
            Location::CentralSkyloft_WaterfallCaveFirstChest => {
                RequirementKey::CentralSkyloft_WaterfallCaveFirstChest
            }
            Location::CentralSkyloft_WaterfallCaveSecondChest => {
                RequirementKey::CentralSkyloft_WaterfallCaveSecondChest
            }
            Location::CentralSkyloft_WrynasCrystals => {
                RequirementKey::CentralSkyloft_WrynasCrystals
            }
            Location::EarthTemple_ChestGuardedByLizalfos => {
                RequirementKey::EarthTemple_ChestGuardedByLizalfos
            }
            Location::EarthTemple_BombBag => RequirementKey::EarthTemple_BombBag,
            Location::EarthTemple_ChestLeftOfMainRoomBridge => {
                RequirementKey::EarthTemple_ChestLeftOfMainRoomBridge
            }
            Location::EarthTemple_ChestBehindBombableRock => {
                RequirementKey::EarthTemple_ChestBehindBombableRock
            }
            Location::EarthTemple_ChestInWestRoom => RequirementKey::EarthTemple_ChestInWestRoom,
            Location::EarthTemple_LeddsGift => RequirementKey::EarthTemple_LeddsGift,
            Location::EarthTemple_BossKeyChest => RequirementKey::EarthTemple_BossKeyChest,
            Location::EarthTemple_VentChest => RequirementKey::EarthTemple_VentChest,
            Location::EarthTemple_ScalderaHeartContainer => {
                RequirementKey::EarthTemple_ScalderaHeartContainer
            }
            Location::EarthTemple_AmberTablet => RequirementKey::EarthTemple_AmberTablet,
            Location::EldinSilentRealm_FireshieldEarrings => {
                RequirementKey::EldinSilentRealm_FireshieldEarrings
            }
            Location::EldinVolcano_ChestBehindBombableWallInFirstRoom => {
                RequirementKey::EldinVolcano_ChestBehindBombableWallInFirstRoom
            }
            Location::EldinVolcano_DiggingSpotBehindBoulderOnSandySlope => {
                RequirementKey::EldinVolcano_DiggingSpotBehindBoulderOnSandySlope
            }
            Location::EldinVolcano_DiggingSpotBelowTower => {
                RequirementKey::EldinVolcano_DiggingSpotBelowTower
            }
            Location::EldinVolcano_DiggingSpotInFrontOfEarthTemple => {
                RequirementKey::EldinVolcano_DiggingSpotInFrontOfEarthTemple
            }
            Location::EldinVolcano_GossipStoneNextToEarthTemple => {
                RequirementKey::EldinVolcano_GossipStoneNextToEarthTemple
            }
            Location::EldinVolcano_DiggingSpotAfterDrainingLava => {
                RequirementKey::EldinVolcano_DiggingSpotAfterDrainingLava
            }
            Location::EldinVolcano_ChestAfterCrawlspace => {
                RequirementKey::EldinVolcano_ChestAfterCrawlspace
            }
            Location::EldinVolcano_ChestBehindBombableWallNearCliff => {
                RequirementKey::EldinVolcano_ChestBehindBombableWallNearCliff
            }
            Location::EldinVolcano_ItemOnCliff => RequirementKey::EldinVolcano_ItemOnCliff,
            Location::EldinVolcano_DiggingSpotAfterVents => {
                RequirementKey::EldinVolcano_DiggingSpotAfterVents
            }
            Location::EldinVolcano_ChestBehindBombableWallNearVolcanoAscent => {
                RequirementKey::EldinVolcano_ChestBehindBombableWallNearVolcanoAscent
            }
            Location::EldinVolcano_GossipStoneInThrillDiggerCave => {
                RequirementKey::EldinVolcano_GossipStoneInThrillDiggerCave
            }
            Location::FaronSilentRealm_WaterScale => RequirementKey::FaronSilentRealm_WaterScale,
            Location::FaronWoods_DeepWoodsChest => RequirementKey::FaronWoods_DeepWoodsChest,
            Location::FaronWoods_ChestBehindBombableRocksNearErla => {
                RequirementKey::FaronWoods_ChestBehindBombableRocksNearErla
            }
            Location::FaronWoods_ItemBehindBombableRock => {
                RequirementKey::FaronWoods_ItemBehindBombableRock
            }
            Location::FaronWoods_ItemOnTree => RequirementKey::FaronWoods_ItemOnTree,
            Location::FaronWoods_Slingshot => RequirementKey::FaronWoods_Slingshot,
            Location::FaronWoods_ChestInsideGreatTree => {
                RequirementKey::FaronWoods_ChestInsideGreatTree
            }
            Location::FireSanctuary_ChestInFirstRoom => {
                RequirementKey::FireSanctuary_ChestInFirstRoom
            }
            Location::FireSanctuary_PlatsChest => RequirementKey::FireSanctuary_PlatsChest,
            Location::FireSanctuary_BossKeyChest => RequirementKey::FireSanctuary_BossKeyChest,
            Location::FireSanctuary_ChestInStaircaseRoom => {
                RequirementKey::FireSanctuary_ChestInStaircaseRoom
            }
            Location::FireSanctuary_MogmaMitts => RequirementKey::FireSanctuary_MogmaMitts,
            Location::FireSanctuary_ChestInSecondRoom => {
                RequirementKey::FireSanctuary_ChestInSecondRoom
            }
            Location::FireSanctuary_ChestOnBalcony => RequirementKey::FireSanctuary_ChestOnBalcony,
            Location::FireSanctuary_ChestAfterBombableWall => {
                RequirementKey::FireSanctuary_ChestAfterBombableWall
            }
            Location::FireSanctuary_ChestAfterSecondTrappedMogma => {
                RequirementKey::FireSanctuary_ChestAfterSecondTrappedMogma
            }
            Location::FireSanctuary_ChestNearFirstTrappedMogma => {
                RequirementKey::FireSanctuary_ChestNearFirstTrappedMogma
            }
            Location::FireSanctuary_FirstChestInWaterFruitRoom => {
                RequirementKey::FireSanctuary_FirstChestInWaterFruitRoom
            }
            Location::FireSanctuary_SecondChestInWaterFruitRoom => {
                RequirementKey::FireSanctuary_SecondChestInWaterFruitRoom
            }
            Location::FireSanctuary_GhirahimHeartContainer => {
                RequirementKey::FireSanctuary_GhirahimHeartContainer
            }
            Location::FireSanctuary_DinsFlame => RequirementKey::FireSanctuary_DinsFlame,
            Location::KnightAcademy_ChestInGoddessStatue => {
                RequirementKey::KnightAcademy_ChestInGoddessStatue
            }
            Location::KnightAcademy_CawlinsLetter => RequirementKey::KnightAcademy_CawlinsLetter,
            Location::KnightAcademy_CrystalInKnightAcademyPlant => {
                RequirementKey::KnightAcademy_CrystalInKnightAcademyPlant
            }
            Location::KnightAcademy_CrystalInLinksRoom => {
                RequirementKey::KnightAcademy_CrystalInLinksRoom
            }
            Location::KnightAcademy_CrystalInZeldasRoom => {
                RequirementKey::KnightAcademy_CrystalInZeldasRoom
            }
            Location::KnightAcademy_FledgesCrystals => {
                RequirementKey::KnightAcademy_FledgesCrystals
            }
            Location::KnightAcademy_FledgesGift => RequirementKey::KnightAcademy_FledgesGift,
            Location::KnightAcademy_GhostPipitsCrystals => {
                RequirementKey::KnightAcademy_GhostPipitsCrystals
            }
            Location::KnightAcademy_InZeldasCloset => RequirementKey::KnightAcademy_InZeldasCloset,
            Location::KnightAcademy_OwlansCrystals => RequirementKey::KnightAcademy_OwlansCrystals,
            Location::KnightAcademy_ChestNearGoddessStatue => {
                RequirementKey::KnightAcademy_ChestNearGoddessStatue
            }
            Location::KnightAcademy_OwlansGift => RequirementKey::KnightAcademy_OwlansGift,
            Location::KnightAcademy_PumpkinArchery600Points => {
                RequirementKey::KnightAcademy_PumpkinArchery600Points
            }
            Location::KnightAcademy_CrystalInSparringHall => {
                RequirementKey::KnightAcademy_CrystalInSparringHall
            }
            Location::KnightAcademy_SparringHallChest => {
                RequirementKey::KnightAcademy_SparringHallChest
            }
            Location::LakeFloria_DragonLairEastChest => {
                RequirementKey::LakeFloria_DragonLairEastChest
            }
            Location::LakeFloria_DragonLairSouthChest => {
                RequirementKey::LakeFloria_DragonLairSouthChest
            }
            Location::LakeFloria_LakeFloriaChest => RequirementKey::LakeFloria_LakeFloriaChest,
            Location::LanayruCaves_Chest => RequirementKey::LanayruCaves_Chest,
            Location::LanayruCaves_GolosGift => RequirementKey::LanayruCaves_GolosGift,
            Location::LanayruCaves_GossipStoneInCenter => {
                RequirementKey::LanayruCaves_GossipStoneInCenter
            }
            Location::LanayruDesert_FireNodeLeftEndingChest => {
                RequirementKey::LanayruDesert_FireNodeLeftEndingChest
            }
            Location::LanayruDesert_FireNodeRightEndingChest => {
                RequirementKey::LanayruDesert_FireNodeRightEndingChest
            }
            Location::LanayruDesert_FireNodeFirstSmallChest => {
                RequirementKey::LanayruDesert_FireNodeFirstSmallChest
            }
            Location::LanayruDesert_FireNodeSecondSmallChest => {
                RequirementKey::LanayruDesert_FireNodeSecondSmallChest
            }
            Location::LanayruDesert_FireNodeShortcutChest => {
                RequirementKey::LanayruDesert_FireNodeShortcutChest
            }
            Location::LanayruDesert_ChestNearHookBeetleFight => {
                RequirementKey::LanayruDesert_ChestNearHookBeetleFight
            }
            Location::LanayruDesert_ChestNearPartyWheel => {
                RequirementKey::LanayruDesert_ChestNearPartyWheel
            }
            Location::LanayruDesert_HookBeetleFight => {
                RequirementKey::LanayruDesert_HookBeetleFight
            }
            Location::LanayruDesert_ChestOnPlatformNearFireNode => {
                RequirementKey::LanayruDesert_ChestOnPlatformNearFireNode
            }
            Location::LanayruDesert_ChestOnPlatformNearLightningNode => {
                RequirementKey::LanayruDesert_ChestOnPlatformNearLightningNode
            }
            Location::LanayruDesert_ChestOnTopOfLanayruMiningFacility => {
                RequirementKey::LanayruDesert_ChestOnTopOfLanayruMiningFacility
            }
            Location::LanayruDesert_SecretPassagewayChest => {
                RequirementKey::LanayruDesert_SecretPassagewayChest
            }
            Location::LanayruDesert_ChestNearSandOasis => {
                RequirementKey::LanayruDesert_ChestNearSandOasis
            }
            Location::LanayruDesert_LightningNodeFirstChest => {
                RequirementKey::LanayruDesert_LightningNodeFirstChest
            }
            Location::LanayruDesert_LightningNodeRaisedChestNearGenerator => {
                RequirementKey::LanayruDesert_LightningNodeRaisedChestNearGenerator
            }
            Location::LanayruDesert_LightningNodeSecondChest => {
                RequirementKey::LanayruDesert_LightningNodeSecondChest
            }
            Location::LanayruDesert_GossipStoneInTempleOfTimeArea => {
                RequirementKey::LanayruDesert_GossipStoneInTempleOfTimeArea
            }
            Location::LanayruMines_ChestAtTheEndOfMines => {
                RequirementKey::LanayruMines_ChestAtTheEndOfMines
            }
            Location::LanayruMines_ChestBehindFirstLanding => {
                RequirementKey::LanayruMines_ChestBehindFirstLanding
            }
            Location::LanayruMines_ChestBehindStatue => {
                RequirementKey::LanayruMines_ChestBehindStatue
            }
            Location::LanayruMines_ChestNearFirstTimeshiftStone => {
                RequirementKey::LanayruMines_ChestNearFirstTimeshiftStone
            }
            Location::LanayruMiningFacility_ChestBehindBars => {
                RequirementKey::LanayruMiningFacility_ChestBehindBars
            }
            Location::LanayruMiningFacility_ChestInKeyLockedRoom => {
                RequirementKey::LanayruMiningFacility_ChestInKeyLockedRoom
            }
            Location::LanayruMiningFacility_ChestInFirstWestRoom => {
                RequirementKey::LanayruMiningFacility_ChestInFirstWestRoom
            }
            Location::LanayruMiningFacility_ChestInsideGustBellowsRoom => {
                RequirementKey::LanayruMiningFacility_ChestInsideGustBellowsRoom
            }
            Location::LanayruMiningFacility_GustBellows => {
                RequirementKey::LanayruMiningFacility_GustBellows
            }
            Location::LanayruMiningFacility_ChestAfterArmosFight => {
                RequirementKey::LanayruMiningFacility_ChestAfterArmosFight
            }
            Location::LanayruMiningFacility_ShortcutChestInMainHub => {
                RequirementKey::LanayruMiningFacility_ShortcutChestInMainHub
            }
            Location::LanayruMiningFacility_BossKeyChest => {
                RequirementKey::LanayruMiningFacility_BossKeyChest
            }
            Location::LanayruMiningFacility_FirstChestInHubRoom => {
                RequirementKey::LanayruMiningFacility_FirstChestInHubRoom
            }
            Location::LanayruMiningFacility_ChestBehindFirstCrawlspace => {
                RequirementKey::LanayruMiningFacility_ChestBehindFirstCrawlspace
            }
            Location::LanayruMiningFacility_ChestInSpikeMaze => {
                RequirementKey::LanayruMiningFacility_ChestInSpikeMaze
            }
            Location::LanayruMiningFacility_MolderachHeartContainer => {
                RequirementKey::LanayruMiningFacility_MolderachHeartContainer
            }
            Location::LanayruMiningFacility_GoddessHarp => {
                RequirementKey::LanayruMiningFacility_GoddessHarp
            }
            Location::LanayruSandSea_PirateStrongholdFirstChest => {
                RequirementKey::LanayruSandSea_PirateStrongholdFirstChest
            }
            Location::LanayruSandSea_PirateStrongholdSecondChest => {
                RequirementKey::LanayruSandSea_PirateStrongholdSecondChest
            }
            Location::LanayruSandSea_PirateStrongholdThirdChest => {
                RequirementKey::LanayruSandSea_PirateStrongholdThirdChest
            }
            Location::LanayruSandSea_GossipStoneInShipyard => {
                RequirementKey::LanayruSandSea_GossipStoneInShipyard
            }
            Location::LanayruSandSea_RicketyCoasterHeartStoppingTrackIn105 => {
                RequirementKey::LanayruSandSea_RicketyCoasterHeartStoppingTrackIn105
            }
            Location::LanayruSandSea_SkippersRetreatSkydiveChest => {
                RequirementKey::LanayruSandSea_SkippersRetreatSkydiveChest
            }
            Location::LanayruSandSea_SkippersRetreatChestOnTopOfCactiPillar => {
                RequirementKey::LanayruSandSea_SkippersRetreatChestOnTopOfCactiPillar
            }
            Location::LanayruSandSea_SkippersRetreatChestAfterMoblin => {
                RequirementKey::LanayruSandSea_SkippersRetreatChestAfterMoblin
            }
            Location::LanayruSandSea_SkippersRetreatChestInShack => {
                RequirementKey::LanayruSandSea_SkippersRetreatChestInShack
            }
            Location::LanayruSilentRealm_Clawshots => RequirementKey::LanayruSilentRealm_Clawshots,
            Location::MogmaTurf_ChestBehindBombableWallAtEntrance => {
                RequirementKey::MogmaTurf_ChestBehindBombableWallAtEntrance
            }
            Location::MogmaTurf_ChestBehindBombableWallInFireMaze => {
                RequirementKey::MogmaTurf_ChestBehindBombableWallInFireMaze
            }
            Location::MogmaTurf_DiggingMittsFight => RequirementKey::MogmaTurf_DiggingMittsFight,
            Location::MogmaTurf_FreeFallChest => RequirementKey::MogmaTurf_FreeFallChest,
            Location::MogmaTurf_SandSlideChest => RequirementKey::MogmaTurf_SandSlideChest,
            Location::Sandship_BossKeyChest => RequirementKey::Sandship_BossKeyChest,
            Location::Sandship_Bow => RequirementKey::Sandship_Bow,
            Location::Sandship_ChestAtTheStern => RequirementKey::Sandship_ChestAtTheStern,
            Location::Sandship_ChestBefore4DoorCorridor => {
                RequirementKey::Sandship_ChestBefore4DoorCorridor
            }
            Location::Sandship_ChestBehindCombinationLock => {
                RequirementKey::Sandship_ChestBehindCombinationLock
            }
            Location::Sandship_RobotInBrigsReward => RequirementKey::Sandship_RobotInBrigsReward,
            Location::Sandship_TreasureRoomFifthChest => {
                RequirementKey::Sandship_TreasureRoomFifthChest
            }
            Location::Sandship_TreasureRoomFirstChest => {
                RequirementKey::Sandship_TreasureRoomFirstChest
            }
            Location::Sandship_TreasureRoomFourthChest => {
                RequirementKey::Sandship_TreasureRoomFourthChest
            }
            Location::Sandship_TreasureRoomSecondChest => {
                RequirementKey::Sandship_TreasureRoomSecondChest
            }
            Location::Sandship_TreasureRoomThirdChest => {
                RequirementKey::Sandship_TreasureRoomThirdChest
            }
            Location::Sandship_NayrusFlame => RequirementKey::Sandship_NayrusFlame,
            Location::Sandship_TentalusHeartContainer => {
                RequirementKey::Sandship_TentalusHeartContainer
            }
            Location::SealedGrounds_GorkosGoddessWallReward => {
                RequirementKey::SealedGrounds_GorkosGoddessWallReward
            }
            Location::SealedGrounds_ZeldasBlessing => RequirementKey::SealedGrounds_ZeldasBlessing,
            Location::SealedGrounds_ChestInsideSealedTemple => {
                RequirementKey::SealedGrounds_ChestInsideSealedTemple
            }
            Location::SealedGrounds_SongFromImpa => RequirementKey::SealedGrounds_SongFromImpa,
            Location::Sky_GossipStoneInsideBambooIsland => {
                RequirementKey::Sky_GossipStoneInsideBambooIsland
            }
            Location::Sky_CrystalInsideLumpyPumpkin => {
                RequirementKey::Sky_CrystalInsideLumpyPumpkin
            }
            Location::Sky_LumpyPumpkinChandelier => RequirementKey::Sky_LumpyPumpkinChandelier,
            Location::Sky_LumpyPumpkinHarpMinigame => RequirementKey::Sky_LumpyPumpkinHarpMinigame,
            Location::Sky_BeedlesIslandCageGoddessChest => {
                RequirementKey::Sky_BeedlesIslandCageGoddessChest
            }
            Location::Sky_BeedlesCrystals => RequirementKey::Sky_BeedlesCrystals,
            Location::Sky_CrystalOnBeedlesShip => RequirementKey::Sky_CrystalOnBeedlesShip,
            Location::Sky_BambooIslandGoddessChest => RequirementKey::Sky_BambooIslandGoddessChest,
            Location::Sky_BeedlesIslandGoddessChest => {
                RequirementKey::Sky_BeedlesIslandGoddessChest
            }
            Location::Sky_ChestInBreakableBoulderNearFunFunIsland => {
                RequirementKey::Sky_ChestInBreakableBoulderNearFunFunIsland
            }
            Location::Sky_ChestInBreakableBoulderNearLumpyPumpkin => {
                RequirementKey::Sky_ChestInBreakableBoulderNearLumpyPumpkin
            }
            Location::Sky_DodohsCrystals => RequirementKey::Sky_DodohsCrystals,
            Location::Sky_FunFunIslandMinigame500Rupees => {
                RequirementKey::Sky_FunFunIslandMinigame500Rupees
            }
            Location::Sky_GoddessChestInCaveOnIslandNextToBambooIsland => {
                RequirementKey::Sky_GoddessChestInCaveOnIslandNextToBambooIsland
            }
            Location::Sky_GoddessChestInsideVolcanicIsland => {
                RequirementKey::Sky_GoddessChestInsideVolcanicIsland
            }
            Location::Sky_GoddessChestOnIslandClosestToFaronPillar => {
                RequirementKey::Sky_GoddessChestOnIslandClosestToFaronPillar
            }
            Location::Sky_GoddessChestOnIslandNextToBambooIsland => {
                RequirementKey::Sky_GoddessChestOnIslandNextToBambooIsland
            }
            Location::Sky_GoddessChestOutsideVolcanicIsland => {
                RequirementKey::Sky_GoddessChestOutsideVolcanicIsland
            }
            Location::Sky_GoddessChestUnderFunFunIsland => {
                RequirementKey::Sky_GoddessChestUnderFunFunIsland
            }
            Location::Sky_GossipStoneInVolcanicIsland => {
                RequirementKey::Sky_GossipStoneInVolcanicIsland
            }
            Location::Sky_LumpyPumpkinGoddessChestOnTheRoof => {
                RequirementKey::Sky_LumpyPumpkinGoddessChestOnTheRoof
            }
            Location::Sky_NortheastIslandCageGoddessChest => {
                RequirementKey::Sky_NortheastIslandCageGoddessChest
            }
            Location::Sky_NortheastIslandGoddessChestBehindBombableRocks => {
                RequirementKey::Sky_NortheastIslandGoddessChestBehindBombableRocks
            }
            Location::Sky_OriellesCrystals => RequirementKey::Sky_OriellesCrystals,
            Location::Sky_SouthwestTripleIslandCageGoddessChest => {
                RequirementKey::Sky_SouthwestTripleIslandCageGoddessChest
            }
            Location::Sky_SouthwestTripleIslandLowerGoddessChest => {
                RequirementKey::Sky_SouthwestTripleIslandLowerGoddessChest
            }
            Location::Sky_SouthwestTripleIslandUpperGoddessChest => {
                RequirementKey::Sky_SouthwestTripleIslandUpperGoddessChest
            }
            Location::Sky_CrystalOutsideLumpyPumpkin => {
                RequirementKey::Sky_CrystalOutsideLumpyPumpkin
            }
            Location::Sky_KinasCrystals => RequirementKey::Sky_KinasCrystals,
            Location::Sky_LumpyPumpkinOutsideGoddessChest => {
                RequirementKey::Sky_LumpyPumpkinOutsideGoddessChest
            }
            Location::SkyKeep_ChestAfterDreadfuse => RequirementKey::SkyKeep_ChestAfterDreadfuse,
            Location::SkyKeep_FirstChest => RequirementKey::SkyKeep_FirstChest,
            Location::SkyloftSilentRealm_StoneOfTrials => {
                RequirementKey::SkyloftSilentRealm_StoneOfTrials
            }
            Location::SkyloftVillage_BertiesCrystals => {
                RequirementKey::SkyloftVillage_BertiesCrystals
            }
            Location::SkyloftVillage_MallarasCrystals => {
                RequirementKey::SkyloftVillage_MallarasCrystals
            }
            Location::SkyloftVillage_CrystalNearPumpkinPatch => {
                RequirementKey::SkyloftVillage_CrystalNearPumpkinPatch
            }
            Location::SkyloftVillage_SparrotsCrystals => {
                RequirementKey::SkyloftVillage_SparrotsCrystals
            }
            Location::Skyview_GhirahimHeartContainer => {
                RequirementKey::Skyview_GhirahimHeartContainer
            }
            Location::Skyview_RubyTablet => RequirementKey::Skyview_RubyTablet,
            Location::Skyview_BossKeyChest => RequirementKey::Skyview_BossKeyChest,
            Location::Skyview_ChestNearBossDoor => RequirementKey::Skyview_ChestNearBossDoor,
            Location::Skyview_ChestBehindTwoEyes => RequirementKey::Skyview_ChestBehindTwoEyes,
            Location::Skyview_ChestOnTreeBranch => RequirementKey::Skyview_ChestOnTreeBranch,
            Location::Skyview_DiggingSpotInCrawlspace => {
                RequirementKey::Skyview_DiggingSpotInCrawlspace
            }
            Location::Skyview_Beetle => RequirementKey::Skyview_Beetle,
            Location::Skyview_ChestBehindThreeEyes => RequirementKey::Skyview_ChestBehindThreeEyes,
            Location::Skyview_ItemBehindBars => RequirementKey::Skyview_ItemBehindBars,
            Location::Thunderhead_BugHeaven10BugsIn3Minutes => {
                RequirementKey::Thunderhead_BugHeaven10BugsIn3Minutes
            }
            Location::Thunderhead_BugHeavenGoddessChest => {
                RequirementKey::Thunderhead_BugHeavenGoddessChest
            }
            Location::Thunderhead_EastIslandChest => RequirementKey::Thunderhead_EastIslandChest,
            Location::Thunderhead_EastIslandGoddessChest => {
                RequirementKey::Thunderhead_EastIslandGoddessChest
            }
            Location::Thunderhead_FirstGoddessChestOnMogmaMittsIsland => {
                RequirementKey::Thunderhead_FirstGoddessChestOnMogmaMittsIsland
            }
            Location::Thunderhead_GoddessChestOnTopOfIsleOfSongs => {
                RequirementKey::Thunderhead_GoddessChestOnTopOfIsleOfSongs
            }
            Location::Thunderhead_GoddessChestOutsideIsleOfSongs => {
                RequirementKey::Thunderhead_GoddessChestOutsideIsleOfSongs
            }
            Location::Thunderhead_SongFromLevias => RequirementKey::Thunderhead_SongFromLevias,
            Location::Thunderhead_IsleOfSongsDinsPower => {
                RequirementKey::Thunderhead_IsleOfSongsDinsPower
            }
            Location::Thunderhead_IsleOfSongsFaroresCourage => {
                RequirementKey::Thunderhead_IsleOfSongsFaroresCourage
            }
            Location::Thunderhead_IsleOfSongsNayrusWisdom => {
                RequirementKey::Thunderhead_IsleOfSongsNayrusWisdom
            }
            Location::VolcanoSummit_BokoBasePouchChest => {
                RequirementKey::VolcanoSummit_BokoBasePouchChest
            }
            Location::VolcanoSummit_SmallChestInVolcanoSummit => {
                RequirementKey::VolcanoSummit_SmallChestInVolcanoSummit
            }
            Location::VolcanoSummit_GossipStoneOutsideFireSanctuary => {
                RequirementKey::VolcanoSummit_GossipStoneOutsideFireSanctuary
            }
            Location::VolcanoSummit_ItemBehindDigging => {
                RequirementKey::VolcanoSummit_ItemBehindDigging
            }
            Location::VolcanoSummit_ChestBehindBombableWallInWaterfallArea => {
                RequirementKey::VolcanoSummit_ChestBehindBombableWallInWaterfallArea
            }
            Location::VolcanoSummit_GossipStoneInWaterfallArea => {
                RequirementKey::VolcanoSummit_GossipStoneInWaterfallArea
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    ActivateFireNode,
    ActivateLightningNode,
    ActivateSkyviewGoddessWall,
    ActivateWaterNode,
    BeatRequiredDungeons,
    BehindTheTempleStatue,
    CanBeatAncientCistern,
    CanBeatEarthTemple,
    CanBeatFireSanctuary,
    CanBeatLanayruMiningFacility,
    CanBeatSandship,
    CanBeatSkyKeep,
    CanBeatSkyview,
    CanFreelyChangeSandshipTemporality,
    CanLowerAcStatue,
    CanPlayCleanCut,
    CanPlayThrillDigger,
    CanRetrievePartyWheel,
    CanSellTreasures,
    DeepWoodsStatue,
    DefeatImprisoned2,
    DefeatedShipyardMolderach,
    DeliveredHotSoup,
    DesertEntranceStatue,
    EldinEntranceStatue,
    EndurancePotion,
    FaronWoodsEntryStatue,
    FloriaWaterfallStatue,
    ForestTempleStatue,
    GoddessCubeEastOfEarthTempleEntrance,
    GoddessCubeWestOfEarthTempleEntrance,
    GoddessCubeAtEldinEntrance,
    GoddessCubeAtLanayruMinesEntrance,
    GoddessCubeAtRideNearTempleOfTime,
    GoddessCubeInAncientHarbour,
    GoddessCubeInDeepWoods,
    GoddessCubeInEldinSlide,
    GoddessCubeInFloriaWaterfall,
    GoddessCubeInLakeFloria,
    GoddessCubeInMogmaTurf,
    GoddessCubeInPirateStronghold,
    GoddessCubeInSandOasis,
    GoddessCubeInSecretPassagewayInDesert,
    GoddessCubeInSkippersRetreat,
    GoddessCubeInSkyviewSpring,
    GoddessCubeInSummitWaterfall,
    GoddessCubeInsideVolcanoSummit,
    GoddessCubeNearFsEntrance,
    GoddessCubeNearHookBeetleFight,
    GoddessCubeNearMogmaTurfEntrance,
    GoddessCubeOnEastGreatTreeWithClawshotsTarget,
    GoddessCubeOnEastGreatTreeWithRope,
    GoddessCubeOnWestGreatTreeNearExit,
    GoddessCubeOnTopOfSkyview,
    GreatTreeStatue,
    HighRupeeFarm,
    InTheWoodsStatue,
    InitialGoddessCube,
    InsideTheVolcanoStatue,
    LakeFloriaStatue,
    LanayruDesertDungeonBeaten,
    LanayruMineEntryStatue,
    LumpyPumpkinQuestStart,
    NorthDesertStatue,
    OpenSharkhead,
    OpenedShed,
    PickUpGuld,
    PickUpLeviasSoup,
    PumpkinCarrying,
    RaiseGoT,
    RaiseLanayruMiningFacility,
    RetrieveCrystalBall,
    RetrieveOolo,
    SaveOrielle,
    SealedGroundsStatue,
    StartImprisoned2,
    StoneCacheStatue,
    TalkToOrielle,
    TalkToPeatriceInBazaar,
    TalkToYerbal,
    TempleEntranceStatue,
    UnlockedZeldasRoom,
    ViewingPlatformStatue,
    VolcanoAscentStatue,
    VolcanoEastStatue,
    WestDesertStatue,
}
impl Into<usize> for Event {
    fn into(self) -> usize {
        self as usize
    }
}
impl BitSetCompatible for Event {
    const ALL: &'static [Event] = &[
        Event::ActivateFireNode,
        Event::ActivateLightningNode,
        Event::ActivateSkyviewGoddessWall,
        Event::ActivateWaterNode,
        Event::BeatRequiredDungeons,
        Event::BehindTheTempleStatue,
        Event::CanBeatAncientCistern,
        Event::CanBeatEarthTemple,
        Event::CanBeatFireSanctuary,
        Event::CanBeatLanayruMiningFacility,
        Event::CanBeatSandship,
        Event::CanBeatSkyKeep,
        Event::CanBeatSkyview,
        Event::CanFreelyChangeSandshipTemporality,
        Event::CanLowerAcStatue,
        Event::CanPlayCleanCut,
        Event::CanPlayThrillDigger,
        Event::CanRetrievePartyWheel,
        Event::CanSellTreasures,
        Event::DeepWoodsStatue,
        Event::DefeatImprisoned2,
        Event::DefeatedShipyardMolderach,
        Event::DeliveredHotSoup,
        Event::DesertEntranceStatue,
        Event::EldinEntranceStatue,
        Event::EndurancePotion,
        Event::FaronWoodsEntryStatue,
        Event::FloriaWaterfallStatue,
        Event::ForestTempleStatue,
        Event::GoddessCubeEastOfEarthTempleEntrance,
        Event::GoddessCubeWestOfEarthTempleEntrance,
        Event::GoddessCubeAtEldinEntrance,
        Event::GoddessCubeAtLanayruMinesEntrance,
        Event::GoddessCubeAtRideNearTempleOfTime,
        Event::GoddessCubeInAncientHarbour,
        Event::GoddessCubeInDeepWoods,
        Event::GoddessCubeInEldinSlide,
        Event::GoddessCubeInFloriaWaterfall,
        Event::GoddessCubeInLakeFloria,
        Event::GoddessCubeInMogmaTurf,
        Event::GoddessCubeInPirateStronghold,
        Event::GoddessCubeInSandOasis,
        Event::GoddessCubeInSecretPassagewayInDesert,
        Event::GoddessCubeInSkippersRetreat,
        Event::GoddessCubeInSkyviewSpring,
        Event::GoddessCubeInSummitWaterfall,
        Event::GoddessCubeInsideVolcanoSummit,
        Event::GoddessCubeNearFsEntrance,
        Event::GoddessCubeNearHookBeetleFight,
        Event::GoddessCubeNearMogmaTurfEntrance,
        Event::GoddessCubeOnEastGreatTreeWithClawshotsTarget,
        Event::GoddessCubeOnEastGreatTreeWithRope,
        Event::GoddessCubeOnWestGreatTreeNearExit,
        Event::GoddessCubeOnTopOfSkyview,
        Event::GreatTreeStatue,
        Event::HighRupeeFarm,
        Event::InTheWoodsStatue,
        Event::InitialGoddessCube,
        Event::InsideTheVolcanoStatue,
        Event::LakeFloriaStatue,
        Event::LanayruDesertDungeonBeaten,
        Event::LanayruMineEntryStatue,
        Event::LumpyPumpkinQuestStart,
        Event::NorthDesertStatue,
        Event::OpenSharkhead,
        Event::OpenedShed,
        Event::PickUpGuld,
        Event::PickUpLeviasSoup,
        Event::PumpkinCarrying,
        Event::RaiseGoT,
        Event::RaiseLanayruMiningFacility,
        Event::RetrieveCrystalBall,
        Event::RetrieveOolo,
        Event::SaveOrielle,
        Event::SealedGroundsStatue,
        Event::StartImprisoned2,
        Event::StoneCacheStatue,
        Event::TalkToOrielle,
        Event::TalkToPeatriceInBazaar,
        Event::TalkToYerbal,
        Event::TempleEntranceStatue,
        Event::UnlockedZeldasRoom,
        Event::ViewingPlatformStatue,
        Event::VolcanoAscentStatue,
        Event::VolcanoEastStatue,
        Event::WestDesertStatue,
    ];
}
impl Event {
    pub fn name(&self) -> &'static str {
        match self {
            Event::ActivateFireNode => "Activate Fire Node",
            Event::ActivateLightningNode => "Activate Lightning Node",
            Event::ActivateSkyviewGoddessWall => "Activate Skyview Goddess Wall",
            Event::ActivateWaterNode => "Activate Water Node",
            Event::BeatRequiredDungeons => "Beat Required Dungeons",
            Event::BehindTheTempleStatue => "Behind the Temple Statue",
            Event::CanBeatAncientCistern => "Can Beat Ancient Cistern",
            Event::CanBeatEarthTemple => "Can Beat Earth Temple",
            Event::CanBeatFireSanctuary => "Can Beat Fire Sanctuary",
            Event::CanBeatLanayruMiningFacility => "Can Beat Lanayru Mining Facility",
            Event::CanBeatSandship => "Can Beat Sandship",
            Event::CanBeatSkyKeep => "Can Beat Sky Keep",
            Event::CanBeatSkyview => "Can Beat Skyview",
            Event::CanFreelyChangeSandshipTemporality => "Can Freely Change Sandship Temporality",
            Event::CanLowerAcStatue => "Can Lower AC Statue",
            Event::CanPlayCleanCut => "Can Play Clean Cut",
            Event::CanPlayThrillDigger => "Can Play Thrill Digger",
            Event::CanRetrievePartyWheel => "Can Retrieve Party Wheel",
            Event::CanSellTreasures => "Can Sell Treasures",
            Event::DeepWoodsStatue => "Deep Woods Statue",
            Event::DefeatImprisoned2 => "Defeat Imprisoned 2",
            Event::DefeatedShipyardMolderach => "Defeated Shipyard Molderach",
            Event::DeliveredHotSoup => "Delivered Hot Soup",
            Event::DesertEntranceStatue => "Desert Entrance Statue",
            Event::EldinEntranceStatue => "Eldin Entrance Statue",
            Event::EndurancePotion => "Endurance Potion",
            Event::FaronWoodsEntryStatue => "Faron Woods Entry Statue",
            Event::FloriaWaterfallStatue => "Floria Waterfall Statue",
            Event::ForestTempleStatue => "Forest Temple Statue",
            Event::GoddessCubeEastOfEarthTempleEntrance => {
                "Goddess Cube East of Earth Temple Entrance"
            }
            Event::GoddessCubeWestOfEarthTempleEntrance => {
                "Goddess Cube West of Earth Temple Entrance"
            }
            Event::GoddessCubeAtEldinEntrance => "Goddess Cube at Eldin Entrance",
            Event::GoddessCubeAtLanayruMinesEntrance => "Goddess Cube at Lanayru Mines Entrance",
            Event::GoddessCubeAtRideNearTempleOfTime => "Goddess Cube at Ride near Temple of Time",
            Event::GoddessCubeInAncientHarbour => "Goddess Cube in Ancient Harbour",
            Event::GoddessCubeInDeepWoods => "Goddess Cube in Deep Woods",
            Event::GoddessCubeInEldinSlide => "Goddess Cube in Eldin Slide",
            Event::GoddessCubeInFloriaWaterfall => "Goddess Cube in Floria Waterfall",
            Event::GoddessCubeInLakeFloria => "Goddess Cube in Lake Floria",
            Event::GoddessCubeInMogmaTurf => "Goddess Cube in Mogma Turf",
            Event::GoddessCubeInPirateStronghold => "Goddess Cube in Pirate Stronghold",
            Event::GoddessCubeInSandOasis => "Goddess Cube in Sand Oasis",
            Event::GoddessCubeInSecretPassagewayInDesert => {
                "Goddess Cube in Secret Passageway in Desert"
            }
            Event::GoddessCubeInSkippersRetreat => "Goddess Cube in Skipper's Retreat",
            Event::GoddessCubeInSkyviewSpring => "Goddess Cube in Skyview Spring",
            Event::GoddessCubeInSummitWaterfall => "Goddess Cube in Summit Waterfall",
            Event::GoddessCubeInsideVolcanoSummit => "Goddess Cube inside Volcano Summit",
            Event::GoddessCubeNearFsEntrance => "Goddess Cube near FS Entrance",
            Event::GoddessCubeNearHookBeetleFight => "Goddess Cube near Hook Beetle Fight",
            Event::GoddessCubeNearMogmaTurfEntrance => "Goddess Cube near Mogma Turf Entrance",
            Event::GoddessCubeOnEastGreatTreeWithClawshotsTarget => {
                "Goddess Cube on East Great Tree with Clawshots Target"
            }
            Event::GoddessCubeOnEastGreatTreeWithRope => {
                "Goddess Cube on East Great Tree with Rope"
            }
            Event::GoddessCubeOnWestGreatTreeNearExit => {
                "Goddess Cube on West Great Tree near Exit"
            }
            Event::GoddessCubeOnTopOfSkyview => "Goddess Cube on top of Skyview",
            Event::GreatTreeStatue => "Great Tree Statue",
            Event::HighRupeeFarm => "High Rupee Farm",
            Event::InTheWoodsStatue => "In the Woods Statue",
            Event::InitialGoddessCube => "Initial Goddess Cube",
            Event::InsideTheVolcanoStatue => "Inside the Volcano Statue",
            Event::LakeFloriaStatue => "Lake Floria Statue",
            Event::LanayruDesertDungeonBeaten => "Lanayru Desert Dungeon Beaten",
            Event::LanayruMineEntryStatue => "Lanayru Mine Entry Statue",
            Event::LumpyPumpkinQuestStart => "Lumpy Pumpkin Quest Start",
            Event::NorthDesertStatue => "North Desert Statue",
            Event::OpenSharkhead => "Open Sharkhead",
            Event::OpenedShed => "Opened Shed",
            Event::PickUpGuld => "Pick up Guld",
            Event::PickUpLeviasSoup => "Pick up Levia's Soup",
            Event::PumpkinCarrying => "Pumpkin Carrying",
            Event::RaiseGoT => "Raise GoT",
            Event::RaiseLanayruMiningFacility => "Raise Lanayru Mining Facility",
            Event::RetrieveCrystalBall => "Retrieve Crystal Ball",
            Event::RetrieveOolo => "Retrieve Oolo",
            Event::SaveOrielle => "Save Orielle",
            Event::SealedGroundsStatue => "Sealed Grounds Statue",
            Event::StartImprisoned2 => "Start Imprisoned 2",
            Event::StoneCacheStatue => "Stone Cache Statue",
            Event::TalkToOrielle => "Talk to Orielle",
            Event::TalkToPeatriceInBazaar => "Talk to Peatrice in Bazaar",
            Event::TalkToYerbal => "Talk to Yerbal",
            Event::TempleEntranceStatue => "Temple Entrance Statue",
            Event::UnlockedZeldasRoom => "Unlocked Zelda's Room",
            Event::ViewingPlatformStatue => "Viewing Platform Statue",
            Event::VolcanoAscentStatue => "Volcano Ascent Statue",
            Event::VolcanoEastStatue => "Volcano East Statue",
            Event::WestDesertStatue => "West Desert Statue",
        }
    }
}
impl Event {
    pub fn requirements(&self) -> &'static [RequirementKey] {
        match self {
Event::ActivateFireNode => &[
RequirementKey::FireNode_End_ActivateFireNode,
],
Event::ActivateLightningNode => &[
RequirementKey::LightningNode_Main_ActivateLightningNode,
],
Event::ActivateSkyviewGoddessWall => &[
RequirementKey::SkyviewTemple_Entry_ActivateSkyviewGoddessWall,
],
Event::ActivateWaterNode => &[
RequirementKey::LanayruDesert_PastToT_ActivateWaterNode,
],
Event::BeatRequiredDungeons => &[
RequirementKey::SealedTemple_Main_BeatRequiredDungeons,
],
Event::BehindTheTempleStatue => &[
RequirementKey::BehindTheTemple_Main_BehindTheTempleStatue,
],
Event::CanBeatAncientCistern => &[
RequirementKey::AncientCisternCandleRoom_Main_CanBeatAncientCistern,
],
Event::CanBeatEarthTemple => &[
RequirementKey::EarthTempleSpring_Main_CanBeatEarthTemple,
],
Event::CanBeatFireSanctuary => &[
RequirementKey::FireSanctuaryFlameRoom_Main_CanBeatFireSanctuary,
],
Event::CanBeatLanayruMiningFacility => &[
RequirementKey::LanayruMiningFacilityToToT_ToTExit_CanBeatLanayruMiningFacility,
],
Event::CanBeatSandship => &[
RequirementKey::SandshipBoss_Main_CanBeatSandship,
],
Event::CanBeatSkyKeep => &[
RequirementKey::SkyKeepEntry_Main_CanBeatSkyKeep,
],
Event::CanBeatSkyview => &[
RequirementKey::SkyviewSpring_Main_CanBeatSkyview,
],
Event::CanFreelyChangeSandshipTemporality => &[
RequirementKey::Sandship_Deck_CanFreelyChangeSandshipTemporality,
],
Event::CanLowerAcStatue => &[
RequirementKey::AncientCistern_MainRoomVines_CanLowerAcStatue,
RequirementKey::AncientCistern_SpiderThread_CanLowerAcStatue,
],
Event::CanPlayCleanCut => &[
RequirementKey::InsideBambooIsland_Main_CanPlayCleanCut,
],
Event::CanPlayThrillDigger => &[
RequirementKey::ThrillDiggerCave_Main_CanPlayThrillDigger,
],
Event::CanRetrievePartyWheel => &[
RequirementKey::LanayruDesert_HookBeetleArea_CanRetrievePartyWheel,
],
Event::CanSellTreasures => &[
RequirementKey::RupinsHouse_Main_CanSellTreasures,
],
Event::DeepWoodsStatue => &[
RequirementKey::DeepWoods_PastBeehive_DeepWoodsStatue,
],
Event::DefeatImprisoned2 => &[
RequirementKey::SealedGroundsSpiral_Lower_DefeatImprisoned2,
],
Event::DefeatedShipyardMolderach => &[
RequirementKey::ShipyardConstructionBay_Lower_DefeatedShipyardMolderach,
],
Event::DeliveredHotSoup => &[
RequirementKey::SparringHall_Main_DeliveredHotSoup,
],
Event::DesertEntranceStatue => &[
RequirementKey::LanayruDesert_HookBeetleArea_DesertEntranceStatue,
],
Event::EldinEntranceStatue => &[
RequirementKey::EldinVolcano_FirstRoom_EldinEntranceStatue,
],
Event::EndurancePotion => &[
RequirementKey::Bazaar_Main_EndurancePotion,
],
Event::FaronWoodsEntryStatue => &[
RequirementKey::FaronWoods_Entry_FaronWoodsEntryStatue,
],
Event::FloriaWaterfallStatue => &[
RequirementKey::FloriaWaterfall_Main_FloriaWaterfallStatue,
],
Event::ForestTempleStatue => &[
RequirementKey::DeepWoods_PastBeehive_ForestTempleStatue,
],
Event::GoddessCubeEastOfEarthTempleEntrance => &[
RequirementKey::EldinVolcano_OutsideEt_GoddessCubeEastOfEarthTempleEntrance,
],
Event::GoddessCubeWestOfEarthTempleEntrance => &[
RequirementKey::EldinVolcano_OutsideEt_GoddessCubeWestOfEarthTempleEntrance,
],
Event::GoddessCubeAtEldinEntrance => &[
RequirementKey::EldinVolcano_FirstRoom_GoddessCubeAtEldinEntrance,
],
Event::GoddessCubeAtLanayruMinesEntrance => &[
RequirementKey::LanayruMines_FirstHalf_GoddessCubeAtLanayruMinesEntrance,
],
Event::GoddessCubeAtRideNearTempleOfTime => &[
RequirementKey::TempleOfTime_NearCube_GoddessCubeAtRideNearTempleOfTime,
],
Event::GoddessCubeInAncientHarbour => &[
RequirementKey::SandSeaDocks_Main_GoddessCubeInAncientHarbour,
],
Event::GoddessCubeInDeepWoods => &[
RequirementKey::DeepWoods_PastBeehive_GoddessCubeInDeepWoods,
],
Event::GoddessCubeInEldinSlide => &[
RequirementKey::EldinVolcano_SandSlide_GoddessCubeInEldinSlide,
],
Event::GoddessCubeInFloriaWaterfall => &[
RequirementKey::FloriaWaterfall_Main_GoddessCubeInFloriaWaterfall,
],
Event::GoddessCubeInLakeFloria => &[
RequirementKey::LakeFloria_StatueSpot_GoddessCubeInLakeFloria,
],
Event::GoddessCubeInMogmaTurf => &[
RequirementKey::MogmaTurf_Main_GoddessCubeInMogmaTurf,
],
Event::GoddessCubeInPirateStronghold => &[
RequirementKey::OutsidePiratesStronghold_InsideSharkhead_GoddessCubeInPirateStronghold,
],
Event::GoddessCubeInSandOasis => &[
RequirementKey::LanayruDesert_SandOasis_GoddessCubeInSandOasis,
],
Event::GoddessCubeInSecretPassagewayInDesert => &[
RequirementKey::LanayruDesert_PastToT_GoddessCubeInSecretPassagewayInDesert,
],
Event::GoddessCubeInSkippersRetreat => &[
RequirementKey::SkippersRetreat_PastMoblin_GoddessCubeInSkippersRetreat,
],
Event::GoddessCubeInSkyviewSpring => &[
RequirementKey::SkyviewSpring_Main_GoddessCubeInSkyviewSpring,
],
Event::GoddessCubeInSummitWaterfall => &[
RequirementKey::VolcanoSummitWaterfall_Main_GoddessCubeInSummitWaterfall,
],
Event::GoddessCubeInsideVolcanoSummit => &[
RequirementKey::InsideVolcanoSummit_Main_GoddessCubeInsideVolcanoSummit,
],
Event::GoddessCubeNearFsEntrance => &[
RequirementKey::OutsideFireSanctuary_ToFireSanctuary_GoddessCubeNearFsEntrance,
],
Event::GoddessCubeNearHookBeetleFight => &[
RequirementKey::LanayruDesert_HookBeetleArea_GoddessCubeNearHookBeetleFight,
],
Event::GoddessCubeNearMogmaTurfEntrance => &[
RequirementKey::EldinVolcano_PastMogmaTurf_GoddessCubeNearMogmaTurfEntrance,
RequirementKey::EldinVolcano_PreMogmaTurf_GoddessCubeNearMogmaTurfEntrance,
],
Event::GoddessCubeOnEastGreatTreeWithClawshotsTarget => &[
RequirementKey::FaronWoods_ClawshotTargetBranch_GoddessCubeOnEastGreatTreeWithClawshotsTarget,
],
Event::GoddessCubeOnEastGreatTreeWithRope => &[
RequirementKey::FaronWoods_GreatTreeTop_GoddessCubeOnEastGreatTreeWithRope,
],
Event::GoddessCubeOnWestGreatTreeNearExit => &[
RequirementKey::FaronWoods_GreatTreePlatforms_GoddessCubeOnWestGreatTreeNearExit,
],
Event::GoddessCubeOnTopOfSkyview => &[
RequirementKey::DeepWoods_PastBeehive_GoddessCubeOnTopOfSkyview,
],
Event::GreatTreeStatue => &[
RequirementKey::FaronWoods_GreatTreeTop_GreatTreeStatue,
],
Event::HighRupeeFarm => &[
RequirementKey::Sky_Field_HighRupeeFarm,
],
Event::InTheWoodsStatue => &[
RequirementKey::FaronWoods_Main_InTheWoodsStatue,
],
Event::InitialGoddessCube => &[
RequirementKey::DeepWoods_PastBeehive_InitialGoddessCube,
],
Event::InsideTheVolcanoStatue => &[
RequirementKey::OutsideFireSanctuary_ToFireSanctuary_InsideTheVolcanoStatue,
],
Event::LakeFloriaStatue => &[
RequirementKey::LakeFloria_StatueSpot_LakeFloriaStatue,
],
Event::LanayruDesertDungeonBeaten => &[
RequirementKey::TempleOfTime_AfterLmf_LanayruDesertDungeonBeaten,
],
Event::LanayruMineEntryStatue => &[
RequirementKey::LanayruMines_FirstHalf_LanayruMineEntryStatue,
],
Event::LumpyPumpkinQuestStart => &[
RequirementKey::LumpyPumpkin_Main_LumpyPumpkinQuestStart,
],
Event::NorthDesertStatue => &[
RequirementKey::LanayruDesert_PastToT_NorthDesertStatue,
],
Event::OpenSharkhead => &[
RequirementKey::InsidePiratesStronghold_Main_OpenSharkhead,
],
Event::OpenedShed => &[
RequirementKey::Skyloft_OutsideSkyloftVillage_OpenedShed,
],
Event::PickUpGuld => &[
RequirementKey::MogmaTurf_Main_PickUpGuld,
],
Event::PickUpLeviasSoup => &[
RequirementKey::LumpyPumpkin_Main_PickUpLeviasSoup,
],
Event::PumpkinCarrying => &[
RequirementKey::Sky_OutsideLumpyPumpkin_PumpkinCarrying,
],
Event::RaiseGoT => &[
RequirementKey::SealedTemple_Main_RaiseGoT,
],
Event::RaiseLanayruMiningFacility => &[
RequirementKey::LanayruDesert_PastToT_RaiseLanayruMiningFacility,
],
Event::RetrieveCrystalBall => &[
RequirementKey::EldinVolcano_OutsideEt_RetrieveCrystalBall,
],
Event::RetrieveOolo => &[
RequirementKey::FaronWoods_Main_RetrieveOolo,
],
Event::SaveOrielle => &[
RequirementKey::Sky_Field_SaveOrielle,
],
Event::SealedGroundsStatue => &[
RequirementKey::SealedGroundsSpiral_Upper_SealedGroundsStatue,
],
Event::StartImprisoned2 => &[
RequirementKey::SealedTemple_Main_StartImprisoned2,
],
Event::StoneCacheStatue => &[
RequirementKey::LanayruDesert_PastToT_StoneCacheStatue,
],
Event::TalkToOrielle => &[
RequirementKey::Sky_Field_TalkToOrielle,
],
Event::TalkToPeatriceInBazaar => &[
RequirementKey::Bazaar_Main_TalkToPeatriceInBazaar,
],
Event::TalkToYerbal => &[
RequirementKey::FaronWoods_GreatTreeTop_TalkToYerbal,
],
Event::TempleEntranceStatue => &[
RequirementKey::EldinVolcano_OutsideEt_TempleEntranceStatue,
],
Event::UnlockedZeldasRoom => &[
RequirementKey::KnightAcademy_AboveZeldasRoom_UnlockedZeldasRoom,
],
Event::ViewingPlatformStatue => &[
RequirementKey::FaronWoods_Main_ViewingPlatformStatue,
],
Event::VolcanoAscentStatue => &[
RequirementKey::EldinVolcano_VolcanoAscent_VolcanoAscentStatue,
],
Event::VolcanoEastStatue => &[
RequirementKey::EldinVolcano_PreMogmaTurf_VolcanoEastStatue,
],
Event::WestDesertStatue => &[
RequirementKey::LanayruDesert_SandOasis_WestDesertStatue,
],
}
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequirementKey {
    AncientCistern_AfterAcGutters_To_AfterWhipHooks,
    AncientCistern_AfterAcGutters_To_MainRoomVines,
    AncientCistern_AfterWhipHooks_To_MainRoomVines,
    AncientCistern_BeforeBokoKeyDoor_To_AfterAcGutters,
    AncientCistern_BehindWaterfall_To_BeforeBokoKeyDoor,
    AncientCistern_MainBasement_To_BeforeBossDoor,
    AncientCistern_MainBasement_To_BossKeyChestArea,
    AncientCistern_MainBasement_To_SpiderThread,
    AncientCistern_MainHub_To_AfterWhipHooks,
    AncientCistern_MainHub_To_BeforeBossDoor,
    AncientCistern_MainHub_To_BehindWaterfall,
    AncientCistern_MainHub_To_MainBasement,
    AncientCistern_MainHub_To_SpiderThread,
    AncientCistern_MainHub_To_WhipChestRoom,
    AncientCistern_MainRoomVines_To_AfterWhipHooks,
    AncientCistern_SpiderThread_To_BossKeyChestArea,
    AncientCistern_SpiderThread_To_MainBasement,
    Skyloft_CentralOutside_To_OutsideGoddessStatue,
    Skyloft_CentralOutside_To_OutsideSkyloftVillage,
    Skyloft_CentralOutside_To_ToSkyKeep,
    Skyloft_CentralOutside_To_WaterfallCaveCrystals,
    Skyloft_PastWaterfallCave_To_WaterfallCaveCrystals,
    Skyloft_ToSkyKeep_To_CentralOutside,
    EarthTemple_AfterBallRolling_To_BossDoorArea,
    EarthTemple_BallRolling_To_AfterBallRolling,
    EarthTemple_Entrance_To_BallRolling,
    EldinVolcano_FirstRoom_To_PreMogmaTurf,
    EldinVolcano_FirstRoom_To_VolcanoAscent,
    EldinVolcano_HotCaveArea_To_SandSlide,
    EldinVolcano_NearThrillDigger_To_OutsideEt,
    EldinVolcano_NearThrillDigger_To_VolcanoAscent,
    EldinVolcano_OutsideEt_To_HotCaveArea,
    EldinVolcano_PastMogmaTurf_To_PreMogmaTurf,
    EldinVolcano_PastMogmaTurf_To_VolcanoAscent,
    EldinVolcano_PastSlide_To_HotCaveArea,
    EldinVolcano_PastSlide_To_VolcanoAscent,
    EldinVolcano_PreMogmaTurf_To_FirstRoom,
    EldinVolcano_PreMogmaTurf_To_PastMogmaTurf,
    EldinVolcano_SandSlide_To_PastSlide,
    EldinVolcano_VolcanoAscent_To_FirstRoom,
    EldinVolcano_VolcanoAscent_To_NearThrillDigger,
    EldinVolcano_VolcanoAscent_To_PastSlide,
    DeepWoods_Entry_To_PastBeehive,
    DeepWoods_PastBeehive_To_Entry,
    FaronWoods_Entry_To_Main,
    FaronWoods_GreatTreeTop_To_ClawshotTargetBranch,
    FaronWoods_GreatTreeTop_To_GreatTreePlatforms,
    FaronWoods_Main_To_ClawshotTargetBranch,
    FaronWoods_Main_To_Entry,
    FaronWoods_Main_To_GreatTreePlatforms,
    GreatTree_Entry_To_Lower,
    GreatTree_Lower_To_Entry,
    GreatTree_Lower_To_Middle,
    GreatTree_Lower_To_PastPlatforms,
    GreatTree_Middle_To_Lower,
    GreatTree_Middle_To_PastPlatforms,
    GreatTree_PastPlatforms_To_Lower,
    GreatTree_Upper_To_Middle,
    FireSanctuaryA_Entry_To_PastFirstWaterPlant,
    FireSanctuaryA_InFrontOfBossDoor_To_UpperStaircaseRoom,
    FireSanctuaryA_PrePlatsArea_To_InFrontOfBossDoor,
    FireSanctuaryA_UpperStaircaseRoom_To_InFrontOfBossDoor,
    FireSanctuaryB_AfterDoubleMagmanosFight_To_UnderDoubleMagmanosFight,
    FireSanctuaryB_FirstOutsideSection_To_PastSecondRoomWithWaterFruit,
    FireSanctuaryB_PastSecondRoomWithWaterFruit_To_WaterFruitRoom,
    FireSanctuaryB_UnderDoubleMagmanosFight_To_LastTrappedMogmaArea,
    FireSanctuaryB_WaterFruitRoom_To_AfterDoubleMagmanosFight,
    KnightAcademy_AboveZeldasRoom_To_Main,
    Skyloft_OutsideGoddessStatue_To_CentralOutside,
    LakeFloria_Entry_To_StatueSpot,
    LakeFloria_StatueSpot_To_ToFaroresLair,
    LakeFloria_ToFaroresLair_To_StatueSpot,
    LanayruCaves_Main_To_ToSandSea,
    LanayruCaves_ToSandSea_To_Main,
    FireNode_Main_To_End,
    LanayruDesert_HookBeetleArea_To_PastToT,
    LanayruDesert_HookBeetleArea_To_SandOasis,
    LanayruDesert_PastToT_To_HookBeetleArea,
    LanayruDesert_SandOasis_To_HookBeetleArea,
    TempleOfTime_AfterLmf_To_NearGossipStone,
    TempleOfTime_End_To_Start,
    TempleOfTime_NearCube_To_End,
    TempleOfTime_NearGossipStone_To_NearCube,
    TempleOfTime_Start_To_NearGossipStone,
    LanayruMines_FirstHalf_To_ToCaves,
    LanayruMines_FirstHalf_To_ToDesert,
    LanayruMines_ToCaves_To_FirstHalf,
    LanayruMiningFacilityA_Entry_To_SecondRoom,
    LanayruMiningFacilityA_FirstKeyLockedRoom_To_GustBellowsRoom,
    LanayruMiningFacilityA_FirstWestRoom_To_MapRoom,
    LanayruMiningFacilityA_SecondRoom_To_FirstKeyLockedRoom,
    LanayruMiningFacilityA_SecondRoom_To_FirstWestRoom,
    LanayruMiningFacilityB_AfterLmfBkRoom_To_InsideLmfBkRoom,
    LanayruMiningFacilityB_AfterLmfBkRoom_To_NearBossDoor,
    LanayruMiningFacilityB_HubRoom_To_NearFirstHubRoomChest,
    LanayruMiningFacilityB_InsideLmfBkRoom_To_AfterLmfBkRoom,
    LanayruMiningFacilityB_NearBossDoor_To_AfterLmfBkRoom,
    LanayruMiningFacilityB_NearBossDoor_To_InsideLmfBkRoom,
    LanayruMiningFacilityB_WestHub_To_NearBossDoor,
    LanayruMiningFacilityToToT_BossDoor_To_ToTExit,
    OutsidePiratesStronghold_Main_To_InsideSharkhead,
    SandSeaDocks_ToCaves_To_Main,
    Shipyard_Main_To_AfterMinecartRide,
    ShipyardConstructionBay_Upper_To_Lower,
    SkippersRetreat_NextToShack_To_PastDekuBaba,
    SkippersRetreat_PastDekuBaba_To_NextToShack,
    SkippersRetreat_PastMoblin_To_PastDekuBaba,
    SkippersRetreat_Start_To_PastMoblin,
    Sandship_Deck_To_PastSpume,
    Sandship_PastSpume_To_SandshipBrig,
    SealedGroundsSpiral_Lower_To_Upper,
    SealedGroundsSpiral_Upper_To_Lower,
    Sky_BeedlesSkyHome_To_BeedleIslandCage,
    Sky_Field_To_BeedleIslandCage,
    Sky_Field_To_OutsideLumpyPumpkin,
    Sky_OutsideLumpyPumpkin_To_Field,
    Skyloft_OutsideSkyloftVillage_To_CentralOutside,
    Skyloft_OutsideSkyloftVillage_To_PastWaterfallCave,
    SkyviewTemple_Entry_To_FirstHub,
    SkyviewTemple_FirstHub_To_MainHub,
    SkyviewTemple_MainHub_To_BossDoorArea,
    OutsideFireSanctuary_Middle_To_ToFireSanctuary,
    OutsideFireSanctuary_ToInsideSummit_To_Middle,
    AncientCistern_ChestAfterWhipHooks,
    AncientCistern_Bokoblin,
    AncientCistern_ChestBehindTheWaterfall,
    AncientCistern_BossKeyChest,
    AncientCistern_ChestInEastPart,
    AncientCistern_ChestNearVines,
    AncientCistern_Whip,
    AncientCistern_KoloktosHeartContainer,
    AncientCistern_FaroresFlame,
    Batreaux_10Crystals,
    Batreaux_30Crystals,
    Batreaux_30CrystalsChest,
    Batreaux_40Crystals,
    Batreaux_5Crystals,
    Batreaux_50Crystals,
    Batreaux_70Crystals,
    Batreaux_70CrystalsSecondReward,
    Batreaux_80Crystals,
    Beedle_1000RupeeItem,
    Beedle_1200RupeeItem,
    Beedle_1600RupeeItem,
    Beedle_300RupeeItem,
    Beedle_50RupeeItem,
    Beedle_600RupeeItem,
    Beedle_800RupeeItem,
    Beedle_First100RupeeItem,
    Beedle_Second100RupeeItem,
    Beedle_Third100RupeeItem,
    CentralSkyloft_BazaarGoddessChest,
    CentralSkyloft_PotionLadysGift,
    CentralSkyloft_CrystalInOrielleAndParrowsHouse,
    CentralSkyloft_PeaterPeatricesCrystals,
    CentralSkyloft_CrystalBetweenWoodenPlanks,
    CentralSkyloft_CrystalOnLightTower,
    CentralSkyloft_CrystalOnWaterfallIsland,
    CentralSkyloft_CrystalOnWestCliff,
    CentralSkyloft_FloatingIslandGoddessChest,
    CentralSkyloft_FloatingIslandGossipStone,
    CentralSkyloft_ItemInBirdNest,
    CentralSkyloft_ParrowsCrystals,
    CentralSkyloft_ParrowsGift,
    CentralSkyloft_ShedChest,
    CentralSkyloft_ShedGoddessChest,
    CentralSkyloft_WaterfallGoddessChest,
    CentralSkyloft_WestCliffGoddessChest,
    CentralSkyloft_CrystalAfterWaterfallCave,
    CentralSkyloft_CrystalInLoftwingPrison,
    CentralSkyloft_WaterfallCaveFirstChest,
    CentralSkyloft_WaterfallCaveSecondChest,
    CentralSkyloft_WrynasCrystals,
    EarthTemple_ChestGuardedByLizalfos,
    EarthTemple_BombBag,
    EarthTemple_ChestLeftOfMainRoomBridge,
    EarthTemple_ChestBehindBombableRock,
    EarthTemple_ChestInWestRoom,
    EarthTemple_LeddsGift,
    EarthTemple_BossKeyChest,
    EarthTemple_VentChest,
    EarthTemple_ScalderaHeartContainer,
    EarthTemple_AmberTablet,
    EldinSilentRealm_FireshieldEarrings,
    EldinVolcano_ChestBehindBombableWallInFirstRoom,
    EldinVolcano_DiggingSpotBehindBoulderOnSandySlope,
    EldinVolcano_DiggingSpotBelowTower,
    EldinVolcano_DiggingSpotInFrontOfEarthTemple,
    EldinVolcano_GossipStoneNextToEarthTemple,
    EldinVolcano_DiggingSpotAfterDrainingLava,
    EldinVolcano_ChestAfterCrawlspace,
    EldinVolcano_ChestBehindBombableWallNearCliff,
    EldinVolcano_ItemOnCliff,
    EldinVolcano_DiggingSpotAfterVents,
    EldinVolcano_ChestBehindBombableWallNearVolcanoAscent,
    EldinVolcano_GossipStoneInThrillDiggerCave,
    FaronSilentRealm_WaterScale,
    FaronWoods_DeepWoodsChest,
    FaronWoods_ChestBehindBombableRocksNearErla,
    FaronWoods_ItemBehindBombableRock,
    FaronWoods_ItemOnTree,
    FaronWoods_Slingshot,
    FaronWoods_ChestInsideGreatTree,
    FireSanctuary_ChestInFirstRoom,
    FireSanctuary_PlatsChest,
    FireSanctuary_BossKeyChest,
    FireSanctuary_ChestInStaircaseRoom,
    FireSanctuary_MogmaMitts,
    FireSanctuary_ChestInSecondRoom,
    FireSanctuary_ChestOnBalcony,
    FireSanctuary_ChestAfterBombableWall,
    FireSanctuary_ChestAfterSecondTrappedMogma,
    FireSanctuary_ChestNearFirstTrappedMogma,
    FireSanctuary_FirstChestInWaterFruitRoom,
    FireSanctuary_SecondChestInWaterFruitRoom,
    FireSanctuary_GhirahimHeartContainer,
    FireSanctuary_DinsFlame,
    KnightAcademy_ChestInGoddessStatue,
    KnightAcademy_CawlinsLetter,
    KnightAcademy_CrystalInKnightAcademyPlant,
    KnightAcademy_CrystalInLinksRoom,
    KnightAcademy_CrystalInZeldasRoom,
    KnightAcademy_FledgesCrystals,
    KnightAcademy_FledgesGift,
    KnightAcademy_GhostPipitsCrystals,
    KnightAcademy_InZeldasCloset,
    KnightAcademy_OwlansCrystals,
    KnightAcademy_ChestNearGoddessStatue,
    KnightAcademy_OwlansGift,
    KnightAcademy_PumpkinArchery600Points,
    KnightAcademy_CrystalInSparringHall,
    KnightAcademy_SparringHallChest,
    LakeFloria_DragonLairEastChest,
    LakeFloria_DragonLairSouthChest,
    LakeFloria_LakeFloriaChest,
    LanayruCaves_Chest,
    LanayruCaves_GolosGift,
    LanayruCaves_GossipStoneInCenter,
    LanayruDesert_FireNodeLeftEndingChest,
    LanayruDesert_FireNodeRightEndingChest,
    LanayruDesert_FireNodeFirstSmallChest,
    LanayruDesert_FireNodeSecondSmallChest,
    LanayruDesert_FireNodeShortcutChest,
    LanayruDesert_ChestNearHookBeetleFight,
    LanayruDesert_ChestNearPartyWheel,
    LanayruDesert_HookBeetleFight,
    LanayruDesert_ChestOnPlatformNearFireNode,
    LanayruDesert_ChestOnPlatformNearLightningNode,
    LanayruDesert_ChestOnTopOfLanayruMiningFacility,
    LanayruDesert_SecretPassagewayChest,
    LanayruDesert_ChestNearSandOasis,
    LanayruDesert_LightningNodeFirstChest,
    LanayruDesert_LightningNodeRaisedChestNearGenerator,
    LanayruDesert_LightningNodeSecondChest,
    LanayruDesert_GossipStoneInTempleOfTimeArea,
    LanayruMines_ChestAtTheEndOfMines,
    LanayruMines_ChestBehindFirstLanding,
    LanayruMines_ChestBehindStatue,
    LanayruMines_ChestNearFirstTimeshiftStone,
    LanayruMiningFacility_ChestBehindBars,
    LanayruMiningFacility_ChestInKeyLockedRoom,
    LanayruMiningFacility_ChestInFirstWestRoom,
    LanayruMiningFacility_ChestInsideGustBellowsRoom,
    LanayruMiningFacility_GustBellows,
    LanayruMiningFacility_ChestAfterArmosFight,
    LanayruMiningFacility_ShortcutChestInMainHub,
    LanayruMiningFacility_BossKeyChest,
    LanayruMiningFacility_FirstChestInHubRoom,
    LanayruMiningFacility_ChestBehindFirstCrawlspace,
    LanayruMiningFacility_ChestInSpikeMaze,
    LanayruMiningFacility_MolderachHeartContainer,
    LanayruMiningFacility_GoddessHarp,
    LanayruSandSea_PirateStrongholdFirstChest,
    LanayruSandSea_PirateStrongholdSecondChest,
    LanayruSandSea_PirateStrongholdThirdChest,
    LanayruSandSea_GossipStoneInShipyard,
    LanayruSandSea_RicketyCoasterHeartStoppingTrackIn105,
    LanayruSandSea_SkippersRetreatSkydiveChest,
    LanayruSandSea_SkippersRetreatChestOnTopOfCactiPillar,
    LanayruSandSea_SkippersRetreatChestAfterMoblin,
    LanayruSandSea_SkippersRetreatChestInShack,
    LanayruSilentRealm_Clawshots,
    MogmaTurf_ChestBehindBombableWallAtEntrance,
    MogmaTurf_ChestBehindBombableWallInFireMaze,
    MogmaTurf_DiggingMittsFight,
    MogmaTurf_FreeFallChest,
    MogmaTurf_SandSlideChest,
    Sandship_BossKeyChest,
    Sandship_Bow,
    Sandship_ChestAtTheStern,
    Sandship_ChestBefore4DoorCorridor,
    Sandship_ChestBehindCombinationLock,
    Sandship_RobotInBrigsReward,
    Sandship_TreasureRoomFifthChest,
    Sandship_TreasureRoomFirstChest,
    Sandship_TreasureRoomFourthChest,
    Sandship_TreasureRoomSecondChest,
    Sandship_TreasureRoomThirdChest,
    Sandship_NayrusFlame,
    Sandship_TentalusHeartContainer,
    SealedGrounds_GorkosGoddessWallReward,
    SealedGrounds_ZeldasBlessing,
    SealedGrounds_ChestInsideSealedTemple,
    SealedGrounds_SongFromImpa,
    Sky_GossipStoneInsideBambooIsland,
    Sky_CrystalInsideLumpyPumpkin,
    Sky_LumpyPumpkinChandelier,
    Sky_LumpyPumpkinHarpMinigame,
    Sky_BeedlesIslandCageGoddessChest,
    Sky_BeedlesCrystals,
    Sky_CrystalOnBeedlesShip,
    Sky_BambooIslandGoddessChest,
    Sky_BeedlesIslandGoddessChest,
    Sky_ChestInBreakableBoulderNearFunFunIsland,
    Sky_ChestInBreakableBoulderNearLumpyPumpkin,
    Sky_DodohsCrystals,
    Sky_FunFunIslandMinigame500Rupees,
    Sky_GoddessChestInCaveOnIslandNextToBambooIsland,
    Sky_GoddessChestInsideVolcanicIsland,
    Sky_GoddessChestOnIslandClosestToFaronPillar,
    Sky_GoddessChestOnIslandNextToBambooIsland,
    Sky_GoddessChestOutsideVolcanicIsland,
    Sky_GoddessChestUnderFunFunIsland,
    Sky_GossipStoneInVolcanicIsland,
    Sky_LumpyPumpkinGoddessChestOnTheRoof,
    Sky_NortheastIslandCageGoddessChest,
    Sky_NortheastIslandGoddessChestBehindBombableRocks,
    Sky_OriellesCrystals,
    Sky_SouthwestTripleIslandCageGoddessChest,
    Sky_SouthwestTripleIslandLowerGoddessChest,
    Sky_SouthwestTripleIslandUpperGoddessChest,
    Sky_CrystalOutsideLumpyPumpkin,
    Sky_KinasCrystals,
    Sky_LumpyPumpkinOutsideGoddessChest,
    SkyKeep_ChestAfterDreadfuse,
    SkyKeep_FirstChest,
    SkyloftSilentRealm_StoneOfTrials,
    SkyloftVillage_BertiesCrystals,
    SkyloftVillage_MallarasCrystals,
    SkyloftVillage_CrystalNearPumpkinPatch,
    SkyloftVillage_SparrotsCrystals,
    Skyview_GhirahimHeartContainer,
    Skyview_RubyTablet,
    Skyview_BossKeyChest,
    Skyview_ChestNearBossDoor,
    Skyview_ChestBehindTwoEyes,
    Skyview_ChestOnTreeBranch,
    Skyview_DiggingSpotInCrawlspace,
    Skyview_Beetle,
    Skyview_ChestBehindThreeEyes,
    Skyview_ItemBehindBars,
    Thunderhead_BugHeaven10BugsIn3Minutes,
    Thunderhead_BugHeavenGoddessChest,
    Thunderhead_EastIslandChest,
    Thunderhead_EastIslandGoddessChest,
    Thunderhead_FirstGoddessChestOnMogmaMittsIsland,
    Thunderhead_GoddessChestOnTopOfIsleOfSongs,
    Thunderhead_GoddessChestOutsideIsleOfSongs,
    Thunderhead_SongFromLevias,
    Thunderhead_IsleOfSongsDinsPower,
    Thunderhead_IsleOfSongsFaroresCourage,
    Thunderhead_IsleOfSongsNayrusWisdom,
    VolcanoSummit_BokoBasePouchChest,
    VolcanoSummit_SmallChestInVolcanoSummit,
    VolcanoSummit_GossipStoneOutsideFireSanctuary,
    VolcanoSummit_ItemBehindDigging,
    VolcanoSummit_ChestBehindBombableWallInWaterfallArea,
    VolcanoSummit_GossipStoneInWaterfallArea,
    AncientCistern_To_AncientCisternBoss,
    AncientCistern_To_FloriaWaterfall,
    AncientCisternBoss_To_AncientCisternCandleRoom,
    BatreauxHouse_To_Skyloft,
    BeedlesShop_To_Sky_Night,
    BeedlesShop_To_Skyloft_Day,
    Bazaar_To_Skyloft_North,
    Bazaar_To_Skyloft_South,
    Bazaar_To_Skyloft_West,
    ParrowAndOriellesHouse_To_Skyloft,
    PeatricesHouse_To_Skyloft,
    PipersHouse_To_Skyloft,
    Skyloft_To_Bazaar_North,
    Skyloft_To_Bazaar_South,
    Skyloft_To_Bazaar_West,
    Skyloft_To_BeedlesShop_Day,
    Skyloft_To_ParrowAndOriellesHouse,
    Skyloft_To_PeatricesHouse,
    Skyloft_To_PipersHouse,
    Skyloft_To_Sky,
    Skyloft_To_SkyloftSilentRealm,
    Skyloft_To_WaterfallCave_Upper,
    Skyloft_To_WrynasHouse,
    Skyloft_To_Sky_PastWaterfallCave,
    Skyloft_To_WaterfallCave_Lower,
    Skyloft_To_SkyKeepEntry,
    WaterfallCave_To_Skyloft_Upper,
    WaterfallCave_To_Skyloft_Lower,
    WrynasHouse_To_Skyloft,
    EarthTemple_To_EarthTempleBoss,
    EarthTemple_To_EldinVolcano,
    EarthTempleBoss_To_EarthTempleSpring,
    EarthTempleSpring_To_EldinVolcano,
    EldinSilentRealm_To_EldinVolcano,
    EldinVolcano_To_Sky_EldinEntranceStatue,
    EldinVolcano_To_InsideVolcanoSummit,
    EldinVolcano_To_ThrillDiggerCave,
    EldinVolcano_To_EarthTemple,
    EldinVolcano_To_Sky_TempleEntranceStatue,
    EldinVolcano_To_MogmaTurf_Skydive,
    EldinVolcano_To_Sky_VolcanoEastStatue,
    EldinVolcano_To_EldinSilentRealm,
    EldinVolcano_To_Sky_VolcanoAscentStatue,
    ThrillDiggerCave_To_EldinVolcano,
    FaronSilentRealm_To_FaronWoods,
    DeepWoods_To_FaronWoods,
    DeepWoods_To_Sky_DeepWoodsStatue,
    DeepWoods_To_Sky_ForestTempleStatue,
    DeepWoods_To_SkyviewTemple,
    FaronWoods_To_BehindTheTemple,
    FaronWoods_To_Sky_FaronWoodsEntryStatue,
    FaronWoods_To_GreatTree_LowerPlatform,
    FaronWoods_To_GreatTree_UpperPlatform,
    FaronWoods_To_GreatTree_Top,
    FaronWoods_To_Sky_GreatTreeStatue,
    FaronWoods_To_DeepWoods,
    FaronWoods_To_FaronSilentRealm,
    FaronWoods_To_GreatTree_Tunnel,
    FaronWoods_To_LakeFloria,
    FaronWoods_To_Sky_InTheWoodsStatue,
    FaronWoods_To_Sky_ViewingPlatformStatue,
    GreatTree_To_FaronWoods_Tunnel,
    GreatTree_To_FaronWoods_LowerPlatform,
    GreatTree_To_FaronWoods_UpperPlatform,
    GreatTree_To_FaronWoods_Top,
    FireSanctuaryA_To_OutsideFireSanctuary,
    FireSanctuaryA_To_FireSanctuaryBoss,
    FireSanctuaryA_To_FireSanctuaryB,
    FireSanctuaryB_To_FireSanctuaryA,
    FireSanctuaryBoss_To_FireSanctuaryFlameRoom,
    InsideGoddessStatue_To_Skyloft,
    KnightAcademy_To_Skyloft_LowerDoors,
    KnightAcademy_To_Skyloft_UpperDoors,
    Skyloft_To_InsideGoddessStatue,
    Skyloft_To_KnightAcademy_Chimney,
    Skyloft_To_KnightAcademy_LowerDoors,
    Skyloft_To_KnightAcademy_UpperDoors,
    Skyloft_To_SparringHall,
    SparringHall_To_Skyloft,
    FaroresLair_To_FloriaWaterfall,
    FaroresLair_To_LakeFloria,
    FloriaWaterfall_To_AncientCistern,
    FloriaWaterfall_To_FaronWoods,
    FloriaWaterfall_To_FaroresLair,
    FloriaWaterfall_To_Sky_FloriaWaterfallStatue,
    LakeFloria_To_Sky_LakeFloriaStatue,
    LakeFloria_To_FaroresLair,
    LanayruCaves_To_LanayruDesert,
    LanayruCaves_To_LanayruMines,
    LanayruCaves_To_SandSeaDocks,
    FireNode_To_LanayruDesert,
    LanayruDesert_To_LanayruMines,
    LanayruDesert_To_Sky_DesertEntranceStatue,
    LanayruDesert_To_FireNode,
    LanayruDesert_To_LanayruMiningFacilityA,
    LanayruDesert_To_LanayruSilentRealm,
    LanayruDesert_To_LightningNode,
    LanayruDesert_To_Sky_NorthDesertStatue,
    LanayruDesert_To_Sky_StoneCacheStatue,
    LanayruDesert_To_TempleOfTime_End,
    LanayruDesert_To_LanayruCaves,
    LanayruDesert_To_Sky_WestDesertStatue,
    LanayruDesert_To_TempleOfTime_Start,
    LightningNode_To_LanayruDesert,
    TempleOfTime_To_LanayruDesert_End,
    TempleOfTime_To_LanayruDesert_Start,
    LanayruMines_To_Sky_LanayruMineEntryStatue,
    LanayruMines_To_LanayruCaves,
    LanayruMines_To_LanayruDesert,
    LanayruMiningFacilityA_To_LanayruDesert,
    LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub2,
    LanayruMiningFacilityA_To_LanayruMiningFacilityB_HubW,
    LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub,
    LanayruMiningFacilityB_To_LanayruMiningFacilityBoss,
    LanayruMiningFacilityBoss_To_LanayruMiningFacilityToToT,
    LanayruMiningFacilityToToT_To_TempleOfTime,
    InsidePiratesStronghold_To_OutsidePiratesStronghold_End,
    InsidePiratesStronghold_To_OutsidePiratesStronghold_Beginning,
    OutsidePiratesStronghold_To_InsidePiratesStronghold_End,
    OutsidePiratesStronghold_To_InsidePiratesStronghold_Beginning,
    OutsidePiratesStronghold_To_SandSea,
    SandSea_To_OutsidePiratesStronghold,
    SandSea_To_SandSeaDocks,
    SandSea_To_Sandship,
    SandSea_To_Shipyard,
    SandSea_To_SkippersRetreat,
    SandSeaDocks_To_SandSea,
    SandSeaDocks_To_Sky_AncientHarbor,
    SandSeaDocks_To_LanayruCaves,
    Shipyard_To_ShipyardConstructionBay_Upper,
    Shipyard_To_SandSea,
    Shipyard_To_ShipyardConstructionBay_Lower,
    ShipyardConstructionBay_To_Shipyard_Lower,
    ShipyardConstructionBay_To_Shipyard_Upper,
    SkippersRetreat_To_SkippersShack,
    SkippersRetreat_To_SandSea,
    SkippersShack_To_SkippersRetreat,
    LanayruSilentRealm_To_LanayruDesert,
    MogmaTurf_To_EldinVolcano_EndVent,
    MogmaTurf_To_EldinVolcano_StartVent,
    Sandship_To_SandSea,
    Sandship_To_SandshipBoss,
    BehindTheTemple_To_FaronWoods,
    BehindTheTemple_To_SealedGroundsSpiral,
    BehindTheTemple_To_SealedTemple,
    BehindTheTemple_To_Sky_BehindTheTempleStatue,
    SealedGroundsSpiral_To_SealedTemple,
    SealedGroundsSpiral_To_Sky_SealedGroundsStatue,
    SealedTemple_To_BehindTheTemple,
    SealedTemple_To_HyliasTemple,
    SealedTemple_To_SealedGroundsSpiral,
    InsideBambooIsland_To_Sky,
    LumpyPumpkin_To_Sky_North,
    LumpyPumpkin_To_Sky_South,
    Sky_To_BeedlesShop_Night,
    Sky_To_BehindTheTemple_BehindTheTempleStatue,
    Sky_To_DeepWoods_DeepWoodsStatue,
    Sky_To_DeepWoods_ForestTempleStatue,
    Sky_To_EldinVolcano_EldinEntranceStatue,
    Sky_To_EldinVolcano_TempleEntranceStatue,
    Sky_To_EldinVolcano_VolcanoEastStatue,
    Sky_To_EldinVolcano_VolcanoAscentStatue,
    Sky_To_FaronWoods_FaronWoodsEntryStatue,
    Sky_To_FaronWoods_GreatTreeStatue,
    Sky_To_FaronWoods_InTheWoodsStatue,
    Sky_To_FaronWoods_ViewingPlatformStatue,
    Sky_To_FloriaWaterfall_FloriaWaterfallStatue,
    Sky_To_InsideBambooIsland,
    Sky_To_InsideThunderhead,
    Sky_To_LakeFloria_LakeFloriaStatue,
    Sky_To_LanayruDesert_DesertEntranceStatue,
    Sky_To_LanayruDesert_NorthDesertStatue,
    Sky_To_LanayruDesert_StoneCacheStatue,
    Sky_To_LanayruDesert_WestDesertStatue,
    Sky_To_LanayruMines_LanayruMineEntryStatue,
    Sky_To_OutsideFireSanctuary_InsideTheVolcanoStatue,
    Sky_To_SealedGroundsSpiral_SealedGroundsStatue,
    Sky_To_Skyloft,
    Sky_To_LumpyPumpkin_North,
    Sky_To_LumpyPumpkin_South,
    SkyKeepEntry_To_Skyloft,
    SkyloftSilentRealm_To_Skyloft,
    BertiesHouse_To_Skyloft,
    GondosHouse_To_Skyloft,
    MallarasHouse_To_Skyloft,
    RupinsHouse_To_Skyloft,
    Skyloft_To_BatreauxHouse,
    Skyloft_To_BertiesHouse,
    Skyloft_To_GondosHouse,
    Skyloft_To_MallarasHouse,
    Skyloft_To_RupinsHouse,
    Skyloft_To_SparrotsHouse,
    SparrotsHouse_To_Skyloft,
    SkyviewBoss_To_SkyviewSpring,
    SkyviewBoss_To_SkyviewTemple,
    SkyviewSpring_To_DeepWoods,
    SkyviewSpring_To_SkyviewBoss,
    SkyviewTemple_To_SkyviewBoss,
    SkyviewTemple_To_DeepWoods,
    InsideThunderhead_To_IsleOfSongs,
    InsideThunderhead_To_Sky,
    IsleOfSongs_To_InsideThunderhead,
    InsideVolcanoSummit_To_EldinVolcano,
    InsideVolcanoSummit_To_OutsideFireSanctuary,
    InsideVolcanoSummit_To_VolcanoSummitWaterfall,
    OutsideFireSanctuary_To_FireSanctuaryA,
    OutsideFireSanctuary_To_Sky_InsideTheVolcanoStatue,
    OutsideFireSanctuary_To_InsideVolcanoSummit,
    VolcanoSummitWaterfall_To_InsideVolcanoSummit,
    FireNode_End_ActivateFireNode,
    LightningNode_Main_ActivateLightningNode,
    SkyviewTemple_Entry_ActivateSkyviewGoddessWall,
    LanayruDesert_PastToT_ActivateWaterNode,
    SealedTemple_Main_BeatRequiredDungeons,
    BehindTheTemple_Main_BehindTheTempleStatue,
    AncientCisternCandleRoom_Main_CanBeatAncientCistern,
    EarthTempleSpring_Main_CanBeatEarthTemple,
    FireSanctuaryFlameRoom_Main_CanBeatFireSanctuary,
    LanayruMiningFacilityToToT_ToTExit_CanBeatLanayruMiningFacility,
    SandshipBoss_Main_CanBeatSandship,
    SkyKeepEntry_Main_CanBeatSkyKeep,
    SkyviewSpring_Main_CanBeatSkyview,
    Sandship_Deck_CanFreelyChangeSandshipTemporality,
    AncientCistern_MainRoomVines_CanLowerAcStatue,
    AncientCistern_SpiderThread_CanLowerAcStatue,
    InsideBambooIsland_Main_CanPlayCleanCut,
    ThrillDiggerCave_Main_CanPlayThrillDigger,
    LanayruDesert_HookBeetleArea_CanRetrievePartyWheel,
    RupinsHouse_Main_CanSellTreasures,
    DeepWoods_PastBeehive_DeepWoodsStatue,
    SealedGroundsSpiral_Lower_DefeatImprisoned2,
    ShipyardConstructionBay_Lower_DefeatedShipyardMolderach,
    SparringHall_Main_DeliveredHotSoup,
    LanayruDesert_HookBeetleArea_DesertEntranceStatue,
    EldinVolcano_FirstRoom_EldinEntranceStatue,
    Bazaar_Main_EndurancePotion,
    FaronWoods_Entry_FaronWoodsEntryStatue,
    FloriaWaterfall_Main_FloriaWaterfallStatue,
    DeepWoods_PastBeehive_ForestTempleStatue,
    EldinVolcano_OutsideEt_GoddessCubeEastOfEarthTempleEntrance,
    EldinVolcano_OutsideEt_GoddessCubeWestOfEarthTempleEntrance,
    EldinVolcano_FirstRoom_GoddessCubeAtEldinEntrance,
    LanayruMines_FirstHalf_GoddessCubeAtLanayruMinesEntrance,
    TempleOfTime_NearCube_GoddessCubeAtRideNearTempleOfTime,
    SandSeaDocks_Main_GoddessCubeInAncientHarbour,
    DeepWoods_PastBeehive_GoddessCubeInDeepWoods,
    EldinVolcano_SandSlide_GoddessCubeInEldinSlide,
    FloriaWaterfall_Main_GoddessCubeInFloriaWaterfall,
    LakeFloria_StatueSpot_GoddessCubeInLakeFloria,
    MogmaTurf_Main_GoddessCubeInMogmaTurf,
    OutsidePiratesStronghold_InsideSharkhead_GoddessCubeInPirateStronghold,
    LanayruDesert_SandOasis_GoddessCubeInSandOasis,
    LanayruDesert_PastToT_GoddessCubeInSecretPassagewayInDesert,
    SkippersRetreat_PastMoblin_GoddessCubeInSkippersRetreat,
    SkyviewSpring_Main_GoddessCubeInSkyviewSpring,
    VolcanoSummitWaterfall_Main_GoddessCubeInSummitWaterfall,
    InsideVolcanoSummit_Main_GoddessCubeInsideVolcanoSummit,
    OutsideFireSanctuary_ToFireSanctuary_GoddessCubeNearFsEntrance,
    LanayruDesert_HookBeetleArea_GoddessCubeNearHookBeetleFight,
    EldinVolcano_PastMogmaTurf_GoddessCubeNearMogmaTurfEntrance,
    EldinVolcano_PreMogmaTurf_GoddessCubeNearMogmaTurfEntrance,
    FaronWoods_ClawshotTargetBranch_GoddessCubeOnEastGreatTreeWithClawshotsTarget,
    FaronWoods_GreatTreeTop_GoddessCubeOnEastGreatTreeWithRope,
    FaronWoods_GreatTreePlatforms_GoddessCubeOnWestGreatTreeNearExit,
    DeepWoods_PastBeehive_GoddessCubeOnTopOfSkyview,
    FaronWoods_GreatTreeTop_GreatTreeStatue,
    Sky_Field_HighRupeeFarm,
    FaronWoods_Main_InTheWoodsStatue,
    DeepWoods_PastBeehive_InitialGoddessCube,
    OutsideFireSanctuary_ToFireSanctuary_InsideTheVolcanoStatue,
    LakeFloria_StatueSpot_LakeFloriaStatue,
    TempleOfTime_AfterLmf_LanayruDesertDungeonBeaten,
    LanayruMines_FirstHalf_LanayruMineEntryStatue,
    LumpyPumpkin_Main_LumpyPumpkinQuestStart,
    LanayruDesert_PastToT_NorthDesertStatue,
    InsidePiratesStronghold_Main_OpenSharkhead,
    Skyloft_OutsideSkyloftVillage_OpenedShed,
    MogmaTurf_Main_PickUpGuld,
    LumpyPumpkin_Main_PickUpLeviasSoup,
    Sky_OutsideLumpyPumpkin_PumpkinCarrying,
    SealedTemple_Main_RaiseGoT,
    LanayruDesert_PastToT_RaiseLanayruMiningFacility,
    EldinVolcano_OutsideEt_RetrieveCrystalBall,
    FaronWoods_Main_RetrieveOolo,
    Sky_Field_SaveOrielle,
    SealedGroundsSpiral_Upper_SealedGroundsStatue,
    SealedTemple_Main_StartImprisoned2,
    LanayruDesert_PastToT_StoneCacheStatue,
    Sky_Field_TalkToOrielle,
    Bazaar_Main_TalkToPeatriceInBazaar,
    FaronWoods_GreatTreeTop_TalkToYerbal,
    EldinVolcano_OutsideEt_TempleEntranceStatue,
    KnightAcademy_AboveZeldasRoom_UnlockedZeldasRoom,
    FaronWoods_Main_ViewingPlatformStatue,
    EldinVolcano_VolcanoAscent_VolcanoAscentStatue,
    EldinVolcano_PreMogmaTurf_VolcanoEastStatue,
    LanayruDesert_SandOasis_WestDesertStatue,
}
impl RequirementKey {
    pub const ALL: &'static [RequirementKey] = &[
RequirementKey::AncientCistern_AfterAcGutters_To_AfterWhipHooks,
RequirementKey::AncientCistern_AfterAcGutters_To_MainRoomVines,
RequirementKey::AncientCistern_AfterWhipHooks_To_MainRoomVines,
RequirementKey::AncientCistern_BeforeBokoKeyDoor_To_AfterAcGutters,
RequirementKey::AncientCistern_BehindWaterfall_To_BeforeBokoKeyDoor,
RequirementKey::AncientCistern_MainBasement_To_BeforeBossDoor,
RequirementKey::AncientCistern_MainBasement_To_BossKeyChestArea,
RequirementKey::AncientCistern_MainBasement_To_SpiderThread,
RequirementKey::AncientCistern_MainHub_To_AfterWhipHooks,
RequirementKey::AncientCistern_MainHub_To_BeforeBossDoor,
RequirementKey::AncientCistern_MainHub_To_BehindWaterfall,
RequirementKey::AncientCistern_MainHub_To_MainBasement,
RequirementKey::AncientCistern_MainHub_To_SpiderThread,
RequirementKey::AncientCistern_MainHub_To_WhipChestRoom,
RequirementKey::AncientCistern_MainRoomVines_To_AfterWhipHooks,
RequirementKey::AncientCistern_SpiderThread_To_BossKeyChestArea,
RequirementKey::AncientCistern_SpiderThread_To_MainBasement,
RequirementKey::Skyloft_CentralOutside_To_OutsideGoddessStatue,
RequirementKey::Skyloft_CentralOutside_To_OutsideSkyloftVillage,
RequirementKey::Skyloft_CentralOutside_To_ToSkyKeep,
RequirementKey::Skyloft_CentralOutside_To_WaterfallCaveCrystals,
RequirementKey::Skyloft_PastWaterfallCave_To_WaterfallCaveCrystals,
RequirementKey::Skyloft_ToSkyKeep_To_CentralOutside,
RequirementKey::EarthTemple_AfterBallRolling_To_BossDoorArea,
RequirementKey::EarthTemple_BallRolling_To_AfterBallRolling,
RequirementKey::EarthTemple_Entrance_To_BallRolling,
RequirementKey::EldinVolcano_FirstRoom_To_PreMogmaTurf,
RequirementKey::EldinVolcano_FirstRoom_To_VolcanoAscent,
RequirementKey::EldinVolcano_HotCaveArea_To_SandSlide,
RequirementKey::EldinVolcano_NearThrillDigger_To_OutsideEt,
RequirementKey::EldinVolcano_NearThrillDigger_To_VolcanoAscent,
RequirementKey::EldinVolcano_OutsideEt_To_HotCaveArea,
RequirementKey::EldinVolcano_PastMogmaTurf_To_PreMogmaTurf,
RequirementKey::EldinVolcano_PastMogmaTurf_To_VolcanoAscent,
RequirementKey::EldinVolcano_PastSlide_To_HotCaveArea,
RequirementKey::EldinVolcano_PastSlide_To_VolcanoAscent,
RequirementKey::EldinVolcano_PreMogmaTurf_To_FirstRoom,
RequirementKey::EldinVolcano_PreMogmaTurf_To_PastMogmaTurf,
RequirementKey::EldinVolcano_SandSlide_To_PastSlide,
RequirementKey::EldinVolcano_VolcanoAscent_To_FirstRoom,
RequirementKey::EldinVolcano_VolcanoAscent_To_NearThrillDigger,
RequirementKey::EldinVolcano_VolcanoAscent_To_PastSlide,
RequirementKey::DeepWoods_Entry_To_PastBeehive,
RequirementKey::DeepWoods_PastBeehive_To_Entry,
RequirementKey::FaronWoods_Entry_To_Main,
RequirementKey::FaronWoods_GreatTreeTop_To_ClawshotTargetBranch,
RequirementKey::FaronWoods_GreatTreeTop_To_GreatTreePlatforms,
RequirementKey::FaronWoods_Main_To_ClawshotTargetBranch,
RequirementKey::FaronWoods_Main_To_Entry,
RequirementKey::FaronWoods_Main_To_GreatTreePlatforms,
RequirementKey::GreatTree_Entry_To_Lower,
RequirementKey::GreatTree_Lower_To_Entry,
RequirementKey::GreatTree_Lower_To_Middle,
RequirementKey::GreatTree_Lower_To_PastPlatforms,
RequirementKey::GreatTree_Middle_To_Lower,
RequirementKey::GreatTree_Middle_To_PastPlatforms,
RequirementKey::GreatTree_PastPlatforms_To_Lower,
RequirementKey::GreatTree_Upper_To_Middle,
RequirementKey::FireSanctuaryA_Entry_To_PastFirstWaterPlant,
RequirementKey::FireSanctuaryA_InFrontOfBossDoor_To_UpperStaircaseRoom,
RequirementKey::FireSanctuaryA_PrePlatsArea_To_InFrontOfBossDoor,
RequirementKey::FireSanctuaryA_UpperStaircaseRoom_To_InFrontOfBossDoor,
RequirementKey::FireSanctuaryB_AfterDoubleMagmanosFight_To_UnderDoubleMagmanosFight,
RequirementKey::FireSanctuaryB_FirstOutsideSection_To_PastSecondRoomWithWaterFruit,
RequirementKey::FireSanctuaryB_PastSecondRoomWithWaterFruit_To_WaterFruitRoom,
RequirementKey::FireSanctuaryB_UnderDoubleMagmanosFight_To_LastTrappedMogmaArea,
RequirementKey::FireSanctuaryB_WaterFruitRoom_To_AfterDoubleMagmanosFight,
RequirementKey::KnightAcademy_AboveZeldasRoom_To_Main,
RequirementKey::Skyloft_OutsideGoddessStatue_To_CentralOutside,
RequirementKey::LakeFloria_Entry_To_StatueSpot,
RequirementKey::LakeFloria_StatueSpot_To_ToFaroresLair,
RequirementKey::LakeFloria_ToFaroresLair_To_StatueSpot,
RequirementKey::LanayruCaves_Main_To_ToSandSea,
RequirementKey::LanayruCaves_ToSandSea_To_Main,
RequirementKey::FireNode_Main_To_End,
RequirementKey::LanayruDesert_HookBeetleArea_To_PastToT,
RequirementKey::LanayruDesert_HookBeetleArea_To_SandOasis,
RequirementKey::LanayruDesert_PastToT_To_HookBeetleArea,
RequirementKey::LanayruDesert_SandOasis_To_HookBeetleArea,
RequirementKey::TempleOfTime_AfterLmf_To_NearGossipStone,
RequirementKey::TempleOfTime_End_To_Start,
RequirementKey::TempleOfTime_NearCube_To_End,
RequirementKey::TempleOfTime_NearGossipStone_To_NearCube,
RequirementKey::TempleOfTime_Start_To_NearGossipStone,
RequirementKey::LanayruMines_FirstHalf_To_ToCaves,
RequirementKey::LanayruMines_FirstHalf_To_ToDesert,
RequirementKey::LanayruMines_ToCaves_To_FirstHalf,
RequirementKey::LanayruMiningFacilityA_Entry_To_SecondRoom,
RequirementKey::LanayruMiningFacilityA_FirstKeyLockedRoom_To_GustBellowsRoom,
RequirementKey::LanayruMiningFacilityA_FirstWestRoom_To_MapRoom,
RequirementKey::LanayruMiningFacilityA_SecondRoom_To_FirstKeyLockedRoom,
RequirementKey::LanayruMiningFacilityA_SecondRoom_To_FirstWestRoom,
RequirementKey::LanayruMiningFacilityB_AfterLmfBkRoom_To_InsideLmfBkRoom,
RequirementKey::LanayruMiningFacilityB_AfterLmfBkRoom_To_NearBossDoor,
RequirementKey::LanayruMiningFacilityB_HubRoom_To_NearFirstHubRoomChest,
RequirementKey::LanayruMiningFacilityB_InsideLmfBkRoom_To_AfterLmfBkRoom,
RequirementKey::LanayruMiningFacilityB_NearBossDoor_To_AfterLmfBkRoom,
RequirementKey::LanayruMiningFacilityB_NearBossDoor_To_InsideLmfBkRoom,
RequirementKey::LanayruMiningFacilityB_WestHub_To_NearBossDoor,
RequirementKey::LanayruMiningFacilityToToT_BossDoor_To_ToTExit,
RequirementKey::OutsidePiratesStronghold_Main_To_InsideSharkhead,
RequirementKey::SandSeaDocks_ToCaves_To_Main,
RequirementKey::Shipyard_Main_To_AfterMinecartRide,
RequirementKey::ShipyardConstructionBay_Upper_To_Lower,
RequirementKey::SkippersRetreat_NextToShack_To_PastDekuBaba,
RequirementKey::SkippersRetreat_PastDekuBaba_To_NextToShack,
RequirementKey::SkippersRetreat_PastMoblin_To_PastDekuBaba,
RequirementKey::SkippersRetreat_Start_To_PastMoblin,
RequirementKey::Sandship_Deck_To_PastSpume,
RequirementKey::Sandship_PastSpume_To_SandshipBrig,
RequirementKey::SealedGroundsSpiral_Lower_To_Upper,
RequirementKey::SealedGroundsSpiral_Upper_To_Lower,
RequirementKey::Sky_BeedlesSkyHome_To_BeedleIslandCage,
RequirementKey::Sky_Field_To_BeedleIslandCage,
RequirementKey::Sky_Field_To_OutsideLumpyPumpkin,
RequirementKey::Sky_OutsideLumpyPumpkin_To_Field,
RequirementKey::Skyloft_OutsideSkyloftVillage_To_CentralOutside,
RequirementKey::Skyloft_OutsideSkyloftVillage_To_PastWaterfallCave,
RequirementKey::SkyviewTemple_Entry_To_FirstHub,
RequirementKey::SkyviewTemple_FirstHub_To_MainHub,
RequirementKey::SkyviewTemple_MainHub_To_BossDoorArea,
RequirementKey::OutsideFireSanctuary_Middle_To_ToFireSanctuary,
RequirementKey::OutsideFireSanctuary_ToInsideSummit_To_Middle,
RequirementKey::AncientCistern_ChestAfterWhipHooks,
RequirementKey::AncientCistern_Bokoblin,
RequirementKey::AncientCistern_ChestBehindTheWaterfall,
RequirementKey::AncientCistern_BossKeyChest,
RequirementKey::AncientCistern_ChestInEastPart,
RequirementKey::AncientCistern_ChestNearVines,
RequirementKey::AncientCistern_Whip,
RequirementKey::AncientCistern_KoloktosHeartContainer,
RequirementKey::AncientCistern_FaroresFlame,
RequirementKey::Batreaux_10Crystals,
RequirementKey::Batreaux_30Crystals,
RequirementKey::Batreaux_30CrystalsChest,
RequirementKey::Batreaux_40Crystals,
RequirementKey::Batreaux_5Crystals,
RequirementKey::Batreaux_50Crystals,
RequirementKey::Batreaux_70Crystals,
RequirementKey::Batreaux_70CrystalsSecondReward,
RequirementKey::Batreaux_80Crystals,
RequirementKey::Beedle_1000RupeeItem,
RequirementKey::Beedle_1200RupeeItem,
RequirementKey::Beedle_1600RupeeItem,
RequirementKey::Beedle_300RupeeItem,
RequirementKey::Beedle_50RupeeItem,
RequirementKey::Beedle_600RupeeItem,
RequirementKey::Beedle_800RupeeItem,
RequirementKey::Beedle_First100RupeeItem,
RequirementKey::Beedle_Second100RupeeItem,
RequirementKey::Beedle_Third100RupeeItem,
RequirementKey::CentralSkyloft_BazaarGoddessChest,
RequirementKey::CentralSkyloft_PotionLadysGift,
RequirementKey::CentralSkyloft_CrystalInOrielleAndParrowsHouse,
RequirementKey::CentralSkyloft_PeaterPeatricesCrystals,
RequirementKey::CentralSkyloft_CrystalBetweenWoodenPlanks,
RequirementKey::CentralSkyloft_CrystalOnLightTower,
RequirementKey::CentralSkyloft_CrystalOnWaterfallIsland,
RequirementKey::CentralSkyloft_CrystalOnWestCliff,
RequirementKey::CentralSkyloft_FloatingIslandGoddessChest,
RequirementKey::CentralSkyloft_FloatingIslandGossipStone,
RequirementKey::CentralSkyloft_ItemInBirdNest,
RequirementKey::CentralSkyloft_ParrowsCrystals,
RequirementKey::CentralSkyloft_ParrowsGift,
RequirementKey::CentralSkyloft_ShedChest,
RequirementKey::CentralSkyloft_ShedGoddessChest,
RequirementKey::CentralSkyloft_WaterfallGoddessChest,
RequirementKey::CentralSkyloft_WestCliffGoddessChest,
RequirementKey::CentralSkyloft_CrystalAfterWaterfallCave,
RequirementKey::CentralSkyloft_CrystalInLoftwingPrison,
RequirementKey::CentralSkyloft_WaterfallCaveFirstChest,
RequirementKey::CentralSkyloft_WaterfallCaveSecondChest,
RequirementKey::CentralSkyloft_WrynasCrystals,
RequirementKey::EarthTemple_ChestGuardedByLizalfos,
RequirementKey::EarthTemple_BombBag,
RequirementKey::EarthTemple_ChestLeftOfMainRoomBridge,
RequirementKey::EarthTemple_ChestBehindBombableRock,
RequirementKey::EarthTemple_ChestInWestRoom,
RequirementKey::EarthTemple_LeddsGift,
RequirementKey::EarthTemple_BossKeyChest,
RequirementKey::EarthTemple_VentChest,
RequirementKey::EarthTemple_ScalderaHeartContainer,
RequirementKey::EarthTemple_AmberTablet,
RequirementKey::EldinSilentRealm_FireshieldEarrings,
RequirementKey::EldinVolcano_ChestBehindBombableWallInFirstRoom,
RequirementKey::EldinVolcano_DiggingSpotBehindBoulderOnSandySlope,
RequirementKey::EldinVolcano_DiggingSpotBelowTower,
RequirementKey::EldinVolcano_DiggingSpotInFrontOfEarthTemple,
RequirementKey::EldinVolcano_GossipStoneNextToEarthTemple,
RequirementKey::EldinVolcano_DiggingSpotAfterDrainingLava,
RequirementKey::EldinVolcano_ChestAfterCrawlspace,
RequirementKey::EldinVolcano_ChestBehindBombableWallNearCliff,
RequirementKey::EldinVolcano_ItemOnCliff,
RequirementKey::EldinVolcano_DiggingSpotAfterVents,
RequirementKey::EldinVolcano_ChestBehindBombableWallNearVolcanoAscent,
RequirementKey::EldinVolcano_GossipStoneInThrillDiggerCave,
RequirementKey::FaronSilentRealm_WaterScale,
RequirementKey::FaronWoods_DeepWoodsChest,
RequirementKey::FaronWoods_ChestBehindBombableRocksNearErla,
RequirementKey::FaronWoods_ItemBehindBombableRock,
RequirementKey::FaronWoods_ItemOnTree,
RequirementKey::FaronWoods_Slingshot,
RequirementKey::FaronWoods_ChestInsideGreatTree,
RequirementKey::FireSanctuary_ChestInFirstRoom,
RequirementKey::FireSanctuary_PlatsChest,
RequirementKey::FireSanctuary_BossKeyChest,
RequirementKey::FireSanctuary_ChestInStaircaseRoom,
RequirementKey::FireSanctuary_MogmaMitts,
RequirementKey::FireSanctuary_ChestInSecondRoom,
RequirementKey::FireSanctuary_ChestOnBalcony,
RequirementKey::FireSanctuary_ChestAfterBombableWall,
RequirementKey::FireSanctuary_ChestAfterSecondTrappedMogma,
RequirementKey::FireSanctuary_ChestNearFirstTrappedMogma,
RequirementKey::FireSanctuary_FirstChestInWaterFruitRoom,
RequirementKey::FireSanctuary_SecondChestInWaterFruitRoom,
RequirementKey::FireSanctuary_GhirahimHeartContainer,
RequirementKey::FireSanctuary_DinsFlame,
RequirementKey::KnightAcademy_ChestInGoddessStatue,
RequirementKey::KnightAcademy_CawlinsLetter,
RequirementKey::KnightAcademy_CrystalInKnightAcademyPlant,
RequirementKey::KnightAcademy_CrystalInLinksRoom,
RequirementKey::KnightAcademy_CrystalInZeldasRoom,
RequirementKey::KnightAcademy_FledgesCrystals,
RequirementKey::KnightAcademy_FledgesGift,
RequirementKey::KnightAcademy_GhostPipitsCrystals,
RequirementKey::KnightAcademy_InZeldasCloset,
RequirementKey::KnightAcademy_OwlansCrystals,
RequirementKey::KnightAcademy_ChestNearGoddessStatue,
RequirementKey::KnightAcademy_OwlansGift,
RequirementKey::KnightAcademy_PumpkinArchery600Points,
RequirementKey::KnightAcademy_CrystalInSparringHall,
RequirementKey::KnightAcademy_SparringHallChest,
RequirementKey::LakeFloria_DragonLairEastChest,
RequirementKey::LakeFloria_DragonLairSouthChest,
RequirementKey::LakeFloria_LakeFloriaChest,
RequirementKey::LanayruCaves_Chest,
RequirementKey::LanayruCaves_GolosGift,
RequirementKey::LanayruCaves_GossipStoneInCenter,
RequirementKey::LanayruDesert_FireNodeLeftEndingChest,
RequirementKey::LanayruDesert_FireNodeRightEndingChest,
RequirementKey::LanayruDesert_FireNodeFirstSmallChest,
RequirementKey::LanayruDesert_FireNodeSecondSmallChest,
RequirementKey::LanayruDesert_FireNodeShortcutChest,
RequirementKey::LanayruDesert_ChestNearHookBeetleFight,
RequirementKey::LanayruDesert_ChestNearPartyWheel,
RequirementKey::LanayruDesert_HookBeetleFight,
RequirementKey::LanayruDesert_ChestOnPlatformNearFireNode,
RequirementKey::LanayruDesert_ChestOnPlatformNearLightningNode,
RequirementKey::LanayruDesert_ChestOnTopOfLanayruMiningFacility,
RequirementKey::LanayruDesert_SecretPassagewayChest,
RequirementKey::LanayruDesert_ChestNearSandOasis,
RequirementKey::LanayruDesert_LightningNodeFirstChest,
RequirementKey::LanayruDesert_LightningNodeRaisedChestNearGenerator,
RequirementKey::LanayruDesert_LightningNodeSecondChest,
RequirementKey::LanayruDesert_GossipStoneInTempleOfTimeArea,
RequirementKey::LanayruMines_ChestAtTheEndOfMines,
RequirementKey::LanayruMines_ChestBehindFirstLanding,
RequirementKey::LanayruMines_ChestBehindStatue,
RequirementKey::LanayruMines_ChestNearFirstTimeshiftStone,
RequirementKey::LanayruMiningFacility_ChestBehindBars,
RequirementKey::LanayruMiningFacility_ChestInKeyLockedRoom,
RequirementKey::LanayruMiningFacility_ChestInFirstWestRoom,
RequirementKey::LanayruMiningFacility_ChestInsideGustBellowsRoom,
RequirementKey::LanayruMiningFacility_GustBellows,
RequirementKey::LanayruMiningFacility_ChestAfterArmosFight,
RequirementKey::LanayruMiningFacility_ShortcutChestInMainHub,
RequirementKey::LanayruMiningFacility_BossKeyChest,
RequirementKey::LanayruMiningFacility_FirstChestInHubRoom,
RequirementKey::LanayruMiningFacility_ChestBehindFirstCrawlspace,
RequirementKey::LanayruMiningFacility_ChestInSpikeMaze,
RequirementKey::LanayruMiningFacility_MolderachHeartContainer,
RequirementKey::LanayruMiningFacility_GoddessHarp,
RequirementKey::LanayruSandSea_PirateStrongholdFirstChest,
RequirementKey::LanayruSandSea_PirateStrongholdSecondChest,
RequirementKey::LanayruSandSea_PirateStrongholdThirdChest,
RequirementKey::LanayruSandSea_GossipStoneInShipyard,
RequirementKey::LanayruSandSea_RicketyCoasterHeartStoppingTrackIn105,
RequirementKey::LanayruSandSea_SkippersRetreatSkydiveChest,
RequirementKey::LanayruSandSea_SkippersRetreatChestOnTopOfCactiPillar,
RequirementKey::LanayruSandSea_SkippersRetreatChestAfterMoblin,
RequirementKey::LanayruSandSea_SkippersRetreatChestInShack,
RequirementKey::LanayruSilentRealm_Clawshots,
RequirementKey::MogmaTurf_ChestBehindBombableWallAtEntrance,
RequirementKey::MogmaTurf_ChestBehindBombableWallInFireMaze,
RequirementKey::MogmaTurf_DiggingMittsFight,
RequirementKey::MogmaTurf_FreeFallChest,
RequirementKey::MogmaTurf_SandSlideChest,
RequirementKey::Sandship_BossKeyChest,
RequirementKey::Sandship_Bow,
RequirementKey::Sandship_ChestAtTheStern,
RequirementKey::Sandship_ChestBefore4DoorCorridor,
RequirementKey::Sandship_ChestBehindCombinationLock,
RequirementKey::Sandship_RobotInBrigsReward,
RequirementKey::Sandship_TreasureRoomFifthChest,
RequirementKey::Sandship_TreasureRoomFirstChest,
RequirementKey::Sandship_TreasureRoomFourthChest,
RequirementKey::Sandship_TreasureRoomSecondChest,
RequirementKey::Sandship_TreasureRoomThirdChest,
RequirementKey::Sandship_NayrusFlame,
RequirementKey::Sandship_TentalusHeartContainer,
RequirementKey::SealedGrounds_GorkosGoddessWallReward,
RequirementKey::SealedGrounds_ZeldasBlessing,
RequirementKey::SealedGrounds_ChestInsideSealedTemple,
RequirementKey::SealedGrounds_SongFromImpa,
RequirementKey::Sky_GossipStoneInsideBambooIsland,
RequirementKey::Sky_CrystalInsideLumpyPumpkin,
RequirementKey::Sky_LumpyPumpkinChandelier,
RequirementKey::Sky_LumpyPumpkinHarpMinigame,
RequirementKey::Sky_BeedlesIslandCageGoddessChest,
RequirementKey::Sky_BeedlesCrystals,
RequirementKey::Sky_CrystalOnBeedlesShip,
RequirementKey::Sky_BambooIslandGoddessChest,
RequirementKey::Sky_BeedlesIslandGoddessChest,
RequirementKey::Sky_ChestInBreakableBoulderNearFunFunIsland,
RequirementKey::Sky_ChestInBreakableBoulderNearLumpyPumpkin,
RequirementKey::Sky_DodohsCrystals,
RequirementKey::Sky_FunFunIslandMinigame500Rupees,
RequirementKey::Sky_GoddessChestInCaveOnIslandNextToBambooIsland,
RequirementKey::Sky_GoddessChestInsideVolcanicIsland,
RequirementKey::Sky_GoddessChestOnIslandClosestToFaronPillar,
RequirementKey::Sky_GoddessChestOnIslandNextToBambooIsland,
RequirementKey::Sky_GoddessChestOutsideVolcanicIsland,
RequirementKey::Sky_GoddessChestUnderFunFunIsland,
RequirementKey::Sky_GossipStoneInVolcanicIsland,
RequirementKey::Sky_LumpyPumpkinGoddessChestOnTheRoof,
RequirementKey::Sky_NortheastIslandCageGoddessChest,
RequirementKey::Sky_NortheastIslandGoddessChestBehindBombableRocks,
RequirementKey::Sky_OriellesCrystals,
RequirementKey::Sky_SouthwestTripleIslandCageGoddessChest,
RequirementKey::Sky_SouthwestTripleIslandLowerGoddessChest,
RequirementKey::Sky_SouthwestTripleIslandUpperGoddessChest,
RequirementKey::Sky_CrystalOutsideLumpyPumpkin,
RequirementKey::Sky_KinasCrystals,
RequirementKey::Sky_LumpyPumpkinOutsideGoddessChest,
RequirementKey::SkyKeep_ChestAfterDreadfuse,
RequirementKey::SkyKeep_FirstChest,
RequirementKey::SkyloftSilentRealm_StoneOfTrials,
RequirementKey::SkyloftVillage_BertiesCrystals,
RequirementKey::SkyloftVillage_MallarasCrystals,
RequirementKey::SkyloftVillage_CrystalNearPumpkinPatch,
RequirementKey::SkyloftVillage_SparrotsCrystals,
RequirementKey::Skyview_GhirahimHeartContainer,
RequirementKey::Skyview_RubyTablet,
RequirementKey::Skyview_BossKeyChest,
RequirementKey::Skyview_ChestNearBossDoor,
RequirementKey::Skyview_ChestBehindTwoEyes,
RequirementKey::Skyview_ChestOnTreeBranch,
RequirementKey::Skyview_DiggingSpotInCrawlspace,
RequirementKey::Skyview_Beetle,
RequirementKey::Skyview_ChestBehindThreeEyes,
RequirementKey::Skyview_ItemBehindBars,
RequirementKey::Thunderhead_BugHeaven10BugsIn3Minutes,
RequirementKey::Thunderhead_BugHeavenGoddessChest,
RequirementKey::Thunderhead_EastIslandChest,
RequirementKey::Thunderhead_EastIslandGoddessChest,
RequirementKey::Thunderhead_FirstGoddessChestOnMogmaMittsIsland,
RequirementKey::Thunderhead_GoddessChestOnTopOfIsleOfSongs,
RequirementKey::Thunderhead_GoddessChestOutsideIsleOfSongs,
RequirementKey::Thunderhead_SongFromLevias,
RequirementKey::Thunderhead_IsleOfSongsDinsPower,
RequirementKey::Thunderhead_IsleOfSongsFaroresCourage,
RequirementKey::Thunderhead_IsleOfSongsNayrusWisdom,
RequirementKey::VolcanoSummit_BokoBasePouchChest,
RequirementKey::VolcanoSummit_SmallChestInVolcanoSummit,
RequirementKey::VolcanoSummit_GossipStoneOutsideFireSanctuary,
RequirementKey::VolcanoSummit_ItemBehindDigging,
RequirementKey::VolcanoSummit_ChestBehindBombableWallInWaterfallArea,
RequirementKey::VolcanoSummit_GossipStoneInWaterfallArea,
RequirementKey::AncientCistern_To_AncientCisternBoss,
RequirementKey::AncientCistern_To_FloriaWaterfall,
RequirementKey::AncientCisternBoss_To_AncientCisternCandleRoom,
RequirementKey::BatreauxHouse_To_Skyloft,
RequirementKey::BeedlesShop_To_Sky_Night,
RequirementKey::BeedlesShop_To_Skyloft_Day,
RequirementKey::Bazaar_To_Skyloft_North,
RequirementKey::Bazaar_To_Skyloft_South,
RequirementKey::Bazaar_To_Skyloft_West,
RequirementKey::ParrowAndOriellesHouse_To_Skyloft,
RequirementKey::PeatricesHouse_To_Skyloft,
RequirementKey::PipersHouse_To_Skyloft,
RequirementKey::Skyloft_To_Bazaar_North,
RequirementKey::Skyloft_To_Bazaar_South,
RequirementKey::Skyloft_To_Bazaar_West,
RequirementKey::Skyloft_To_BeedlesShop_Day,
RequirementKey::Skyloft_To_ParrowAndOriellesHouse,
RequirementKey::Skyloft_To_PeatricesHouse,
RequirementKey::Skyloft_To_PipersHouse,
RequirementKey::Skyloft_To_Sky,
RequirementKey::Skyloft_To_SkyloftSilentRealm,
RequirementKey::Skyloft_To_WaterfallCave_Upper,
RequirementKey::Skyloft_To_WrynasHouse,
RequirementKey::Skyloft_To_Sky_PastWaterfallCave,
RequirementKey::Skyloft_To_WaterfallCave_Lower,
RequirementKey::Skyloft_To_SkyKeepEntry,
RequirementKey::WaterfallCave_To_Skyloft_Upper,
RequirementKey::WaterfallCave_To_Skyloft_Lower,
RequirementKey::WrynasHouse_To_Skyloft,
RequirementKey::EarthTemple_To_EarthTempleBoss,
RequirementKey::EarthTemple_To_EldinVolcano,
RequirementKey::EarthTempleBoss_To_EarthTempleSpring,
RequirementKey::EarthTempleSpring_To_EldinVolcano,
RequirementKey::EldinSilentRealm_To_EldinVolcano,
RequirementKey::EldinVolcano_To_Sky_EldinEntranceStatue,
RequirementKey::EldinVolcano_To_InsideVolcanoSummit,
RequirementKey::EldinVolcano_To_ThrillDiggerCave,
RequirementKey::EldinVolcano_To_EarthTemple,
RequirementKey::EldinVolcano_To_Sky_TempleEntranceStatue,
RequirementKey::EldinVolcano_To_MogmaTurf_Skydive,
RequirementKey::EldinVolcano_To_Sky_VolcanoEastStatue,
RequirementKey::EldinVolcano_To_EldinSilentRealm,
RequirementKey::EldinVolcano_To_Sky_VolcanoAscentStatue,
RequirementKey::ThrillDiggerCave_To_EldinVolcano,
RequirementKey::FaronSilentRealm_To_FaronWoods,
RequirementKey::DeepWoods_To_FaronWoods,
RequirementKey::DeepWoods_To_Sky_DeepWoodsStatue,
RequirementKey::DeepWoods_To_Sky_ForestTempleStatue,
RequirementKey::DeepWoods_To_SkyviewTemple,
RequirementKey::FaronWoods_To_BehindTheTemple,
RequirementKey::FaronWoods_To_Sky_FaronWoodsEntryStatue,
RequirementKey::FaronWoods_To_GreatTree_LowerPlatform,
RequirementKey::FaronWoods_To_GreatTree_UpperPlatform,
RequirementKey::FaronWoods_To_GreatTree_Top,
RequirementKey::FaronWoods_To_Sky_GreatTreeStatue,
RequirementKey::FaronWoods_To_DeepWoods,
RequirementKey::FaronWoods_To_FaronSilentRealm,
RequirementKey::FaronWoods_To_GreatTree_Tunnel,
RequirementKey::FaronWoods_To_LakeFloria,
RequirementKey::FaronWoods_To_Sky_InTheWoodsStatue,
RequirementKey::FaronWoods_To_Sky_ViewingPlatformStatue,
RequirementKey::GreatTree_To_FaronWoods_Tunnel,
RequirementKey::GreatTree_To_FaronWoods_LowerPlatform,
RequirementKey::GreatTree_To_FaronWoods_UpperPlatform,
RequirementKey::GreatTree_To_FaronWoods_Top,
RequirementKey::FireSanctuaryA_To_OutsideFireSanctuary,
RequirementKey::FireSanctuaryA_To_FireSanctuaryBoss,
RequirementKey::FireSanctuaryA_To_FireSanctuaryB,
RequirementKey::FireSanctuaryB_To_FireSanctuaryA,
RequirementKey::FireSanctuaryBoss_To_FireSanctuaryFlameRoom,
RequirementKey::InsideGoddessStatue_To_Skyloft,
RequirementKey::KnightAcademy_To_Skyloft_LowerDoors,
RequirementKey::KnightAcademy_To_Skyloft_UpperDoors,
RequirementKey::Skyloft_To_InsideGoddessStatue,
RequirementKey::Skyloft_To_KnightAcademy_Chimney,
RequirementKey::Skyloft_To_KnightAcademy_LowerDoors,
RequirementKey::Skyloft_To_KnightAcademy_UpperDoors,
RequirementKey::Skyloft_To_SparringHall,
RequirementKey::SparringHall_To_Skyloft,
RequirementKey::FaroresLair_To_FloriaWaterfall,
RequirementKey::FaroresLair_To_LakeFloria,
RequirementKey::FloriaWaterfall_To_AncientCistern,
RequirementKey::FloriaWaterfall_To_FaronWoods,
RequirementKey::FloriaWaterfall_To_FaroresLair,
RequirementKey::FloriaWaterfall_To_Sky_FloriaWaterfallStatue,
RequirementKey::LakeFloria_To_Sky_LakeFloriaStatue,
RequirementKey::LakeFloria_To_FaroresLair,
RequirementKey::LanayruCaves_To_LanayruDesert,
RequirementKey::LanayruCaves_To_LanayruMines,
RequirementKey::LanayruCaves_To_SandSeaDocks,
RequirementKey::FireNode_To_LanayruDesert,
RequirementKey::LanayruDesert_To_LanayruMines,
RequirementKey::LanayruDesert_To_Sky_DesertEntranceStatue,
RequirementKey::LanayruDesert_To_FireNode,
RequirementKey::LanayruDesert_To_LanayruMiningFacilityA,
RequirementKey::LanayruDesert_To_LanayruSilentRealm,
RequirementKey::LanayruDesert_To_LightningNode,
RequirementKey::LanayruDesert_To_Sky_NorthDesertStatue,
RequirementKey::LanayruDesert_To_Sky_StoneCacheStatue,
RequirementKey::LanayruDesert_To_TempleOfTime_End,
RequirementKey::LanayruDesert_To_LanayruCaves,
RequirementKey::LanayruDesert_To_Sky_WestDesertStatue,
RequirementKey::LanayruDesert_To_TempleOfTime_Start,
RequirementKey::LightningNode_To_LanayruDesert,
RequirementKey::TempleOfTime_To_LanayruDesert_End,
RequirementKey::TempleOfTime_To_LanayruDesert_Start,
RequirementKey::LanayruMines_To_Sky_LanayruMineEntryStatue,
RequirementKey::LanayruMines_To_LanayruCaves,
RequirementKey::LanayruMines_To_LanayruDesert,
RequirementKey::LanayruMiningFacilityA_To_LanayruDesert,
RequirementKey::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub2,
RequirementKey::LanayruMiningFacilityA_To_LanayruMiningFacilityB_HubW,
RequirementKey::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub,
RequirementKey::LanayruMiningFacilityB_To_LanayruMiningFacilityBoss,
RequirementKey::LanayruMiningFacilityBoss_To_LanayruMiningFacilityToToT,
RequirementKey::LanayruMiningFacilityToToT_To_TempleOfTime,
RequirementKey::InsidePiratesStronghold_To_OutsidePiratesStronghold_End,
RequirementKey::InsidePiratesStronghold_To_OutsidePiratesStronghold_Beginning,
RequirementKey::OutsidePiratesStronghold_To_InsidePiratesStronghold_End,
RequirementKey::OutsidePiratesStronghold_To_InsidePiratesStronghold_Beginning,
RequirementKey::OutsidePiratesStronghold_To_SandSea,
RequirementKey::SandSea_To_OutsidePiratesStronghold,
RequirementKey::SandSea_To_SandSeaDocks,
RequirementKey::SandSea_To_Sandship,
RequirementKey::SandSea_To_Shipyard,
RequirementKey::SandSea_To_SkippersRetreat,
RequirementKey::SandSeaDocks_To_SandSea,
RequirementKey::SandSeaDocks_To_Sky_AncientHarbor,
RequirementKey::SandSeaDocks_To_LanayruCaves,
RequirementKey::Shipyard_To_ShipyardConstructionBay_Upper,
RequirementKey::Shipyard_To_SandSea,
RequirementKey::Shipyard_To_ShipyardConstructionBay_Lower,
RequirementKey::ShipyardConstructionBay_To_Shipyard_Lower,
RequirementKey::ShipyardConstructionBay_To_Shipyard_Upper,
RequirementKey::SkippersRetreat_To_SkippersShack,
RequirementKey::SkippersRetreat_To_SandSea,
RequirementKey::SkippersShack_To_SkippersRetreat,
RequirementKey::LanayruSilentRealm_To_LanayruDesert,
RequirementKey::MogmaTurf_To_EldinVolcano_EndVent,
RequirementKey::MogmaTurf_To_EldinVolcano_StartVent,
RequirementKey::Sandship_To_SandSea,
RequirementKey::Sandship_To_SandshipBoss,
RequirementKey::BehindTheTemple_To_FaronWoods,
RequirementKey::BehindTheTemple_To_SealedGroundsSpiral,
RequirementKey::BehindTheTemple_To_SealedTemple,
RequirementKey::BehindTheTemple_To_Sky_BehindTheTempleStatue,
RequirementKey::SealedGroundsSpiral_To_SealedTemple,
RequirementKey::SealedGroundsSpiral_To_Sky_SealedGroundsStatue,
RequirementKey::SealedTemple_To_BehindTheTemple,
RequirementKey::SealedTemple_To_HyliasTemple,
RequirementKey::SealedTemple_To_SealedGroundsSpiral,
RequirementKey::InsideBambooIsland_To_Sky,
RequirementKey::LumpyPumpkin_To_Sky_North,
RequirementKey::LumpyPumpkin_To_Sky_South,
RequirementKey::Sky_To_BeedlesShop_Night,
RequirementKey::Sky_To_BehindTheTemple_BehindTheTempleStatue,
RequirementKey::Sky_To_DeepWoods_DeepWoodsStatue,
RequirementKey::Sky_To_DeepWoods_ForestTempleStatue,
RequirementKey::Sky_To_EldinVolcano_EldinEntranceStatue,
RequirementKey::Sky_To_EldinVolcano_TempleEntranceStatue,
RequirementKey::Sky_To_EldinVolcano_VolcanoEastStatue,
RequirementKey::Sky_To_EldinVolcano_VolcanoAscentStatue,
RequirementKey::Sky_To_FaronWoods_FaronWoodsEntryStatue,
RequirementKey::Sky_To_FaronWoods_GreatTreeStatue,
RequirementKey::Sky_To_FaronWoods_InTheWoodsStatue,
RequirementKey::Sky_To_FaronWoods_ViewingPlatformStatue,
RequirementKey::Sky_To_FloriaWaterfall_FloriaWaterfallStatue,
RequirementKey::Sky_To_InsideBambooIsland,
RequirementKey::Sky_To_InsideThunderhead,
RequirementKey::Sky_To_LakeFloria_LakeFloriaStatue,
RequirementKey::Sky_To_LanayruDesert_DesertEntranceStatue,
RequirementKey::Sky_To_LanayruDesert_NorthDesertStatue,
RequirementKey::Sky_To_LanayruDesert_StoneCacheStatue,
RequirementKey::Sky_To_LanayruDesert_WestDesertStatue,
RequirementKey::Sky_To_LanayruMines_LanayruMineEntryStatue,
RequirementKey::Sky_To_OutsideFireSanctuary_InsideTheVolcanoStatue,
RequirementKey::Sky_To_SealedGroundsSpiral_SealedGroundsStatue,
RequirementKey::Sky_To_Skyloft,
RequirementKey::Sky_To_LumpyPumpkin_North,
RequirementKey::Sky_To_LumpyPumpkin_South,
RequirementKey::SkyKeepEntry_To_Skyloft,
RequirementKey::SkyloftSilentRealm_To_Skyloft,
RequirementKey::BertiesHouse_To_Skyloft,
RequirementKey::GondosHouse_To_Skyloft,
RequirementKey::MallarasHouse_To_Skyloft,
RequirementKey::RupinsHouse_To_Skyloft,
RequirementKey::Skyloft_To_BatreauxHouse,
RequirementKey::Skyloft_To_BertiesHouse,
RequirementKey::Skyloft_To_GondosHouse,
RequirementKey::Skyloft_To_MallarasHouse,
RequirementKey::Skyloft_To_RupinsHouse,
RequirementKey::Skyloft_To_SparrotsHouse,
RequirementKey::SparrotsHouse_To_Skyloft,
RequirementKey::SkyviewBoss_To_SkyviewSpring,
RequirementKey::SkyviewBoss_To_SkyviewTemple,
RequirementKey::SkyviewSpring_To_DeepWoods,
RequirementKey::SkyviewSpring_To_SkyviewBoss,
RequirementKey::SkyviewTemple_To_SkyviewBoss,
RequirementKey::SkyviewTemple_To_DeepWoods,
RequirementKey::InsideThunderhead_To_IsleOfSongs,
RequirementKey::InsideThunderhead_To_Sky,
RequirementKey::IsleOfSongs_To_InsideThunderhead,
RequirementKey::InsideVolcanoSummit_To_EldinVolcano,
RequirementKey::InsideVolcanoSummit_To_OutsideFireSanctuary,
RequirementKey::InsideVolcanoSummit_To_VolcanoSummitWaterfall,
RequirementKey::OutsideFireSanctuary_To_FireSanctuaryA,
RequirementKey::OutsideFireSanctuary_To_Sky_InsideTheVolcanoStatue,
RequirementKey::OutsideFireSanctuary_To_InsideVolcanoSummit,
RequirementKey::VolcanoSummitWaterfall_To_InsideVolcanoSummit,
RequirementKey::FireNode_End_ActivateFireNode,
RequirementKey::LightningNode_Main_ActivateLightningNode,
RequirementKey::SkyviewTemple_Entry_ActivateSkyviewGoddessWall,
RequirementKey::LanayruDesert_PastToT_ActivateWaterNode,
RequirementKey::SealedTemple_Main_BeatRequiredDungeons,
RequirementKey::BehindTheTemple_Main_BehindTheTempleStatue,
RequirementKey::AncientCisternCandleRoom_Main_CanBeatAncientCistern,
RequirementKey::EarthTempleSpring_Main_CanBeatEarthTemple,
RequirementKey::FireSanctuaryFlameRoom_Main_CanBeatFireSanctuary,
RequirementKey::LanayruMiningFacilityToToT_ToTExit_CanBeatLanayruMiningFacility,
RequirementKey::SandshipBoss_Main_CanBeatSandship,
RequirementKey::SkyKeepEntry_Main_CanBeatSkyKeep,
RequirementKey::SkyviewSpring_Main_CanBeatSkyview,
RequirementKey::Sandship_Deck_CanFreelyChangeSandshipTemporality,
RequirementKey::AncientCistern_MainRoomVines_CanLowerAcStatue,
RequirementKey::AncientCistern_SpiderThread_CanLowerAcStatue,
RequirementKey::InsideBambooIsland_Main_CanPlayCleanCut,
RequirementKey::ThrillDiggerCave_Main_CanPlayThrillDigger,
RequirementKey::LanayruDesert_HookBeetleArea_CanRetrievePartyWheel,
RequirementKey::RupinsHouse_Main_CanSellTreasures,
RequirementKey::DeepWoods_PastBeehive_DeepWoodsStatue,
RequirementKey::SealedGroundsSpiral_Lower_DefeatImprisoned2,
RequirementKey::ShipyardConstructionBay_Lower_DefeatedShipyardMolderach,
RequirementKey::SparringHall_Main_DeliveredHotSoup,
RequirementKey::LanayruDesert_HookBeetleArea_DesertEntranceStatue,
RequirementKey::EldinVolcano_FirstRoom_EldinEntranceStatue,
RequirementKey::Bazaar_Main_EndurancePotion,
RequirementKey::FaronWoods_Entry_FaronWoodsEntryStatue,
RequirementKey::FloriaWaterfall_Main_FloriaWaterfallStatue,
RequirementKey::DeepWoods_PastBeehive_ForestTempleStatue,
RequirementKey::EldinVolcano_OutsideEt_GoddessCubeEastOfEarthTempleEntrance,
RequirementKey::EldinVolcano_OutsideEt_GoddessCubeWestOfEarthTempleEntrance,
RequirementKey::EldinVolcano_FirstRoom_GoddessCubeAtEldinEntrance,
RequirementKey::LanayruMines_FirstHalf_GoddessCubeAtLanayruMinesEntrance,
RequirementKey::TempleOfTime_NearCube_GoddessCubeAtRideNearTempleOfTime,
RequirementKey::SandSeaDocks_Main_GoddessCubeInAncientHarbour,
RequirementKey::DeepWoods_PastBeehive_GoddessCubeInDeepWoods,
RequirementKey::EldinVolcano_SandSlide_GoddessCubeInEldinSlide,
RequirementKey::FloriaWaterfall_Main_GoddessCubeInFloriaWaterfall,
RequirementKey::LakeFloria_StatueSpot_GoddessCubeInLakeFloria,
RequirementKey::MogmaTurf_Main_GoddessCubeInMogmaTurf,
RequirementKey::OutsidePiratesStronghold_InsideSharkhead_GoddessCubeInPirateStronghold,
RequirementKey::LanayruDesert_SandOasis_GoddessCubeInSandOasis,
RequirementKey::LanayruDesert_PastToT_GoddessCubeInSecretPassagewayInDesert,
RequirementKey::SkippersRetreat_PastMoblin_GoddessCubeInSkippersRetreat,
RequirementKey::SkyviewSpring_Main_GoddessCubeInSkyviewSpring,
RequirementKey::VolcanoSummitWaterfall_Main_GoddessCubeInSummitWaterfall,
RequirementKey::InsideVolcanoSummit_Main_GoddessCubeInsideVolcanoSummit,
RequirementKey::OutsideFireSanctuary_ToFireSanctuary_GoddessCubeNearFsEntrance,
RequirementKey::LanayruDesert_HookBeetleArea_GoddessCubeNearHookBeetleFight,
RequirementKey::EldinVolcano_PastMogmaTurf_GoddessCubeNearMogmaTurfEntrance,
RequirementKey::EldinVolcano_PreMogmaTurf_GoddessCubeNearMogmaTurfEntrance,
RequirementKey::FaronWoods_ClawshotTargetBranch_GoddessCubeOnEastGreatTreeWithClawshotsTarget,
RequirementKey::FaronWoods_GreatTreeTop_GoddessCubeOnEastGreatTreeWithRope,
RequirementKey::FaronWoods_GreatTreePlatforms_GoddessCubeOnWestGreatTreeNearExit,
RequirementKey::DeepWoods_PastBeehive_GoddessCubeOnTopOfSkyview,
RequirementKey::FaronWoods_GreatTreeTop_GreatTreeStatue,
RequirementKey::Sky_Field_HighRupeeFarm,
RequirementKey::FaronWoods_Main_InTheWoodsStatue,
RequirementKey::DeepWoods_PastBeehive_InitialGoddessCube,
RequirementKey::OutsideFireSanctuary_ToFireSanctuary_InsideTheVolcanoStatue,
RequirementKey::LakeFloria_StatueSpot_LakeFloriaStatue,
RequirementKey::TempleOfTime_AfterLmf_LanayruDesertDungeonBeaten,
RequirementKey::LanayruMines_FirstHalf_LanayruMineEntryStatue,
RequirementKey::LumpyPumpkin_Main_LumpyPumpkinQuestStart,
RequirementKey::LanayruDesert_PastToT_NorthDesertStatue,
RequirementKey::InsidePiratesStronghold_Main_OpenSharkhead,
RequirementKey::Skyloft_OutsideSkyloftVillage_OpenedShed,
RequirementKey::MogmaTurf_Main_PickUpGuld,
RequirementKey::LumpyPumpkin_Main_PickUpLeviasSoup,
RequirementKey::Sky_OutsideLumpyPumpkin_PumpkinCarrying,
RequirementKey::SealedTemple_Main_RaiseGoT,
RequirementKey::LanayruDesert_PastToT_RaiseLanayruMiningFacility,
RequirementKey::EldinVolcano_OutsideEt_RetrieveCrystalBall,
RequirementKey::FaronWoods_Main_RetrieveOolo,
RequirementKey::Sky_Field_SaveOrielle,
RequirementKey::SealedGroundsSpiral_Upper_SealedGroundsStatue,
RequirementKey::SealedTemple_Main_StartImprisoned2,
RequirementKey::LanayruDesert_PastToT_StoneCacheStatue,
RequirementKey::Sky_Field_TalkToOrielle,
RequirementKey::Bazaar_Main_TalkToPeatriceInBazaar,
RequirementKey::FaronWoods_GreatTreeTop_TalkToYerbal,
RequirementKey::EldinVolcano_OutsideEt_TempleEntranceStatue,
RequirementKey::KnightAcademy_AboveZeldasRoom_UnlockedZeldasRoom,
RequirementKey::FaronWoods_Main_ViewingPlatformStatue,
RequirementKey::EldinVolcano_VolcanoAscent_VolcanoAscentStatue,
RequirementKey::EldinVolcano_PreMogmaTurf_VolcanoEastStatue,
RequirementKey::LanayruDesert_SandOasis_WestDesertStatue,
];
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Exit {
    AncientCistern_To_AncientCisternBoss,
    AncientCistern_To_FloriaWaterfall,
    AncientCisternBoss_To_AncientCisternCandleRoom,
    BatreauxHouse_To_Skyloft,
    BeedlesShop_To_Sky_Night,
    BeedlesShop_To_Skyloft_Day,
    Bazaar_To_Skyloft_North,
    Bazaar_To_Skyloft_South,
    Bazaar_To_Skyloft_West,
    ParrowAndOriellesHouse_To_Skyloft,
    PeatricesHouse_To_Skyloft,
    PipersHouse_To_Skyloft,
    Skyloft_To_Bazaar_North,
    Skyloft_To_Bazaar_South,
    Skyloft_To_Bazaar_West,
    Skyloft_To_BeedlesShop_Day,
    Skyloft_To_ParrowAndOriellesHouse,
    Skyloft_To_PeatricesHouse,
    Skyloft_To_PipersHouse,
    Skyloft_To_Sky,
    Skyloft_To_SkyloftSilentRealm,
    Skyloft_To_WaterfallCave_Upper,
    Skyloft_To_WrynasHouse,
    Skyloft_To_Sky_PastWaterfallCave,
    Skyloft_To_WaterfallCave_Lower,
    Skyloft_To_SkyKeepEntry,
    WaterfallCave_To_Skyloft_Upper,
    WaterfallCave_To_Skyloft_Lower,
    WrynasHouse_To_Skyloft,
    EarthTemple_To_EarthTempleBoss,
    EarthTemple_To_EldinVolcano,
    EarthTempleBoss_To_EarthTempleSpring,
    EarthTempleSpring_To_EldinVolcano,
    EldinSilentRealm_To_EldinVolcano,
    EldinVolcano_To_Sky_EldinEntranceStatue,
    EldinVolcano_To_InsideVolcanoSummit,
    EldinVolcano_To_ThrillDiggerCave,
    EldinVolcano_To_EarthTemple,
    EldinVolcano_To_Sky_TempleEntranceStatue,
    EldinVolcano_To_MogmaTurf_Skydive,
    EldinVolcano_To_Sky_VolcanoEastStatue,
    EldinVolcano_To_EldinSilentRealm,
    EldinVolcano_To_Sky_VolcanoAscentStatue,
    ThrillDiggerCave_To_EldinVolcano,
    FaronSilentRealm_To_FaronWoods,
    DeepWoods_To_FaronWoods,
    DeepWoods_To_Sky_DeepWoodsStatue,
    DeepWoods_To_Sky_ForestTempleStatue,
    DeepWoods_To_SkyviewTemple,
    FaronWoods_To_BehindTheTemple,
    FaronWoods_To_Sky_FaronWoodsEntryStatue,
    FaronWoods_To_GreatTree_LowerPlatform,
    FaronWoods_To_GreatTree_UpperPlatform,
    FaronWoods_To_GreatTree_Top,
    FaronWoods_To_Sky_GreatTreeStatue,
    FaronWoods_To_DeepWoods,
    FaronWoods_To_FaronSilentRealm,
    FaronWoods_To_GreatTree_Tunnel,
    FaronWoods_To_LakeFloria,
    FaronWoods_To_Sky_InTheWoodsStatue,
    FaronWoods_To_Sky_ViewingPlatformStatue,
    GreatTree_To_FaronWoods_Tunnel,
    GreatTree_To_FaronWoods_LowerPlatform,
    GreatTree_To_FaronWoods_UpperPlatform,
    GreatTree_To_FaronWoods_Top,
    FireSanctuaryA_To_OutsideFireSanctuary,
    FireSanctuaryA_To_FireSanctuaryBoss,
    FireSanctuaryA_To_FireSanctuaryB,
    FireSanctuaryB_To_FireSanctuaryA,
    FireSanctuaryBoss_To_FireSanctuaryFlameRoom,
    InsideGoddessStatue_To_Skyloft,
    KnightAcademy_To_Skyloft_LowerDoors,
    KnightAcademy_To_Skyloft_UpperDoors,
    Skyloft_To_InsideGoddessStatue,
    Skyloft_To_KnightAcademy_Chimney,
    Skyloft_To_KnightAcademy_LowerDoors,
    Skyloft_To_KnightAcademy_UpperDoors,
    Skyloft_To_SparringHall,
    SparringHall_To_Skyloft,
    FaroresLair_To_FloriaWaterfall,
    FaroresLair_To_LakeFloria,
    FloriaWaterfall_To_AncientCistern,
    FloriaWaterfall_To_FaronWoods,
    FloriaWaterfall_To_FaroresLair,
    FloriaWaterfall_To_Sky_FloriaWaterfallStatue,
    LakeFloria_To_Sky_LakeFloriaStatue,
    LakeFloria_To_FaroresLair,
    LanayruCaves_To_LanayruDesert,
    LanayruCaves_To_LanayruMines,
    LanayruCaves_To_SandSeaDocks,
    FireNode_To_LanayruDesert,
    LanayruDesert_To_LanayruMines,
    LanayruDesert_To_Sky_DesertEntranceStatue,
    LanayruDesert_To_FireNode,
    LanayruDesert_To_LanayruMiningFacilityA,
    LanayruDesert_To_LanayruSilentRealm,
    LanayruDesert_To_LightningNode,
    LanayruDesert_To_Sky_NorthDesertStatue,
    LanayruDesert_To_Sky_StoneCacheStatue,
    LanayruDesert_To_TempleOfTime_End,
    LanayruDesert_To_LanayruCaves,
    LanayruDesert_To_Sky_WestDesertStatue,
    LanayruDesert_To_TempleOfTime_Start,
    LightningNode_To_LanayruDesert,
    TempleOfTime_To_LanayruDesert_End,
    TempleOfTime_To_LanayruDesert_Start,
    LanayruMines_To_Sky_LanayruMineEntryStatue,
    LanayruMines_To_LanayruCaves,
    LanayruMines_To_LanayruDesert,
    LanayruMiningFacilityA_To_LanayruDesert,
    LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub2,
    LanayruMiningFacilityA_To_LanayruMiningFacilityB_HubW,
    LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub,
    LanayruMiningFacilityB_To_LanayruMiningFacilityBoss,
    LanayruMiningFacilityBoss_To_LanayruMiningFacilityToToT,
    LanayruMiningFacilityToToT_To_TempleOfTime,
    InsidePiratesStronghold_To_OutsidePiratesStronghold_End,
    InsidePiratesStronghold_To_OutsidePiratesStronghold_Beginning,
    OutsidePiratesStronghold_To_InsidePiratesStronghold_End,
    OutsidePiratesStronghold_To_InsidePiratesStronghold_Beginning,
    OutsidePiratesStronghold_To_SandSea,
    SandSea_To_OutsidePiratesStronghold,
    SandSea_To_SandSeaDocks,
    SandSea_To_Sandship,
    SandSea_To_Shipyard,
    SandSea_To_SkippersRetreat,
    SandSeaDocks_To_SandSea,
    SandSeaDocks_To_Sky_AncientHarbor,
    SandSeaDocks_To_LanayruCaves,
    Shipyard_To_ShipyardConstructionBay_Upper,
    Shipyard_To_SandSea,
    Shipyard_To_ShipyardConstructionBay_Lower,
    ShipyardConstructionBay_To_Shipyard_Lower,
    ShipyardConstructionBay_To_Shipyard_Upper,
    SkippersRetreat_To_SkippersShack,
    SkippersRetreat_To_SandSea,
    SkippersShack_To_SkippersRetreat,
    LanayruSilentRealm_To_LanayruDesert,
    MogmaTurf_To_EldinVolcano_EndVent,
    MogmaTurf_To_EldinVolcano_StartVent,
    Sandship_To_SandSea,
    Sandship_To_SandshipBoss,
    BehindTheTemple_To_FaronWoods,
    BehindTheTemple_To_SealedGroundsSpiral,
    BehindTheTemple_To_SealedTemple,
    BehindTheTemple_To_Sky_BehindTheTempleStatue,
    SealedGroundsSpiral_To_SealedTemple,
    SealedGroundsSpiral_To_Sky_SealedGroundsStatue,
    SealedTemple_To_BehindTheTemple,
    SealedTemple_To_HyliasTemple,
    SealedTemple_To_SealedGroundsSpiral,
    InsideBambooIsland_To_Sky,
    LumpyPumpkin_To_Sky_North,
    LumpyPumpkin_To_Sky_South,
    Sky_To_BeedlesShop_Night,
    Sky_To_BehindTheTemple_BehindTheTempleStatue,
    Sky_To_DeepWoods_DeepWoodsStatue,
    Sky_To_DeepWoods_ForestTempleStatue,
    Sky_To_EldinVolcano_EldinEntranceStatue,
    Sky_To_EldinVolcano_TempleEntranceStatue,
    Sky_To_EldinVolcano_VolcanoEastStatue,
    Sky_To_EldinVolcano_VolcanoAscentStatue,
    Sky_To_FaronWoods_FaronWoodsEntryStatue,
    Sky_To_FaronWoods_GreatTreeStatue,
    Sky_To_FaronWoods_InTheWoodsStatue,
    Sky_To_FaronWoods_ViewingPlatformStatue,
    Sky_To_FloriaWaterfall_FloriaWaterfallStatue,
    Sky_To_InsideBambooIsland,
    Sky_To_InsideThunderhead,
    Sky_To_LakeFloria_LakeFloriaStatue,
    Sky_To_LanayruDesert_DesertEntranceStatue,
    Sky_To_LanayruDesert_NorthDesertStatue,
    Sky_To_LanayruDesert_StoneCacheStatue,
    Sky_To_LanayruDesert_WestDesertStatue,
    Sky_To_LanayruMines_LanayruMineEntryStatue,
    Sky_To_OutsideFireSanctuary_InsideTheVolcanoStatue,
    Sky_To_SealedGroundsSpiral_SealedGroundsStatue,
    Sky_To_Skyloft,
    Sky_To_LumpyPumpkin_North,
    Sky_To_LumpyPumpkin_South,
    SkyKeepEntry_To_Skyloft,
    SkyloftSilentRealm_To_Skyloft,
    BertiesHouse_To_Skyloft,
    GondosHouse_To_Skyloft,
    MallarasHouse_To_Skyloft,
    RupinsHouse_To_Skyloft,
    Skyloft_To_BatreauxHouse,
    Skyloft_To_BertiesHouse,
    Skyloft_To_GondosHouse,
    Skyloft_To_MallarasHouse,
    Skyloft_To_RupinsHouse,
    Skyloft_To_SparrotsHouse,
    SparrotsHouse_To_Skyloft,
    SkyviewBoss_To_SkyviewSpring,
    SkyviewBoss_To_SkyviewTemple,
    SkyviewSpring_To_DeepWoods,
    SkyviewSpring_To_SkyviewBoss,
    SkyviewTemple_To_SkyviewBoss,
    SkyviewTemple_To_DeepWoods,
    InsideThunderhead_To_IsleOfSongs,
    InsideThunderhead_To_Sky,
    IsleOfSongs_To_InsideThunderhead,
    InsideVolcanoSummit_To_EldinVolcano,
    InsideVolcanoSummit_To_OutsideFireSanctuary,
    InsideVolcanoSummit_To_VolcanoSummitWaterfall,
    OutsideFireSanctuary_To_FireSanctuaryA,
    OutsideFireSanctuary_To_Sky_InsideTheVolcanoStatue,
    OutsideFireSanctuary_To_InsideVolcanoSummit,
    VolcanoSummitWaterfall_To_InsideVolcanoSummit,
}
impl Into<usize> for Exit {
    fn into(self) -> usize {
        self as usize
    }
}
impl BitSetCompatible for Exit {
    const ALL: &'static [Exit] = &[
        Exit::AncientCistern_To_AncientCisternBoss,
        Exit::AncientCistern_To_FloriaWaterfall,
        Exit::AncientCisternBoss_To_AncientCisternCandleRoom,
        Exit::BatreauxHouse_To_Skyloft,
        Exit::BeedlesShop_To_Sky_Night,
        Exit::BeedlesShop_To_Skyloft_Day,
        Exit::Bazaar_To_Skyloft_North,
        Exit::Bazaar_To_Skyloft_South,
        Exit::Bazaar_To_Skyloft_West,
        Exit::ParrowAndOriellesHouse_To_Skyloft,
        Exit::PeatricesHouse_To_Skyloft,
        Exit::PipersHouse_To_Skyloft,
        Exit::Skyloft_To_Bazaar_North,
        Exit::Skyloft_To_Bazaar_South,
        Exit::Skyloft_To_Bazaar_West,
        Exit::Skyloft_To_BeedlesShop_Day,
        Exit::Skyloft_To_ParrowAndOriellesHouse,
        Exit::Skyloft_To_PeatricesHouse,
        Exit::Skyloft_To_PipersHouse,
        Exit::Skyloft_To_Sky,
        Exit::Skyloft_To_SkyloftSilentRealm,
        Exit::Skyloft_To_WaterfallCave_Upper,
        Exit::Skyloft_To_WrynasHouse,
        Exit::Skyloft_To_Sky_PastWaterfallCave,
        Exit::Skyloft_To_WaterfallCave_Lower,
        Exit::Skyloft_To_SkyKeepEntry,
        Exit::WaterfallCave_To_Skyloft_Upper,
        Exit::WaterfallCave_To_Skyloft_Lower,
        Exit::WrynasHouse_To_Skyloft,
        Exit::EarthTemple_To_EarthTempleBoss,
        Exit::EarthTemple_To_EldinVolcano,
        Exit::EarthTempleBoss_To_EarthTempleSpring,
        Exit::EarthTempleSpring_To_EldinVolcano,
        Exit::EldinSilentRealm_To_EldinVolcano,
        Exit::EldinVolcano_To_Sky_EldinEntranceStatue,
        Exit::EldinVolcano_To_InsideVolcanoSummit,
        Exit::EldinVolcano_To_ThrillDiggerCave,
        Exit::EldinVolcano_To_EarthTemple,
        Exit::EldinVolcano_To_Sky_TempleEntranceStatue,
        Exit::EldinVolcano_To_MogmaTurf_Skydive,
        Exit::EldinVolcano_To_Sky_VolcanoEastStatue,
        Exit::EldinVolcano_To_EldinSilentRealm,
        Exit::EldinVolcano_To_Sky_VolcanoAscentStatue,
        Exit::ThrillDiggerCave_To_EldinVolcano,
        Exit::FaronSilentRealm_To_FaronWoods,
        Exit::DeepWoods_To_FaronWoods,
        Exit::DeepWoods_To_Sky_DeepWoodsStatue,
        Exit::DeepWoods_To_Sky_ForestTempleStatue,
        Exit::DeepWoods_To_SkyviewTemple,
        Exit::FaronWoods_To_BehindTheTemple,
        Exit::FaronWoods_To_Sky_FaronWoodsEntryStatue,
        Exit::FaronWoods_To_GreatTree_LowerPlatform,
        Exit::FaronWoods_To_GreatTree_UpperPlatform,
        Exit::FaronWoods_To_GreatTree_Top,
        Exit::FaronWoods_To_Sky_GreatTreeStatue,
        Exit::FaronWoods_To_DeepWoods,
        Exit::FaronWoods_To_FaronSilentRealm,
        Exit::FaronWoods_To_GreatTree_Tunnel,
        Exit::FaronWoods_To_LakeFloria,
        Exit::FaronWoods_To_Sky_InTheWoodsStatue,
        Exit::FaronWoods_To_Sky_ViewingPlatformStatue,
        Exit::GreatTree_To_FaronWoods_Tunnel,
        Exit::GreatTree_To_FaronWoods_LowerPlatform,
        Exit::GreatTree_To_FaronWoods_UpperPlatform,
        Exit::GreatTree_To_FaronWoods_Top,
        Exit::FireSanctuaryA_To_OutsideFireSanctuary,
        Exit::FireSanctuaryA_To_FireSanctuaryBoss,
        Exit::FireSanctuaryA_To_FireSanctuaryB,
        Exit::FireSanctuaryB_To_FireSanctuaryA,
        Exit::FireSanctuaryBoss_To_FireSanctuaryFlameRoom,
        Exit::InsideGoddessStatue_To_Skyloft,
        Exit::KnightAcademy_To_Skyloft_LowerDoors,
        Exit::KnightAcademy_To_Skyloft_UpperDoors,
        Exit::Skyloft_To_InsideGoddessStatue,
        Exit::Skyloft_To_KnightAcademy_Chimney,
        Exit::Skyloft_To_KnightAcademy_LowerDoors,
        Exit::Skyloft_To_KnightAcademy_UpperDoors,
        Exit::Skyloft_To_SparringHall,
        Exit::SparringHall_To_Skyloft,
        Exit::FaroresLair_To_FloriaWaterfall,
        Exit::FaroresLair_To_LakeFloria,
        Exit::FloriaWaterfall_To_AncientCistern,
        Exit::FloriaWaterfall_To_FaronWoods,
        Exit::FloriaWaterfall_To_FaroresLair,
        Exit::FloriaWaterfall_To_Sky_FloriaWaterfallStatue,
        Exit::LakeFloria_To_Sky_LakeFloriaStatue,
        Exit::LakeFloria_To_FaroresLair,
        Exit::LanayruCaves_To_LanayruDesert,
        Exit::LanayruCaves_To_LanayruMines,
        Exit::LanayruCaves_To_SandSeaDocks,
        Exit::FireNode_To_LanayruDesert,
        Exit::LanayruDesert_To_LanayruMines,
        Exit::LanayruDesert_To_Sky_DesertEntranceStatue,
        Exit::LanayruDesert_To_FireNode,
        Exit::LanayruDesert_To_LanayruMiningFacilityA,
        Exit::LanayruDesert_To_LanayruSilentRealm,
        Exit::LanayruDesert_To_LightningNode,
        Exit::LanayruDesert_To_Sky_NorthDesertStatue,
        Exit::LanayruDesert_To_Sky_StoneCacheStatue,
        Exit::LanayruDesert_To_TempleOfTime_End,
        Exit::LanayruDesert_To_LanayruCaves,
        Exit::LanayruDesert_To_Sky_WestDesertStatue,
        Exit::LanayruDesert_To_TempleOfTime_Start,
        Exit::LightningNode_To_LanayruDesert,
        Exit::TempleOfTime_To_LanayruDesert_End,
        Exit::TempleOfTime_To_LanayruDesert_Start,
        Exit::LanayruMines_To_Sky_LanayruMineEntryStatue,
        Exit::LanayruMines_To_LanayruCaves,
        Exit::LanayruMines_To_LanayruDesert,
        Exit::LanayruMiningFacilityA_To_LanayruDesert,
        Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub2,
        Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_HubW,
        Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub,
        Exit::LanayruMiningFacilityB_To_LanayruMiningFacilityBoss,
        Exit::LanayruMiningFacilityBoss_To_LanayruMiningFacilityToToT,
        Exit::LanayruMiningFacilityToToT_To_TempleOfTime,
        Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_End,
        Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_Beginning,
        Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_End,
        Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_Beginning,
        Exit::OutsidePiratesStronghold_To_SandSea,
        Exit::SandSea_To_OutsidePiratesStronghold,
        Exit::SandSea_To_SandSeaDocks,
        Exit::SandSea_To_Sandship,
        Exit::SandSea_To_Shipyard,
        Exit::SandSea_To_SkippersRetreat,
        Exit::SandSeaDocks_To_SandSea,
        Exit::SandSeaDocks_To_Sky_AncientHarbor,
        Exit::SandSeaDocks_To_LanayruCaves,
        Exit::Shipyard_To_ShipyardConstructionBay_Upper,
        Exit::Shipyard_To_SandSea,
        Exit::Shipyard_To_ShipyardConstructionBay_Lower,
        Exit::ShipyardConstructionBay_To_Shipyard_Lower,
        Exit::ShipyardConstructionBay_To_Shipyard_Upper,
        Exit::SkippersRetreat_To_SkippersShack,
        Exit::SkippersRetreat_To_SandSea,
        Exit::SkippersShack_To_SkippersRetreat,
        Exit::LanayruSilentRealm_To_LanayruDesert,
        Exit::MogmaTurf_To_EldinVolcano_EndVent,
        Exit::MogmaTurf_To_EldinVolcano_StartVent,
        Exit::Sandship_To_SandSea,
        Exit::Sandship_To_SandshipBoss,
        Exit::BehindTheTemple_To_FaronWoods,
        Exit::BehindTheTemple_To_SealedGroundsSpiral,
        Exit::BehindTheTemple_To_SealedTemple,
        Exit::BehindTheTemple_To_Sky_BehindTheTempleStatue,
        Exit::SealedGroundsSpiral_To_SealedTemple,
        Exit::SealedGroundsSpiral_To_Sky_SealedGroundsStatue,
        Exit::SealedTemple_To_BehindTheTemple,
        Exit::SealedTemple_To_HyliasTemple,
        Exit::SealedTemple_To_SealedGroundsSpiral,
        Exit::InsideBambooIsland_To_Sky,
        Exit::LumpyPumpkin_To_Sky_North,
        Exit::LumpyPumpkin_To_Sky_South,
        Exit::Sky_To_BeedlesShop_Night,
        Exit::Sky_To_BehindTheTemple_BehindTheTempleStatue,
        Exit::Sky_To_DeepWoods_DeepWoodsStatue,
        Exit::Sky_To_DeepWoods_ForestTempleStatue,
        Exit::Sky_To_EldinVolcano_EldinEntranceStatue,
        Exit::Sky_To_EldinVolcano_TempleEntranceStatue,
        Exit::Sky_To_EldinVolcano_VolcanoEastStatue,
        Exit::Sky_To_EldinVolcano_VolcanoAscentStatue,
        Exit::Sky_To_FaronWoods_FaronWoodsEntryStatue,
        Exit::Sky_To_FaronWoods_GreatTreeStatue,
        Exit::Sky_To_FaronWoods_InTheWoodsStatue,
        Exit::Sky_To_FaronWoods_ViewingPlatformStatue,
        Exit::Sky_To_FloriaWaterfall_FloriaWaterfallStatue,
        Exit::Sky_To_InsideBambooIsland,
        Exit::Sky_To_InsideThunderhead,
        Exit::Sky_To_LakeFloria_LakeFloriaStatue,
        Exit::Sky_To_LanayruDesert_DesertEntranceStatue,
        Exit::Sky_To_LanayruDesert_NorthDesertStatue,
        Exit::Sky_To_LanayruDesert_StoneCacheStatue,
        Exit::Sky_To_LanayruDesert_WestDesertStatue,
        Exit::Sky_To_LanayruMines_LanayruMineEntryStatue,
        Exit::Sky_To_OutsideFireSanctuary_InsideTheVolcanoStatue,
        Exit::Sky_To_SealedGroundsSpiral_SealedGroundsStatue,
        Exit::Sky_To_Skyloft,
        Exit::Sky_To_LumpyPumpkin_North,
        Exit::Sky_To_LumpyPumpkin_South,
        Exit::SkyKeepEntry_To_Skyloft,
        Exit::SkyloftSilentRealm_To_Skyloft,
        Exit::BertiesHouse_To_Skyloft,
        Exit::GondosHouse_To_Skyloft,
        Exit::MallarasHouse_To_Skyloft,
        Exit::RupinsHouse_To_Skyloft,
        Exit::Skyloft_To_BatreauxHouse,
        Exit::Skyloft_To_BertiesHouse,
        Exit::Skyloft_To_GondosHouse,
        Exit::Skyloft_To_MallarasHouse,
        Exit::Skyloft_To_RupinsHouse,
        Exit::Skyloft_To_SparrotsHouse,
        Exit::SparrotsHouse_To_Skyloft,
        Exit::SkyviewBoss_To_SkyviewSpring,
        Exit::SkyviewBoss_To_SkyviewTemple,
        Exit::SkyviewSpring_To_DeepWoods,
        Exit::SkyviewSpring_To_SkyviewBoss,
        Exit::SkyviewTemple_To_SkyviewBoss,
        Exit::SkyviewTemple_To_DeepWoods,
        Exit::InsideThunderhead_To_IsleOfSongs,
        Exit::InsideThunderhead_To_Sky,
        Exit::IsleOfSongs_To_InsideThunderhead,
        Exit::InsideVolcanoSummit_To_EldinVolcano,
        Exit::InsideVolcanoSummit_To_OutsideFireSanctuary,
        Exit::InsideVolcanoSummit_To_VolcanoSummitWaterfall,
        Exit::OutsideFireSanctuary_To_FireSanctuaryA,
        Exit::OutsideFireSanctuary_To_Sky_InsideTheVolcanoStatue,
        Exit::OutsideFireSanctuary_To_InsideVolcanoSummit,
        Exit::VolcanoSummitWaterfall_To_InsideVolcanoSummit,
    ];
}
impl Exit {
    pub fn name(&self) -> &'static str {
        match self {
            Exit::AncientCistern_To_AncientCisternBoss => "Ancient Cistern -> Ancient Cistern Boss",
            Exit::AncientCistern_To_FloriaWaterfall => "Ancient Cistern -> Floria Waterfall",
            Exit::AncientCisternBoss_To_AncientCisternCandleRoom => {
                "Ancient Cistern Boss -> Ancient Cistern Candle Room"
            }
            Exit::BatreauxHouse_To_Skyloft => "Batreaux' House -> Skyloft",
            Exit::BeedlesShop_To_Sky_Night => "Beedle's Shop -> Sky (Night)",
            Exit::BeedlesShop_To_Skyloft_Day => "Beedle's Shop -> Skyloft (Day)",
            Exit::Bazaar_To_Skyloft_North => "Bazaar -> Skyloft (North)",
            Exit::Bazaar_To_Skyloft_South => "Bazaar -> Skyloft (South)",
            Exit::Bazaar_To_Skyloft_West => "Bazaar -> Skyloft (West)",
            Exit::ParrowAndOriellesHouse_To_Skyloft => "Parrow and Orielle's House -> Skyloft",
            Exit::PeatricesHouse_To_Skyloft => "Peatrice's House -> Skyloft",
            Exit::PipersHouse_To_Skyloft => "Piper's House -> Skyloft",
            Exit::Skyloft_To_Bazaar_North => "Skyloft -> Bazaar (North)",
            Exit::Skyloft_To_Bazaar_South => "Skyloft -> Bazaar (South)",
            Exit::Skyloft_To_Bazaar_West => "Skyloft -> Bazaar (West)",
            Exit::Skyloft_To_BeedlesShop_Day => "Skyloft -> Beedle's Shop (Day)",
            Exit::Skyloft_To_ParrowAndOriellesHouse => "Skyloft -> Parrow and Orielle's House",
            Exit::Skyloft_To_PeatricesHouse => "Skyloft -> Peatrice's House",
            Exit::Skyloft_To_PipersHouse => "Skyloft -> Piper's House",
            Exit::Skyloft_To_Sky => "Skyloft -> Sky",
            Exit::Skyloft_To_SkyloftSilentRealm => "Skyloft -> Skyloft Silent Realm",
            Exit::Skyloft_To_WaterfallCave_Upper => "Skyloft -> Waterfall Cave (Upper)",
            Exit::Skyloft_To_WrynasHouse => "Skyloft -> Wryna's House",
            Exit::Skyloft_To_Sky_PastWaterfallCave => "Skyloft -> Sky (Past Waterfall Cave)",
            Exit::Skyloft_To_WaterfallCave_Lower => "Skyloft -> Waterfall Cave (Lower)",
            Exit::Skyloft_To_SkyKeepEntry => "Skyloft -> Sky Keep Entry",
            Exit::WaterfallCave_To_Skyloft_Upper => "Waterfall Cave -> Skyloft (Upper)",
            Exit::WaterfallCave_To_Skyloft_Lower => "Waterfall Cave -> Skyloft (Lower)",
            Exit::WrynasHouse_To_Skyloft => "Wryna's House -> Skyloft",
            Exit::EarthTemple_To_EarthTempleBoss => "Earth Temple -> Earth Temple Boss",
            Exit::EarthTemple_To_EldinVolcano => "Earth Temple -> Eldin Volcano",
            Exit::EarthTempleBoss_To_EarthTempleSpring => {
                "Earth Temple Boss -> Earth Temple Spring"
            }
            Exit::EarthTempleSpring_To_EldinVolcano => "Earth Temple Spring -> Eldin Volcano",
            Exit::EldinSilentRealm_To_EldinVolcano => "Eldin Silent Realm -> Eldin Volcano",
            Exit::EldinVolcano_To_Sky_EldinEntranceStatue => {
                "Eldin Volcano -> Sky (Eldin Entrance Statue)"
            }
            Exit::EldinVolcano_To_InsideVolcanoSummit => "Eldin Volcano -> Inside Volcano Summit",
            Exit::EldinVolcano_To_ThrillDiggerCave => "Eldin Volcano -> Thrill Digger Cave",
            Exit::EldinVolcano_To_EarthTemple => "Eldin Volcano -> Earth Temple",
            Exit::EldinVolcano_To_Sky_TempleEntranceStatue => {
                "Eldin Volcano -> Sky (Temple Entrance Statue)"
            }
            Exit::EldinVolcano_To_MogmaTurf_Skydive => "Eldin Volcano -> Mogma Turf (Skydive)",
            Exit::EldinVolcano_To_Sky_VolcanoEastStatue => {
                "Eldin Volcano -> Sky (Volcano East Statue)"
            }
            Exit::EldinVolcano_To_EldinSilentRealm => "Eldin Volcano -> Eldin Silent Realm",
            Exit::EldinVolcano_To_Sky_VolcanoAscentStatue => {
                "Eldin Volcano -> Sky (Volcano Ascent Statue)"
            }
            Exit::ThrillDiggerCave_To_EldinVolcano => "Thrill Digger Cave -> Eldin Volcano",
            Exit::FaronSilentRealm_To_FaronWoods => "Faron Silent Realm -> Faron Woods",
            Exit::DeepWoods_To_FaronWoods => "Deep Woods -> Faron Woods",
            Exit::DeepWoods_To_Sky_DeepWoodsStatue => "Deep Woods -> Sky (Deep Woods Statue)",
            Exit::DeepWoods_To_Sky_ForestTempleStatue => "Deep Woods -> Sky (Forest Temple Statue)",
            Exit::DeepWoods_To_SkyviewTemple => "Deep Woods -> Skyview Temple",
            Exit::FaronWoods_To_BehindTheTemple => "Faron Woods -> Behind the Temple",
            Exit::FaronWoods_To_Sky_FaronWoodsEntryStatue => {
                "Faron Woods -> Sky (Faron Woods Entry Statue)"
            }
            Exit::FaronWoods_To_GreatTree_LowerPlatform => {
                "Faron Woods -> Great Tree (Lower Platform)"
            }
            Exit::FaronWoods_To_GreatTree_UpperPlatform => {
                "Faron Woods -> Great Tree (Upper Platform)"
            }
            Exit::FaronWoods_To_GreatTree_Top => "Faron Woods -> Great Tree (Top)",
            Exit::FaronWoods_To_Sky_GreatTreeStatue => "Faron Woods -> Sky (Great Tree Statue)",
            Exit::FaronWoods_To_DeepWoods => "Faron Woods -> Deep Woods",
            Exit::FaronWoods_To_FaronSilentRealm => "Faron Woods -> Faron Silent Realm",
            Exit::FaronWoods_To_GreatTree_Tunnel => "Faron Woods -> Great Tree (Tunnel)",
            Exit::FaronWoods_To_LakeFloria => "Faron Woods -> Lake Floria",
            Exit::FaronWoods_To_Sky_InTheWoodsStatue => "Faron Woods -> Sky (In the Woods Statue)",
            Exit::FaronWoods_To_Sky_ViewingPlatformStatue => {
                "Faron Woods -> Sky (Viewing Platform Statue)"
            }
            Exit::GreatTree_To_FaronWoods_Tunnel => "Great Tree -> Faron Woods (Tunnel)",
            Exit::GreatTree_To_FaronWoods_LowerPlatform => {
                "Great Tree -> Faron Woods (Lower Platform)"
            }
            Exit::GreatTree_To_FaronWoods_UpperPlatform => {
                "Great Tree -> Faron Woods (Upper Platform)"
            }
            Exit::GreatTree_To_FaronWoods_Top => "Great Tree -> Faron Woods (Top)",
            Exit::FireSanctuaryA_To_OutsideFireSanctuary => {
                "Fire Sanctuary A -> Outside Fire Sanctuary"
            }
            Exit::FireSanctuaryA_To_FireSanctuaryBoss => "Fire Sanctuary A -> Fire Sanctuary Boss",
            Exit::FireSanctuaryA_To_FireSanctuaryB => "Fire Sanctuary A -> Fire Sanctuary B",
            Exit::FireSanctuaryB_To_FireSanctuaryA => "Fire Sanctuary B -> Fire Sanctuary A",
            Exit::FireSanctuaryBoss_To_FireSanctuaryFlameRoom => {
                "Fire Sanctuary Boss -> Fire Sanctuary Flame Room"
            }
            Exit::InsideGoddessStatue_To_Skyloft => "Inside Goddess Statue -> Skyloft",
            Exit::KnightAcademy_To_Skyloft_LowerDoors => "Knight Academy -> Skyloft (Lower Doors)",
            Exit::KnightAcademy_To_Skyloft_UpperDoors => "Knight Academy -> Skyloft (Upper Doors)",
            Exit::Skyloft_To_InsideGoddessStatue => "Skyloft -> Inside Goddess Statue",
            Exit::Skyloft_To_KnightAcademy_Chimney => "Skyloft -> Knight Academy (Chimney)",
            Exit::Skyloft_To_KnightAcademy_LowerDoors => "Skyloft -> Knight Academy (Lower Doors)",
            Exit::Skyloft_To_KnightAcademy_UpperDoors => "Skyloft -> Knight Academy (Upper Doors)",
            Exit::Skyloft_To_SparringHall => "Skyloft -> Sparring Hall",
            Exit::SparringHall_To_Skyloft => "Sparring Hall -> Skyloft",
            Exit::FaroresLair_To_FloriaWaterfall => "Farore's Lair -> Floria Waterfall",
            Exit::FaroresLair_To_LakeFloria => "Farore's Lair -> Lake Floria",
            Exit::FloriaWaterfall_To_AncientCistern => "Floria Waterfall -> Ancient Cistern",
            Exit::FloriaWaterfall_To_FaronWoods => "Floria Waterfall -> Faron Woods",
            Exit::FloriaWaterfall_To_FaroresLair => "Floria Waterfall -> Farore's Lair",
            Exit::FloriaWaterfall_To_Sky_FloriaWaterfallStatue => {
                "Floria Waterfall -> Sky (Floria Waterfall Statue)"
            }
            Exit::LakeFloria_To_Sky_LakeFloriaStatue => "Lake Floria -> Sky (Lake Floria Statue)",
            Exit::LakeFloria_To_FaroresLair => "Lake Floria -> Farore's Lair",
            Exit::LanayruCaves_To_LanayruDesert => "Lanayru Caves -> Lanayru Desert",
            Exit::LanayruCaves_To_LanayruMines => "Lanayru Caves -> Lanayru Mines",
            Exit::LanayruCaves_To_SandSeaDocks => "Lanayru Caves -> Sand Sea Docks",
            Exit::FireNode_To_LanayruDesert => "Fire Node -> Lanayru Desert",
            Exit::LanayruDesert_To_LanayruMines => "Lanayru Desert -> Lanayru Mines",
            Exit::LanayruDesert_To_Sky_DesertEntranceStatue => {
                "Lanayru Desert -> Sky (Desert Entrance Statue)"
            }
            Exit::LanayruDesert_To_FireNode => "Lanayru Desert -> Fire Node",
            Exit::LanayruDesert_To_LanayruMiningFacilityA => {
                "Lanayru Desert -> Lanayru Mining Facility A"
            }
            Exit::LanayruDesert_To_LanayruSilentRealm => "Lanayru Desert -> Lanayru Silent Realm",
            Exit::LanayruDesert_To_LightningNode => "Lanayru Desert -> Lightning Node",
            Exit::LanayruDesert_To_Sky_NorthDesertStatue => {
                "Lanayru Desert -> Sky (North Desert Statue)"
            }
            Exit::LanayruDesert_To_Sky_StoneCacheStatue => {
                "Lanayru Desert -> Sky (Stone Cache Statue)"
            }
            Exit::LanayruDesert_To_TempleOfTime_End => "Lanayru Desert -> Temple of Time (End)",
            Exit::LanayruDesert_To_LanayruCaves => "Lanayru Desert -> Lanayru Caves",
            Exit::LanayruDesert_To_Sky_WestDesertStatue => {
                "Lanayru Desert -> Sky (West Desert Statue)"
            }
            Exit::LanayruDesert_To_TempleOfTime_Start => "Lanayru Desert -> Temple of Time (Start)",
            Exit::LightningNode_To_LanayruDesert => "Lightning Node -> Lanayru Desert",
            Exit::TempleOfTime_To_LanayruDesert_End => "Temple of Time -> Lanayru Desert (End)",
            Exit::TempleOfTime_To_LanayruDesert_Start => "Temple of Time -> Lanayru Desert (Start)",
            Exit::LanayruMines_To_Sky_LanayruMineEntryStatue => {
                "Lanayru Mines -> Sky (Lanayru Mine Entry Statue)"
            }
            Exit::LanayruMines_To_LanayruCaves => "Lanayru Mines -> Lanayru Caves",
            Exit::LanayruMines_To_LanayruDesert => "Lanayru Mines -> Lanayru Desert",
            Exit::LanayruMiningFacilityA_To_LanayruDesert => {
                "Lanayru Mining Facility A -> Lanayru Desert"
            }
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub2 => {
                "Lanayru Mining Facility A -> Lanayru Mining Facility B (Hub 2)"
            }
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_HubW => {
                "Lanayru Mining Facility A -> Lanayru Mining Facility B (Hub W)"
            }
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub => {
                "Lanayru Mining Facility A -> Lanayru Mining Facility B (Hub)"
            }
            Exit::LanayruMiningFacilityB_To_LanayruMiningFacilityBoss => {
                "Lanayru Mining Facility B -> Lanayru Mining Facility Boss"
            }
            Exit::LanayruMiningFacilityBoss_To_LanayruMiningFacilityToToT => {
                "Lanayru Mining Facility Boss -> Lanayru Mining Facility to ToT"
            }
            Exit::LanayruMiningFacilityToToT_To_TempleOfTime => {
                "Lanayru Mining Facility to ToT -> Temple of Time"
            }
            Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_End => {
                "Inside Pirate's Stronghold -> Outside Pirate's Stronghold (End)"
            }
            Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_Beginning => {
                "Inside Pirate's Stronghold -> Outside Pirate's Stronghold (Beginning)"
            }
            Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_End => {
                "Outside Pirate's Stronghold -> Inside Pirate's Stronghold (End)"
            }
            Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_Beginning => {
                "Outside Pirate's Stronghold -> Inside Pirate's Stronghold (Beginning)"
            }
            Exit::OutsidePiratesStronghold_To_SandSea => "Outside Pirate's Stronghold -> Sand Sea",
            Exit::SandSea_To_OutsidePiratesStronghold => "Sand Sea -> Outside Pirate's Stronghold",
            Exit::SandSea_To_SandSeaDocks => "Sand Sea -> Sand Sea Docks",
            Exit::SandSea_To_Sandship => "Sand Sea -> Sandship",
            Exit::SandSea_To_Shipyard => "Sand Sea -> Shipyard",
            Exit::SandSea_To_SkippersRetreat => "Sand Sea -> Skipper's Retreat",
            Exit::SandSeaDocks_To_SandSea => "Sand Sea Docks -> Sand Sea",
            Exit::SandSeaDocks_To_Sky_AncientHarbor => "Sand Sea Docks -> Sky (Ancient Harbor)",
            Exit::SandSeaDocks_To_LanayruCaves => "Sand Sea Docks -> Lanayru Caves",
            Exit::Shipyard_To_ShipyardConstructionBay_Upper => {
                "Shipyard -> Shipyard Construction Bay (Upper)"
            }
            Exit::Shipyard_To_SandSea => "Shipyard -> Sand Sea",
            Exit::Shipyard_To_ShipyardConstructionBay_Lower => {
                "Shipyard -> Shipyard Construction Bay (Lower)"
            }
            Exit::ShipyardConstructionBay_To_Shipyard_Lower => {
                "Shipyard Construction Bay -> Shipyard (Lower)"
            }
            Exit::ShipyardConstructionBay_To_Shipyard_Upper => {
                "Shipyard Construction Bay -> Shipyard (Upper)"
            }
            Exit::SkippersRetreat_To_SkippersShack => "Skipper's Retreat -> Skipper's Shack",
            Exit::SkippersRetreat_To_SandSea => "Skipper's Retreat -> Sand Sea",
            Exit::SkippersShack_To_SkippersRetreat => "Skipper's Shack -> Skipper's Retreat",
            Exit::LanayruSilentRealm_To_LanayruDesert => "Lanayru Silent Realm -> Lanayru Desert",
            Exit::MogmaTurf_To_EldinVolcano_EndVent => "Mogma Turf -> Eldin Volcano (End Vent)",
            Exit::MogmaTurf_To_EldinVolcano_StartVent => "Mogma Turf -> Eldin Volcano (Start Vent)",
            Exit::Sandship_To_SandSea => "Sandship -> Sand Sea",
            Exit::Sandship_To_SandshipBoss => "Sandship -> Sandship Boss",
            Exit::BehindTheTemple_To_FaronWoods => "Behind the Temple -> Faron Woods",
            Exit::BehindTheTemple_To_SealedGroundsSpiral => {
                "Behind the Temple -> Sealed Grounds Spiral"
            }
            Exit::BehindTheTemple_To_SealedTemple => "Behind the Temple -> Sealed Temple",
            Exit::BehindTheTemple_To_Sky_BehindTheTempleStatue => {
                "Behind the Temple -> Sky (Behind the Temple Statue)"
            }
            Exit::SealedGroundsSpiral_To_SealedTemple => "Sealed Grounds Spiral -> Sealed Temple",
            Exit::SealedGroundsSpiral_To_Sky_SealedGroundsStatue => {
                "Sealed Grounds Spiral -> Sky (Sealed Grounds Statue)"
            }
            Exit::SealedTemple_To_BehindTheTemple => "Sealed Temple -> Behind the Temple",
            Exit::SealedTemple_To_HyliasTemple => "Sealed Temple -> Hylia's Temple",
            Exit::SealedTemple_To_SealedGroundsSpiral => "Sealed Temple -> Sealed Grounds Spiral",
            Exit::InsideBambooIsland_To_Sky => "Inside Bamboo Island -> Sky",
            Exit::LumpyPumpkin_To_Sky_North => "Lumpy Pumpkin -> Sky (North)",
            Exit::LumpyPumpkin_To_Sky_South => "Lumpy Pumpkin -> Sky (South)",
            Exit::Sky_To_BeedlesShop_Night => "Sky -> Beedle's Shop (Night)",
            Exit::Sky_To_BehindTheTemple_BehindTheTempleStatue => {
                "Sky -> Behind the Temple (Behind the Temple Statue)"
            }
            Exit::Sky_To_DeepWoods_DeepWoodsStatue => "Sky -> Deep Woods (Deep Woods Statue)",
            Exit::Sky_To_DeepWoods_ForestTempleStatue => "Sky -> Deep Woods (Forest Temple Statue)",
            Exit::Sky_To_EldinVolcano_EldinEntranceStatue => {
                "Sky -> Eldin Volcano (Eldin Entrance Statue)"
            }
            Exit::Sky_To_EldinVolcano_TempleEntranceStatue => {
                "Sky -> Eldin Volcano (Temple Entrance Statue)"
            }
            Exit::Sky_To_EldinVolcano_VolcanoEastStatue => {
                "Sky -> Eldin Volcano (Volcano East Statue)"
            }
            Exit::Sky_To_EldinVolcano_VolcanoAscentStatue => {
                "Sky -> Eldin Volcano (Volcano Ascent Statue)"
            }
            Exit::Sky_To_FaronWoods_FaronWoodsEntryStatue => {
                "Sky -> Faron Woods (Faron Woods Entry Statue)"
            }
            Exit::Sky_To_FaronWoods_GreatTreeStatue => "Sky -> Faron Woods (Great Tree Statue)",
            Exit::Sky_To_FaronWoods_InTheWoodsStatue => "Sky -> Faron Woods (In the Woods Statue)",
            Exit::Sky_To_FaronWoods_ViewingPlatformStatue => {
                "Sky -> Faron Woods (Viewing Platform Statue)"
            }
            Exit::Sky_To_FloriaWaterfall_FloriaWaterfallStatue => {
                "Sky -> Floria Waterfall (Floria Waterfall Statue)"
            }
            Exit::Sky_To_InsideBambooIsland => "Sky -> Inside Bamboo Island",
            Exit::Sky_To_InsideThunderhead => "Sky -> Inside Thunderhead",
            Exit::Sky_To_LakeFloria_LakeFloriaStatue => "Sky -> Lake Floria (Lake Floria Statue)",
            Exit::Sky_To_LanayruDesert_DesertEntranceStatue => {
                "Sky -> Lanayru Desert (Desert Entrance Statue)"
            }
            Exit::Sky_To_LanayruDesert_NorthDesertStatue => {
                "Sky -> Lanayru Desert (North Desert Statue)"
            }
            Exit::Sky_To_LanayruDesert_StoneCacheStatue => {
                "Sky -> Lanayru Desert (Stone Cache Statue)"
            }
            Exit::Sky_To_LanayruDesert_WestDesertStatue => {
                "Sky -> Lanayru Desert (West Desert Statue)"
            }
            Exit::Sky_To_LanayruMines_LanayruMineEntryStatue => {
                "Sky -> Lanayru Mines (Lanayru Mine Entry Statue)"
            }
            Exit::Sky_To_OutsideFireSanctuary_InsideTheVolcanoStatue => {
                "Sky -> Outside Fire Sanctuary (Inside the Volcano Statue)"
            }
            Exit::Sky_To_SealedGroundsSpiral_SealedGroundsStatue => {
                "Sky -> Sealed Grounds Spiral (Sealed Grounds Statue)"
            }
            Exit::Sky_To_Skyloft => "Sky -> Skyloft",
            Exit::Sky_To_LumpyPumpkin_North => "Sky -> Lumpy Pumpkin (North)",
            Exit::Sky_To_LumpyPumpkin_South => "Sky -> Lumpy Pumpkin (South)",
            Exit::SkyKeepEntry_To_Skyloft => "Sky Keep Entry -> Skyloft",
            Exit::SkyloftSilentRealm_To_Skyloft => "Skyloft Silent Realm -> Skyloft",
            Exit::BertiesHouse_To_Skyloft => "Bertie's House -> Skyloft",
            Exit::GondosHouse_To_Skyloft => "Gondo's House -> Skyloft",
            Exit::MallarasHouse_To_Skyloft => "Mallara's House -> Skyloft",
            Exit::RupinsHouse_To_Skyloft => "Rupin's House -> Skyloft",
            Exit::Skyloft_To_BatreauxHouse => "Skyloft -> Batreaux' House",
            Exit::Skyloft_To_BertiesHouse => "Skyloft -> Bertie's House",
            Exit::Skyloft_To_GondosHouse => "Skyloft -> Gondo's House",
            Exit::Skyloft_To_MallarasHouse => "Skyloft -> Mallara's House",
            Exit::Skyloft_To_RupinsHouse => "Skyloft -> Rupin's House",
            Exit::Skyloft_To_SparrotsHouse => "Skyloft -> Sparrot's House",
            Exit::SparrotsHouse_To_Skyloft => "Sparrot's House -> Skyloft",
            Exit::SkyviewBoss_To_SkyviewSpring => "Skyview Boss -> Skyview Spring",
            Exit::SkyviewBoss_To_SkyviewTemple => "Skyview Boss -> Skyview Temple",
            Exit::SkyviewSpring_To_DeepWoods => "Skyview Spring -> Deep Woods",
            Exit::SkyviewSpring_To_SkyviewBoss => "Skyview Spring -> Skyview Boss",
            Exit::SkyviewTemple_To_SkyviewBoss => "Skyview Temple -> Skyview Boss",
            Exit::SkyviewTemple_To_DeepWoods => "Skyview Temple -> Deep Woods",
            Exit::InsideThunderhead_To_IsleOfSongs => "Inside Thunderhead -> Isle of Songs",
            Exit::InsideThunderhead_To_Sky => "Inside Thunderhead -> Sky",
            Exit::IsleOfSongs_To_InsideThunderhead => "Isle of Songs -> Inside Thunderhead",
            Exit::InsideVolcanoSummit_To_EldinVolcano => "Inside Volcano Summit -> Eldin Volcano",
            Exit::InsideVolcanoSummit_To_OutsideFireSanctuary => {
                "Inside Volcano Summit -> Outside Fire Sanctuary"
            }
            Exit::InsideVolcanoSummit_To_VolcanoSummitWaterfall => {
                "Inside Volcano Summit -> Volcano Summit Waterfall"
            }
            Exit::OutsideFireSanctuary_To_FireSanctuaryA => {
                "Outside Fire Sanctuary -> Fire Sanctuary A"
            }
            Exit::OutsideFireSanctuary_To_Sky_InsideTheVolcanoStatue => {
                "Outside Fire Sanctuary -> Sky (Inside the Volcano Statue)"
            }
            Exit::OutsideFireSanctuary_To_InsideVolcanoSummit => {
                "Outside Fire Sanctuary -> Inside Volcano Summit"
            }
            Exit::VolcanoSummitWaterfall_To_InsideVolcanoSummit => {
                "Volcano Summit Waterfall -> Inside Volcano Summit"
            }
        }
    }
}
impl Exit {
    pub fn area(&self) -> Area {
        match self {
            Exit::AncientCistern_To_AncientCisternBoss => Area::AncientCistern_BeforeBossDoor,
            Exit::AncientCistern_To_FloriaWaterfall => Area::AncientCistern_MainHub,
            Exit::AncientCisternBoss_To_AncientCisternCandleRoom => Area::AncientCisternBoss_Main,
            Exit::BatreauxHouse_To_Skyloft => Area::BatreauxHouse_Main,
            Exit::BeedlesShop_To_Sky_Night => Area::BeedlesShop_Main,
            Exit::BeedlesShop_To_Skyloft_Day => Area::BeedlesShop_Main,
            Exit::Bazaar_To_Skyloft_North => Area::Bazaar_Main,
            Exit::Bazaar_To_Skyloft_South => Area::Bazaar_Main,
            Exit::Bazaar_To_Skyloft_West => Area::Bazaar_Main,
            Exit::ParrowAndOriellesHouse_To_Skyloft => Area::ParrowAndOriellesHouse_Main,
            Exit::PeatricesHouse_To_Skyloft => Area::PeatricesHouse_Main,
            Exit::PipersHouse_To_Skyloft => Area::PipersHouse_Main,
            Exit::Skyloft_To_Bazaar_North => Area::Skyloft_CentralOutside,
            Exit::Skyloft_To_Bazaar_South => Area::Skyloft_CentralOutside,
            Exit::Skyloft_To_Bazaar_West => Area::Skyloft_CentralOutside,
            Exit::Skyloft_To_BeedlesShop_Day => Area::Skyloft_CentralOutside,
            Exit::Skyloft_To_ParrowAndOriellesHouse => Area::Skyloft_CentralOutside,
            Exit::Skyloft_To_PeatricesHouse => Area::Skyloft_CentralOutside,
            Exit::Skyloft_To_PipersHouse => Area::Skyloft_CentralOutside,
            Exit::Skyloft_To_Sky => Area::Skyloft_CentralOutside,
            Exit::Skyloft_To_SkyloftSilentRealm => Area::Skyloft_CentralOutside,
            Exit::Skyloft_To_WaterfallCave_Upper => Area::Skyloft_CentralOutside,
            Exit::Skyloft_To_WrynasHouse => Area::Skyloft_CentralOutside,
            Exit::Skyloft_To_Sky_PastWaterfallCave => Area::Skyloft_PastWaterfallCave,
            Exit::Skyloft_To_WaterfallCave_Lower => Area::Skyloft_PastWaterfallCave,
            Exit::Skyloft_To_SkyKeepEntry => Area::Skyloft_ToSkyKeep,
            Exit::WaterfallCave_To_Skyloft_Upper => Area::WaterfallCave_Main,
            Exit::WaterfallCave_To_Skyloft_Lower => Area::WaterfallCave_Main,
            Exit::WrynasHouse_To_Skyloft => Area::WrynasHouse_Main,
            Exit::EarthTemple_To_EarthTempleBoss => Area::EarthTemple_BossDoorArea,
            Exit::EarthTemple_To_EldinVolcano => Area::EarthTemple_Entrance,
            Exit::EarthTempleBoss_To_EarthTempleSpring => Area::EarthTempleBoss_Main,
            Exit::EarthTempleSpring_To_EldinVolcano => Area::EarthTempleSpring_Main,
            Exit::EldinSilentRealm_To_EldinVolcano => Area::EldinSilentRealm_Trial,
            Exit::EldinVolcano_To_Sky_EldinEntranceStatue => Area::EldinVolcano_FirstRoom,
            Exit::EldinVolcano_To_InsideVolcanoSummit => Area::EldinVolcano_HotCaveArea,
            Exit::EldinVolcano_To_ThrillDiggerCave => Area::EldinVolcano_NearThrillDigger,
            Exit::EldinVolcano_To_EarthTemple => Area::EldinVolcano_OutsideEt,
            Exit::EldinVolcano_To_Sky_TempleEntranceStatue => Area::EldinVolcano_OutsideEt,
            Exit::EldinVolcano_To_MogmaTurf_Skydive => Area::EldinVolcano_PreMogmaTurf,
            Exit::EldinVolcano_To_Sky_VolcanoEastStatue => Area::EldinVolcano_PreMogmaTurf,
            Exit::EldinVolcano_To_EldinSilentRealm => Area::EldinVolcano_VolcanoAscent,
            Exit::EldinVolcano_To_Sky_VolcanoAscentStatue => Area::EldinVolcano_VolcanoAscent,
            Exit::ThrillDiggerCave_To_EldinVolcano => Area::ThrillDiggerCave_Main,
            Exit::FaronSilentRealm_To_FaronWoods => Area::FaronSilentRealm_Trial,
            Exit::DeepWoods_To_FaronWoods => Area::DeepWoods_Entry,
            Exit::DeepWoods_To_Sky_DeepWoodsStatue => Area::DeepWoods_PastBeehive,
            Exit::DeepWoods_To_Sky_ForestTempleStatue => Area::DeepWoods_PastBeehive,
            Exit::DeepWoods_To_SkyviewTemple => Area::DeepWoods_PastBeehive,
            Exit::FaronWoods_To_BehindTheTemple => Area::FaronWoods_Entry,
            Exit::FaronWoods_To_Sky_FaronWoodsEntryStatue => Area::FaronWoods_Entry,
            Exit::FaronWoods_To_GreatTree_LowerPlatform => Area::FaronWoods_GreatTreePlatforms,
            Exit::FaronWoods_To_GreatTree_UpperPlatform => Area::FaronWoods_GreatTreePlatforms,
            Exit::FaronWoods_To_GreatTree_Top => Area::FaronWoods_GreatTreeTop,
            Exit::FaronWoods_To_Sky_GreatTreeStatue => Area::FaronWoods_GreatTreeTop,
            Exit::FaronWoods_To_DeepWoods => Area::FaronWoods_Main,
            Exit::FaronWoods_To_FaronSilentRealm => Area::FaronWoods_Main,
            Exit::FaronWoods_To_GreatTree_Tunnel => Area::FaronWoods_Main,
            Exit::FaronWoods_To_LakeFloria => Area::FaronWoods_Main,
            Exit::FaronWoods_To_Sky_InTheWoodsStatue => Area::FaronWoods_Main,
            Exit::FaronWoods_To_Sky_ViewingPlatformStatue => Area::FaronWoods_Main,
            Exit::GreatTree_To_FaronWoods_Tunnel => Area::GreatTree_Entry,
            Exit::GreatTree_To_FaronWoods_LowerPlatform => Area::GreatTree_PastPlatforms,
            Exit::GreatTree_To_FaronWoods_UpperPlatform => Area::GreatTree_Upper,
            Exit::GreatTree_To_FaronWoods_Top => Area::GreatTree_Upper,
            Exit::FireSanctuaryA_To_OutsideFireSanctuary => Area::FireSanctuaryA_Entry,
            Exit::FireSanctuaryA_To_FireSanctuaryBoss => Area::FireSanctuaryA_InFrontOfBossDoor,
            Exit::FireSanctuaryA_To_FireSanctuaryB => Area::FireSanctuaryA_PastFirstWaterPlant,
            Exit::FireSanctuaryB_To_FireSanctuaryA => Area::FireSanctuaryB_UnderDoubleMagmanosFight,
            Exit::FireSanctuaryBoss_To_FireSanctuaryFlameRoom => Area::FireSanctuaryBoss_Main,
            Exit::InsideGoddessStatue_To_Skyloft => Area::InsideGoddessStatue_Main,
            Exit::KnightAcademy_To_Skyloft_LowerDoors => Area::KnightAcademy_Main,
            Exit::KnightAcademy_To_Skyloft_UpperDoors => Area::KnightAcademy_Main,
            Exit::Skyloft_To_InsideGoddessStatue => Area::Skyloft_OutsideGoddessStatue,
            Exit::Skyloft_To_KnightAcademy_Chimney => Area::Skyloft_OutsideGoddessStatue,
            Exit::Skyloft_To_KnightAcademy_LowerDoors => Area::Skyloft_OutsideGoddessStatue,
            Exit::Skyloft_To_KnightAcademy_UpperDoors => Area::Skyloft_OutsideGoddessStatue,
            Exit::Skyloft_To_SparringHall => Area::Skyloft_OutsideGoddessStatue,
            Exit::SparringHall_To_Skyloft => Area::SparringHall_Main,
            Exit::FaroresLair_To_FloriaWaterfall => Area::FaroresLair_Main,
            Exit::FaroresLair_To_LakeFloria => Area::FaroresLair_Main,
            Exit::FloriaWaterfall_To_AncientCistern => Area::FloriaWaterfall_Main,
            Exit::FloriaWaterfall_To_FaronWoods => Area::FloriaWaterfall_Main,
            Exit::FloriaWaterfall_To_FaroresLair => Area::FloriaWaterfall_Main,
            Exit::FloriaWaterfall_To_Sky_FloriaWaterfallStatue => Area::FloriaWaterfall_Main,
            Exit::LakeFloria_To_Sky_LakeFloriaStatue => Area::LakeFloria_StatueSpot,
            Exit::LakeFloria_To_FaroresLair => Area::LakeFloria_ToFaroresLair,
            Exit::LanayruCaves_To_LanayruDesert => Area::LanayruCaves_Main,
            Exit::LanayruCaves_To_LanayruMines => Area::LanayruCaves_Main,
            Exit::LanayruCaves_To_SandSeaDocks => Area::LanayruCaves_ToSandSea,
            Exit::FireNode_To_LanayruDesert => Area::FireNode_Main,
            Exit::LanayruDesert_To_LanayruMines => Area::LanayruDesert_HookBeetleArea,
            Exit::LanayruDesert_To_Sky_DesertEntranceStatue => Area::LanayruDesert_HookBeetleArea,
            Exit::LanayruDesert_To_FireNode => Area::LanayruDesert_PastToT,
            Exit::LanayruDesert_To_LanayruMiningFacilityA => Area::LanayruDesert_PastToT,
            Exit::LanayruDesert_To_LanayruSilentRealm => Area::LanayruDesert_PastToT,
            Exit::LanayruDesert_To_LightningNode => Area::LanayruDesert_PastToT,
            Exit::LanayruDesert_To_Sky_NorthDesertStatue => Area::LanayruDesert_PastToT,
            Exit::LanayruDesert_To_Sky_StoneCacheStatue => Area::LanayruDesert_PastToT,
            Exit::LanayruDesert_To_TempleOfTime_End => Area::LanayruDesert_PastToT,
            Exit::LanayruDesert_To_LanayruCaves => Area::LanayruDesert_SandOasis,
            Exit::LanayruDesert_To_Sky_WestDesertStatue => Area::LanayruDesert_SandOasis,
            Exit::LanayruDesert_To_TempleOfTime_Start => Area::LanayruDesert_SandOasis,
            Exit::LightningNode_To_LanayruDesert => Area::LightningNode_Main,
            Exit::TempleOfTime_To_LanayruDesert_End => Area::TempleOfTime_End,
            Exit::TempleOfTime_To_LanayruDesert_Start => Area::TempleOfTime_Start,
            Exit::LanayruMines_To_Sky_LanayruMineEntryStatue => Area::LanayruMines_FirstHalf,
            Exit::LanayruMines_To_LanayruCaves => Area::LanayruMines_ToCaves,
            Exit::LanayruMines_To_LanayruDesert => Area::LanayruMines_ToDesert,
            Exit::LanayruMiningFacilityA_To_LanayruDesert => Area::LanayruMiningFacilityA_Entry,
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub2 => {
                Area::LanayruMiningFacilityA_GustBellowsRoom
            }
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_HubW => {
                Area::LanayruMiningFacilityA_MapRoom
            }
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub => {
                Area::LanayruMiningFacilityA_SecondRoom
            }
            Exit::LanayruMiningFacilityB_To_LanayruMiningFacilityBoss => {
                Area::LanayruMiningFacilityB_NearBossDoor
            }
            Exit::LanayruMiningFacilityBoss_To_LanayruMiningFacilityToToT => {
                Area::LanayruMiningFacilityBoss_Main
            }
            Exit::LanayruMiningFacilityToToT_To_TempleOfTime => {
                Area::LanayruMiningFacilityToToT_ToTExit
            }
            Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_End => {
                Area::InsidePiratesStronghold_Main
            }
            Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_Beginning => {
                Area::InsidePiratesStronghold_Main
            }
            Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_End => {
                Area::OutsidePiratesStronghold_InsideSharkhead
            }
            Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_Beginning => {
                Area::OutsidePiratesStronghold_Main
            }
            Exit::OutsidePiratesStronghold_To_SandSea => Area::OutsidePiratesStronghold_Main,
            Exit::SandSea_To_OutsidePiratesStronghold => Area::SandSea_Main,
            Exit::SandSea_To_SandSeaDocks => Area::SandSea_Main,
            Exit::SandSea_To_Sandship => Area::SandSea_Main,
            Exit::SandSea_To_Shipyard => Area::SandSea_Main,
            Exit::SandSea_To_SkippersRetreat => Area::SandSea_Main,
            Exit::SandSeaDocks_To_SandSea => Area::SandSeaDocks_Main,
            Exit::SandSeaDocks_To_Sky_AncientHarbor => Area::SandSeaDocks_Main,
            Exit::SandSeaDocks_To_LanayruCaves => Area::SandSeaDocks_ToCaves,
            Exit::Shipyard_To_ShipyardConstructionBay_Upper => Area::Shipyard_AfterMinecartRide,
            Exit::Shipyard_To_SandSea => Area::Shipyard_Main,
            Exit::Shipyard_To_ShipyardConstructionBay_Lower => Area::Shipyard_Main,
            Exit::ShipyardConstructionBay_To_Shipyard_Lower => Area::ShipyardConstructionBay_Lower,
            Exit::ShipyardConstructionBay_To_Shipyard_Upper => Area::ShipyardConstructionBay_Upper,
            Exit::SkippersRetreat_To_SkippersShack => Area::SkippersRetreat_NextToShack,
            Exit::SkippersRetreat_To_SandSea => Area::SkippersRetreat_Start,
            Exit::SkippersShack_To_SkippersRetreat => Area::SkippersShack_Main,
            Exit::LanayruSilentRealm_To_LanayruDesert => Area::LanayruSilentRealm_Trial,
            Exit::MogmaTurf_To_EldinVolcano_EndVent => Area::MogmaTurf_Main,
            Exit::MogmaTurf_To_EldinVolcano_StartVent => Area::MogmaTurf_Main,
            Exit::Sandship_To_SandSea => Area::Sandship_Deck,
            Exit::Sandship_To_SandshipBoss => Area::Sandship_PastSpume,
            Exit::BehindTheTemple_To_FaronWoods => Area::BehindTheTemple_Main,
            Exit::BehindTheTemple_To_SealedGroundsSpiral => Area::BehindTheTemple_Main,
            Exit::BehindTheTemple_To_SealedTemple => Area::BehindTheTemple_Main,
            Exit::BehindTheTemple_To_Sky_BehindTheTempleStatue => Area::BehindTheTemple_Main,
            Exit::SealedGroundsSpiral_To_SealedTemple => Area::SealedGroundsSpiral_Lower,
            Exit::SealedGroundsSpiral_To_Sky_SealedGroundsStatue => Area::SealedGroundsSpiral_Upper,
            Exit::SealedTemple_To_BehindTheTemple => Area::SealedTemple_Main,
            Exit::SealedTemple_To_HyliasTemple => Area::SealedTemple_Main,
            Exit::SealedTemple_To_SealedGroundsSpiral => Area::SealedTemple_Main,
            Exit::InsideBambooIsland_To_Sky => Area::InsideBambooIsland_Main,
            Exit::LumpyPumpkin_To_Sky_North => Area::LumpyPumpkin_Main,
            Exit::LumpyPumpkin_To_Sky_South => Area::LumpyPumpkin_Main,
            Exit::Sky_To_BeedlesShop_Night => Area::Sky_BeedlesSkyHome,
            Exit::Sky_To_BehindTheTemple_BehindTheTempleStatue => Area::Sky_Field,
            Exit::Sky_To_DeepWoods_DeepWoodsStatue => Area::Sky_Field,
            Exit::Sky_To_DeepWoods_ForestTempleStatue => Area::Sky_Field,
            Exit::Sky_To_EldinVolcano_EldinEntranceStatue => Area::Sky_Field,
            Exit::Sky_To_EldinVolcano_TempleEntranceStatue => Area::Sky_Field,
            Exit::Sky_To_EldinVolcano_VolcanoEastStatue => Area::Sky_Field,
            Exit::Sky_To_EldinVolcano_VolcanoAscentStatue => Area::Sky_Field,
            Exit::Sky_To_FaronWoods_FaronWoodsEntryStatue => Area::Sky_Field,
            Exit::Sky_To_FaronWoods_GreatTreeStatue => Area::Sky_Field,
            Exit::Sky_To_FaronWoods_InTheWoodsStatue => Area::Sky_Field,
            Exit::Sky_To_FaronWoods_ViewingPlatformStatue => Area::Sky_Field,
            Exit::Sky_To_FloriaWaterfall_FloriaWaterfallStatue => Area::Sky_Field,
            Exit::Sky_To_InsideBambooIsland => Area::Sky_Field,
            Exit::Sky_To_InsideThunderhead => Area::Sky_Field,
            Exit::Sky_To_LakeFloria_LakeFloriaStatue => Area::Sky_Field,
            Exit::Sky_To_LanayruDesert_DesertEntranceStatue => Area::Sky_Field,
            Exit::Sky_To_LanayruDesert_NorthDesertStatue => Area::Sky_Field,
            Exit::Sky_To_LanayruDesert_StoneCacheStatue => Area::Sky_Field,
            Exit::Sky_To_LanayruDesert_WestDesertStatue => Area::Sky_Field,
            Exit::Sky_To_LanayruMines_LanayruMineEntryStatue => Area::Sky_Field,
            Exit::Sky_To_OutsideFireSanctuary_InsideTheVolcanoStatue => Area::Sky_Field,
            Exit::Sky_To_SealedGroundsSpiral_SealedGroundsStatue => Area::Sky_Field,
            Exit::Sky_To_Skyloft => Area::Sky_Field,
            Exit::Sky_To_LumpyPumpkin_North => Area::Sky_OutsideLumpyPumpkin,
            Exit::Sky_To_LumpyPumpkin_South => Area::Sky_OutsideLumpyPumpkin,
            Exit::SkyKeepEntry_To_Skyloft => Area::SkyKeepEntry_Main,
            Exit::SkyloftSilentRealm_To_Skyloft => Area::SkyloftSilentRealm_Trial,
            Exit::BertiesHouse_To_Skyloft => Area::BertiesHouse_Main,
            Exit::GondosHouse_To_Skyloft => Area::GondosHouse_Main,
            Exit::MallarasHouse_To_Skyloft => Area::MallarasHouse_Main,
            Exit::RupinsHouse_To_Skyloft => Area::RupinsHouse_Main,
            Exit::Skyloft_To_BatreauxHouse => Area::Skyloft_OutsideSkyloftVillage,
            Exit::Skyloft_To_BertiesHouse => Area::Skyloft_OutsideSkyloftVillage,
            Exit::Skyloft_To_GondosHouse => Area::Skyloft_OutsideSkyloftVillage,
            Exit::Skyloft_To_MallarasHouse => Area::Skyloft_OutsideSkyloftVillage,
            Exit::Skyloft_To_RupinsHouse => Area::Skyloft_OutsideSkyloftVillage,
            Exit::Skyloft_To_SparrotsHouse => Area::Skyloft_OutsideSkyloftVillage,
            Exit::SparrotsHouse_To_Skyloft => Area::SparrotsHouse_Main,
            Exit::SkyviewBoss_To_SkyviewSpring => Area::SkyviewBoss_Main,
            Exit::SkyviewBoss_To_SkyviewTemple => Area::SkyviewBoss_Main,
            Exit::SkyviewSpring_To_DeepWoods => Area::SkyviewSpring_Main,
            Exit::SkyviewSpring_To_SkyviewBoss => Area::SkyviewSpring_Main,
            Exit::SkyviewTemple_To_SkyviewBoss => Area::SkyviewTemple_BossDoorArea,
            Exit::SkyviewTemple_To_DeepWoods => Area::SkyviewTemple_Entry,
            Exit::InsideThunderhead_To_IsleOfSongs => Area::InsideThunderhead_Main,
            Exit::InsideThunderhead_To_Sky => Area::InsideThunderhead_Main,
            Exit::IsleOfSongs_To_InsideThunderhead => Area::IsleOfSongs_Main,
            Exit::InsideVolcanoSummit_To_EldinVolcano => Area::InsideVolcanoSummit_Main,
            Exit::InsideVolcanoSummit_To_OutsideFireSanctuary => Area::InsideVolcanoSummit_Main,
            Exit::InsideVolcanoSummit_To_VolcanoSummitWaterfall => Area::InsideVolcanoSummit_Main,
            Exit::OutsideFireSanctuary_To_FireSanctuaryA => {
                Area::OutsideFireSanctuary_ToFireSanctuary
            }
            Exit::OutsideFireSanctuary_To_Sky_InsideTheVolcanoStatue => {
                Area::OutsideFireSanctuary_ToFireSanctuary
            }
            Exit::OutsideFireSanctuary_To_InsideVolcanoSummit => {
                Area::OutsideFireSanctuary_ToInsideSummit
            }
            Exit::VolcanoSummitWaterfall_To_InsideVolcanoSummit => {
                Area::VolcanoSummitWaterfall_Main
            }
        }
    }
    pub fn vanilla_entrance(&self) -> Entrance {
        match self {
            Exit::AncientCistern_To_AncientCisternBoss => {
                Entrance::AncientCisternBoss_From_AncientCistern
            }
            Exit::AncientCistern_To_FloriaWaterfall => {
                Entrance::FloriaWaterfall_From_AncientCistern
            }
            Exit::AncientCisternBoss_To_AncientCisternCandleRoom => {
                Entrance::AncientCisternCandleRoom_From_AncientCisternBoss
            }
            Exit::BatreauxHouse_To_Skyloft => Entrance::Skyloft_From_BatreauxHouse,
            Exit::BeedlesShop_To_Sky_Night => Entrance::Sky_From_BeedlesShop_Night,
            Exit::BeedlesShop_To_Skyloft_Day => Entrance::Skyloft_From_BeedlesShop_Day,
            Exit::Bazaar_To_Skyloft_North => Entrance::Skyloft_From_Bazaar_North,
            Exit::Bazaar_To_Skyloft_South => Entrance::Skyloft_From_Bazaar_South,
            Exit::Bazaar_To_Skyloft_West => Entrance::Skyloft_From_Bazaar_West,
            Exit::ParrowAndOriellesHouse_To_Skyloft => {
                Entrance::Skyloft_From_ParrowAndOriellesHouse
            }
            Exit::PeatricesHouse_To_Skyloft => Entrance::Skyloft_From_PeatricesHouse,
            Exit::PipersHouse_To_Skyloft => Entrance::Skyloft_From_PipersHouse,
            Exit::Skyloft_To_Bazaar_North => Entrance::Bazaar_From_Skyloft_North,
            Exit::Skyloft_To_Bazaar_South => Entrance::Bazaar_From_Skyloft_South,
            Exit::Skyloft_To_Bazaar_West => Entrance::Bazaar_From_Skyloft_West,
            Exit::Skyloft_To_BeedlesShop_Day => Entrance::BeedlesShop_From_Skyloft_Day,
            Exit::Skyloft_To_ParrowAndOriellesHouse => {
                Entrance::ParrowAndOriellesHouse_From_Skyloft
            }
            Exit::Skyloft_To_PeatricesHouse => Entrance::PeatricesHouse_From_Skyloft,
            Exit::Skyloft_To_PipersHouse => Entrance::PipersHouse_From_Skyloft,
            Exit::Skyloft_To_Sky => Entrance::Sky_From_Skyloft,
            Exit::Skyloft_To_SkyloftSilentRealm => Entrance::SkyloftSilentRealm_From_Skyloft,
            Exit::Skyloft_To_WaterfallCave_Upper => Entrance::WaterfallCave_From_Skyloft_Upper,
            Exit::Skyloft_To_WrynasHouse => Entrance::WrynasHouse_From_Skyloft,
            Exit::Skyloft_To_Sky_PastWaterfallCave => Entrance::Sky_From_Skyloft_PastWaterfallCave,
            Exit::Skyloft_To_WaterfallCave_Lower => Entrance::WaterfallCave_From_Skyloft_Lower,
            Exit::Skyloft_To_SkyKeepEntry => Entrance::SkyKeepEntry_From_Skyloft,
            Exit::WaterfallCave_To_Skyloft_Upper => Entrance::Skyloft_From_WaterfallCave_Upper,
            Exit::WaterfallCave_To_Skyloft_Lower => Entrance::Skyloft_From_WaterfallCave_Lower,
            Exit::WrynasHouse_To_Skyloft => Entrance::Skyloft_From_WrynasHouse,
            Exit::EarthTemple_To_EarthTempleBoss => Entrance::EarthTempleBoss_From_EarthTemple,
            Exit::EarthTemple_To_EldinVolcano => Entrance::EldinVolcano_From_EarthTemple,
            Exit::EarthTempleBoss_To_EarthTempleSpring => {
                Entrance::EarthTempleSpring_From_EarthTempleBoss
            }
            Exit::EarthTempleSpring_To_EldinVolcano => {
                Entrance::EldinVolcano_From_EarthTempleSpring
            }
            Exit::EldinSilentRealm_To_EldinVolcano => Entrance::EldinVolcano_From_EldinSilentRealm,
            Exit::EldinVolcano_To_Sky_EldinEntranceStatue => {
                Entrance::Sky_From_EldinVolcano_EldinEntranceStatue
            }
            Exit::EldinVolcano_To_InsideVolcanoSummit => {
                Entrance::InsideVolcanoSummit_From_EldinVolcano
            }
            Exit::EldinVolcano_To_ThrillDiggerCave => Entrance::ThrillDiggerCave_From_EldinVolcano,
            Exit::EldinVolcano_To_EarthTemple => Entrance::EarthTemple_From_EldinVolcano,
            Exit::EldinVolcano_To_Sky_TempleEntranceStatue => {
                Entrance::Sky_From_EldinVolcano_TempleEntranceStatue
            }
            Exit::EldinVolcano_To_MogmaTurf_Skydive => {
                Entrance::MogmaTurf_From_EldinVolcano_Skydive
            }
            Exit::EldinVolcano_To_Sky_VolcanoEastStatue => {
                Entrance::Sky_From_EldinVolcano_VolcanoEastStatue
            }
            Exit::EldinVolcano_To_EldinSilentRealm => Entrance::EldinSilentRealm_From_EldinVolcano,
            Exit::EldinVolcano_To_Sky_VolcanoAscentStatue => {
                Entrance::Sky_From_EldinVolcano_VolcanoAscentStatue
            }
            Exit::ThrillDiggerCave_To_EldinVolcano => Entrance::EldinVolcano_From_ThrillDiggerCave,
            Exit::FaronSilentRealm_To_FaronWoods => Entrance::FaronWoods_From_FaronSilentRealm,
            Exit::DeepWoods_To_FaronWoods => Entrance::FaronWoods_From_DeepWoods,
            Exit::DeepWoods_To_Sky_DeepWoodsStatue => Entrance::Sky_From_DeepWoods_DeepWoodsStatue,
            Exit::DeepWoods_To_Sky_ForestTempleStatue => {
                Entrance::Sky_From_DeepWoods_ForestTempleStatue
            }
            Exit::DeepWoods_To_SkyviewTemple => Entrance::SkyviewTemple_From_DeepWoods,
            Exit::FaronWoods_To_BehindTheTemple => Entrance::BehindTheTemple_From_FaronWoods,
            Exit::FaronWoods_To_Sky_FaronWoodsEntryStatue => {
                Entrance::Sky_From_FaronWoods_FaronWoodsEntryStatue
            }
            Exit::FaronWoods_To_GreatTree_LowerPlatform => {
                Entrance::GreatTree_From_FaronWoods_LowerPlatform
            }
            Exit::FaronWoods_To_GreatTree_UpperPlatform => {
                Entrance::GreatTree_From_FaronWoods_UpperPlatform
            }
            Exit::FaronWoods_To_GreatTree_Top => Entrance::GreatTree_From_FaronWoods_Top,
            Exit::FaronWoods_To_Sky_GreatTreeStatue => {
                Entrance::Sky_From_FaronWoods_GreatTreeStatue
            }
            Exit::FaronWoods_To_DeepWoods => Entrance::DeepWoods_From_FaronWoods,
            Exit::FaronWoods_To_FaronSilentRealm => Entrance::FaronSilentRealm_From_FaronWoods,
            Exit::FaronWoods_To_GreatTree_Tunnel => Entrance::GreatTree_From_FaronWoods_Tunnel,
            Exit::FaronWoods_To_LakeFloria => Entrance::LakeFloria_From_FaronWoods,
            Exit::FaronWoods_To_Sky_InTheWoodsStatue => {
                Entrance::Sky_From_FaronWoods_InTheWoodsStatue
            }
            Exit::FaronWoods_To_Sky_ViewingPlatformStatue => {
                Entrance::Sky_From_FaronWoods_ViewingPlatformStatue
            }
            Exit::GreatTree_To_FaronWoods_Tunnel => Entrance::FaronWoods_From_GreatTree_Tunnel,
            Exit::GreatTree_To_FaronWoods_LowerPlatform => {
                Entrance::FaronWoods_From_GreatTree_LowerPlatform
            }
            Exit::GreatTree_To_FaronWoods_UpperPlatform => {
                Entrance::FaronWoods_From_GreatTree_UpperPlatform
            }
            Exit::GreatTree_To_FaronWoods_Top => Entrance::FaronWoods_From_GreatTree_Top,
            Exit::FireSanctuaryA_To_OutsideFireSanctuary => {
                Entrance::OutsideFireSanctuary_From_FireSanctuaryA
            }
            Exit::FireSanctuaryA_To_FireSanctuaryBoss => {
                Entrance::FireSanctuaryBoss_From_FireSanctuaryA
            }
            Exit::FireSanctuaryA_To_FireSanctuaryB => Entrance::FireSanctuaryB_From_FireSanctuaryA,
            Exit::FireSanctuaryB_To_FireSanctuaryA => Entrance::FireSanctuaryA_From_FireSanctuaryB,
            Exit::FireSanctuaryBoss_To_FireSanctuaryFlameRoom => {
                Entrance::FireSanctuaryFlameRoom_From_FireSanctuaryBoss
            }
            Exit::InsideGoddessStatue_To_Skyloft => Entrance::Skyloft_From_InsideGoddessStatue,
            Exit::KnightAcademy_To_Skyloft_LowerDoors => {
                Entrance::Skyloft_From_KnightAcademy_LowerDoors
            }
            Exit::KnightAcademy_To_Skyloft_UpperDoors => {
                Entrance::Skyloft_From_KnightAcademy_UpperDoors
            }
            Exit::Skyloft_To_InsideGoddessStatue => Entrance::InsideGoddessStatue_From_Skyloft,
            Exit::Skyloft_To_KnightAcademy_Chimney => Entrance::KnightAcademy_From_Skyloft_Chimney,
            Exit::Skyloft_To_KnightAcademy_LowerDoors => {
                Entrance::KnightAcademy_From_Skyloft_LowerDoors
            }
            Exit::Skyloft_To_KnightAcademy_UpperDoors => {
                Entrance::KnightAcademy_From_Skyloft_UpperDoors
            }
            Exit::Skyloft_To_SparringHall => Entrance::SparringHall_From_Skyloft,
            Exit::SparringHall_To_Skyloft => Entrance::Skyloft_From_SparringHall,
            Exit::FaroresLair_To_FloriaWaterfall => Entrance::FloriaWaterfall_From_FaroresLair,
            Exit::FaroresLair_To_LakeFloria => Entrance::LakeFloria_From_FaroresLair,
            Exit::FloriaWaterfall_To_AncientCistern => {
                Entrance::AncientCistern_From_FloriaWaterfall
            }
            Exit::FloriaWaterfall_To_FaronWoods => Entrance::FaronWoods_From_FloriaWaterfall,
            Exit::FloriaWaterfall_To_FaroresLair => Entrance::FaroresLair_From_FloriaWaterfall,
            Exit::FloriaWaterfall_To_Sky_FloriaWaterfallStatue => {
                Entrance::Sky_From_FloriaWaterfall_FloriaWaterfallStatue
            }
            Exit::LakeFloria_To_Sky_LakeFloriaStatue => {
                Entrance::Sky_From_LakeFloria_LakeFloriaStatue
            }
            Exit::LakeFloria_To_FaroresLair => Entrance::FaroresLair_From_LakeFloria,
            Exit::LanayruCaves_To_LanayruDesert => Entrance::LanayruDesert_From_LanayruCaves,
            Exit::LanayruCaves_To_LanayruMines => Entrance::LanayruMines_From_LanayruCaves,
            Exit::LanayruCaves_To_SandSeaDocks => Entrance::SandSeaDocks_From_LanayruCaves,
            Exit::FireNode_To_LanayruDesert => Entrance::LanayruDesert_From_FireNode,
            Exit::LanayruDesert_To_LanayruMines => Entrance::LanayruMines_From_LanayruDesert,
            Exit::LanayruDesert_To_Sky_DesertEntranceStatue => {
                Entrance::Sky_From_LanayruDesert_DesertEntranceStatue
            }
            Exit::LanayruDesert_To_FireNode => Entrance::FireNode_From_LanayruDesert,
            Exit::LanayruDesert_To_LanayruMiningFacilityA => {
                Entrance::LanayruMiningFacilityA_From_LanayruDesert
            }
            Exit::LanayruDesert_To_LanayruSilentRealm => {
                Entrance::LanayruSilentRealm_From_LanayruDesert
            }
            Exit::LanayruDesert_To_LightningNode => Entrance::LightningNode_From_LanayruDesert,
            Exit::LanayruDesert_To_Sky_NorthDesertStatue => {
                Entrance::Sky_From_LanayruDesert_NorthDesertStatue
            }
            Exit::LanayruDesert_To_Sky_StoneCacheStatue => {
                Entrance::Sky_From_LanayruDesert_StoneCacheStatue
            }
            Exit::LanayruDesert_To_TempleOfTime_End => {
                Entrance::TempleOfTime_From_LanayruDesert_End
            }
            Exit::LanayruDesert_To_LanayruCaves => Entrance::LanayruCaves_From_LanayruDesert,
            Exit::LanayruDesert_To_Sky_WestDesertStatue => {
                Entrance::Sky_From_LanayruDesert_WestDesertStatue
            }
            Exit::LanayruDesert_To_TempleOfTime_Start => {
                Entrance::TempleOfTime_From_LanayruDesert_Start
            }
            Exit::LightningNode_To_LanayruDesert => Entrance::LanayruDesert_From_LightningNode,
            Exit::TempleOfTime_To_LanayruDesert_End => {
                Entrance::LanayruDesert_From_TempleOfTime_End
            }
            Exit::TempleOfTime_To_LanayruDesert_Start => {
                Entrance::LanayruDesert_From_TempleOfTime_Start
            }
            Exit::LanayruMines_To_Sky_LanayruMineEntryStatue => {
                Entrance::Sky_From_LanayruMines_LanayruMineEntryStatue
            }
            Exit::LanayruMines_To_LanayruCaves => Entrance::LanayruCaves_From_LanayruMines,
            Exit::LanayruMines_To_LanayruDesert => Entrance::LanayruDesert_From_LanayruMines,
            Exit::LanayruMiningFacilityA_To_LanayruDesert => {
                Entrance::LanayruDesert_From_LanayruMiningFacilityA
            }
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub2 => {
                Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub2
            }
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_HubW => {
                Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_HubW
            }
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub => {
                Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub
            }
            Exit::LanayruMiningFacilityB_To_LanayruMiningFacilityBoss => {
                Entrance::LanayruMiningFacilityBoss_From_LanayruMiningFacilityB
            }
            Exit::LanayruMiningFacilityBoss_To_LanayruMiningFacilityToToT => {
                Entrance::LanayruMiningFacilityToToT_From_LanayruMiningFacilityBoss
            }
            Exit::LanayruMiningFacilityToToT_To_TempleOfTime => {
                Entrance::TempleOfTime_From_LanayruMiningFacilityToToT
            }
            Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_End => {
                Entrance::OutsidePiratesStronghold_From_InsidePiratesStronghold_End
            }
            Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_Beginning => {
                Entrance::OutsidePiratesStronghold_From_InsidePiratesStronghold_Beginning
            }
            Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_End => {
                Entrance::InsidePiratesStronghold_From_OutsidePiratesStronghold_End
            }
            Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_Beginning => {
                Entrance::InsidePiratesStronghold_From_OutsidePiratesStronghold_Beginning
            }
            Exit::OutsidePiratesStronghold_To_SandSea => {
                Entrance::SandSea_From_OutsidePiratesStronghold
            }
            Exit::SandSea_To_OutsidePiratesStronghold => {
                Entrance::OutsidePiratesStronghold_From_SandSea
            }
            Exit::SandSea_To_SandSeaDocks => Entrance::SandSeaDocks_From_SandSea,
            Exit::SandSea_To_Sandship => Entrance::Sandship_From_SandSea,
            Exit::SandSea_To_Shipyard => Entrance::Shipyard_From_SandSea,
            Exit::SandSea_To_SkippersRetreat => Entrance::SkippersRetreat_From_SandSea,
            Exit::SandSeaDocks_To_SandSea => Entrance::SandSea_From_SandSeaDocks,
            Exit::SandSeaDocks_To_Sky_AncientHarbor => {
                Entrance::Sky_From_SandSeaDocks_AncientHarbor
            }
            Exit::SandSeaDocks_To_LanayruCaves => Entrance::LanayruCaves_From_SandSeaDocks,
            Exit::Shipyard_To_ShipyardConstructionBay_Upper => {
                Entrance::ShipyardConstructionBay_From_Shipyard_Upper
            }
            Exit::Shipyard_To_SandSea => Entrance::SandSea_From_Shipyard,
            Exit::Shipyard_To_ShipyardConstructionBay_Lower => {
                Entrance::ShipyardConstructionBay_From_Shipyard_Lower
            }
            Exit::ShipyardConstructionBay_To_Shipyard_Lower => {
                Entrance::Shipyard_From_ShipyardConstructionBay_Lower
            }
            Exit::ShipyardConstructionBay_To_Shipyard_Upper => {
                Entrance::Shipyard_From_ShipyardConstructionBay_Upper
            }
            Exit::SkippersRetreat_To_SkippersShack => Entrance::SkippersShack_From_SkippersRetreat,
            Exit::SkippersRetreat_To_SandSea => Entrance::SandSea_From_SkippersRetreat,
            Exit::SkippersShack_To_SkippersRetreat => Entrance::SkippersRetreat_From_SkippersShack,
            Exit::LanayruSilentRealm_To_LanayruDesert => {
                Entrance::LanayruDesert_From_LanayruSilentRealm
            }
            Exit::MogmaTurf_To_EldinVolcano_EndVent => {
                Entrance::EldinVolcano_From_MogmaTurf_EndVent
            }
            Exit::MogmaTurf_To_EldinVolcano_StartVent => {
                Entrance::EldinVolcano_From_MogmaTurf_StartVent
            }
            Exit::Sandship_To_SandSea => Entrance::SandSea_From_Sandship,
            Exit::Sandship_To_SandshipBoss => Entrance::SandshipBoss_From_Sandship,
            Exit::BehindTheTemple_To_FaronWoods => Entrance::FaronWoods_From_BehindTheTemple,
            Exit::BehindTheTemple_To_SealedGroundsSpiral => {
                Entrance::SealedGroundsSpiral_From_BehindTheTemple
            }
            Exit::BehindTheTemple_To_SealedTemple => Entrance::SealedTemple_From_BehindTheTemple,
            Exit::BehindTheTemple_To_Sky_BehindTheTempleStatue => {
                Entrance::Sky_From_BehindTheTemple_BehindTheTempleStatue
            }
            Exit::SealedGroundsSpiral_To_SealedTemple => {
                Entrance::SealedTemple_From_SealedGroundsSpiral
            }
            Exit::SealedGroundsSpiral_To_Sky_SealedGroundsStatue => {
                Entrance::Sky_From_SealedGroundsSpiral_SealedGroundsStatue
            }
            Exit::SealedTemple_To_BehindTheTemple => Entrance::BehindTheTemple_From_SealedTemple,
            Exit::SealedTemple_To_HyliasTemple => Entrance::HyliasTemple_From_SealedTemple,
            Exit::SealedTemple_To_SealedGroundsSpiral => {
                Entrance::SealedGroundsSpiral_From_SealedTemple
            }
            Exit::InsideBambooIsland_To_Sky => Entrance::Sky_From_InsideBambooIsland,
            Exit::LumpyPumpkin_To_Sky_North => Entrance::Sky_From_LumpyPumpkin_North,
            Exit::LumpyPumpkin_To_Sky_South => Entrance::Sky_From_LumpyPumpkin_South,
            Exit::Sky_To_BeedlesShop_Night => Entrance::BeedlesShop_From_Sky_Night,
            Exit::Sky_To_BehindTheTemple_BehindTheTempleStatue => {
                Entrance::BehindTheTemple_From_Sky_BehindTheTempleStatue
            }
            Exit::Sky_To_DeepWoods_DeepWoodsStatue => Entrance::DeepWoods_From_Sky_DeepWoodsStatue,
            Exit::Sky_To_DeepWoods_ForestTempleStatue => {
                Entrance::DeepWoods_From_Sky_ForestTempleStatue
            }
            Exit::Sky_To_EldinVolcano_EldinEntranceStatue => {
                Entrance::EldinVolcano_From_Sky_EldinEntranceStatue
            }
            Exit::Sky_To_EldinVolcano_TempleEntranceStatue => {
                Entrance::EldinVolcano_From_Sky_TempleEntranceStatue
            }
            Exit::Sky_To_EldinVolcano_VolcanoEastStatue => {
                Entrance::EldinVolcano_From_Sky_VolcanoEastStatue
            }
            Exit::Sky_To_EldinVolcano_VolcanoAscentStatue => {
                Entrance::EldinVolcano_From_Sky_VolcanoAscentStatue
            }
            Exit::Sky_To_FaronWoods_FaronWoodsEntryStatue => {
                Entrance::FaronWoods_From_Sky_FaronWoodsEntryStatue
            }
            Exit::Sky_To_FaronWoods_GreatTreeStatue => {
                Entrance::FaronWoods_From_Sky_GreatTreeStatue
            }
            Exit::Sky_To_FaronWoods_InTheWoodsStatue => {
                Entrance::FaronWoods_From_Sky_InTheWoodsStatue
            }
            Exit::Sky_To_FaronWoods_ViewingPlatformStatue => {
                Entrance::FaronWoods_From_Sky_ViewingPlatformStatue
            }
            Exit::Sky_To_FloriaWaterfall_FloriaWaterfallStatue => {
                Entrance::FloriaWaterfall_From_Sky_FloriaWaterfallStatue
            }
            Exit::Sky_To_InsideBambooIsland => Entrance::InsideBambooIsland_From_Sky,
            Exit::Sky_To_InsideThunderhead => Entrance::InsideThunderhead_From_Sky,
            Exit::Sky_To_LakeFloria_LakeFloriaStatue => {
                Entrance::LakeFloria_From_Sky_LakeFloriaStatue
            }
            Exit::Sky_To_LanayruDesert_DesertEntranceStatue => {
                Entrance::LanayruDesert_From_Sky_DesertEntranceStatue
            }
            Exit::Sky_To_LanayruDesert_NorthDesertStatue => {
                Entrance::LanayruDesert_From_Sky_NorthDesertStatue
            }
            Exit::Sky_To_LanayruDesert_StoneCacheStatue => {
                Entrance::LanayruDesert_From_Sky_StoneCacheStatue
            }
            Exit::Sky_To_LanayruDesert_WestDesertStatue => {
                Entrance::LanayruDesert_From_Sky_WestDesertStatue
            }
            Exit::Sky_To_LanayruMines_LanayruMineEntryStatue => {
                Entrance::LanayruMines_From_Sky_LanayruMineEntryStatue
            }
            Exit::Sky_To_OutsideFireSanctuary_InsideTheVolcanoStatue => {
                Entrance::OutsideFireSanctuary_From_Sky_InsideTheVolcanoStatue
            }
            Exit::Sky_To_SealedGroundsSpiral_SealedGroundsStatue => {
                Entrance::SealedGroundsSpiral_From_Sky_SealedGroundsStatue
            }
            Exit::Sky_To_Skyloft => Entrance::Skyloft_From_Sky,
            Exit::Sky_To_LumpyPumpkin_North => Entrance::LumpyPumpkin_From_Sky_North,
            Exit::Sky_To_LumpyPumpkin_South => Entrance::LumpyPumpkin_From_Sky_South,
            Exit::SkyKeepEntry_To_Skyloft => Entrance::Skyloft_From_SkyKeepEntry,
            Exit::SkyloftSilentRealm_To_Skyloft => Entrance::Skyloft_From_SkyloftSilentRealm,
            Exit::BertiesHouse_To_Skyloft => Entrance::Skyloft_From_BertiesHouse,
            Exit::GondosHouse_To_Skyloft => Entrance::Skyloft_From_GondosHouse,
            Exit::MallarasHouse_To_Skyloft => Entrance::Skyloft_From_MallarasHouse,
            Exit::RupinsHouse_To_Skyloft => Entrance::Skyloft_From_RupinsHouse,
            Exit::Skyloft_To_BatreauxHouse => Entrance::BatreauxHouse_From_Skyloft,
            Exit::Skyloft_To_BertiesHouse => Entrance::BertiesHouse_From_Skyloft,
            Exit::Skyloft_To_GondosHouse => Entrance::GondosHouse_From_Skyloft,
            Exit::Skyloft_To_MallarasHouse => Entrance::MallarasHouse_From_Skyloft,
            Exit::Skyloft_To_RupinsHouse => Entrance::RupinsHouse_From_Skyloft,
            Exit::Skyloft_To_SparrotsHouse => Entrance::SparrotsHouse_From_Skyloft,
            Exit::SparrotsHouse_To_Skyloft => Entrance::Skyloft_From_SparrotsHouse,
            Exit::SkyviewBoss_To_SkyviewSpring => Entrance::SkyviewSpring_From_SkyviewBoss,
            Exit::SkyviewBoss_To_SkyviewTemple => Entrance::SkyviewTemple_From_SkyviewBoss,
            Exit::SkyviewSpring_To_DeepWoods => Entrance::DeepWoods_From_SkyviewSpring,
            Exit::SkyviewSpring_To_SkyviewBoss => Entrance::SkyviewBoss_From_SkyviewSpring,
            Exit::SkyviewTemple_To_SkyviewBoss => Entrance::SkyviewBoss_From_SkyviewTemple,
            Exit::SkyviewTemple_To_DeepWoods => Entrance::DeepWoods_From_SkyviewTemple,
            Exit::InsideThunderhead_To_IsleOfSongs => Entrance::IsleOfSongs_From_InsideThunderhead,
            Exit::InsideThunderhead_To_Sky => Entrance::Sky_From_InsideThunderhead,
            Exit::IsleOfSongs_To_InsideThunderhead => Entrance::InsideThunderhead_From_IsleOfSongs,
            Exit::InsideVolcanoSummit_To_EldinVolcano => {
                Entrance::EldinVolcano_From_InsideVolcanoSummit
            }
            Exit::InsideVolcanoSummit_To_OutsideFireSanctuary => {
                Entrance::OutsideFireSanctuary_From_InsideVolcanoSummit
            }
            Exit::InsideVolcanoSummit_To_VolcanoSummitWaterfall => {
                Entrance::VolcanoSummitWaterfall_From_InsideVolcanoSummit
            }
            Exit::OutsideFireSanctuary_To_FireSanctuaryA => {
                Entrance::FireSanctuaryA_From_OutsideFireSanctuary
            }
            Exit::OutsideFireSanctuary_To_Sky_InsideTheVolcanoStatue => {
                Entrance::Sky_From_OutsideFireSanctuary_InsideTheVolcanoStatue
            }
            Exit::OutsideFireSanctuary_To_InsideVolcanoSummit => {
                Entrance::InsideVolcanoSummit_From_OutsideFireSanctuary
            }
            Exit::VolcanoSummitWaterfall_To_InsideVolcanoSummit => {
                Entrance::InsideVolcanoSummit_From_VolcanoSummitWaterfall
            }
        }
    }
    pub fn requirement_key(&self) -> RequirementKey {
        match self {
            Exit::AncientCistern_To_AncientCisternBoss => {
                RequirementKey::AncientCistern_To_AncientCisternBoss
            }
            Exit::AncientCistern_To_FloriaWaterfall => {
                RequirementKey::AncientCistern_To_FloriaWaterfall
            }
            Exit::AncientCisternBoss_To_AncientCisternCandleRoom => {
                RequirementKey::AncientCisternBoss_To_AncientCisternCandleRoom
            }
            Exit::BatreauxHouse_To_Skyloft => RequirementKey::BatreauxHouse_To_Skyloft,
            Exit::BeedlesShop_To_Sky_Night => RequirementKey::BeedlesShop_To_Sky_Night,
            Exit::BeedlesShop_To_Skyloft_Day => RequirementKey::BeedlesShop_To_Skyloft_Day,
            Exit::Bazaar_To_Skyloft_North => RequirementKey::Bazaar_To_Skyloft_North,
            Exit::Bazaar_To_Skyloft_South => RequirementKey::Bazaar_To_Skyloft_South,
            Exit::Bazaar_To_Skyloft_West => RequirementKey::Bazaar_To_Skyloft_West,
            Exit::ParrowAndOriellesHouse_To_Skyloft => {
                RequirementKey::ParrowAndOriellesHouse_To_Skyloft
            }
            Exit::PeatricesHouse_To_Skyloft => RequirementKey::PeatricesHouse_To_Skyloft,
            Exit::PipersHouse_To_Skyloft => RequirementKey::PipersHouse_To_Skyloft,
            Exit::Skyloft_To_Bazaar_North => RequirementKey::Skyloft_To_Bazaar_North,
            Exit::Skyloft_To_Bazaar_South => RequirementKey::Skyloft_To_Bazaar_South,
            Exit::Skyloft_To_Bazaar_West => RequirementKey::Skyloft_To_Bazaar_West,
            Exit::Skyloft_To_BeedlesShop_Day => RequirementKey::Skyloft_To_BeedlesShop_Day,
            Exit::Skyloft_To_ParrowAndOriellesHouse => {
                RequirementKey::Skyloft_To_ParrowAndOriellesHouse
            }
            Exit::Skyloft_To_PeatricesHouse => RequirementKey::Skyloft_To_PeatricesHouse,
            Exit::Skyloft_To_PipersHouse => RequirementKey::Skyloft_To_PipersHouse,
            Exit::Skyloft_To_Sky => RequirementKey::Skyloft_To_Sky,
            Exit::Skyloft_To_SkyloftSilentRealm => RequirementKey::Skyloft_To_SkyloftSilentRealm,
            Exit::Skyloft_To_WaterfallCave_Upper => RequirementKey::Skyloft_To_WaterfallCave_Upper,
            Exit::Skyloft_To_WrynasHouse => RequirementKey::Skyloft_To_WrynasHouse,
            Exit::Skyloft_To_Sky_PastWaterfallCave => {
                RequirementKey::Skyloft_To_Sky_PastWaterfallCave
            }
            Exit::Skyloft_To_WaterfallCave_Lower => RequirementKey::Skyloft_To_WaterfallCave_Lower,
            Exit::Skyloft_To_SkyKeepEntry => RequirementKey::Skyloft_To_SkyKeepEntry,
            Exit::WaterfallCave_To_Skyloft_Upper => RequirementKey::WaterfallCave_To_Skyloft_Upper,
            Exit::WaterfallCave_To_Skyloft_Lower => RequirementKey::WaterfallCave_To_Skyloft_Lower,
            Exit::WrynasHouse_To_Skyloft => RequirementKey::WrynasHouse_To_Skyloft,
            Exit::EarthTemple_To_EarthTempleBoss => RequirementKey::EarthTemple_To_EarthTempleBoss,
            Exit::EarthTemple_To_EldinVolcano => RequirementKey::EarthTemple_To_EldinVolcano,
            Exit::EarthTempleBoss_To_EarthTempleSpring => {
                RequirementKey::EarthTempleBoss_To_EarthTempleSpring
            }
            Exit::EarthTempleSpring_To_EldinVolcano => {
                RequirementKey::EarthTempleSpring_To_EldinVolcano
            }
            Exit::EldinSilentRealm_To_EldinVolcano => {
                RequirementKey::EldinSilentRealm_To_EldinVolcano
            }
            Exit::EldinVolcano_To_Sky_EldinEntranceStatue => {
                RequirementKey::EldinVolcano_To_Sky_EldinEntranceStatue
            }
            Exit::EldinVolcano_To_InsideVolcanoSummit => {
                RequirementKey::EldinVolcano_To_InsideVolcanoSummit
            }
            Exit::EldinVolcano_To_ThrillDiggerCave => {
                RequirementKey::EldinVolcano_To_ThrillDiggerCave
            }
            Exit::EldinVolcano_To_EarthTemple => RequirementKey::EldinVolcano_To_EarthTemple,
            Exit::EldinVolcano_To_Sky_TempleEntranceStatue => {
                RequirementKey::EldinVolcano_To_Sky_TempleEntranceStatue
            }
            Exit::EldinVolcano_To_MogmaTurf_Skydive => {
                RequirementKey::EldinVolcano_To_MogmaTurf_Skydive
            }
            Exit::EldinVolcano_To_Sky_VolcanoEastStatue => {
                RequirementKey::EldinVolcano_To_Sky_VolcanoEastStatue
            }
            Exit::EldinVolcano_To_EldinSilentRealm => {
                RequirementKey::EldinVolcano_To_EldinSilentRealm
            }
            Exit::EldinVolcano_To_Sky_VolcanoAscentStatue => {
                RequirementKey::EldinVolcano_To_Sky_VolcanoAscentStatue
            }
            Exit::ThrillDiggerCave_To_EldinVolcano => {
                RequirementKey::ThrillDiggerCave_To_EldinVolcano
            }
            Exit::FaronSilentRealm_To_FaronWoods => RequirementKey::FaronSilentRealm_To_FaronWoods,
            Exit::DeepWoods_To_FaronWoods => RequirementKey::DeepWoods_To_FaronWoods,
            Exit::DeepWoods_To_Sky_DeepWoodsStatue => {
                RequirementKey::DeepWoods_To_Sky_DeepWoodsStatue
            }
            Exit::DeepWoods_To_Sky_ForestTempleStatue => {
                RequirementKey::DeepWoods_To_Sky_ForestTempleStatue
            }
            Exit::DeepWoods_To_SkyviewTemple => RequirementKey::DeepWoods_To_SkyviewTemple,
            Exit::FaronWoods_To_BehindTheTemple => RequirementKey::FaronWoods_To_BehindTheTemple,
            Exit::FaronWoods_To_Sky_FaronWoodsEntryStatue => {
                RequirementKey::FaronWoods_To_Sky_FaronWoodsEntryStatue
            }
            Exit::FaronWoods_To_GreatTree_LowerPlatform => {
                RequirementKey::FaronWoods_To_GreatTree_LowerPlatform
            }
            Exit::FaronWoods_To_GreatTree_UpperPlatform => {
                RequirementKey::FaronWoods_To_GreatTree_UpperPlatform
            }
            Exit::FaronWoods_To_GreatTree_Top => RequirementKey::FaronWoods_To_GreatTree_Top,
            Exit::FaronWoods_To_Sky_GreatTreeStatue => {
                RequirementKey::FaronWoods_To_Sky_GreatTreeStatue
            }
            Exit::FaronWoods_To_DeepWoods => RequirementKey::FaronWoods_To_DeepWoods,
            Exit::FaronWoods_To_FaronSilentRealm => RequirementKey::FaronWoods_To_FaronSilentRealm,
            Exit::FaronWoods_To_GreatTree_Tunnel => RequirementKey::FaronWoods_To_GreatTree_Tunnel,
            Exit::FaronWoods_To_LakeFloria => RequirementKey::FaronWoods_To_LakeFloria,
            Exit::FaronWoods_To_Sky_InTheWoodsStatue => {
                RequirementKey::FaronWoods_To_Sky_InTheWoodsStatue
            }
            Exit::FaronWoods_To_Sky_ViewingPlatformStatue => {
                RequirementKey::FaronWoods_To_Sky_ViewingPlatformStatue
            }
            Exit::GreatTree_To_FaronWoods_Tunnel => RequirementKey::GreatTree_To_FaronWoods_Tunnel,
            Exit::GreatTree_To_FaronWoods_LowerPlatform => {
                RequirementKey::GreatTree_To_FaronWoods_LowerPlatform
            }
            Exit::GreatTree_To_FaronWoods_UpperPlatform => {
                RequirementKey::GreatTree_To_FaronWoods_UpperPlatform
            }
            Exit::GreatTree_To_FaronWoods_Top => RequirementKey::GreatTree_To_FaronWoods_Top,
            Exit::FireSanctuaryA_To_OutsideFireSanctuary => {
                RequirementKey::FireSanctuaryA_To_OutsideFireSanctuary
            }
            Exit::FireSanctuaryA_To_FireSanctuaryBoss => {
                RequirementKey::FireSanctuaryA_To_FireSanctuaryBoss
            }
            Exit::FireSanctuaryA_To_FireSanctuaryB => {
                RequirementKey::FireSanctuaryA_To_FireSanctuaryB
            }
            Exit::FireSanctuaryB_To_FireSanctuaryA => {
                RequirementKey::FireSanctuaryB_To_FireSanctuaryA
            }
            Exit::FireSanctuaryBoss_To_FireSanctuaryFlameRoom => {
                RequirementKey::FireSanctuaryBoss_To_FireSanctuaryFlameRoom
            }
            Exit::InsideGoddessStatue_To_Skyloft => RequirementKey::InsideGoddessStatue_To_Skyloft,
            Exit::KnightAcademy_To_Skyloft_LowerDoors => {
                RequirementKey::KnightAcademy_To_Skyloft_LowerDoors
            }
            Exit::KnightAcademy_To_Skyloft_UpperDoors => {
                RequirementKey::KnightAcademy_To_Skyloft_UpperDoors
            }
            Exit::Skyloft_To_InsideGoddessStatue => RequirementKey::Skyloft_To_InsideGoddessStatue,
            Exit::Skyloft_To_KnightAcademy_Chimney => {
                RequirementKey::Skyloft_To_KnightAcademy_Chimney
            }
            Exit::Skyloft_To_KnightAcademy_LowerDoors => {
                RequirementKey::Skyloft_To_KnightAcademy_LowerDoors
            }
            Exit::Skyloft_To_KnightAcademy_UpperDoors => {
                RequirementKey::Skyloft_To_KnightAcademy_UpperDoors
            }
            Exit::Skyloft_To_SparringHall => RequirementKey::Skyloft_To_SparringHall,
            Exit::SparringHall_To_Skyloft => RequirementKey::SparringHall_To_Skyloft,
            Exit::FaroresLair_To_FloriaWaterfall => RequirementKey::FaroresLair_To_FloriaWaterfall,
            Exit::FaroresLair_To_LakeFloria => RequirementKey::FaroresLair_To_LakeFloria,
            Exit::FloriaWaterfall_To_AncientCistern => {
                RequirementKey::FloriaWaterfall_To_AncientCistern
            }
            Exit::FloriaWaterfall_To_FaronWoods => RequirementKey::FloriaWaterfall_To_FaronWoods,
            Exit::FloriaWaterfall_To_FaroresLair => RequirementKey::FloriaWaterfall_To_FaroresLair,
            Exit::FloriaWaterfall_To_Sky_FloriaWaterfallStatue => {
                RequirementKey::FloriaWaterfall_To_Sky_FloriaWaterfallStatue
            }
            Exit::LakeFloria_To_Sky_LakeFloriaStatue => {
                RequirementKey::LakeFloria_To_Sky_LakeFloriaStatue
            }
            Exit::LakeFloria_To_FaroresLair => RequirementKey::LakeFloria_To_FaroresLair,
            Exit::LanayruCaves_To_LanayruDesert => RequirementKey::LanayruCaves_To_LanayruDesert,
            Exit::LanayruCaves_To_LanayruMines => RequirementKey::LanayruCaves_To_LanayruMines,
            Exit::LanayruCaves_To_SandSeaDocks => RequirementKey::LanayruCaves_To_SandSeaDocks,
            Exit::FireNode_To_LanayruDesert => RequirementKey::FireNode_To_LanayruDesert,
            Exit::LanayruDesert_To_LanayruMines => RequirementKey::LanayruDesert_To_LanayruMines,
            Exit::LanayruDesert_To_Sky_DesertEntranceStatue => {
                RequirementKey::LanayruDesert_To_Sky_DesertEntranceStatue
            }
            Exit::LanayruDesert_To_FireNode => RequirementKey::LanayruDesert_To_FireNode,
            Exit::LanayruDesert_To_LanayruMiningFacilityA => {
                RequirementKey::LanayruDesert_To_LanayruMiningFacilityA
            }
            Exit::LanayruDesert_To_LanayruSilentRealm => {
                RequirementKey::LanayruDesert_To_LanayruSilentRealm
            }
            Exit::LanayruDesert_To_LightningNode => RequirementKey::LanayruDesert_To_LightningNode,
            Exit::LanayruDesert_To_Sky_NorthDesertStatue => {
                RequirementKey::LanayruDesert_To_Sky_NorthDesertStatue
            }
            Exit::LanayruDesert_To_Sky_StoneCacheStatue => {
                RequirementKey::LanayruDesert_To_Sky_StoneCacheStatue
            }
            Exit::LanayruDesert_To_TempleOfTime_End => {
                RequirementKey::LanayruDesert_To_TempleOfTime_End
            }
            Exit::LanayruDesert_To_LanayruCaves => RequirementKey::LanayruDesert_To_LanayruCaves,
            Exit::LanayruDesert_To_Sky_WestDesertStatue => {
                RequirementKey::LanayruDesert_To_Sky_WestDesertStatue
            }
            Exit::LanayruDesert_To_TempleOfTime_Start => {
                RequirementKey::LanayruDesert_To_TempleOfTime_Start
            }
            Exit::LightningNode_To_LanayruDesert => RequirementKey::LightningNode_To_LanayruDesert,
            Exit::TempleOfTime_To_LanayruDesert_End => {
                RequirementKey::TempleOfTime_To_LanayruDesert_End
            }
            Exit::TempleOfTime_To_LanayruDesert_Start => {
                RequirementKey::TempleOfTime_To_LanayruDesert_Start
            }
            Exit::LanayruMines_To_Sky_LanayruMineEntryStatue => {
                RequirementKey::LanayruMines_To_Sky_LanayruMineEntryStatue
            }
            Exit::LanayruMines_To_LanayruCaves => RequirementKey::LanayruMines_To_LanayruCaves,
            Exit::LanayruMines_To_LanayruDesert => RequirementKey::LanayruMines_To_LanayruDesert,
            Exit::LanayruMiningFacilityA_To_LanayruDesert => {
                RequirementKey::LanayruMiningFacilityA_To_LanayruDesert
            }
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub2 => {
                RequirementKey::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub2
            }
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_HubW => {
                RequirementKey::LanayruMiningFacilityA_To_LanayruMiningFacilityB_HubW
            }
            Exit::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub => {
                RequirementKey::LanayruMiningFacilityA_To_LanayruMiningFacilityB_Hub
            }
            Exit::LanayruMiningFacilityB_To_LanayruMiningFacilityBoss => {
                RequirementKey::LanayruMiningFacilityB_To_LanayruMiningFacilityBoss
            }
            Exit::LanayruMiningFacilityBoss_To_LanayruMiningFacilityToToT => {
                RequirementKey::LanayruMiningFacilityBoss_To_LanayruMiningFacilityToToT
            }
            Exit::LanayruMiningFacilityToToT_To_TempleOfTime => {
                RequirementKey::LanayruMiningFacilityToToT_To_TempleOfTime
            }
            Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_End => {
                RequirementKey::InsidePiratesStronghold_To_OutsidePiratesStronghold_End
            }
            Exit::InsidePiratesStronghold_To_OutsidePiratesStronghold_Beginning => {
                RequirementKey::InsidePiratesStronghold_To_OutsidePiratesStronghold_Beginning
            }
            Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_End => {
                RequirementKey::OutsidePiratesStronghold_To_InsidePiratesStronghold_End
            }
            Exit::OutsidePiratesStronghold_To_InsidePiratesStronghold_Beginning => {
                RequirementKey::OutsidePiratesStronghold_To_InsidePiratesStronghold_Beginning
            }
            Exit::OutsidePiratesStronghold_To_SandSea => {
                RequirementKey::OutsidePiratesStronghold_To_SandSea
            }
            Exit::SandSea_To_OutsidePiratesStronghold => {
                RequirementKey::SandSea_To_OutsidePiratesStronghold
            }
            Exit::SandSea_To_SandSeaDocks => RequirementKey::SandSea_To_SandSeaDocks,
            Exit::SandSea_To_Sandship => RequirementKey::SandSea_To_Sandship,
            Exit::SandSea_To_Shipyard => RequirementKey::SandSea_To_Shipyard,
            Exit::SandSea_To_SkippersRetreat => RequirementKey::SandSea_To_SkippersRetreat,
            Exit::SandSeaDocks_To_SandSea => RequirementKey::SandSeaDocks_To_SandSea,
            Exit::SandSeaDocks_To_Sky_AncientHarbor => {
                RequirementKey::SandSeaDocks_To_Sky_AncientHarbor
            }
            Exit::SandSeaDocks_To_LanayruCaves => RequirementKey::SandSeaDocks_To_LanayruCaves,
            Exit::Shipyard_To_ShipyardConstructionBay_Upper => {
                RequirementKey::Shipyard_To_ShipyardConstructionBay_Upper
            }
            Exit::Shipyard_To_SandSea => RequirementKey::Shipyard_To_SandSea,
            Exit::Shipyard_To_ShipyardConstructionBay_Lower => {
                RequirementKey::Shipyard_To_ShipyardConstructionBay_Lower
            }
            Exit::ShipyardConstructionBay_To_Shipyard_Lower => {
                RequirementKey::ShipyardConstructionBay_To_Shipyard_Lower
            }
            Exit::ShipyardConstructionBay_To_Shipyard_Upper => {
                RequirementKey::ShipyardConstructionBay_To_Shipyard_Upper
            }
            Exit::SkippersRetreat_To_SkippersShack => {
                RequirementKey::SkippersRetreat_To_SkippersShack
            }
            Exit::SkippersRetreat_To_SandSea => RequirementKey::SkippersRetreat_To_SandSea,
            Exit::SkippersShack_To_SkippersRetreat => {
                RequirementKey::SkippersShack_To_SkippersRetreat
            }
            Exit::LanayruSilentRealm_To_LanayruDesert => {
                RequirementKey::LanayruSilentRealm_To_LanayruDesert
            }
            Exit::MogmaTurf_To_EldinVolcano_EndVent => {
                RequirementKey::MogmaTurf_To_EldinVolcano_EndVent
            }
            Exit::MogmaTurf_To_EldinVolcano_StartVent => {
                RequirementKey::MogmaTurf_To_EldinVolcano_StartVent
            }
            Exit::Sandship_To_SandSea => RequirementKey::Sandship_To_SandSea,
            Exit::Sandship_To_SandshipBoss => RequirementKey::Sandship_To_SandshipBoss,
            Exit::BehindTheTemple_To_FaronWoods => RequirementKey::BehindTheTemple_To_FaronWoods,
            Exit::BehindTheTemple_To_SealedGroundsSpiral => {
                RequirementKey::BehindTheTemple_To_SealedGroundsSpiral
            }
            Exit::BehindTheTemple_To_SealedTemple => {
                RequirementKey::BehindTheTemple_To_SealedTemple
            }
            Exit::BehindTheTemple_To_Sky_BehindTheTempleStatue => {
                RequirementKey::BehindTheTemple_To_Sky_BehindTheTempleStatue
            }
            Exit::SealedGroundsSpiral_To_SealedTemple => {
                RequirementKey::SealedGroundsSpiral_To_SealedTemple
            }
            Exit::SealedGroundsSpiral_To_Sky_SealedGroundsStatue => {
                RequirementKey::SealedGroundsSpiral_To_Sky_SealedGroundsStatue
            }
            Exit::SealedTemple_To_BehindTheTemple => {
                RequirementKey::SealedTemple_To_BehindTheTemple
            }
            Exit::SealedTemple_To_HyliasTemple => RequirementKey::SealedTemple_To_HyliasTemple,
            Exit::SealedTemple_To_SealedGroundsSpiral => {
                RequirementKey::SealedTemple_To_SealedGroundsSpiral
            }
            Exit::InsideBambooIsland_To_Sky => RequirementKey::InsideBambooIsland_To_Sky,
            Exit::LumpyPumpkin_To_Sky_North => RequirementKey::LumpyPumpkin_To_Sky_North,
            Exit::LumpyPumpkin_To_Sky_South => RequirementKey::LumpyPumpkin_To_Sky_South,
            Exit::Sky_To_BeedlesShop_Night => RequirementKey::Sky_To_BeedlesShop_Night,
            Exit::Sky_To_BehindTheTemple_BehindTheTempleStatue => {
                RequirementKey::Sky_To_BehindTheTemple_BehindTheTempleStatue
            }
            Exit::Sky_To_DeepWoods_DeepWoodsStatue => {
                RequirementKey::Sky_To_DeepWoods_DeepWoodsStatue
            }
            Exit::Sky_To_DeepWoods_ForestTempleStatue => {
                RequirementKey::Sky_To_DeepWoods_ForestTempleStatue
            }
            Exit::Sky_To_EldinVolcano_EldinEntranceStatue => {
                RequirementKey::Sky_To_EldinVolcano_EldinEntranceStatue
            }
            Exit::Sky_To_EldinVolcano_TempleEntranceStatue => {
                RequirementKey::Sky_To_EldinVolcano_TempleEntranceStatue
            }
            Exit::Sky_To_EldinVolcano_VolcanoEastStatue => {
                RequirementKey::Sky_To_EldinVolcano_VolcanoEastStatue
            }
            Exit::Sky_To_EldinVolcano_VolcanoAscentStatue => {
                RequirementKey::Sky_To_EldinVolcano_VolcanoAscentStatue
            }
            Exit::Sky_To_FaronWoods_FaronWoodsEntryStatue => {
                RequirementKey::Sky_To_FaronWoods_FaronWoodsEntryStatue
            }
            Exit::Sky_To_FaronWoods_GreatTreeStatue => {
                RequirementKey::Sky_To_FaronWoods_GreatTreeStatue
            }
            Exit::Sky_To_FaronWoods_InTheWoodsStatue => {
                RequirementKey::Sky_To_FaronWoods_InTheWoodsStatue
            }
            Exit::Sky_To_FaronWoods_ViewingPlatformStatue => {
                RequirementKey::Sky_To_FaronWoods_ViewingPlatformStatue
            }
            Exit::Sky_To_FloriaWaterfall_FloriaWaterfallStatue => {
                RequirementKey::Sky_To_FloriaWaterfall_FloriaWaterfallStatue
            }
            Exit::Sky_To_InsideBambooIsland => RequirementKey::Sky_To_InsideBambooIsland,
            Exit::Sky_To_InsideThunderhead => RequirementKey::Sky_To_InsideThunderhead,
            Exit::Sky_To_LakeFloria_LakeFloriaStatue => {
                RequirementKey::Sky_To_LakeFloria_LakeFloriaStatue
            }
            Exit::Sky_To_LanayruDesert_DesertEntranceStatue => {
                RequirementKey::Sky_To_LanayruDesert_DesertEntranceStatue
            }
            Exit::Sky_To_LanayruDesert_NorthDesertStatue => {
                RequirementKey::Sky_To_LanayruDesert_NorthDesertStatue
            }
            Exit::Sky_To_LanayruDesert_StoneCacheStatue => {
                RequirementKey::Sky_To_LanayruDesert_StoneCacheStatue
            }
            Exit::Sky_To_LanayruDesert_WestDesertStatue => {
                RequirementKey::Sky_To_LanayruDesert_WestDesertStatue
            }
            Exit::Sky_To_LanayruMines_LanayruMineEntryStatue => {
                RequirementKey::Sky_To_LanayruMines_LanayruMineEntryStatue
            }
            Exit::Sky_To_OutsideFireSanctuary_InsideTheVolcanoStatue => {
                RequirementKey::Sky_To_OutsideFireSanctuary_InsideTheVolcanoStatue
            }
            Exit::Sky_To_SealedGroundsSpiral_SealedGroundsStatue => {
                RequirementKey::Sky_To_SealedGroundsSpiral_SealedGroundsStatue
            }
            Exit::Sky_To_Skyloft => RequirementKey::Sky_To_Skyloft,
            Exit::Sky_To_LumpyPumpkin_North => RequirementKey::Sky_To_LumpyPumpkin_North,
            Exit::Sky_To_LumpyPumpkin_South => RequirementKey::Sky_To_LumpyPumpkin_South,
            Exit::SkyKeepEntry_To_Skyloft => RequirementKey::SkyKeepEntry_To_Skyloft,
            Exit::SkyloftSilentRealm_To_Skyloft => RequirementKey::SkyloftSilentRealm_To_Skyloft,
            Exit::BertiesHouse_To_Skyloft => RequirementKey::BertiesHouse_To_Skyloft,
            Exit::GondosHouse_To_Skyloft => RequirementKey::GondosHouse_To_Skyloft,
            Exit::MallarasHouse_To_Skyloft => RequirementKey::MallarasHouse_To_Skyloft,
            Exit::RupinsHouse_To_Skyloft => RequirementKey::RupinsHouse_To_Skyloft,
            Exit::Skyloft_To_BatreauxHouse => RequirementKey::Skyloft_To_BatreauxHouse,
            Exit::Skyloft_To_BertiesHouse => RequirementKey::Skyloft_To_BertiesHouse,
            Exit::Skyloft_To_GondosHouse => RequirementKey::Skyloft_To_GondosHouse,
            Exit::Skyloft_To_MallarasHouse => RequirementKey::Skyloft_To_MallarasHouse,
            Exit::Skyloft_To_RupinsHouse => RequirementKey::Skyloft_To_RupinsHouse,
            Exit::Skyloft_To_SparrotsHouse => RequirementKey::Skyloft_To_SparrotsHouse,
            Exit::SparrotsHouse_To_Skyloft => RequirementKey::SparrotsHouse_To_Skyloft,
            Exit::SkyviewBoss_To_SkyviewSpring => RequirementKey::SkyviewBoss_To_SkyviewSpring,
            Exit::SkyviewBoss_To_SkyviewTemple => RequirementKey::SkyviewBoss_To_SkyviewTemple,
            Exit::SkyviewSpring_To_DeepWoods => RequirementKey::SkyviewSpring_To_DeepWoods,
            Exit::SkyviewSpring_To_SkyviewBoss => RequirementKey::SkyviewSpring_To_SkyviewBoss,
            Exit::SkyviewTemple_To_SkyviewBoss => RequirementKey::SkyviewTemple_To_SkyviewBoss,
            Exit::SkyviewTemple_To_DeepWoods => RequirementKey::SkyviewTemple_To_DeepWoods,
            Exit::InsideThunderhead_To_IsleOfSongs => {
                RequirementKey::InsideThunderhead_To_IsleOfSongs
            }
            Exit::InsideThunderhead_To_Sky => RequirementKey::InsideThunderhead_To_Sky,
            Exit::IsleOfSongs_To_InsideThunderhead => {
                RequirementKey::IsleOfSongs_To_InsideThunderhead
            }
            Exit::InsideVolcanoSummit_To_EldinVolcano => {
                RequirementKey::InsideVolcanoSummit_To_EldinVolcano
            }
            Exit::InsideVolcanoSummit_To_OutsideFireSanctuary => {
                RequirementKey::InsideVolcanoSummit_To_OutsideFireSanctuary
            }
            Exit::InsideVolcanoSummit_To_VolcanoSummitWaterfall => {
                RequirementKey::InsideVolcanoSummit_To_VolcanoSummitWaterfall
            }
            Exit::OutsideFireSanctuary_To_FireSanctuaryA => {
                RequirementKey::OutsideFireSanctuary_To_FireSanctuaryA
            }
            Exit::OutsideFireSanctuary_To_Sky_InsideTheVolcanoStatue => {
                RequirementKey::OutsideFireSanctuary_To_Sky_InsideTheVolcanoStatue
            }
            Exit::OutsideFireSanctuary_To_InsideVolcanoSummit => {
                RequirementKey::OutsideFireSanctuary_To_InsideVolcanoSummit
            }
            Exit::VolcanoSummitWaterfall_To_InsideVolcanoSummit => {
                RequirementKey::VolcanoSummitWaterfall_To_InsideVolcanoSummit
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Entrance {
    AncientCisternBoss_From_AncientCistern,
    FloriaWaterfall_From_AncientCistern,
    AncientCisternCandleRoom_From_AncientCisternBoss,
    Skyloft_From_BatreauxHouse,
    Sky_From_BeedlesShop_Night,
    Skyloft_From_BeedlesShop_Day,
    Skyloft_From_Bazaar_North,
    Skyloft_From_Bazaar_South,
    Skyloft_From_Bazaar_West,
    Skyloft_From_ParrowAndOriellesHouse,
    Skyloft_From_PeatricesHouse,
    Skyloft_From_PipersHouse,
    Bazaar_From_Skyloft_North,
    Bazaar_From_Skyloft_South,
    Bazaar_From_Skyloft_West,
    BeedlesShop_From_Skyloft_Day,
    ParrowAndOriellesHouse_From_Skyloft,
    PeatricesHouse_From_Skyloft,
    PipersHouse_From_Skyloft,
    Sky_From_Skyloft,
    SkyloftSilentRealm_From_Skyloft,
    WaterfallCave_From_Skyloft_Upper,
    WrynasHouse_From_Skyloft,
    Sky_From_Skyloft_PastWaterfallCave,
    WaterfallCave_From_Skyloft_Lower,
    SkyKeepEntry_From_Skyloft,
    Skyloft_From_WaterfallCave_Upper,
    Skyloft_From_WaterfallCave_Lower,
    Skyloft_From_WrynasHouse,
    EarthTempleBoss_From_EarthTemple,
    EldinVolcano_From_EarthTemple,
    EarthTempleSpring_From_EarthTempleBoss,
    EldinVolcano_From_EarthTempleSpring,
    EldinVolcano_From_EldinSilentRealm,
    Sky_From_EldinVolcano_EldinEntranceStatue,
    InsideVolcanoSummit_From_EldinVolcano,
    ThrillDiggerCave_From_EldinVolcano,
    EarthTemple_From_EldinVolcano,
    Sky_From_EldinVolcano_TempleEntranceStatue,
    MogmaTurf_From_EldinVolcano_Skydive,
    Sky_From_EldinVolcano_VolcanoEastStatue,
    EldinSilentRealm_From_EldinVolcano,
    Sky_From_EldinVolcano_VolcanoAscentStatue,
    EldinVolcano_From_ThrillDiggerCave,
    FaronWoods_From_FaronSilentRealm,
    FaronWoods_From_DeepWoods,
    Sky_From_DeepWoods_DeepWoodsStatue,
    Sky_From_DeepWoods_ForestTempleStatue,
    SkyviewTemple_From_DeepWoods,
    BehindTheTemple_From_FaronWoods,
    Sky_From_FaronWoods_FaronWoodsEntryStatue,
    GreatTree_From_FaronWoods_LowerPlatform,
    GreatTree_From_FaronWoods_UpperPlatform,
    GreatTree_From_FaronWoods_Top,
    Sky_From_FaronWoods_GreatTreeStatue,
    DeepWoods_From_FaronWoods,
    FaronSilentRealm_From_FaronWoods,
    GreatTree_From_FaronWoods_Tunnel,
    LakeFloria_From_FaronWoods,
    Sky_From_FaronWoods_InTheWoodsStatue,
    Sky_From_FaronWoods_ViewingPlatformStatue,
    FaronWoods_From_GreatTree_Tunnel,
    FaronWoods_From_GreatTree_LowerPlatform,
    FaronWoods_From_GreatTree_UpperPlatform,
    FaronWoods_From_GreatTree_Top,
    OutsideFireSanctuary_From_FireSanctuaryA,
    FireSanctuaryBoss_From_FireSanctuaryA,
    FireSanctuaryB_From_FireSanctuaryA,
    FireSanctuaryA_From_FireSanctuaryB,
    FireSanctuaryFlameRoom_From_FireSanctuaryBoss,
    Skyloft_From_InsideGoddessStatue,
    Skyloft_From_KnightAcademy_LowerDoors,
    Skyloft_From_KnightAcademy_UpperDoors,
    InsideGoddessStatue_From_Skyloft,
    KnightAcademy_From_Skyloft_Chimney,
    KnightAcademy_From_Skyloft_LowerDoors,
    KnightAcademy_From_Skyloft_UpperDoors,
    SparringHall_From_Skyloft,
    Skyloft_From_SparringHall,
    FloriaWaterfall_From_FaroresLair,
    LakeFloria_From_FaroresLair,
    AncientCistern_From_FloriaWaterfall,
    FaronWoods_From_FloriaWaterfall,
    FaroresLair_From_FloriaWaterfall,
    Sky_From_FloriaWaterfall_FloriaWaterfallStatue,
    Sky_From_LakeFloria_LakeFloriaStatue,
    FaroresLair_From_LakeFloria,
    LanayruDesert_From_LanayruCaves,
    LanayruMines_From_LanayruCaves,
    SandSeaDocks_From_LanayruCaves,
    LanayruDesert_From_FireNode,
    LanayruMines_From_LanayruDesert,
    Sky_From_LanayruDesert_DesertEntranceStatue,
    FireNode_From_LanayruDesert,
    LanayruMiningFacilityA_From_LanayruDesert,
    LanayruSilentRealm_From_LanayruDesert,
    LightningNode_From_LanayruDesert,
    Sky_From_LanayruDesert_NorthDesertStatue,
    Sky_From_LanayruDesert_StoneCacheStatue,
    TempleOfTime_From_LanayruDesert_End,
    LanayruCaves_From_LanayruDesert,
    Sky_From_LanayruDesert_WestDesertStatue,
    TempleOfTime_From_LanayruDesert_Start,
    LanayruDesert_From_LightningNode,
    LanayruDesert_From_TempleOfTime_End,
    LanayruDesert_From_TempleOfTime_Start,
    Sky_From_LanayruMines_LanayruMineEntryStatue,
    LanayruCaves_From_LanayruMines,
    LanayruDesert_From_LanayruMines,
    LanayruDesert_From_LanayruMiningFacilityA,
    LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub2,
    LanayruMiningFacilityB_From_LanayruMiningFacilityA_HubW,
    LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub,
    LanayruMiningFacilityBoss_From_LanayruMiningFacilityB,
    LanayruMiningFacilityToToT_From_LanayruMiningFacilityBoss,
    TempleOfTime_From_LanayruMiningFacilityToToT,
    OutsidePiratesStronghold_From_InsidePiratesStronghold_End,
    OutsidePiratesStronghold_From_InsidePiratesStronghold_Beginning,
    InsidePiratesStronghold_From_OutsidePiratesStronghold_End,
    InsidePiratesStronghold_From_OutsidePiratesStronghold_Beginning,
    SandSea_From_OutsidePiratesStronghold,
    OutsidePiratesStronghold_From_SandSea,
    SandSeaDocks_From_SandSea,
    Sandship_From_SandSea,
    Shipyard_From_SandSea,
    SkippersRetreat_From_SandSea,
    SandSea_From_SandSeaDocks,
    Sky_From_SandSeaDocks_AncientHarbor,
    LanayruCaves_From_SandSeaDocks,
    ShipyardConstructionBay_From_Shipyard_Upper,
    SandSea_From_Shipyard,
    ShipyardConstructionBay_From_Shipyard_Lower,
    Shipyard_From_ShipyardConstructionBay_Lower,
    Shipyard_From_ShipyardConstructionBay_Upper,
    SkippersShack_From_SkippersRetreat,
    SandSea_From_SkippersRetreat,
    SkippersRetreat_From_SkippersShack,
    LanayruDesert_From_LanayruSilentRealm,
    EldinVolcano_From_MogmaTurf_EndVent,
    EldinVolcano_From_MogmaTurf_StartVent,
    SandSea_From_Sandship,
    SandshipBoss_From_Sandship,
    FaronWoods_From_BehindTheTemple,
    SealedGroundsSpiral_From_BehindTheTemple,
    SealedTemple_From_BehindTheTemple,
    Sky_From_BehindTheTemple_BehindTheTempleStatue,
    SealedTemple_From_SealedGroundsSpiral,
    Sky_From_SealedGroundsSpiral_SealedGroundsStatue,
    BehindTheTemple_From_SealedTemple,
    HyliasTemple_From_SealedTemple,
    SealedGroundsSpiral_From_SealedTemple,
    Sky_From_InsideBambooIsland,
    Sky_From_LumpyPumpkin_North,
    Sky_From_LumpyPumpkin_South,
    BeedlesShop_From_Sky_Night,
    BehindTheTemple_From_Sky_BehindTheTempleStatue,
    DeepWoods_From_Sky_DeepWoodsStatue,
    DeepWoods_From_Sky_ForestTempleStatue,
    EldinVolcano_From_Sky_EldinEntranceStatue,
    EldinVolcano_From_Sky_TempleEntranceStatue,
    EldinVolcano_From_Sky_VolcanoEastStatue,
    EldinVolcano_From_Sky_VolcanoAscentStatue,
    FaronWoods_From_Sky_FaronWoodsEntryStatue,
    FaronWoods_From_Sky_GreatTreeStatue,
    FaronWoods_From_Sky_InTheWoodsStatue,
    FaronWoods_From_Sky_ViewingPlatformStatue,
    FloriaWaterfall_From_Sky_FloriaWaterfallStatue,
    InsideBambooIsland_From_Sky,
    InsideThunderhead_From_Sky,
    LakeFloria_From_Sky_LakeFloriaStatue,
    LanayruDesert_From_Sky_DesertEntranceStatue,
    LanayruDesert_From_Sky_NorthDesertStatue,
    LanayruDesert_From_Sky_StoneCacheStatue,
    LanayruDesert_From_Sky_WestDesertStatue,
    LanayruMines_From_Sky_LanayruMineEntryStatue,
    OutsideFireSanctuary_From_Sky_InsideTheVolcanoStatue,
    SealedGroundsSpiral_From_Sky_SealedGroundsStatue,
    Skyloft_From_Sky,
    LumpyPumpkin_From_Sky_North,
    LumpyPumpkin_From_Sky_South,
    Skyloft_From_SkyKeepEntry,
    Skyloft_From_SkyloftSilentRealm,
    Skyloft_From_BertiesHouse,
    Skyloft_From_GondosHouse,
    Skyloft_From_MallarasHouse,
    Skyloft_From_RupinsHouse,
    BatreauxHouse_From_Skyloft,
    BertiesHouse_From_Skyloft,
    GondosHouse_From_Skyloft,
    MallarasHouse_From_Skyloft,
    RupinsHouse_From_Skyloft,
    SparrotsHouse_From_Skyloft,
    Skyloft_From_SparrotsHouse,
    SkyviewSpring_From_SkyviewBoss,
    SkyviewTemple_From_SkyviewBoss,
    DeepWoods_From_SkyviewSpring,
    SkyviewBoss_From_SkyviewSpring,
    SkyviewBoss_From_SkyviewTemple,
    DeepWoods_From_SkyviewTemple,
    IsleOfSongs_From_InsideThunderhead,
    Sky_From_InsideThunderhead,
    InsideThunderhead_From_IsleOfSongs,
    EldinVolcano_From_InsideVolcanoSummit,
    OutsideFireSanctuary_From_InsideVolcanoSummit,
    VolcanoSummitWaterfall_From_InsideVolcanoSummit,
    FireSanctuaryA_From_OutsideFireSanctuary,
    Sky_From_OutsideFireSanctuary_InsideTheVolcanoStatue,
    InsideVolcanoSummit_From_OutsideFireSanctuary,
    InsideVolcanoSummit_From_VolcanoSummitWaterfall,
}
impl Into<usize> for Entrance {
    fn into(self) -> usize {
        self as usize
    }
}
impl BitSetCompatible for Entrance {
    const ALL: &'static [Entrance] = &[
        Entrance::AncientCisternBoss_From_AncientCistern,
        Entrance::FloriaWaterfall_From_AncientCistern,
        Entrance::AncientCisternCandleRoom_From_AncientCisternBoss,
        Entrance::Skyloft_From_BatreauxHouse,
        Entrance::Sky_From_BeedlesShop_Night,
        Entrance::Skyloft_From_BeedlesShop_Day,
        Entrance::Skyloft_From_Bazaar_North,
        Entrance::Skyloft_From_Bazaar_South,
        Entrance::Skyloft_From_Bazaar_West,
        Entrance::Skyloft_From_ParrowAndOriellesHouse,
        Entrance::Skyloft_From_PeatricesHouse,
        Entrance::Skyloft_From_PipersHouse,
        Entrance::Bazaar_From_Skyloft_North,
        Entrance::Bazaar_From_Skyloft_South,
        Entrance::Bazaar_From_Skyloft_West,
        Entrance::BeedlesShop_From_Skyloft_Day,
        Entrance::ParrowAndOriellesHouse_From_Skyloft,
        Entrance::PeatricesHouse_From_Skyloft,
        Entrance::PipersHouse_From_Skyloft,
        Entrance::Sky_From_Skyloft,
        Entrance::SkyloftSilentRealm_From_Skyloft,
        Entrance::WaterfallCave_From_Skyloft_Upper,
        Entrance::WrynasHouse_From_Skyloft,
        Entrance::Sky_From_Skyloft_PastWaterfallCave,
        Entrance::WaterfallCave_From_Skyloft_Lower,
        Entrance::SkyKeepEntry_From_Skyloft,
        Entrance::Skyloft_From_WaterfallCave_Upper,
        Entrance::Skyloft_From_WaterfallCave_Lower,
        Entrance::Skyloft_From_WrynasHouse,
        Entrance::EarthTempleBoss_From_EarthTemple,
        Entrance::EldinVolcano_From_EarthTemple,
        Entrance::EarthTempleSpring_From_EarthTempleBoss,
        Entrance::EldinVolcano_From_EarthTempleSpring,
        Entrance::EldinVolcano_From_EldinSilentRealm,
        Entrance::Sky_From_EldinVolcano_EldinEntranceStatue,
        Entrance::InsideVolcanoSummit_From_EldinVolcano,
        Entrance::ThrillDiggerCave_From_EldinVolcano,
        Entrance::EarthTemple_From_EldinVolcano,
        Entrance::Sky_From_EldinVolcano_TempleEntranceStatue,
        Entrance::MogmaTurf_From_EldinVolcano_Skydive,
        Entrance::Sky_From_EldinVolcano_VolcanoEastStatue,
        Entrance::EldinSilentRealm_From_EldinVolcano,
        Entrance::Sky_From_EldinVolcano_VolcanoAscentStatue,
        Entrance::EldinVolcano_From_ThrillDiggerCave,
        Entrance::FaronWoods_From_FaronSilentRealm,
        Entrance::FaronWoods_From_DeepWoods,
        Entrance::Sky_From_DeepWoods_DeepWoodsStatue,
        Entrance::Sky_From_DeepWoods_ForestTempleStatue,
        Entrance::SkyviewTemple_From_DeepWoods,
        Entrance::BehindTheTemple_From_FaronWoods,
        Entrance::Sky_From_FaronWoods_FaronWoodsEntryStatue,
        Entrance::GreatTree_From_FaronWoods_LowerPlatform,
        Entrance::GreatTree_From_FaronWoods_UpperPlatform,
        Entrance::GreatTree_From_FaronWoods_Top,
        Entrance::Sky_From_FaronWoods_GreatTreeStatue,
        Entrance::DeepWoods_From_FaronWoods,
        Entrance::FaronSilentRealm_From_FaronWoods,
        Entrance::GreatTree_From_FaronWoods_Tunnel,
        Entrance::LakeFloria_From_FaronWoods,
        Entrance::Sky_From_FaronWoods_InTheWoodsStatue,
        Entrance::Sky_From_FaronWoods_ViewingPlatformStatue,
        Entrance::FaronWoods_From_GreatTree_Tunnel,
        Entrance::FaronWoods_From_GreatTree_LowerPlatform,
        Entrance::FaronWoods_From_GreatTree_UpperPlatform,
        Entrance::FaronWoods_From_GreatTree_Top,
        Entrance::OutsideFireSanctuary_From_FireSanctuaryA,
        Entrance::FireSanctuaryBoss_From_FireSanctuaryA,
        Entrance::FireSanctuaryB_From_FireSanctuaryA,
        Entrance::FireSanctuaryA_From_FireSanctuaryB,
        Entrance::FireSanctuaryFlameRoom_From_FireSanctuaryBoss,
        Entrance::Skyloft_From_InsideGoddessStatue,
        Entrance::Skyloft_From_KnightAcademy_LowerDoors,
        Entrance::Skyloft_From_KnightAcademy_UpperDoors,
        Entrance::InsideGoddessStatue_From_Skyloft,
        Entrance::KnightAcademy_From_Skyloft_Chimney,
        Entrance::KnightAcademy_From_Skyloft_LowerDoors,
        Entrance::KnightAcademy_From_Skyloft_UpperDoors,
        Entrance::SparringHall_From_Skyloft,
        Entrance::Skyloft_From_SparringHall,
        Entrance::FloriaWaterfall_From_FaroresLair,
        Entrance::LakeFloria_From_FaroresLair,
        Entrance::AncientCistern_From_FloriaWaterfall,
        Entrance::FaronWoods_From_FloriaWaterfall,
        Entrance::FaroresLair_From_FloriaWaterfall,
        Entrance::Sky_From_FloriaWaterfall_FloriaWaterfallStatue,
        Entrance::Sky_From_LakeFloria_LakeFloriaStatue,
        Entrance::FaroresLair_From_LakeFloria,
        Entrance::LanayruDesert_From_LanayruCaves,
        Entrance::LanayruMines_From_LanayruCaves,
        Entrance::SandSeaDocks_From_LanayruCaves,
        Entrance::LanayruDesert_From_FireNode,
        Entrance::LanayruMines_From_LanayruDesert,
        Entrance::Sky_From_LanayruDesert_DesertEntranceStatue,
        Entrance::FireNode_From_LanayruDesert,
        Entrance::LanayruMiningFacilityA_From_LanayruDesert,
        Entrance::LanayruSilentRealm_From_LanayruDesert,
        Entrance::LightningNode_From_LanayruDesert,
        Entrance::Sky_From_LanayruDesert_NorthDesertStatue,
        Entrance::Sky_From_LanayruDesert_StoneCacheStatue,
        Entrance::TempleOfTime_From_LanayruDesert_End,
        Entrance::LanayruCaves_From_LanayruDesert,
        Entrance::Sky_From_LanayruDesert_WestDesertStatue,
        Entrance::TempleOfTime_From_LanayruDesert_Start,
        Entrance::LanayruDesert_From_LightningNode,
        Entrance::LanayruDesert_From_TempleOfTime_End,
        Entrance::LanayruDesert_From_TempleOfTime_Start,
        Entrance::Sky_From_LanayruMines_LanayruMineEntryStatue,
        Entrance::LanayruCaves_From_LanayruMines,
        Entrance::LanayruDesert_From_LanayruMines,
        Entrance::LanayruDesert_From_LanayruMiningFacilityA,
        Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub2,
        Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_HubW,
        Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub,
        Entrance::LanayruMiningFacilityBoss_From_LanayruMiningFacilityB,
        Entrance::LanayruMiningFacilityToToT_From_LanayruMiningFacilityBoss,
        Entrance::TempleOfTime_From_LanayruMiningFacilityToToT,
        Entrance::OutsidePiratesStronghold_From_InsidePiratesStronghold_End,
        Entrance::OutsidePiratesStronghold_From_InsidePiratesStronghold_Beginning,
        Entrance::InsidePiratesStronghold_From_OutsidePiratesStronghold_End,
        Entrance::InsidePiratesStronghold_From_OutsidePiratesStronghold_Beginning,
        Entrance::SandSea_From_OutsidePiratesStronghold,
        Entrance::OutsidePiratesStronghold_From_SandSea,
        Entrance::SandSeaDocks_From_SandSea,
        Entrance::Sandship_From_SandSea,
        Entrance::Shipyard_From_SandSea,
        Entrance::SkippersRetreat_From_SandSea,
        Entrance::SandSea_From_SandSeaDocks,
        Entrance::Sky_From_SandSeaDocks_AncientHarbor,
        Entrance::LanayruCaves_From_SandSeaDocks,
        Entrance::ShipyardConstructionBay_From_Shipyard_Upper,
        Entrance::SandSea_From_Shipyard,
        Entrance::ShipyardConstructionBay_From_Shipyard_Lower,
        Entrance::Shipyard_From_ShipyardConstructionBay_Lower,
        Entrance::Shipyard_From_ShipyardConstructionBay_Upper,
        Entrance::SkippersShack_From_SkippersRetreat,
        Entrance::SandSea_From_SkippersRetreat,
        Entrance::SkippersRetreat_From_SkippersShack,
        Entrance::LanayruDesert_From_LanayruSilentRealm,
        Entrance::EldinVolcano_From_MogmaTurf_EndVent,
        Entrance::EldinVolcano_From_MogmaTurf_StartVent,
        Entrance::SandSea_From_Sandship,
        Entrance::SandshipBoss_From_Sandship,
        Entrance::FaronWoods_From_BehindTheTemple,
        Entrance::SealedGroundsSpiral_From_BehindTheTemple,
        Entrance::SealedTemple_From_BehindTheTemple,
        Entrance::Sky_From_BehindTheTemple_BehindTheTempleStatue,
        Entrance::SealedTemple_From_SealedGroundsSpiral,
        Entrance::Sky_From_SealedGroundsSpiral_SealedGroundsStatue,
        Entrance::BehindTheTemple_From_SealedTemple,
        Entrance::HyliasTemple_From_SealedTemple,
        Entrance::SealedGroundsSpiral_From_SealedTemple,
        Entrance::Sky_From_InsideBambooIsland,
        Entrance::Sky_From_LumpyPumpkin_North,
        Entrance::Sky_From_LumpyPumpkin_South,
        Entrance::BeedlesShop_From_Sky_Night,
        Entrance::BehindTheTemple_From_Sky_BehindTheTempleStatue,
        Entrance::DeepWoods_From_Sky_DeepWoodsStatue,
        Entrance::DeepWoods_From_Sky_ForestTempleStatue,
        Entrance::EldinVolcano_From_Sky_EldinEntranceStatue,
        Entrance::EldinVolcano_From_Sky_TempleEntranceStatue,
        Entrance::EldinVolcano_From_Sky_VolcanoEastStatue,
        Entrance::EldinVolcano_From_Sky_VolcanoAscentStatue,
        Entrance::FaronWoods_From_Sky_FaronWoodsEntryStatue,
        Entrance::FaronWoods_From_Sky_GreatTreeStatue,
        Entrance::FaronWoods_From_Sky_InTheWoodsStatue,
        Entrance::FaronWoods_From_Sky_ViewingPlatformStatue,
        Entrance::FloriaWaterfall_From_Sky_FloriaWaterfallStatue,
        Entrance::InsideBambooIsland_From_Sky,
        Entrance::InsideThunderhead_From_Sky,
        Entrance::LakeFloria_From_Sky_LakeFloriaStatue,
        Entrance::LanayruDesert_From_Sky_DesertEntranceStatue,
        Entrance::LanayruDesert_From_Sky_NorthDesertStatue,
        Entrance::LanayruDesert_From_Sky_StoneCacheStatue,
        Entrance::LanayruDesert_From_Sky_WestDesertStatue,
        Entrance::LanayruMines_From_Sky_LanayruMineEntryStatue,
        Entrance::OutsideFireSanctuary_From_Sky_InsideTheVolcanoStatue,
        Entrance::SealedGroundsSpiral_From_Sky_SealedGroundsStatue,
        Entrance::Skyloft_From_Sky,
        Entrance::LumpyPumpkin_From_Sky_North,
        Entrance::LumpyPumpkin_From_Sky_South,
        Entrance::Skyloft_From_SkyKeepEntry,
        Entrance::Skyloft_From_SkyloftSilentRealm,
        Entrance::Skyloft_From_BertiesHouse,
        Entrance::Skyloft_From_GondosHouse,
        Entrance::Skyloft_From_MallarasHouse,
        Entrance::Skyloft_From_RupinsHouse,
        Entrance::BatreauxHouse_From_Skyloft,
        Entrance::BertiesHouse_From_Skyloft,
        Entrance::GondosHouse_From_Skyloft,
        Entrance::MallarasHouse_From_Skyloft,
        Entrance::RupinsHouse_From_Skyloft,
        Entrance::SparrotsHouse_From_Skyloft,
        Entrance::Skyloft_From_SparrotsHouse,
        Entrance::SkyviewSpring_From_SkyviewBoss,
        Entrance::SkyviewTemple_From_SkyviewBoss,
        Entrance::DeepWoods_From_SkyviewSpring,
        Entrance::SkyviewBoss_From_SkyviewSpring,
        Entrance::SkyviewBoss_From_SkyviewTemple,
        Entrance::DeepWoods_From_SkyviewTemple,
        Entrance::IsleOfSongs_From_InsideThunderhead,
        Entrance::Sky_From_InsideThunderhead,
        Entrance::InsideThunderhead_From_IsleOfSongs,
        Entrance::EldinVolcano_From_InsideVolcanoSummit,
        Entrance::OutsideFireSanctuary_From_InsideVolcanoSummit,
        Entrance::VolcanoSummitWaterfall_From_InsideVolcanoSummit,
        Entrance::FireSanctuaryA_From_OutsideFireSanctuary,
        Entrance::Sky_From_OutsideFireSanctuary_InsideTheVolcanoStatue,
        Entrance::InsideVolcanoSummit_From_OutsideFireSanctuary,
        Entrance::InsideVolcanoSummit_From_VolcanoSummitWaterfall,
    ];
}
impl Entrance {
    pub fn name(&self) -> &'static str {
        match self {
            Entrance::AncientCisternBoss_From_AncientCistern => {
                "Ancient Cistern Boss (from Ancient Cistern)"
            }
            Entrance::FloriaWaterfall_From_AncientCistern => {
                "Floria Waterfall (from Ancient Cistern)"
            }
            Entrance::AncientCisternCandleRoom_From_AncientCisternBoss => {
                "Ancient Cistern Candle Room (from Ancient Cistern Boss)"
            }
            Entrance::Skyloft_From_BatreauxHouse => "Skyloft (from Batreaux' House)",
            Entrance::Sky_From_BeedlesShop_Night => "Sky (from Beedle's Shop) (Night)",
            Entrance::Skyloft_From_BeedlesShop_Day => "Skyloft (from Beedle's Shop) (Day)",
            Entrance::Skyloft_From_Bazaar_North => "Skyloft (from Bazaar) (North)",
            Entrance::Skyloft_From_Bazaar_South => "Skyloft (from Bazaar) (South)",
            Entrance::Skyloft_From_Bazaar_West => "Skyloft (from Bazaar) (West)",
            Entrance::Skyloft_From_ParrowAndOriellesHouse => {
                "Skyloft (from Parrow and Orielle's House)"
            }
            Entrance::Skyloft_From_PeatricesHouse => "Skyloft (from Peatrice's House)",
            Entrance::Skyloft_From_PipersHouse => "Skyloft (from Piper's House)",
            Entrance::Bazaar_From_Skyloft_North => "Bazaar (from Skyloft) (North)",
            Entrance::Bazaar_From_Skyloft_South => "Bazaar (from Skyloft) (South)",
            Entrance::Bazaar_From_Skyloft_West => "Bazaar (from Skyloft) (West)",
            Entrance::BeedlesShop_From_Skyloft_Day => "Beedle's Shop (from Skyloft) (Day)",
            Entrance::ParrowAndOriellesHouse_From_Skyloft => {
                "Parrow and Orielle's House (from Skyloft)"
            }
            Entrance::PeatricesHouse_From_Skyloft => "Peatrice's House (from Skyloft)",
            Entrance::PipersHouse_From_Skyloft => "Piper's House (from Skyloft)",
            Entrance::Sky_From_Skyloft => "Sky (from Skyloft)",
            Entrance::SkyloftSilentRealm_From_Skyloft => "Skyloft Silent Realm (from Skyloft)",
            Entrance::WaterfallCave_From_Skyloft_Upper => "Waterfall Cave (from Skyloft) (Upper)",
            Entrance::WrynasHouse_From_Skyloft => "Wryna's House (from Skyloft)",
            Entrance::Sky_From_Skyloft_PastWaterfallCave => {
                "Sky (from Skyloft) (Past Waterfall Cave)"
            }
            Entrance::WaterfallCave_From_Skyloft_Lower => "Waterfall Cave (from Skyloft) (Lower)",
            Entrance::SkyKeepEntry_From_Skyloft => "Sky Keep Entry (from Skyloft)",
            Entrance::Skyloft_From_WaterfallCave_Upper => "Skyloft (from Waterfall Cave) (Upper)",
            Entrance::Skyloft_From_WaterfallCave_Lower => "Skyloft (from Waterfall Cave) (Lower)",
            Entrance::Skyloft_From_WrynasHouse => "Skyloft (from Wryna's House)",
            Entrance::EarthTempleBoss_From_EarthTemple => "Earth Temple Boss (from Earth Temple)",
            Entrance::EldinVolcano_From_EarthTemple => "Eldin Volcano (from Earth Temple)",
            Entrance::EarthTempleSpring_From_EarthTempleBoss => {
                "Earth Temple Spring (from Earth Temple Boss)"
            }
            Entrance::EldinVolcano_From_EarthTempleSpring => {
                "Eldin Volcano (from Earth Temple Spring)"
            }
            Entrance::EldinVolcano_From_EldinSilentRealm => {
                "Eldin Volcano (from Eldin Silent Realm)"
            }
            Entrance::Sky_From_EldinVolcano_EldinEntranceStatue => {
                "Sky (from Eldin Volcano) (Eldin Entrance Statue)"
            }
            Entrance::InsideVolcanoSummit_From_EldinVolcano => {
                "Inside Volcano Summit (from Eldin Volcano)"
            }
            Entrance::ThrillDiggerCave_From_EldinVolcano => {
                "Thrill Digger Cave (from Eldin Volcano)"
            }
            Entrance::EarthTemple_From_EldinVolcano => "Earth Temple (from Eldin Volcano)",
            Entrance::Sky_From_EldinVolcano_TempleEntranceStatue => {
                "Sky (from Eldin Volcano) (Temple Entrance Statue)"
            }
            Entrance::MogmaTurf_From_EldinVolcano_Skydive => {
                "Mogma Turf (from Eldin Volcano) (Skydive)"
            }
            Entrance::Sky_From_EldinVolcano_VolcanoEastStatue => {
                "Sky (from Eldin Volcano) (Volcano East Statue)"
            }
            Entrance::EldinSilentRealm_From_EldinVolcano => {
                "Eldin Silent Realm (from Eldin Volcano)"
            }
            Entrance::Sky_From_EldinVolcano_VolcanoAscentStatue => {
                "Sky (from Eldin Volcano) (Volcano Ascent Statue)"
            }
            Entrance::EldinVolcano_From_ThrillDiggerCave => {
                "Eldin Volcano (from Thrill Digger Cave)"
            }
            Entrance::FaronWoods_From_FaronSilentRealm => "Faron Woods (from Faron Silent Realm)",
            Entrance::FaronWoods_From_DeepWoods => "Faron Woods (from Deep Woods)",
            Entrance::Sky_From_DeepWoods_DeepWoodsStatue => {
                "Sky (from Deep Woods) (Deep Woods Statue)"
            }
            Entrance::Sky_From_DeepWoods_ForestTempleStatue => {
                "Sky (from Deep Woods) (Forest Temple Statue)"
            }
            Entrance::SkyviewTemple_From_DeepWoods => "Skyview Temple (from Deep Woods)",
            Entrance::BehindTheTemple_From_FaronWoods => "Behind the Temple (from Faron Woods)",
            Entrance::Sky_From_FaronWoods_FaronWoodsEntryStatue => {
                "Sky (from Faron Woods) (Faron Woods Entry Statue)"
            }
            Entrance::GreatTree_From_FaronWoods_LowerPlatform => {
                "Great Tree (from Faron Woods) (Lower Platform)"
            }
            Entrance::GreatTree_From_FaronWoods_UpperPlatform => {
                "Great Tree (from Faron Woods) (Upper Platform)"
            }
            Entrance::GreatTree_From_FaronWoods_Top => "Great Tree (from Faron Woods) (Top)",
            Entrance::Sky_From_FaronWoods_GreatTreeStatue => {
                "Sky (from Faron Woods) (Great Tree Statue)"
            }
            Entrance::DeepWoods_From_FaronWoods => "Deep Woods (from Faron Woods)",
            Entrance::FaronSilentRealm_From_FaronWoods => "Faron Silent Realm (from Faron Woods)",
            Entrance::GreatTree_From_FaronWoods_Tunnel => "Great Tree (from Faron Woods) (Tunnel)",
            Entrance::LakeFloria_From_FaronWoods => "Lake Floria (from Faron Woods)",
            Entrance::Sky_From_FaronWoods_InTheWoodsStatue => {
                "Sky (from Faron Woods) (In the Woods Statue)"
            }
            Entrance::Sky_From_FaronWoods_ViewingPlatformStatue => {
                "Sky (from Faron Woods) (Viewing Platform Statue)"
            }
            Entrance::FaronWoods_From_GreatTree_Tunnel => "Faron Woods (from Great Tree) (Tunnel)",
            Entrance::FaronWoods_From_GreatTree_LowerPlatform => {
                "Faron Woods (from Great Tree) (Lower Platform)"
            }
            Entrance::FaronWoods_From_GreatTree_UpperPlatform => {
                "Faron Woods (from Great Tree) (Upper Platform)"
            }
            Entrance::FaronWoods_From_GreatTree_Top => "Faron Woods (from Great Tree) (Top)",
            Entrance::OutsideFireSanctuary_From_FireSanctuaryA => {
                "Outside Fire Sanctuary (from Fire Sanctuary A)"
            }
            Entrance::FireSanctuaryBoss_From_FireSanctuaryA => {
                "Fire Sanctuary Boss (from Fire Sanctuary A)"
            }
            Entrance::FireSanctuaryB_From_FireSanctuaryA => {
                "Fire Sanctuary B (from Fire Sanctuary A)"
            }
            Entrance::FireSanctuaryA_From_FireSanctuaryB => {
                "Fire Sanctuary A (from Fire Sanctuary B)"
            }
            Entrance::FireSanctuaryFlameRoom_From_FireSanctuaryBoss => {
                "Fire Sanctuary Flame Room (from Fire Sanctuary Boss)"
            }
            Entrance::Skyloft_From_InsideGoddessStatue => "Skyloft (from Inside Goddess Statue)",
            Entrance::Skyloft_From_KnightAcademy_LowerDoors => {
                "Skyloft (from Knight Academy) (Lower Doors)"
            }
            Entrance::Skyloft_From_KnightAcademy_UpperDoors => {
                "Skyloft (from Knight Academy) (Upper Doors)"
            }
            Entrance::InsideGoddessStatue_From_Skyloft => "Inside Goddess Statue (from Skyloft)",
            Entrance::KnightAcademy_From_Skyloft_Chimney => {
                "Knight Academy (from Skyloft) (Chimney)"
            }
            Entrance::KnightAcademy_From_Skyloft_LowerDoors => {
                "Knight Academy (from Skyloft) (Lower Doors)"
            }
            Entrance::KnightAcademy_From_Skyloft_UpperDoors => {
                "Knight Academy (from Skyloft) (Upper Doors)"
            }
            Entrance::SparringHall_From_Skyloft => "Sparring Hall (from Skyloft)",
            Entrance::Skyloft_From_SparringHall => "Skyloft (from Sparring Hall)",
            Entrance::FloriaWaterfall_From_FaroresLair => "Floria Waterfall (from Farore's Lair)",
            Entrance::LakeFloria_From_FaroresLair => "Lake Floria (from Farore's Lair)",
            Entrance::AncientCistern_From_FloriaWaterfall => {
                "Ancient Cistern (from Floria Waterfall)"
            }
            Entrance::FaronWoods_From_FloriaWaterfall => "Faron Woods (from Floria Waterfall)",
            Entrance::FaroresLair_From_FloriaWaterfall => "Farore's Lair (from Floria Waterfall)",
            Entrance::Sky_From_FloriaWaterfall_FloriaWaterfallStatue => {
                "Sky (from Floria Waterfall) (Floria Waterfall Statue)"
            }
            Entrance::Sky_From_LakeFloria_LakeFloriaStatue => {
                "Sky (from Lake Floria) (Lake Floria Statue)"
            }
            Entrance::FaroresLair_From_LakeFloria => "Farore's Lair (from Lake Floria)",
            Entrance::LanayruDesert_From_LanayruCaves => "Lanayru Desert (from Lanayru Caves)",
            Entrance::LanayruMines_From_LanayruCaves => "Lanayru Mines (from Lanayru Caves)",
            Entrance::SandSeaDocks_From_LanayruCaves => "Sand Sea Docks (from Lanayru Caves)",
            Entrance::LanayruDesert_From_FireNode => "Lanayru Desert (from Fire Node)",
            Entrance::LanayruMines_From_LanayruDesert => "Lanayru Mines (from Lanayru Desert)",
            Entrance::Sky_From_LanayruDesert_DesertEntranceStatue => {
                "Sky (from Lanayru Desert) (Desert Entrance Statue)"
            }
            Entrance::FireNode_From_LanayruDesert => "Fire Node (from Lanayru Desert)",
            Entrance::LanayruMiningFacilityA_From_LanayruDesert => {
                "Lanayru Mining Facility A (from Lanayru Desert)"
            }
            Entrance::LanayruSilentRealm_From_LanayruDesert => {
                "Lanayru Silent Realm (from Lanayru Desert)"
            }
            Entrance::LightningNode_From_LanayruDesert => "Lightning Node (from Lanayru Desert)",
            Entrance::Sky_From_LanayruDesert_NorthDesertStatue => {
                "Sky (from Lanayru Desert) (North Desert Statue)"
            }
            Entrance::Sky_From_LanayruDesert_StoneCacheStatue => {
                "Sky (from Lanayru Desert) (Stone Cache Statue)"
            }
            Entrance::TempleOfTime_From_LanayruDesert_End => {
                "Temple of Time (from Lanayru Desert) (End)"
            }
            Entrance::LanayruCaves_From_LanayruDesert => "Lanayru Caves (from Lanayru Desert)",
            Entrance::Sky_From_LanayruDesert_WestDesertStatue => {
                "Sky (from Lanayru Desert) (West Desert Statue)"
            }
            Entrance::TempleOfTime_From_LanayruDesert_Start => {
                "Temple of Time (from Lanayru Desert) (Start)"
            }
            Entrance::LanayruDesert_From_LightningNode => "Lanayru Desert (from Lightning Node)",
            Entrance::LanayruDesert_From_TempleOfTime_End => {
                "Lanayru Desert (from Temple of Time) (End)"
            }
            Entrance::LanayruDesert_From_TempleOfTime_Start => {
                "Lanayru Desert (from Temple of Time) (Start)"
            }
            Entrance::Sky_From_LanayruMines_LanayruMineEntryStatue => {
                "Sky (from Lanayru Mines) (Lanayru Mine Entry Statue)"
            }
            Entrance::LanayruCaves_From_LanayruMines => "Lanayru Caves (from Lanayru Mines)",
            Entrance::LanayruDesert_From_LanayruMines => "Lanayru Desert (from Lanayru Mines)",
            Entrance::LanayruDesert_From_LanayruMiningFacilityA => {
                "Lanayru Desert (from Lanayru Mining Facility A)"
            }
            Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub2 => {
                "Lanayru Mining Facility B (from Lanayru Mining Facility A) (Hub 2)"
            }
            Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_HubW => {
                "Lanayru Mining Facility B (from Lanayru Mining Facility A) (Hub W)"
            }
            Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub => {
                "Lanayru Mining Facility B (from Lanayru Mining Facility A) (Hub)"
            }
            Entrance::LanayruMiningFacilityBoss_From_LanayruMiningFacilityB => {
                "Lanayru Mining Facility Boss (from Lanayru Mining Facility B)"
            }
            Entrance::LanayruMiningFacilityToToT_From_LanayruMiningFacilityBoss => {
                "Lanayru Mining Facility to ToT (from Lanayru Mining Facility Boss)"
            }
            Entrance::TempleOfTime_From_LanayruMiningFacilityToToT => {
                "Temple of Time (from Lanayru Mining Facility to ToT)"
            }
            Entrance::OutsidePiratesStronghold_From_InsidePiratesStronghold_End => {
                "Outside Pirate's Stronghold (from Inside Pirate's Stronghold) (End)"
            }
            Entrance::OutsidePiratesStronghold_From_InsidePiratesStronghold_Beginning => {
                "Outside Pirate's Stronghold (from Inside Pirate's Stronghold) (Beginning)"
            }
            Entrance::InsidePiratesStronghold_From_OutsidePiratesStronghold_End => {
                "Inside Pirate's Stronghold (from Outside Pirate's Stronghold) (End)"
            }
            Entrance::InsidePiratesStronghold_From_OutsidePiratesStronghold_Beginning => {
                "Inside Pirate's Stronghold (from Outside Pirate's Stronghold) (Beginning)"
            }
            Entrance::SandSea_From_OutsidePiratesStronghold => {
                "Sand Sea (from Outside Pirate's Stronghold)"
            }
            Entrance::OutsidePiratesStronghold_From_SandSea => {
                "Outside Pirate's Stronghold (from Sand Sea)"
            }
            Entrance::SandSeaDocks_From_SandSea => "Sand Sea Docks (from Sand Sea)",
            Entrance::Sandship_From_SandSea => "Sandship (from Sand Sea)",
            Entrance::Shipyard_From_SandSea => "Shipyard (from Sand Sea)",
            Entrance::SkippersRetreat_From_SandSea => "Skipper's Retreat (from Sand Sea)",
            Entrance::SandSea_From_SandSeaDocks => "Sand Sea (from Sand Sea Docks)",
            Entrance::Sky_From_SandSeaDocks_AncientHarbor => {
                "Sky (from Sand Sea Docks) (Ancient Harbor)"
            }
            Entrance::LanayruCaves_From_SandSeaDocks => "Lanayru Caves (from Sand Sea Docks)",
            Entrance::ShipyardConstructionBay_From_Shipyard_Upper => {
                "Shipyard Construction Bay (from Shipyard) (Upper)"
            }
            Entrance::SandSea_From_Shipyard => "Sand Sea (from Shipyard)",
            Entrance::ShipyardConstructionBay_From_Shipyard_Lower => {
                "Shipyard Construction Bay (from Shipyard) (Lower)"
            }
            Entrance::Shipyard_From_ShipyardConstructionBay_Lower => {
                "Shipyard (from Shipyard Construction Bay) (Lower)"
            }
            Entrance::Shipyard_From_ShipyardConstructionBay_Upper => {
                "Shipyard (from Shipyard Construction Bay) (Upper)"
            }
            Entrance::SkippersShack_From_SkippersRetreat => {
                "Skipper's Shack (from Skipper's Retreat)"
            }
            Entrance::SandSea_From_SkippersRetreat => "Sand Sea (from Skipper's Retreat)",
            Entrance::SkippersRetreat_From_SkippersShack => {
                "Skipper's Retreat (from Skipper's Shack)"
            }
            Entrance::LanayruDesert_From_LanayruSilentRealm => {
                "Lanayru Desert (from Lanayru Silent Realm)"
            }
            Entrance::EldinVolcano_From_MogmaTurf_EndVent => {
                "Eldin Volcano (from Mogma Turf) (End Vent)"
            }
            Entrance::EldinVolcano_From_MogmaTurf_StartVent => {
                "Eldin Volcano (from Mogma Turf) (Start Vent)"
            }
            Entrance::SandSea_From_Sandship => "Sand Sea (from Sandship)",
            Entrance::SandshipBoss_From_Sandship => "Sandship Boss (from Sandship)",
            Entrance::FaronWoods_From_BehindTheTemple => "Faron Woods (from Behind the Temple)",
            Entrance::SealedGroundsSpiral_From_BehindTheTemple => {
                "Sealed Grounds Spiral (from Behind the Temple)"
            }
            Entrance::SealedTemple_From_BehindTheTemple => "Sealed Temple (from Behind the Temple)",
            Entrance::Sky_From_BehindTheTemple_BehindTheTempleStatue => {
                "Sky (from Behind the Temple) (Behind the Temple Statue)"
            }
            Entrance::SealedTemple_From_SealedGroundsSpiral => {
                "Sealed Temple (from Sealed Grounds Spiral)"
            }
            Entrance::Sky_From_SealedGroundsSpiral_SealedGroundsStatue => {
                "Sky (from Sealed Grounds Spiral) (Sealed Grounds Statue)"
            }
            Entrance::BehindTheTemple_From_SealedTemple => "Behind the Temple (from Sealed Temple)",
            Entrance::HyliasTemple_From_SealedTemple => "Hylia's Temple (from Sealed Temple)",
            Entrance::SealedGroundsSpiral_From_SealedTemple => {
                "Sealed Grounds Spiral (from Sealed Temple)"
            }
            Entrance::Sky_From_InsideBambooIsland => "Sky (from Inside Bamboo Island)",
            Entrance::Sky_From_LumpyPumpkin_North => "Sky (from Lumpy Pumpkin) (North)",
            Entrance::Sky_From_LumpyPumpkin_South => "Sky (from Lumpy Pumpkin) (South)",
            Entrance::BeedlesShop_From_Sky_Night => "Beedle's Shop (from Sky) (Night)",
            Entrance::BehindTheTemple_From_Sky_BehindTheTempleStatue => {
                "Behind the Temple (from Sky) (Behind the Temple Statue)"
            }
            Entrance::DeepWoods_From_Sky_DeepWoodsStatue => {
                "Deep Woods (from Sky) (Deep Woods Statue)"
            }
            Entrance::DeepWoods_From_Sky_ForestTempleStatue => {
                "Deep Woods (from Sky) (Forest Temple Statue)"
            }
            Entrance::EldinVolcano_From_Sky_EldinEntranceStatue => {
                "Eldin Volcano (from Sky) (Eldin Entrance Statue)"
            }
            Entrance::EldinVolcano_From_Sky_TempleEntranceStatue => {
                "Eldin Volcano (from Sky) (Temple Entrance Statue)"
            }
            Entrance::EldinVolcano_From_Sky_VolcanoEastStatue => {
                "Eldin Volcano (from Sky) (Volcano East Statue)"
            }
            Entrance::EldinVolcano_From_Sky_VolcanoAscentStatue => {
                "Eldin Volcano (from Sky) (Volcano Ascent Statue)"
            }
            Entrance::FaronWoods_From_Sky_FaronWoodsEntryStatue => {
                "Faron Woods (from Sky) (Faron Woods Entry Statue)"
            }
            Entrance::FaronWoods_From_Sky_GreatTreeStatue => {
                "Faron Woods (from Sky) (Great Tree Statue)"
            }
            Entrance::FaronWoods_From_Sky_InTheWoodsStatue => {
                "Faron Woods (from Sky) (In the Woods Statue)"
            }
            Entrance::FaronWoods_From_Sky_ViewingPlatformStatue => {
                "Faron Woods (from Sky) (Viewing Platform Statue)"
            }
            Entrance::FloriaWaterfall_From_Sky_FloriaWaterfallStatue => {
                "Floria Waterfall (from Sky) (Floria Waterfall Statue)"
            }
            Entrance::InsideBambooIsland_From_Sky => "Inside Bamboo Island (from Sky)",
            Entrance::InsideThunderhead_From_Sky => "Inside Thunderhead (from Sky)",
            Entrance::LakeFloria_From_Sky_LakeFloriaStatue => {
                "Lake Floria (from Sky) (Lake Floria Statue)"
            }
            Entrance::LanayruDesert_From_Sky_DesertEntranceStatue => {
                "Lanayru Desert (from Sky) (Desert Entrance Statue)"
            }
            Entrance::LanayruDesert_From_Sky_NorthDesertStatue => {
                "Lanayru Desert (from Sky) (North Desert Statue)"
            }
            Entrance::LanayruDesert_From_Sky_StoneCacheStatue => {
                "Lanayru Desert (from Sky) (Stone Cache Statue)"
            }
            Entrance::LanayruDesert_From_Sky_WestDesertStatue => {
                "Lanayru Desert (from Sky) (West Desert Statue)"
            }
            Entrance::LanayruMines_From_Sky_LanayruMineEntryStatue => {
                "Lanayru Mines (from Sky) (Lanayru Mine Entry Statue)"
            }
            Entrance::OutsideFireSanctuary_From_Sky_InsideTheVolcanoStatue => {
                "Outside Fire Sanctuary (from Sky) (Inside the Volcano Statue)"
            }
            Entrance::SealedGroundsSpiral_From_Sky_SealedGroundsStatue => {
                "Sealed Grounds Spiral (from Sky) (Sealed Grounds Statue)"
            }
            Entrance::Skyloft_From_Sky => "Skyloft (from Sky)",
            Entrance::LumpyPumpkin_From_Sky_North => "Lumpy Pumpkin (from Sky) (North)",
            Entrance::LumpyPumpkin_From_Sky_South => "Lumpy Pumpkin (from Sky) (South)",
            Entrance::Skyloft_From_SkyKeepEntry => "Skyloft (from Sky Keep Entry)",
            Entrance::Skyloft_From_SkyloftSilentRealm => "Skyloft (from Skyloft Silent Realm)",
            Entrance::Skyloft_From_BertiesHouse => "Skyloft (from Bertie's House)",
            Entrance::Skyloft_From_GondosHouse => "Skyloft (from Gondo's House)",
            Entrance::Skyloft_From_MallarasHouse => "Skyloft (from Mallara's House)",
            Entrance::Skyloft_From_RupinsHouse => "Skyloft (from Rupin's House)",
            Entrance::BatreauxHouse_From_Skyloft => "Batreaux' House (from Skyloft)",
            Entrance::BertiesHouse_From_Skyloft => "Bertie's House (from Skyloft)",
            Entrance::GondosHouse_From_Skyloft => "Gondo's House (from Skyloft)",
            Entrance::MallarasHouse_From_Skyloft => "Mallara's House (from Skyloft)",
            Entrance::RupinsHouse_From_Skyloft => "Rupin's House (from Skyloft)",
            Entrance::SparrotsHouse_From_Skyloft => "Sparrot's House (from Skyloft)",
            Entrance::Skyloft_From_SparrotsHouse => "Skyloft (from Sparrot's House)",
            Entrance::SkyviewSpring_From_SkyviewBoss => "Skyview Spring (from Skyview Boss)",
            Entrance::SkyviewTemple_From_SkyviewBoss => "Skyview Temple (from Skyview Boss)",
            Entrance::DeepWoods_From_SkyviewSpring => "Deep Woods (from Skyview Spring)",
            Entrance::SkyviewBoss_From_SkyviewSpring => "Skyview Boss (from Skyview Spring)",
            Entrance::SkyviewBoss_From_SkyviewTemple => "Skyview Boss (from Skyview Temple)",
            Entrance::DeepWoods_From_SkyviewTemple => "Deep Woods (from Skyview Temple)",
            Entrance::IsleOfSongs_From_InsideThunderhead => {
                "Isle of Songs (from Inside Thunderhead)"
            }
            Entrance::Sky_From_InsideThunderhead => "Sky (from Inside Thunderhead)",
            Entrance::InsideThunderhead_From_IsleOfSongs => {
                "Inside Thunderhead (from Isle of Songs)"
            }
            Entrance::EldinVolcano_From_InsideVolcanoSummit => {
                "Eldin Volcano (from Inside Volcano Summit)"
            }
            Entrance::OutsideFireSanctuary_From_InsideVolcanoSummit => {
                "Outside Fire Sanctuary (from Inside Volcano Summit)"
            }
            Entrance::VolcanoSummitWaterfall_From_InsideVolcanoSummit => {
                "Volcano Summit Waterfall (from Inside Volcano Summit)"
            }
            Entrance::FireSanctuaryA_From_OutsideFireSanctuary => {
                "Fire Sanctuary A (from Outside Fire Sanctuary)"
            }
            Entrance::Sky_From_OutsideFireSanctuary_InsideTheVolcanoStatue => {
                "Sky (from Outside Fire Sanctuary) (Inside the Volcano Statue)"
            }
            Entrance::InsideVolcanoSummit_From_OutsideFireSanctuary => {
                "Inside Volcano Summit (from Outside Fire Sanctuary)"
            }
            Entrance::InsideVolcanoSummit_From_VolcanoSummitWaterfall => {
                "Inside Volcano Summit (from Volcano Summit Waterfall)"
            }
        }
    }
}
impl Entrance {
    pub fn area(&self) -> Area {
        match self {
            Entrance::AncientCisternBoss_From_AncientCistern => Area::AncientCisternBoss_Main,
            Entrance::FloriaWaterfall_From_AncientCistern => Area::FloriaWaterfall_Main,
            Entrance::AncientCisternCandleRoom_From_AncientCisternBoss => {
                Area::AncientCisternCandleRoom_Main
            }
            Entrance::Skyloft_From_BatreauxHouse => Area::Skyloft_CentralOutside,
            Entrance::Sky_From_BeedlesShop_Night => Area::Sky_BeedlesSkyHome,
            Entrance::Skyloft_From_BeedlesShop_Day => Area::Skyloft_CentralOutside,
            Entrance::Skyloft_From_Bazaar_North => Area::Skyloft_CentralOutside,
            Entrance::Skyloft_From_Bazaar_South => Area::Skyloft_CentralOutside,
            Entrance::Skyloft_From_Bazaar_West => Area::Skyloft_CentralOutside,
            Entrance::Skyloft_From_ParrowAndOriellesHouse => Area::Skyloft_CentralOutside,
            Entrance::Skyloft_From_PeatricesHouse => Area::Skyloft_CentralOutside,
            Entrance::Skyloft_From_PipersHouse => Area::Skyloft_CentralOutside,
            Entrance::Bazaar_From_Skyloft_North => Area::Bazaar_Main,
            Entrance::Bazaar_From_Skyloft_South => Area::Bazaar_Main,
            Entrance::Bazaar_From_Skyloft_West => Area::Bazaar_Main,
            Entrance::BeedlesShop_From_Skyloft_Day => Area::BeedlesShop_Main,
            Entrance::ParrowAndOriellesHouse_From_Skyloft => Area::ParrowAndOriellesHouse_Main,
            Entrance::PeatricesHouse_From_Skyloft => Area::PeatricesHouse_Main,
            Entrance::PipersHouse_From_Skyloft => Area::PipersHouse_Main,
            Entrance::Sky_From_Skyloft => Area::Sky_Field,
            Entrance::SkyloftSilentRealm_From_Skyloft => Area::SkyloftSilentRealm_Trial,
            Entrance::WaterfallCave_From_Skyloft_Upper => Area::WaterfallCave_Main,
            Entrance::WrynasHouse_From_Skyloft => Area::WrynasHouse_Main,
            Entrance::Sky_From_Skyloft_PastWaterfallCave => Area::Sky_Field,
            Entrance::WaterfallCave_From_Skyloft_Lower => Area::WaterfallCave_Main,
            Entrance::SkyKeepEntry_From_Skyloft => Area::SkyKeepEntry_Main,
            Entrance::Skyloft_From_WaterfallCave_Upper => Area::Skyloft_CentralOutside,
            Entrance::Skyloft_From_WaterfallCave_Lower => Area::Skyloft_PastWaterfallCave,
            Entrance::Skyloft_From_WrynasHouse => Area::Skyloft_CentralOutside,
            Entrance::EarthTempleBoss_From_EarthTemple => Area::EarthTempleBoss_Main,
            Entrance::EldinVolcano_From_EarthTemple => Area::EldinVolcano_OutsideEt,
            Entrance::EarthTempleSpring_From_EarthTempleBoss => Area::EarthTempleSpring_Main,
            Entrance::EldinVolcano_From_EarthTempleSpring => Area::EldinVolcano_OutsideEt,
            Entrance::EldinVolcano_From_EldinSilentRealm => Area::EldinVolcano_VolcanoAscent,
            Entrance::Sky_From_EldinVolcano_EldinEntranceStatue => Area::Sky_Field,
            Entrance::InsideVolcanoSummit_From_EldinVolcano => Area::InsideVolcanoSummit_Main,
            Entrance::ThrillDiggerCave_From_EldinVolcano => Area::ThrillDiggerCave_Main,
            Entrance::EarthTemple_From_EldinVolcano => Area::EarthTemple_Entrance,
            Entrance::Sky_From_EldinVolcano_TempleEntranceStatue => Area::Sky_Field,
            Entrance::MogmaTurf_From_EldinVolcano_Skydive => Area::MogmaTurf_Main,
            Entrance::Sky_From_EldinVolcano_VolcanoEastStatue => Area::Sky_Field,
            Entrance::EldinSilentRealm_From_EldinVolcano => Area::EldinSilentRealm_Trial,
            Entrance::Sky_From_EldinVolcano_VolcanoAscentStatue => Area::Sky_Field,
            Entrance::EldinVolcano_From_ThrillDiggerCave => Area::EldinVolcano_NearThrillDigger,
            Entrance::FaronWoods_From_FaronSilentRealm => Area::FaronWoods_Main,
            Entrance::FaronWoods_From_DeepWoods => Area::FaronWoods_Main,
            Entrance::Sky_From_DeepWoods_DeepWoodsStatue => Area::Sky_Field,
            Entrance::Sky_From_DeepWoods_ForestTempleStatue => Area::Sky_Field,
            Entrance::SkyviewTemple_From_DeepWoods => Area::SkyviewTemple_Entry,
            Entrance::BehindTheTemple_From_FaronWoods => Area::BehindTheTemple_Main,
            Entrance::Sky_From_FaronWoods_FaronWoodsEntryStatue => Area::Sky_Field,
            Entrance::GreatTree_From_FaronWoods_LowerPlatform => Area::GreatTree_Lower,
            Entrance::GreatTree_From_FaronWoods_UpperPlatform => Area::GreatTree_Upper,
            Entrance::GreatTree_From_FaronWoods_Top => Area::GreatTree_Upper,
            Entrance::Sky_From_FaronWoods_GreatTreeStatue => Area::Sky_Field,
            Entrance::DeepWoods_From_FaronWoods => Area::DeepWoods_Entry,
            Entrance::FaronSilentRealm_From_FaronWoods => Area::FaronSilentRealm_Trial,
            Entrance::GreatTree_From_FaronWoods_Tunnel => Area::GreatTree_Entry,
            Entrance::LakeFloria_From_FaronWoods => Area::LakeFloria_Entry,
            Entrance::Sky_From_FaronWoods_InTheWoodsStatue => Area::Sky_Field,
            Entrance::Sky_From_FaronWoods_ViewingPlatformStatue => Area::Sky_Field,
            Entrance::FaronWoods_From_GreatTree_Tunnel => Area::FaronWoods_Main,
            Entrance::FaronWoods_From_GreatTree_LowerPlatform => {
                Area::FaronWoods_GreatTreePlatforms
            }
            Entrance::FaronWoods_From_GreatTree_UpperPlatform => {
                Area::FaronWoods_GreatTreePlatforms
            }
            Entrance::FaronWoods_From_GreatTree_Top => Area::FaronWoods_GreatTreeTop,
            Entrance::OutsideFireSanctuary_From_FireSanctuaryA => {
                Area::OutsideFireSanctuary_ToFireSanctuary
            }
            Entrance::FireSanctuaryBoss_From_FireSanctuaryA => Area::FireSanctuaryBoss_Main,
            Entrance::FireSanctuaryB_From_FireSanctuaryA => {
                Area::FireSanctuaryB_FirstOutsideSection
            }
            Entrance::FireSanctuaryA_From_FireSanctuaryB => Area::FireSanctuaryA_PrePlatsArea,
            Entrance::FireSanctuaryFlameRoom_From_FireSanctuaryBoss => {
                Area::FireSanctuaryFlameRoom_Main
            }
            Entrance::Skyloft_From_InsideGoddessStatue => Area::Skyloft_OutsideGoddessStatue,
            Entrance::Skyloft_From_KnightAcademy_LowerDoors => Area::Skyloft_OutsideGoddessStatue,
            Entrance::Skyloft_From_KnightAcademy_UpperDoors => Area::Skyloft_OutsideGoddessStatue,
            Entrance::InsideGoddessStatue_From_Skyloft => Area::InsideGoddessStatue_Main,
            Entrance::KnightAcademy_From_Skyloft_Chimney => Area::KnightAcademy_AboveZeldasRoom,
            Entrance::KnightAcademy_From_Skyloft_LowerDoors => Area::KnightAcademy_Main,
            Entrance::KnightAcademy_From_Skyloft_UpperDoors => Area::KnightAcademy_Main,
            Entrance::SparringHall_From_Skyloft => Area::SparringHall_Main,
            Entrance::Skyloft_From_SparringHall => Area::Skyloft_OutsideGoddessStatue,
            Entrance::FloriaWaterfall_From_FaroresLair => Area::FloriaWaterfall_Main,
            Entrance::LakeFloria_From_FaroresLair => Area::LakeFloria_ToFaroresLair,
            Entrance::AncientCistern_From_FloriaWaterfall => Area::AncientCistern_MainHub,
            Entrance::FaronWoods_From_FloriaWaterfall => Area::FaronWoods_Main,
            Entrance::FaroresLair_From_FloriaWaterfall => Area::FaroresLair_Main,
            Entrance::Sky_From_FloriaWaterfall_FloriaWaterfallStatue => Area::Sky_Field,
            Entrance::Sky_From_LakeFloria_LakeFloriaStatue => Area::Sky_Field,
            Entrance::FaroresLair_From_LakeFloria => Area::FaroresLair_Main,
            Entrance::LanayruDesert_From_LanayruCaves => Area::LanayruDesert_SandOasis,
            Entrance::LanayruMines_From_LanayruCaves => Area::LanayruMines_ToCaves,
            Entrance::SandSeaDocks_From_LanayruCaves => Area::SandSeaDocks_ToCaves,
            Entrance::LanayruDesert_From_FireNode => Area::LanayruDesert_PastToT,
            Entrance::LanayruMines_From_LanayruDesert => Area::LanayruMines_ToDesert,
            Entrance::Sky_From_LanayruDesert_DesertEntranceStatue => Area::Sky_Field,
            Entrance::FireNode_From_LanayruDesert => Area::FireNode_Main,
            Entrance::LanayruMiningFacilityA_From_LanayruDesert => {
                Area::LanayruMiningFacilityA_Entry
            }
            Entrance::LanayruSilentRealm_From_LanayruDesert => Area::LanayruSilentRealm_Trial,
            Entrance::LightningNode_From_LanayruDesert => Area::LightningNode_Main,
            Entrance::Sky_From_LanayruDesert_NorthDesertStatue => Area::Sky_Field,
            Entrance::Sky_From_LanayruDesert_StoneCacheStatue => Area::Sky_Field,
            Entrance::TempleOfTime_From_LanayruDesert_End => Area::TempleOfTime_End,
            Entrance::LanayruCaves_From_LanayruDesert => Area::LanayruCaves_Main,
            Entrance::Sky_From_LanayruDesert_WestDesertStatue => Area::Sky_Field,
            Entrance::TempleOfTime_From_LanayruDesert_Start => Area::TempleOfTime_Start,
            Entrance::LanayruDesert_From_LightningNode => Area::LanayruDesert_PastToT,
            Entrance::LanayruDesert_From_TempleOfTime_End => Area::LanayruDesert_PastToT,
            Entrance::LanayruDesert_From_TempleOfTime_Start => Area::LanayruDesert_SandOasis,
            Entrance::Sky_From_LanayruMines_LanayruMineEntryStatue => Area::Sky_Field,
            Entrance::LanayruCaves_From_LanayruMines => Area::LanayruCaves_Main,
            Entrance::LanayruDesert_From_LanayruMines => Area::LanayruDesert_HookBeetleArea,
            Entrance::LanayruDesert_From_LanayruMiningFacilityA => Area::LanayruDesert_PastToT,
            Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub2 => {
                Area::LanayruMiningFacilityB_NearFirstHubRoomChest
            }
            Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_HubW => {
                Area::LanayruMiningFacilityB_WestHub
            }
            Entrance::LanayruMiningFacilityB_From_LanayruMiningFacilityA_Hub => {
                Area::LanayruMiningFacilityB_HubRoom
            }
            Entrance::LanayruMiningFacilityBoss_From_LanayruMiningFacilityB => {
                Area::LanayruMiningFacilityBoss_Main
            }
            Entrance::LanayruMiningFacilityToToT_From_LanayruMiningFacilityBoss => {
                Area::LanayruMiningFacilityToToT_BossDoor
            }
            Entrance::TempleOfTime_From_LanayruMiningFacilityToToT => Area::TempleOfTime_AfterLmf,
            Entrance::OutsidePiratesStronghold_From_InsidePiratesStronghold_End => {
                Area::OutsidePiratesStronghold_InsideSharkhead
            }
            Entrance::OutsidePiratesStronghold_From_InsidePiratesStronghold_Beginning => {
                Area::OutsidePiratesStronghold_Main
            }
            Entrance::InsidePiratesStronghold_From_OutsidePiratesStronghold_End => {
                Area::InsidePiratesStronghold_Main
            }
            Entrance::InsidePiratesStronghold_From_OutsidePiratesStronghold_Beginning => {
                Area::InsidePiratesStronghold_Main
            }
            Entrance::SandSea_From_OutsidePiratesStronghold => Area::SandSea_Main,
            Entrance::OutsidePiratesStronghold_From_SandSea => Area::OutsidePiratesStronghold_Main,
            Entrance::SandSeaDocks_From_SandSea => Area::SandSeaDocks_Main,
            Entrance::Sandship_From_SandSea => Area::Sandship_Deck,
            Entrance::Shipyard_From_SandSea => Area::Shipyard_Main,
            Entrance::SkippersRetreat_From_SandSea => Area::SkippersRetreat_Start,
            Entrance::SandSea_From_SandSeaDocks => Area::SandSea_Main,
            Entrance::Sky_From_SandSeaDocks_AncientHarbor => Area::Sky_Field,
            Entrance::LanayruCaves_From_SandSeaDocks => Area::LanayruCaves_ToSandSea,
            Entrance::ShipyardConstructionBay_From_Shipyard_Upper => {
                Area::ShipyardConstructionBay_Upper
            }
            Entrance::SandSea_From_Shipyard => Area::SandSea_Main,
            Entrance::ShipyardConstructionBay_From_Shipyard_Lower => {
                Area::ShipyardConstructionBay_Lower
            }
            Entrance::Shipyard_From_ShipyardConstructionBay_Lower => Area::Shipyard_Main,
            Entrance::Shipyard_From_ShipyardConstructionBay_Upper => {
                Area::Shipyard_AfterMinecartRide
            }
            Entrance::SkippersShack_From_SkippersRetreat => Area::SkippersShack_Main,
            Entrance::SandSea_From_SkippersRetreat => Area::SandSea_Main,
            Entrance::SkippersRetreat_From_SkippersShack => Area::SkippersRetreat_NextToShack,
            Entrance::LanayruDesert_From_LanayruSilentRealm => Area::LanayruDesert_PastToT,
            Entrance::EldinVolcano_From_MogmaTurf_EndVent => Area::EldinVolcano_PastMogmaTurf,
            Entrance::EldinVolcano_From_MogmaTurf_StartVent => Area::EldinVolcano_PreMogmaTurf,
            Entrance::SandSea_From_Sandship => Area::SandSea_Main,
            Entrance::SandshipBoss_From_Sandship => Area::SandshipBoss_Main,
            Entrance::FaronWoods_From_BehindTheTemple => Area::FaronWoods_Entry,
            Entrance::SealedGroundsSpiral_From_BehindTheTemple => Area::SealedGroundsSpiral_Upper,
            Entrance::SealedTemple_From_BehindTheTemple => Area::SealedTemple_Main,
            Entrance::Sky_From_BehindTheTemple_BehindTheTempleStatue => Area::Sky_Field,
            Entrance::SealedTemple_From_SealedGroundsSpiral => Area::SealedTemple_Main,
            Entrance::Sky_From_SealedGroundsSpiral_SealedGroundsStatue => Area::Sky_Field,
            Entrance::BehindTheTemple_From_SealedTemple => Area::BehindTheTemple_Main,
            Entrance::HyliasTemple_From_SealedTemple => Area::HyliasTemple_Main,
            Entrance::SealedGroundsSpiral_From_SealedTemple => Area::SealedGroundsSpiral_Lower,
            Entrance::Sky_From_InsideBambooIsland => Area::Sky_Field,
            Entrance::Sky_From_LumpyPumpkin_North => Area::Sky_OutsideLumpyPumpkin,
            Entrance::Sky_From_LumpyPumpkin_South => Area::Sky_OutsideLumpyPumpkin,
            Entrance::BeedlesShop_From_Sky_Night => Area::BeedlesShop_Main,
            Entrance::BehindTheTemple_From_Sky_BehindTheTempleStatue => Area::BehindTheTemple_Main,
            Entrance::DeepWoods_From_Sky_DeepWoodsStatue => Area::DeepWoods_PastBeehive,
            Entrance::DeepWoods_From_Sky_ForestTempleStatue => Area::DeepWoods_PastBeehive,
            Entrance::EldinVolcano_From_Sky_EldinEntranceStatue => Area::EldinVolcano_FirstRoom,
            Entrance::EldinVolcano_From_Sky_TempleEntranceStatue => Area::EldinVolcano_OutsideEt,
            Entrance::EldinVolcano_From_Sky_VolcanoEastStatue => Area::EldinVolcano_PreMogmaTurf,
            Entrance::EldinVolcano_From_Sky_VolcanoAscentStatue => Area::EldinVolcano_VolcanoAscent,
            Entrance::FaronWoods_From_Sky_FaronWoodsEntryStatue => Area::FaronWoods_Entry,
            Entrance::FaronWoods_From_Sky_GreatTreeStatue => Area::FaronWoods_GreatTreeTop,
            Entrance::FaronWoods_From_Sky_InTheWoodsStatue => Area::FaronWoods_Main,
            Entrance::FaronWoods_From_Sky_ViewingPlatformStatue => Area::FaronWoods_Main,
            Entrance::FloriaWaterfall_From_Sky_FloriaWaterfallStatue => Area::FloriaWaterfall_Main,
            Entrance::InsideBambooIsland_From_Sky => Area::InsideBambooIsland_Main,
            Entrance::InsideThunderhead_From_Sky => Area::InsideThunderhead_Main,
            Entrance::LakeFloria_From_Sky_LakeFloriaStatue => Area::LakeFloria_StatueSpot,
            Entrance::LanayruDesert_From_Sky_DesertEntranceStatue => {
                Area::LanayruDesert_HookBeetleArea
            }
            Entrance::LanayruDesert_From_Sky_NorthDesertStatue => Area::LanayruDesert_PastToT,
            Entrance::LanayruDesert_From_Sky_StoneCacheStatue => Area::LanayruDesert_PastToT,
            Entrance::LanayruDesert_From_Sky_WestDesertStatue => Area::LanayruDesert_SandOasis,
            Entrance::LanayruMines_From_Sky_LanayruMineEntryStatue => Area::LanayruMines_FirstHalf,
            Entrance::OutsideFireSanctuary_From_Sky_InsideTheVolcanoStatue => {
                Area::OutsideFireSanctuary_ToFireSanctuary
            }
            Entrance::SealedGroundsSpiral_From_Sky_SealedGroundsStatue => {
                Area::SealedGroundsSpiral_Upper
            }
            Entrance::Skyloft_From_Sky => Area::Skyloft_CentralOutside,
            Entrance::LumpyPumpkin_From_Sky_North => Area::LumpyPumpkin_Main,
            Entrance::LumpyPumpkin_From_Sky_South => Area::LumpyPumpkin_Main,
            Entrance::Skyloft_From_SkyKeepEntry => Area::Skyloft_ToSkyKeep,
            Entrance::Skyloft_From_SkyloftSilentRealm => Area::Skyloft_CentralOutside,
            Entrance::Skyloft_From_BertiesHouse => Area::Skyloft_CentralOutside,
            Entrance::Skyloft_From_GondosHouse => Area::Skyloft_CentralOutside,
            Entrance::Skyloft_From_MallarasHouse => Area::Skyloft_CentralOutside,
            Entrance::Skyloft_From_RupinsHouse => Area::Skyloft_CentralOutside,
            Entrance::BatreauxHouse_From_Skyloft => Area::BatreauxHouse_Main,
            Entrance::BertiesHouse_From_Skyloft => Area::BertiesHouse_Main,
            Entrance::GondosHouse_From_Skyloft => Area::GondosHouse_Main,
            Entrance::MallarasHouse_From_Skyloft => Area::MallarasHouse_Main,
            Entrance::RupinsHouse_From_Skyloft => Area::RupinsHouse_Main,
            Entrance::SparrotsHouse_From_Skyloft => Area::SparrotsHouse_Main,
            Entrance::Skyloft_From_SparrotsHouse => Area::Skyloft_CentralOutside,
            Entrance::SkyviewSpring_From_SkyviewBoss => Area::SkyviewSpring_Main,
            Entrance::SkyviewTemple_From_SkyviewBoss => Area::SkyviewTemple_BossDoorArea,
            Entrance::DeepWoods_From_SkyviewSpring => Area::DeepWoods_PastBeehive,
            Entrance::SkyviewBoss_From_SkyviewSpring => Area::SkyviewBoss_Main,
            Entrance::SkyviewBoss_From_SkyviewTemple => Area::SkyviewBoss_Main,
            Entrance::DeepWoods_From_SkyviewTemple => Area::DeepWoods_PastBeehive,
            Entrance::IsleOfSongs_From_InsideThunderhead => Area::IsleOfSongs_Main,
            Entrance::Sky_From_InsideThunderhead => Area::Sky_Field,
            Entrance::InsideThunderhead_From_IsleOfSongs => Area::InsideThunderhead_Main,
            Entrance::EldinVolcano_From_InsideVolcanoSummit => Area::EldinVolcano_HotCaveArea,
            Entrance::OutsideFireSanctuary_From_InsideVolcanoSummit => {
                Area::OutsideFireSanctuary_ToInsideSummit
            }
            Entrance::VolcanoSummitWaterfall_From_InsideVolcanoSummit => {
                Area::VolcanoSummitWaterfall_Main
            }
            Entrance::FireSanctuaryA_From_OutsideFireSanctuary => Area::FireSanctuaryA_Entry,
            Entrance::Sky_From_OutsideFireSanctuary_InsideTheVolcanoStatue => Area::Sky_Field,
            Entrance::InsideVolcanoSummit_From_OutsideFireSanctuary => {
                Area::InsideVolcanoSummit_Main
            }
            Entrance::InsideVolcanoSummit_From_VolcanoSummitWaterfall => {
                Area::InsideVolcanoSummit_Main
            }
        }
    }
}
