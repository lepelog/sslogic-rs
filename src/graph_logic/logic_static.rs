use std::collections::{BTreeMap, HashMap};

use super::logic_structs::{
    AllowedToD, AreaKey, Entrance, Exit, Inventory, LocationKey, LogicKey, LogicKeyMapper,
    PassagewayKey,
};

pub struct Placement {
    pub start_items: Inventory<'static>,
    pub passageways: HashMap<Exit, Entrance>,
    pub locations: HashMap<LocationKey, LogicKey>,
}

/// information to allow patching an item to this
/// location
pub enum PatchTarget {
    AddOarc {
        stage: String,
        layer: u8,
    },
    TBox {
        stage: String,
        layer: u8,
        room: u8,
        chestflag: u8,
    },
}

pub struct Location {
    name: String,
    key: LogicKey,
    area_key: AreaKey,
}

pub enum SpecializedLocation {
    // currently, there is no restriction for locations holding certain items
    // types, patch targets
    ItemLocation { patch_targets: Vec<PatchTarget> },
    EventLocation,
}

/// An area represents
pub struct Area {
    name: String,
    key: AreaKey,
    stage_key: LogicKey,
    /// sometimes it's necessary to overwrite
    /// the region on the area level, e.g. outside Skyloft
    region: Option<String>,
    allowed_tod: AllowedToD,
    exits: Vec<PassagewayKey>,
    entrances: Vec<PassagewayKey>,
}

/// A Stage maps 1:1 to an ingame stage
/// A Stage can have multiple areas, but each area belongs to exactly one stage
pub struct Stage {
    name: String,
    // key that references this stage
    key: LogicKey,
    // name internal to the game
    internal_name: String,
    allowed_tod: AllowedToD,
    region: Option<String>,
    areas: BTreeMap<LogicKey, Area>,
}

/// has all the information that never changes, such as what areas exist, what exits exist
/// and probably more
pub struct LogicStatic {
    stages: BTreeMap<LogicKey, Stage>,
    mapper: LogicKeyMapper,
}

#[derive(thiserror::Error, Debug)]
pub enum KeyLookupError {
    #[error("Key {1} was not found for type {0}")]
    NotFound(&'static str, String),
}

impl LogicStatic {
    /// get the area belonging to the area key
    ///
    /// panics if the area doesn't exist, which should really not happen
    pub fn get_area(&self, area_key: AreaKey) -> &Area {
        let stage = self.get_stage(area_key.stage);
        let Some(area) = stage.areas.get(&area_key.area) else {
            panic!(
                "Could not find area {} in stage {}",
                self.mapper
                    .convert_to_string(&area_key.area)
                    .unwrap_or("INVALID"),
                stage.name
            );
        };
        area
    }

    /// get the stage belonging to the logic key
    ///
    /// panics if the stage doesn't exist, which should really not happen
    pub fn get_stage(&self, stage_key: LogicKey) -> &Stage {
        let Some(stage) = self.stages.get(&stage_key) else {
            panic!(
                "Could not find stage by key: {}",
                self.mapper
                    .convert_to_string(&stage_key)
                    .unwrap_or("INVALID")
            );
        };
        stage
    }

    pub fn find_stage(&self, stage_name: &str) -> Result<&Stage, KeyLookupError> {
        let stage_key = self
            .mapper
            .convert_to_num_assuming_present(stage_name)
            .map_err(|name| KeyLookupError::NotFound("Stage", name.to_string()))?;
        Ok(self.get_stage(stage_key))
    }

    pub fn find_area<'a>(
        &'a self,
        stage_name: &str,
        area_name: &str,
    ) -> Result<&'a Area, KeyLookupError> {
        let stage = self.find_stage(stage_name)?;
        let area_key = self
            .mapper
            .convert_to_num_assuming_present(area_name)
            .map_err(|name| KeyLookupError::NotFound("Area", name.to_string()))?;
        let Some(area) = stage.areas.get(&area_key) else {
            panic!(
                "Could not find area {} in stage {}",
                self.mapper
                    .convert_to_string(&area_key)
                    .unwrap_or("INVALID"),
                stage.name
            );
        };
        Ok(area)
    }
}
