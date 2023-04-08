use std::collections::HashMap;

use crate::{
    explorer::Inventory,
    items::{self, Item},
    logic::{self, Area, Location, RequirementKey, Event},
    logic_static::TimeOfDay,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LmfStartState {
    Open,
    MainNode,
    Nodes,
}

pub struct Options {
    lmf_start_state: LmfStartState,
    beedles_shop_vanilla: bool,
}

fn lmf_is_open(options: &Options) -> bool {
    options.lmf_start_state == LmfStartState::Open
}

#[derive(Clone)]
pub enum RequirementExpression {
    And(Vec<RequirementExpression>),
    Or(Vec<RequirementExpression>),
    Item(Item, u8),
    Event(Event),
    // TimeOfDay::all() means that it doesn't matter,
    // it otherwise uses the specific allowed TimeOfDay
    Area(Area, TimeOfDay),
    Fixed(bool),
    Option(fn(&Options) -> bool),
}

impl RequirementExpression {
    pub fn check(&self, inventory: &Inventory, options: &Options, allowed_tod: TimeOfDay) -> bool {
        match self {
            RequirementExpression::And(exprs) => exprs
                .iter()
                .all(|e| e.check(inventory, options, allowed_tod)),
            RequirementExpression::Or(exprs) => exprs
                .iter()
                .any(|e| e.check(inventory, options, allowed_tod)),
            RequirementExpression::Item(item, count) => inventory.item_count(*item) >= *count,
            RequirementExpression::Event(event) => inventory.has_event(*event),
            RequirementExpression::Area(area, tod) => !inventory
                .get_area_tod(*area)
                .intersection(allowed_tod)
                .intersection(*tod)
                .is_empty(),
            RequirementExpression::Fixed(val) => *val,
            RequirementExpression::Option(fun) => fun(options),
        }
    }
}

const IMPOSSIBLE: RequirementExpression = RequirementExpression::Fixed(false);
const NOTHING: RequirementExpression = RequirementExpression::Fixed(true);

pub struct Requirements<'a> {
    parent: Option<&'a Requirements<'a>>,
    requirements: HashMap<RequirementKey, RequirementExpression>,
}

impl Requirements<'static> {
    pub fn new() -> Self {
        Self::new_from_map(Default::default())
    }

    pub fn new_from_map(requirements: HashMap<RequirementKey, RequirementExpression>) -> Self {
        Self {
            parent: None,
            requirements,
        }
    }
}

impl<'a> Requirements<'a> {
    pub fn check(
        &self,
        requirement: RequirementKey,
        inventory: &Inventory,
        options: &Options,
        allowed_tod: TimeOfDay,
    ) -> bool {
        if let Some(req) = self.requirements.get(&requirement) {
            req.check(inventory, options, allowed_tod)
        } else if let Some(parent) = self.parent {
            parent.check(requirement, inventory, options, allowed_tod)
        } else {
            false
        }
    }
}

pub fn get_base_requirements() -> Requirements<'static> {
    use RequirementExpression::*;
    let mut locations = HashMap::new();
    locations.insert(
        RequirementKey::KnightAcademy_OwlansGift,
        Area(logic::Area::Skyloft_CentralOutside, TimeOfDay::Day),
    );
    locations.insert(
        RequirementKey::KnightAcademy_CawlinsLetter,
        And(vec![
            Area(logic::Area::KnightAcademy_Main, TimeOfDay::Night),
            Item(items::Item::GoddessHarp, 1),
        ]),
    );

    Requirements {
        parent: None,
        requirements: locations,
    }
}
