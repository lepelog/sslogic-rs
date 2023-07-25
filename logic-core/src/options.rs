#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartingSword {
    Swordless,
    PracticeSword,
    GoddessSword,
    GoddessLongsword,
    GoddessWhiteSword,
    MasterSword,
    TrueMasterSword,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LmfStartState {
    Open,
    MainNode,
    Nodes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriforceShuffle {
    Vanilla,
    SkyKeep,
    Anywhere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomizeEntrances {
    None,
    RequiredDungeonsSeparately,
    AllDungeons,
    AllDungeonsSkyKeep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RupoorMode {
    None,
    Added,
    Insanity,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub starting_tablet_count: u8,
    pub open_thunderhead: bool,
    pub starting_sword: StartingSword,
    pub required_dungeon_count: u8,
    pub imp2_skip: bool,
    pub empty_unrequired_dungeons: bool,
    pub triforce_required: bool,
    pub triforce_shuffle: TriforceShuffle,
    pub randomize_entrances: RandomizeEntrances,
    pub lmf_start_state: LmfStartState,
    pub beedles_shop_vanilla: bool,
}

// TODO
impl Options {
    pub fn new() -> Self {
        Options {
            starting_tablet_count: 0,
            open_thunderhead: false,
            starting_sword: StartingSword::GoddessSword,
            required_dungeon_count: 2,
            empty_unrequired_dungeons: true,
            imp2_skip: true,
            triforce_required: true,
            triforce_shuffle: TriforceShuffle::Anywhere,
            randomize_entrances: RandomizeEntrances::None,
            lmf_start_state: LmfStartState::Nodes,
            beedles_shop_vanilla: false,
        }
    }
}
