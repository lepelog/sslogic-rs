
use std::convert::TryFrom;

#[derive(Debug)]
pub enum Item {
    Slingshot,
    BombBag,
    GustBellows,
    Whip,
    Bow,
    BugNet,
    WaterScale,
    FireshieldEarrings,
    Clawshots,
    StoneOfTrials,
    SeaChart,
    EmeraldTablet,
    RubyTablet,
    AmberTablet,
    BabyRattle,
    CawlinsLetter,
    HornedColossusBeetle,
    GoddessHarp,
    BalladOfTheGoddess,
    FaroresCourage,
    NayrusWisdom,
    DinsPower,
    FaronSongOfTheHeroPart,
    EldinSongOfTheHeroPart,
    LanayruSongOfTheHeroPart,
    SpiralCharge,
    GratitudeCrystalPack,
    GratitudeCrystal,
    ProgressiveSword,
    ProgressiveMitts,
    ProgressiveBeetle,
    ProgressivePouch,
    KeyPiece,
    EmptyBottle,
    ProgressiveWallet,
    ExtraWallet,
    SVSmallKey,
    SVBossKey,
    ETBossKey,
    LMFSmallKey,
    LMFBossKey,
    ACSmallKey,
    ACBossKey,
    SSHSmallKey,
    SSHBossKey,
    FSSmallKey,
    FSBossKey,
    SKSmallKey,
    CavesSmallKey,
}

pub enum ItemRequirement {
    SingleItem(Item),
    CountedItem(Item, usize),
}

impl TryFrom<&str> for Item {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Slingshot" => Ok(Item::Slingshot),
            "Bomb Bag" => Ok(Item::BombBag),
            "Gust Bellows" => Ok(Item::GustBellows),
            "Whip" => Ok(Item::Whip),
            "Bow" => Ok(Item::Bow),
            "Bug Net" => Ok(Item::BugNet),
            "Water Scale" => Ok(Item::WaterScale),
            "Fireshield Earrings" => Ok(Item::FireshieldEarrings),
            "Clawshots" => Ok(Item::Clawshots),
            "Stone of Trials" => Ok(Item::StoneOfTrials),
            "Sea Chart" => Ok(Item::SeaChart),
            "Emerald Tablet" => Ok(Item::EmeraldTablet),
            "Ruby Tablet" => Ok(Item::RubyTablet),
            "Amber Tablet" => Ok(Item::AmberTablet),
            "Baby Rattle" => Ok(Item::BabyRattle),
            "Cawlin's Letter" => Ok(Item::CawlinsLetter),
            "Horned Colossus Beetle" => Ok(Item::HornedColossusBeetle),
            "Goddess Harp" => Ok(Item::GoddessHarp),
            "Ballad of the Goddess" => Ok(Item::BalladOfTheGoddess),
            "Farore's Courage" => Ok(Item::FaroresCourage),
            "Nayru's Wisdom" => Ok(Item::NayrusWisdom),
            "Din's Power" => Ok(Item::DinsPower),
            "Faron Song of the Hero Part" => Ok(Item::FaronSongOfTheHeroPart),
            "Eldin Song of the Hero Part" => Ok(Item::EldinSongOfTheHeroPart),
            "Lanayru Song of the Hero Part" => Ok(Item::LanayruSongOfTheHeroPart),
            "Spiral Charge" => Ok(Item::SpiralCharge),
            "5 Gratitude Crystals" => Ok(Item::GratitudeCrystalPack),
            "Gratitude Crystal" => Ok(Item::GratitudeCrystal),
            "Progressive Sword" => Ok(Item::ProgressiveSword),
            "Progressive Mitts" => Ok(Item::ProgressiveMitts),
            "Progressive Beetle" => Ok(Item::ProgressiveBeetle),
            "Progressive Pouch" => Ok(Item::ProgressivePouch),
            "Key Piece" => Ok(Item::KeyPiece),
            "Empty Bottle" => Ok(Item::EmptyBottle),
            "Progressive Wallet" => Ok(Item::ProgressiveWallet),
            "Extra Wallet" => Ok(Item::ExtraWallet),
            "SV Small Key" => Ok(Item::SVSmallKey),
            "SV Boss Key" => Ok(Item::SVBossKey),
            "ET Boss Key" => Ok(Item::ETBossKey),
            "LMF Small Key" => Ok(Item::LMFSmallKey),
            "LMF Boss Key" => Ok(Item::LMFBossKey),
            "AC Small Key" => Ok(Item::ACSmallKey),
            "AC Boss Key" => Ok(Item::ACBossKey),
            "SSH Small Key" => Ok(Item::SSHSmallKey),
            "SSH Boss Key" => Ok(Item::SSHBossKey),
            "FS Small Key" => Ok(Item::FSSmallKey),
            "FS Boss Key" => Ok(Item::FSBossKey),
            "SK Small Key" => Ok(Item::SKSmallKey),
            "LanayruCaves Small Key" => Ok(Item::CavesSmallKey),
            _ => Err("not a valid item!")
        }
    }
}