#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
GoddessHarp,
ProgressiveBow,
Clawshots,
SpiralCharge,
AncientCisternBossKey,
FireSanctuaryBossKey,
SandshipBossKey,
SkyviewBossKey,
EarthTempleBossKey,
LanayruMiningFacilityBossKey,
GustBellows,
ProgressiveSlingshot,
WaterScale,
BombBag,
TriforceOfCourage,
TriforceOfPower,
TriforceOfWisdom,
SeaChart,
ProgressivePouch,
Whip,
FireshieldEarrings,
EmptyBottle,
CawlinsLetter,
HornedColossusBeetle,
BabyRattle,
EmeraldTablet,
RubyTablet,
AmberTablet,
StoneOfTrials,
BalladOfTheGoddess,
FaroresCourage,
NayrusWisdom,
DinsPower,
FaronSongOfTheHeroPart,
EldinSongOfTheHeroPart,
LanayruSongOfTheHeroPart,
LifeTreeSeedling,
LifeTreeFruit,
ProgressiveSword,
KeyPiece,
GratitudeCrystalPack,
GratitudeCrystal,
ProgressiveBeetle,
ProgressiveMitts,
ProgressiveBugNet,
ProgressiveWallet,
ExtraWallet,
SkyviewSmallKey,
LanayruMiningFacilitySmallKey,
AncientCisternSmallKey,
FireSanctuarySmallKey,
SandshipSmallKey,
SkyKeepSmallKey,
LanayruCavesSmallKey,
GreenRupee,
BlueRupee,
RedRupee,
SilverRupee,
GoldRupee,
Rupoor,
X5Bombs,
X10Bombs,
DekuSeeds10,
SemiRareTreasure,
RareTreasure,
BugMedal,
HeartContainer,
HeartPiece,
HeartMedal,
RupeeMedal,
TreasureMedal,
PotionMedal,
CursedMedal,
LifeMedal,
WoodenShield,
HylianShield,
SmallSeedSatchel,
SmallQuiver,
SmallBombBag,
Tumbleweed,
EldinOre,
DuskRelic,
MonsterHorn,
EvilCrystal,
GoldenSkull,
GoddessPlume,
SkyviewMap,
EarthTempleMap,
LanayruMiningFacilityMap,
AncientCisternMap,
FireSanctuaryMap,
SandshipMap,
SkyKeepMap,
}
impl Item {
pub const ALL: &'static [Item] = &[
Item::GoddessHarp,
Item::ProgressiveBow,
Item::Clawshots,
Item::SpiralCharge,
Item::AncientCisternBossKey,
Item::FireSanctuaryBossKey,
Item::SandshipBossKey,
Item::SkyviewBossKey,
Item::EarthTempleBossKey,
Item::LanayruMiningFacilityBossKey,
Item::GustBellows,
Item::ProgressiveSlingshot,
Item::WaterScale,
Item::BombBag,
Item::TriforceOfCourage,
Item::TriforceOfPower,
Item::TriforceOfWisdom,
Item::SeaChart,
Item::ProgressivePouch,
Item::Whip,
Item::FireshieldEarrings,
Item::EmptyBottle,
Item::CawlinsLetter,
Item::HornedColossusBeetle,
Item::BabyRattle,
Item::EmeraldTablet,
Item::RubyTablet,
Item::AmberTablet,
Item::StoneOfTrials,
Item::BalladOfTheGoddess,
Item::FaroresCourage,
Item::NayrusWisdom,
Item::DinsPower,
Item::FaronSongOfTheHeroPart,
Item::EldinSongOfTheHeroPart,
Item::LanayruSongOfTheHeroPart,
Item::LifeTreeSeedling,
Item::LifeTreeFruit,
Item::ProgressiveSword,
Item::KeyPiece,
Item::GratitudeCrystalPack,
Item::GratitudeCrystal,
Item::ProgressiveBeetle,
Item::ProgressiveMitts,
Item::ProgressiveBugNet,
Item::ProgressiveWallet,
Item::ExtraWallet,
Item::SkyviewSmallKey,
Item::LanayruMiningFacilitySmallKey,
Item::AncientCisternSmallKey,
Item::FireSanctuarySmallKey,
Item::SandshipSmallKey,
Item::SkyKeepSmallKey,
Item::LanayruCavesSmallKey,
Item::GreenRupee,
Item::BlueRupee,
Item::RedRupee,
Item::SilverRupee,
Item::GoldRupee,
Item::Rupoor,
Item::X5Bombs,
Item::X10Bombs,
Item::DekuSeeds10,
Item::SemiRareTreasure,
Item::RareTreasure,
Item::BugMedal,
Item::HeartContainer,
Item::HeartPiece,
Item::HeartMedal,
Item::RupeeMedal,
Item::TreasureMedal,
Item::PotionMedal,
Item::CursedMedal,
Item::LifeMedal,
Item::WoodenShield,
Item::HylianShield,
Item::SmallSeedSatchel,
Item::SmallQuiver,
Item::SmallBombBag,
Item::Tumbleweed,
Item::EldinOre,
Item::DuskRelic,
Item::MonsterHorn,
Item::EvilCrystal,
Item::GoldenSkull,
Item::GoddessPlume,
Item::SkyviewMap,
Item::EarthTempleMap,
Item::LanayruMiningFacilityMap,
Item::AncientCisternMap,
Item::FireSanctuaryMap,
Item::SandshipMap,
Item::SkyKeepMap,
];
pub fn name(&self) -> &'static str {
match self {
Item::GoddessHarp => "Goddess Harp",
Item::ProgressiveBow => "Progressive Bow",
Item::Clawshots => "Clawshots",
Item::SpiralCharge => "Spiral Charge",
Item::AncientCisternBossKey => "Ancient Cistern Boss Key",
Item::FireSanctuaryBossKey => "Fire Sanctuary Boss Key",
Item::SandshipBossKey => "Sandship Boss Key",
Item::SkyviewBossKey => "Skyview Boss Key",
Item::EarthTempleBossKey => "Earth Temple Boss Key",
Item::LanayruMiningFacilityBossKey => "Lanayru Mining Facility Boss Key",
Item::GustBellows => "Gust Bellows",
Item::ProgressiveSlingshot => "Progressive Slingshot",
Item::WaterScale => "Water Scale",
Item::BombBag => "Bomb Bag",
Item::TriforceOfCourage => "Triforce of Courage",
Item::TriforceOfPower => "Triforce of Power",
Item::TriforceOfWisdom => "Triforce of Wisdom",
Item::SeaChart => "Sea Chart",
Item::ProgressivePouch => "Progressive Pouch",
Item::Whip => "Whip",
Item::FireshieldEarrings => "Fireshield Earrings",
Item::EmptyBottle => "Empty Bottle",
Item::CawlinsLetter => "Cawlin's Letter",
Item::HornedColossusBeetle => "Horned Colossus Beetle",
Item::BabyRattle => "Baby Rattle",
Item::EmeraldTablet => "Emerald Tablet",
Item::RubyTablet => "Ruby Tablet",
Item::AmberTablet => "Amber Tablet",
Item::StoneOfTrials => "Stone of Trials",
Item::BalladOfTheGoddess => "Ballad of the Goddess",
Item::FaroresCourage => "Farore's Courage",
Item::NayrusWisdom => "Nayru's Wisdom",
Item::DinsPower => "Din's Power",
Item::FaronSongOfTheHeroPart => "Faron Song of the Hero Part",
Item::EldinSongOfTheHeroPart => "Eldin Song of the Hero Part",
Item::LanayruSongOfTheHeroPart => "Lanayru Song of the Hero Part",
Item::LifeTreeSeedling => "Life Tree Seedling",
Item::LifeTreeFruit => "Life Tree Fruit",
Item::ProgressiveSword => "Progressive Sword",
Item::KeyPiece => "Key Piece",
Item::GratitudeCrystalPack => "Gratitude Crystal Pack",
Item::GratitudeCrystal => "Gratitude Crystal",
Item::ProgressiveBeetle => "Progressive Beetle",
Item::ProgressiveMitts => "Progressive Mitts",
Item::ProgressiveBugNet => "Progressive Bug Net",
Item::ProgressiveWallet => "Progressive Wallet",
Item::ExtraWallet => "Extra Wallet",
Item::SkyviewSmallKey => "Skyview Small Key",
Item::LanayruMiningFacilitySmallKey => "Lanayru Mining Facility Small Key",
Item::AncientCisternSmallKey => "Ancient Cistern Small Key",
Item::FireSanctuarySmallKey => "Fire Sanctuary Small Key",
Item::SandshipSmallKey => "Sandship Small Key",
Item::SkyKeepSmallKey => "Sky Keep Small Key",
Item::LanayruCavesSmallKey => "LanayruCaves Small Key",
Item::GreenRupee => "Green Rupee",
Item::BlueRupee => "Blue Rupee",
Item::RedRupee => "Red Rupee",
Item::SilverRupee => "Silver Rupee",
Item::GoldRupee => "Gold Rupee",
Item::Rupoor => "Rupoor",
Item::X5Bombs => "5 Bombs",
Item::X10Bombs => "10 Bombs",
Item::DekuSeeds10 => "Deku Seeds 10",
Item::SemiRareTreasure => "Semi Rare Treasure",
Item::RareTreasure => "Rare Treasure",
Item::BugMedal => "Bug Medal",
Item::HeartContainer => "Heart Container",
Item::HeartPiece => "Heart Piece",
Item::HeartMedal => "Heart Medal",
Item::RupeeMedal => "Rupee Medal",
Item::TreasureMedal => "Treasure Medal",
Item::PotionMedal => "Potion Medal",
Item::CursedMedal => "Cursed Medal",
Item::LifeMedal => "Life Medal",
Item::WoodenShield => "Wooden Shield",
Item::HylianShield => "Hylian Shield",
Item::SmallSeedSatchel => "Small Seed Satchel",
Item::SmallQuiver => "Small Quiver",
Item::SmallBombBag => "Small Bomb Bag",
Item::Tumbleweed => "Tumbleweed",
Item::EldinOre => "Eldin Ore",
Item::DuskRelic => "Dusk Relic",
Item::MonsterHorn => "Monster Horn",
Item::EvilCrystal => "Evil Crystal",
Item::GoldenSkull => "Golden Skull",
Item::GoddessPlume => "Goddess Plume",
Item::SkyviewMap => "Skyview Map",
Item::EarthTempleMap => "Earth Temple Map",
Item::LanayruMiningFacilityMap => "Lanayru Mining Facility Map",
Item::AncientCisternMap => "Ancient Cistern Map",
Item::FireSanctuaryMap => "Fire Sanctuary Map",
Item::SandshipMap => "Sandship Map",
Item::SkyKeepMap => "Sky Keep Map",
}
}
}
pub const SINGLE_ITEM_COUNT: usize = 38;
pub const COUNTED_ITEM_COUNT: usize = 16;
