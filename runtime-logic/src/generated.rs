#[derive(Clone, Copy, PartialEq, Eq)]
struct RegionId(u16);
#[derive(Clone, Copy, PartialEq, Eq)]
struct StageId(u16);
#[derive(Clone, Copy, PartialEq, Eq)]
struct AreaId(u16);

const STAGE_LIST: [&'static [StageId]; 5] = [
    &[StageId(0),StageId(1),StageId(2),],
    &[StageId(3)],
    &[StageId(4),StageId(5),StageId(6),],
    &[StageId(7),StageId(8),],
    &[StageId(9),StageId(10),StageId(11),],
];

impl RegionId {
    pub fn stages(&self) -> &'static [StageId] {
        STAGE_LIST[self.0 as usize]
    }
}

mod sky {
    use super::RegionId;

    mod field {
        const ID: crate::generated::StageId = crate::generated::StageId(0);
    }
    const ID: RegionId = RegionId(0);
}
