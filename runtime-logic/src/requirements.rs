use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};

use anyhow::{bail, Context};

use crate::{
    explorer::Inventory,
    structure::{
        AreaId, ContextLoadable, EventId, Item, ItemId, LogicContext, RequirementKey, TimeOfDay,
    },
    Options,
};

#[derive(Clone)]
pub enum RequirementExpression<'a> {
    And(Vec<RequirementExpression<'a>>),
    Or(Vec<RequirementExpression<'a>>),
    Item(ItemId, u8),
    Event(EventId),
    // TimeOfDay::all() means that it doesn't matter,
    // it otherwise uses the specific allowed TimeOfDay
    Area(AreaId, TimeOfDay),
    Fixed(bool),
    Ref(&'a RequirementExpression<'a>),
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
        }
    }

    pub fn remove_used_items(
        &self,
        inventory: &Inventory,
        options: &Options,
        items: &mut HashSet<ItemId>,
    ) {
        // if the requirement is met, don't consider items used
        if !self.check(inventory, options, TimeOfDay::all()) {
            match self {
                RequirementExpression::And(exprs) | RequirementExpression::Or(exprs) => {
                    for expr in exprs.iter() {
                        expr.remove_used_items(inventory, options, items);
                    }
                }
                RequirementExpression::Item(item, ..) => {
                    items.remove(item);
                }
                RequirementExpression::Event(..) => (),
                RequirementExpression::Area(..) => (),
                RequirementExpression::Fixed(..) => (),
                RequirementExpression::Ref(req) => req.remove_used_items(inventory, options, items),
            };
        }
    }

    fn display_helper(&self, ctx: &LogicContext, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequirementExpression::And(exprs) => {
                f.write_str("AND")?;
                f.debug_list()
                    .entries(exprs.iter().map(|e| e.debug(ctx)))
                    .finish()?;
            }
            RequirementExpression::Or(exprs) => {
                f.write_str("OR")?;
                f.debug_list()
                    .entries(exprs.iter().map(|e| e.debug(ctx)))
                    .finish()?;
            }
            RequirementExpression::Item(item, count) => {
                f.write_fmt(format_args!("{} x{}", item.name(ctx), count))?;
            }
            RequirementExpression::Event(event) => {
                f.write_fmt(format_args!("Event: {}", event.name(ctx)))?;
            }
            RequirementExpression::Area(area, tod) => {
                f.write_fmt(format_args!(
                    "{} - {}({:?})",
                    area.ctx(ctx).stage.name(ctx),
                    area.name(ctx),
                    tod
                ))?;
            }
            RequirementExpression::Fixed(val) => f.write_fmt(format_args!("{}", val))?,
            RequirementExpression::Ref(req) => req.display_helper(ctx, f)?,
            // RequirementExpression::Option(fun) => fun(options),
        }
        Ok(())
    }

    pub fn debug(&'a self, ctx: &'a LogicContext) -> ReqDisplay<'a> {
        ReqDisplay { req: self, ctx }
    }

    pub fn owned(&self) -> RequirementExpression<'static> {
        match self {
            RequirementExpression::And(reqs) => 
                RequirementExpression::And(reqs.iter().map(|req| req.owned()).collect()),
            RequirementExpression::Or(reqs) =>
                RequirementExpression::Or(reqs.iter().map(|req| req.owned()).collect()),
            RequirementExpression::Item(item_id, count) => RequirementExpression::Item(
                *item_id, *count),
            RequirementExpression::Event(event_id) => RequirementExpression::Event(*event_id),
            RequirementExpression::Area(area, tod) => RequirementExpression::Area(*area, *tod),
            RequirementExpression::Fixed(value) => RequirementExpression::Fixed(*value),
            RequirementExpression::Ref(inner) => inner.owned(),
        }
    }
}

pub struct ReqDisplay<'a> {
    req: &'a RequirementExpression<'a>,
    ctx: &'a LogicContext,
}

impl<'a> Debug for ReqDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.req.display_helper(self.ctx, f)
    }
}

#[derive(PartialEq)]
enum LogicType {
    None,
    Or,
    And,
}

fn find_closing_parenthesis(s: &str, start_index: usize) -> Option<usize> {
    // assert that we start with the parenthesis already open
    let mut par_depth = 1;
    for (idx, c) in s.chars().enumerate().skip(start_index) {
        match c {
            '(' => par_depth += 1,
            ')' => {
                par_depth -= 1;
                if par_depth == 0 {
                    return Some(idx);
                }
            }
            _ => {}
        }
    }
    None
}

