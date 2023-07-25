use std::collections::HashMap;

use crate::{
    explorer::Inventory,
    generated::items::{self, Item},
    generated::logic::{self, Area, Event, Location, RequirementKey},
    logic_static::TimeOfDay,
    options::{LmfStartState, Options},
};

fn lmf_is_open(options: &Options) -> bool {
    options.lmf_start_state == LmfStartState::Open
}

#[derive(Clone)]
pub enum RequirementExpression<'a> {
    And(Vec<RequirementExpression<'a>>),
    Or(Vec<RequirementExpression<'a>>),
    Item(Item, u8),
    Event(Event),
    // TimeOfDay::all() means that it doesn't matter,
    // it otherwise uses the specific allowed TimeOfDay
    Area(Area, TimeOfDay),
    Fixed(bool),
    Ref(&'a RequirementExpression<'a>),
    Option(fn(&Options) -> bool),
}

impl<'a> RequirementExpression<'a> {
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
            RequirementExpression::Ref(req) => req.check(inventory, options, allowed_tod),
            RequirementExpression::Option(fun) => fun(options),
        }
    }
}

pub const IMPOSSIBLE: RequirementExpression = RequirementExpression::Fixed(false);
pub const NOTHING: RequirementExpression = RequirementExpression::Fixed(true);

pub struct Requirements<'a> {
    parent: Option<&'a Requirements<'a>>,
    requirements: HashMap<RequirementKey, RequirementExpression<'a>>,
}

impl Requirements<'static> {
    pub fn new() -> Self {
        Self::new_from_map(Default::default())
    }

    pub fn new_from_map(
        requirements: HashMap<RequirementKey, RequirementExpression<'static>>,
    ) -> Self {
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

    pub fn create_layer(&'a self) -> Requirements<'a> {
        Requirements {
            parent: Some(self),
            requirements: Default::default(),
        }
    }

    pub fn get_owned_requirement(
        &self,
        requirement: RequirementKey,
    ) -> Option<RequirementExpression<'a>> {
        match self.requirements.get(&requirement) {
            Some(req) => Some(req.clone()),
            None => self.parent.and_then(|parent| {
                parent
                    .get_requirement(requirement)
                    .map(|r| RequirementExpression::Ref(r))
            }),
        }
    }

    pub fn get_requirement(
        &'a self,
        requirement: RequirementKey,
    ) -> Option<&'a RequirementExpression<'a>> {
        self.requirements
            .get(&requirement)
            .or_else(|| self.parent.and_then(|p| p.get_requirement(requirement)))
    }

    pub fn set_requirement(
        &mut self,
        requirement_key: RequirementKey,
        requirement_expression: RequirementExpression<'static>,
    ) {
        self.requirements
            .insert(requirement_key, requirement_expression);
    }

    /// Add the requirement of "requirement" to "combined_requirement"
    /// useful, for example, to temporarily mark the need to get a certain gossip stone for reaching another check:
    /// `req.combine.requirement(RequirementKey::GossipStoneInSummit, RequirementKey::KnightAcademy_FledgesGift)`
    pub fn combine_requirement(
        &mut self,
        requirement: RequirementKey,
        combined_requirement: RequirementKey,
    ) {
        // TODO: the requirements should really exist in all cases, but should there be
        // a default just in case? But this should be fine since it signals a bug
        let combined = RequirementExpression::And(vec![
            self.get_owned_requirement(requirement).unwrap(),
            self.get_owned_requirement(combined_requirement).unwrap(),
        ]);
        self.requirements.insert(combined_requirement, combined);
    }
}
