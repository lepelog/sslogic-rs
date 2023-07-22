use logic_core::Item;

use Item::*;
pub const PROGRESS_ITEMS: &[(Item, u8)] = &[
    (BombBag, 1),
    (GustBellows, 1),
    (Whip, 1),
    (Clawshots, 1),
    (WaterScale, 1),
    (FireshieldEarrings, 1),
    (SeaChart, 1),
    (EmeraldTablet, 1),
    (RubyTablet, 1),
    (AmberTablet, 1),
    (StoneOfTrials, 1),
    (BabyRattle, 1),
    (CawlinsLetter, 1),
    (HornedColossusBeetle, 1),
    (GoddessHarp, 1),
    (BalladOfTheGoddess, 1),
    (FaroresCourage, 1),
    (NayrusWisdom, 1),
    (DinsPower, 1),
    (FaronSongOfTheHeroPart, 1),
    (EldinSongOfTheHeroPart, 1),
    (LanayruSongOfTheHeroPart, 1),
    (LifeTreeFruit, 1),
    (TriforceOfCourage, 1),
    (TriforceOfPower, 1),
    (TriforceOfWisdom, 1),
    (GratitudeCrystalPack, 13),
    (GratitudeCrystal, 15),
    (ProgressiveSword, 6),
    (ProgressiveMitts, 2),
    (ProgressiveSlingshot, 2),
    (ProgressiveBeetle, 4),
    (ProgressiveBow, 3),
    (ProgressiveBugNet, 2),
    (ProgressivePouch, 5),
    (KeyPiece, 5),
    (EmptyBottle, 5),
    (ProgressiveWallet, 4),
    (ExtraWallet, 3),
    (SpiralCharge, 1),
    // dungeon
    (SkyviewSmallKey, 2),
    (SkyviewBossKey, 1),
    (EarthTempleBossKey, 1),
    (LanayruMiningFacilitySmallKey, 1),
    (LanayruMiningFacilityBossKey, 1),
    (AncientCisternSmallKey, 2),
    (AncientCisternBossKey, 1),
    (SandshipSmallKey, 2),
    (SandshipBossKey, 1),
    (FireSanctuarySmallKey, 3),
    (FireSanctuaryBossKey, 1),
    (SkyKeepSmallKey, 1),
    (LanayruCavesSmallKey, 1),
];

pub fn add_item_pool(item_pool: &[(Item, u8)], items: &mut Vec<Item>) {
    for (item, count) in item_pool {
        for _ in 0..*count {
            items.push(*item);
        }
    }
}
