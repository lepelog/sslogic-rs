use std::{collections::HashMap, fmt::Display};

use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;

/// Encapsulates everything obtained from logic files
/// never changes, and thus can always be immutably borrowed everywhere
pub struct LogicContext {
    pub areas: Vec<Area>,
    pub stages: Vec<Stage>,
    pub regions: Vec<Region>,
    pub exits: Vec<Exit>,
    pub entrance: Vec<Entrance>,
    pub locations: Vec<Location>,
    pub events: Vec<Event>,
    pub items: HashMap<ItemId, Item>,
}

impl LogicContext {
    pub fn load<C: ContextLoadable>(&self, id: C) -> &C::Out {
        id.ctx(self)
    }

    pub fn item_by_name(&self, name: &str) -> Option<ItemId> {
        self.items
            .iter()
            .find(|(_, v)| v.name == name)
            .map(|(k, _)| *k)
    }

    pub fn find<C: ContextLoadable>(&self, name: &str) -> Option<C> {
        C::find(self, name)
    }

    pub fn display_iter<'a, C: ContextLoadable, I: Iterator<Item = C> + Clone>(
        &'a self,
        iter: I,
    ) -> ContextIterDisplay<'a, C, I> {
        ContextIterDisplay { ctx: self, iter }
    }

    pub fn display_iter2<'a, C: ContextLoadable, I: IntoIterator<Item = C>>(
        &'a self,
        iter: I,
    ) -> ContextIterDisplay<'a, C, impl Iterator<Item = C> + Clone>
    where
        <I as IntoIterator>::IntoIter: Clone,
    {
        ContextIterDisplay {
            ctx: self,
            iter: iter.into_iter(),
        }
    }
}

pub struct ContextIterDisplay<'a, C: ContextLoadable, I: Iterator<Item = C> + Clone> {
    ctx: &'a LogicContext,
    iter: I,
}

impl<'a, C: ContextLoadable, I: Iterator<Item = C> + Clone> Display
    for ContextIterDisplay<'a, C, I>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.iter.clone().map(|i| i.name(self.ctx)))
            .finish()
    }
}

pub trait ContextLoadable: Sized {
    type Out;

    fn ctx<'a>(&self, context: &'a LogicContext) -> &'a Self::Out;
    fn name<'a>(&self, context: &'a LogicContext) -> &'a str;
    fn find(context: &LogicContext, name: &str) -> Option<Self>;
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TimeOfDay: u8 {
        const Day   = 1 << 0;
        const Night = 1 << 1;
        const Both   = Self::Day.bits() | Self::Night.bits();
    }
}

impl TimeOfDay {
    pub fn to_token(&self) -> TokenStream {
        match *self {
            TimeOfDay::Day => quote!(TimeOfDay::Day),
            TimeOfDay::Night => quote!(TimeOfDay::Night),
            TimeOfDay::Both => quote!(TimeOfDay::Both),
            _ => unreachable!(),
        }
    }
}

// TODO inner num shouldn't be pub
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AreaId(pub u16);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StageId(pub u16);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionId(pub u16);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExitId(pub u16);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntranceId(pub u16);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocationId(pub u16);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId(pub u16);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(pub u16);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequirementKey {
    Exit(ExitId),
    LogicExit { from: AreaId, to: AreaId },
    Location(LocationId),
    Event(EventId),
}

impl From<ExitId> for RequirementKey {
    fn from(value: ExitId) -> Self {
        Self::Exit(value)
    }
}

impl From<LocationId> for RequirementKey {
    fn from(value: LocationId) -> Self {
        Self::Location(value)
    }
}

impl From<EventId> for RequirementKey {
    fn from(value: EventId) -> Self {
        Self::Event(value)
    }
}

pub struct Area {
    pub id: AreaId,
    pub name: String,
    // includes the stage
    pub full_name: String,
    pub ident: String,
    pub region: RegionId,
    pub stage: StageId,
    pub time_of_day: TimeOfDay,
    pub can_sleep: bool,
    pub locations: Vec<LocationId>,
    pub map_exits: Vec<ExitId>,
    pub map_entrances: Vec<EntranceId>,
    pub logic_exits: Vec<AreaId>, // list of other areas this area leads to (directly)
    pub logic_entrances: Vec<AreaId>, // reverse of the above
}