impl RequirementExpression<'static> {
    pub fn parse(
        string: &str,
        area: Option<AreaId>,
        macros: &[&HashMap<&str, RequirementExpression<'static>>],
        item_names: &HashMap<&str, ItemId>,
        event_names: &HashMap<&str, EventId>,
    ) -> anyhow::Result<Self> {
        let base = Self::parse_rec(string, area, macros, item_names, event_names)?;
        if let Some(area) = area {
            let area_req = RequirementExpression::Area(area, TimeOfDay::all());
            Ok(match base {
                RequirementExpression::And(mut reqs) => {
                    reqs.push(area_req);
                    RequirementExpression::And(reqs)
                }
                _ => RequirementExpression::And(vec![base, area_req]),
            })
        } else {
            Ok(base)
        }
    }
    pub fn parse_rec(
        string: &str,
        area: Option<AreaId>,
        macros: &[&HashMap<&str, RequirementExpression<'static>>],
        item_names: &HashMap<&str, ItemId>,
        event_names: &HashMap<&str, EventId>,
    ) -> anyhow::Result<Self> {
        let mut logic_type = LogicType::None;
        let mut current_rest = string.trim();
        let mut current_level = Vec::new();
        let split_chars: &[_] = &['&', '|'];
        // until exhausted
        while current_rest != "" {
            if current_rest.starts_with('(') {
                // handle nested logic expression
                let closing_p = find_closing_parenthesis(current_rest, 1).with_context(|| {
                    format!("Closing parenthesis not found on string '{}'", current_rest)
                })?;
                current_level.push(RequirementExpression::parse_rec(
                    &current_rest[1..closing_p],
                    area,
                    macros,
                    item_names,
                    event_names,
                )?);
                current_rest = current_rest[closing_p + 1..].trim();
                continue;
            }
            // find either '&' or '|', only the same type can be on one level
            if let Some(pos) = current_rest.find(split_chars) {
                match &current_rest[pos..=pos] {
                    "&" => {
                        if logic_type == LogicType::Or {
                            bail!("& and | mixed for {}", string);
                        }
                        logic_type = LogicType::And;
                    }
                    "|" => {
                        if logic_type == LogicType::And {
                            bail!("& and | mixed for {}", string);
                        }
                        logic_type = LogicType::Or;
                    }
                    _ => unreachable!(),
                };
                // add requirement to list
                let requirement = current_rest[..pos].trim();
                if requirement != "" {
                    current_level.push(RequirementExpression::parse_base_requirement(
                        requirement,
                        area,
                        macros,
                        item_names,
                        event_names,
                    )?);
                }
                current_rest = current_rest[pos + 1..].trim();
            } else {
                // if they can't be found, the rest is one base expression and we're done
                if current_rest != "" {
                    current_level.push(RequirementExpression::parse_base_requirement(
                        current_rest.trim(),
                        area,
                        macros,
                        item_names,
                        event_names,
                    )?);
                }
                break;
            }
        }
        if current_level.len() == 1 {
            return Ok(current_level.remove(0));
        }
        match logic_type {
            LogicType::And => {
                return Ok(RequirementExpression::And(current_level));
            }
            LogicType::Or => {
                return Ok(RequirementExpression::Or(current_level));
            }
            _ => {
                bail!("no logic type, that should not be possible: {}", string)
            }
        }
    }

    fn parse_base_requirement(
        s: &str,
        area: Option<AreaId>,
        macros: &[&HashMap<&str, RequirementExpression<'static>>],
        item_names: &HashMap<&str, ItemId>,
        event_names: &HashMap<&str, EventId>,
    ) -> anyhow::Result<Self> {
        if s == "Nothing" {
            return Ok(RequirementExpression::Fixed(true));
        }
        if s == "Impossible" {
            return Ok(RequirementExpression::Fixed(false));
        }
        if s == "Daytime" {
            return Ok(RequirementExpression::Area(area.unwrap(), TimeOfDay::Day));
        }
        if s == "Nighttime" {
            return Ok(RequirementExpression::Area(area.unwrap(), TimeOfDay::Night));
        }
        // macros
        if let Some(elem) = macros.iter().find_map(|map| map.get(s)) {
            return Ok(elem.clone());
        }
        // item count requirement
        if let Some((item_part, count_part)) = s.rsplit_once(" ") {
            if count_part.starts_with("x") {
                if let Ok(count) = count_part[1..].parse::<usize>() {
                    return Ok(RequirementExpression::Item(
                        item_names
                            .get(item_part)
                            .cloned()
                            .with_context(|| format!("{s}"))?,
                        count as u8,
                    ));
                }
            }
        }
        // items
        if let Some(item_id) = item_names.get(s).cloned() {
            return Ok(RequirementExpression::Item(item_id, 1));
        }
        if s.contains("Trick") || s.contains("Option") {
            // return Ok(RequirementExpression::Option(s.to_string()));
            return Ok(RequirementExpression::Fixed(true)); // TODO
        }
        if let Some(event_id) = event_names.get(s).cloned() {
            return Ok(RequirementExpression::Event(event_id));
        }
        bail!("unknown expression: {s}");
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
        requirement_expression: RequirementExpression<'a>,
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
