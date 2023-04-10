use crate::{
    items::Item,
    logic::{Location, Region},
};

enum PatchTarget {
    TBox {
        stage: &'static str,
        layer: u8,
        room: u8,
        tboxid: u8,
    },
}

pub struct CheckDef {
    name: &'static str,
    region: Region,
    location: Location,
    vanilla_item: Option<Item>,
    patches: &'static [PatchTarget],
}