impl ContextLoadable for AreaId {
    type Out = Area;

    fn ctx<'a>(&self, context: &'a LogicContext) -> &'a Self::Out {
        &context.areas[self.0 as usize]
    }

    fn name<'a>(&self, context: &'a LogicContext) -> &'a str {
        &context.areas[self.0 as usize].name
    }

    fn find(context: &LogicContext, name: &str) -> Option<Self> {
        context.areas.iter().find(|a| &a.name == name).map(|a| a.id)
    }
}

pub struct Stage {
    pub id: StageId,
    pub name: String,
    pub ident: String,
    pub areas: Vec<AreaId>,
}

impl ContextLoadable for StageId {
    type Out = Stage;

    fn ctx<'a>(&self, context: &'a LogicContext) -> &'a Self::Out {
        &context.stages[self.0 as usize]
    }

    fn name<'a>(&self, context: &'a LogicContext) -> &'a str {
        &context.stages[self.0 as usize].name
    }

    fn find(context: &LogicContext, name: &str) -> Option<Self> {
        context
            .stages
            .iter()
            .find(|a| &a.name == name)
            .map(|a| a.id)
    }
}

pub struct Region {
    pub id: RegionId,
    pub name: String,
    pub areas: Vec<AreaId>,
    pub ident: String,
}

impl ContextLoadable for RegionId {
    type Out = Region;

    fn ctx<'a>(&self, context: &'a LogicContext) -> &'a Self::Out {
        &context.regions[self.0 as usize]
    }

    fn name<'a>(&self, context: &'a LogicContext) -> &'a str {
        &context.regions[self.0 as usize].name
    }

    fn find(context: &LogicContext, name: &str) -> Option<Self> {
        context
            .regions
            .iter()
            .find(|a| &a.name == name)
            .map(|a| a.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum DoubleDoor {
    No,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum ConnectionShuffleType {
    Never,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DoorConnection<T: Copy> {
    No,
    Left(T),
    Right(T),
}

impl<T: Copy> DoorConnection<T> {
    pub fn is_no(&self) -> bool {
        matches!(self, DoorConnection::No)
    }
    pub fn has_left_neighbor(&self) -> bool {
        matches!(self, DoorConnection::Left(_))
    }
    pub fn has_right_neighbor(&self) -> bool {
        matches!(self, DoorConnection::Right(_))
    }
    pub fn get_left_neighbor(&self) -> Option<T> {
        match self {
            DoorConnection::Left(val) => Some(*val),
            _ => None,
        }
    }
    pub fn get_right_neighbor(&self) -> Option<T> {
        match self {
            DoorConnection::Right(val) => Some(*val),
            _ => None,
        }
    }
    pub fn is_left_door(&self) -> bool {
        self.has_right_neighbor()
    }
    pub fn is_right_door(&self) -> bool {
        self.has_left_neighbor()
    }
    pub fn is_opposite<U: Copy>(&self, other: DoorConnection<U>) -> bool {
        match (self, other) {
            (DoorConnection::No, DoorConnection::No) => true,
            (DoorConnection::Left(_), DoorConnection::Right(_)) => true,
            (DoorConnection::Right(_), DoorConnection::Left(_)) => true,
            _ => false,
        }
    }
    pub fn is_same<U: Copy>(&self, other: DoorConnection<U>) -> bool {
        match (self, other) {
            (DoorConnection::No, DoorConnection::No) => true,
            (DoorConnection::Left(_), DoorConnection::Left(_)) => true,
            (DoorConnection::Right(_), DoorConnection::Right(_)) => true,
            _ => false,
        }
    }
}

// coupled: when entrance and exit are on the same stage and in the same *place*
// example: the exit from skyloft to mallara's house is coupled with the entrance on skyloft from mallara's house

pub struct Entrance {
    pub id: EntranceId,
    pub area: AreaId,
    pub from: AreaId,
    pub disambiguation: Option<String>,
    pub display_name: String,
    pub ident: String,
    pub door_connection: DoorConnection<EntranceId>,
    pub connection_shuffle_type: ConnectionShuffleType,
    // these might be ambiguous, multiple exits can lead to the same entrance
    // pub coupled_exit: Option<ExitId>,
    // pub vanilla_exit: Option<ExitId>,
    // technical details
    pub patch_info: Option<EntrancePatchInfo>,
}

pub struct EntrancePatchInfo {
    // technical details
    pub stage_name: String,
    pub room: u8,
    pub layer: u8,
    pub entrance_id: u8,
}

impl ContextLoadable for EntranceId {
    type Out = Entrance;

    fn ctx<'a>(&self, context: &'a LogicContext) -> &'a Self::Out {
        &context.entrance[self.0 as usize]
    }

    fn name<'a>(&self, context: &'a LogicContext) -> &'a str {
        &context.entrance[self.0 as usize].display_name
    }

    fn find(context: &LogicContext, name: &str) -> Option<Self> {
        context
            .entrance
            .iter()
            .find(|a| &a.display_name == name)
            .map(|a| a.id)
    }
}

pub struct ExitPatchInfo {
    // technical details
    pub stage_name: String,
    pub room: u8,
    pub exit_idx: u8,
}

pub struct Exit {
    pub id: ExitId,
    pub area: AreaId,
    pub to: AreaId,
    pub disambiguation: Option<String>,
    pub display_name: String,
    pub ident: String,
    pub door_connection: DoorConnection<ExitId>,
    pub connection_shuffle_type: ConnectionShuffleType,
    pub coupled_entrance: Option<EntranceId>,
    pub vanilla_entrance: Option<EntranceId>,
    pub patch_info: Vec<ExitPatchInfo>,
}

impl ContextLoadable for ExitId {
    type Out = Exit;

    fn ctx<'a>(&self, context: &'a LogicContext) -> &'a Self::Out {
        &context.exits[self.0 as usize]
    }

    fn name<'a>(&self, context: &'a LogicContext) -> &'a str {
        &context.exits[self.0 as usize].display_name
    }

    fn find(context: &LogicContext, name: &str) -> Option<Self> {
        context
            .exits
            .iter()
            .find(|a| &a.display_name == name)
            .map(|a| a.id)
    }
}

pub struct Location {
    pub id: LocationId,
    pub area: AreaId,
    pub name: String,
    /// includes region info
    pub display_name: String,
    pub ident: String,
    pub kind: LocationKind,
    // TODO: technical detail enum
}

pub enum PatchTarget {
    // TODO
}

pub enum LocationKind {
    Check {
        vanilla_item: ItemId,
        patches: Vec<PatchTarget>,
    },
    GossipStone {
        text_path: String,
    },
}

impl ContextLoadable for LocationId {
    type Out = Location;

    fn ctx<'a>(&self, context: &'a LogicContext) -> &'a Self::Out {
        &context.locations[self.0 as usize]
    }

    fn name<'a>(&self, context: &'a LogicContext) -> &'a str {
        &context.locations[self.0 as usize].display_name
    }

    fn find(context: &LogicContext, name: &str) -> Option<Self> {
        context
            .locations
            .iter()
            .find(|a| &a.display_name == name)
            .map(|a| a.id)
    }
}

pub struct Event {
    pub id: EventId,
    pub name: String,
    pub ident: String,
}

impl ContextLoadable for EventId {
    type Out = Event;

    fn ctx<'a>(&self, context: &'a LogicContext) -> &'a Self::Out {
        &context.events[self.0 as usize]
    }

    fn name<'a>(&self, context: &'a LogicContext) -> &'a str {
        &context.events[self.0 as usize].name
    }

    fn find(context: &LogicContext, name: &str) -> Option<Self> {
        context
            .events
            .iter()
            .find(|a| &a.name == name)
            .map(|a| a.id)
    }
}

pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub ident: String,
    // TODO technical stuff
}

impl ContextLoadable for ItemId {
    type Out = Item;

    fn ctx<'a>(&self, context: &'a LogicContext) -> &'a Self::Out {
        &context.items[self]
    }

    fn name<'a>(&self, context: &'a LogicContext) -> &'a str {
        &context.items[self].name
    }

    fn find(context: &LogicContext, name: &str) -> Option<Self> {
        context
            .items
            .values()
            .find(|a| &a.name == name)
            .map(|a| a.id)
    }
}
