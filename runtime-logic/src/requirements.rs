use core::fmt;
use std::{collections::HashMap, fmt::Debug};

use anyhow::{bail, Context};
use heck::{ToPascalCase, ToSnekCase};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use syn::Ident;

use crate::{
    dumper::convert_to_upper_camel_case,
    structure::{
        AreaId, ContextLoadable, EventId, ItemId, LogicContext, TimeOfDay,
    },
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
    OptionEnabled {
        option: String,
        enabled: bool,
    },
    OptionIs {
        option: String,
        value: String,
        not: bool,
    },
    Ref(&'a RequirementExpression<'a>),
}

impl<'a> RequirementExpression<'a> {
    pub fn dump(&self, ctx: &LogicContext) -> TokenStream {
        match self {
            RequirementExpression::And(exprs) => {
                let dumped_exprs = exprs.iter().map(|e| e.dump(ctx));
                // TODO: make this a Cow?
                quote!(RequirementExpression::And(vec![#(#dumped_exprs),*]))
            }
            RequirementExpression::Or(exprs) => {
                let dumped_exprs = exprs.iter().map(|e| e.dump(ctx));
                quote!(RequirementExpression::Or(vec![#(#dumped_exprs),*]))
            }
            RequirementExpression::Item(item, count) => {
                let item_ident = Ident::new(&item.ctx(ctx).ident, Span::call_site());
                quote!(RequirementExpression::Item(Item::#item_ident, #count))
            }
            RequirementExpression::Area(area, tod) => {
                let area_ident = Ident::new(&area.ctx(ctx).ident, Span::call_site());
                let tod_tokens = tod.to_token();
                quote!(RequirementExpression::Area(Area::#area_ident, #tod_tokens))
            }
            RequirementExpression::Fixed(value) => {
                quote!(RequirementExpression::Fixed(#value))
            }
            RequirementExpression::Event(event) => {
                let event_ident = Ident::new(&event.ctx(ctx).ident, Span::call_site());
                quote!(RequirementExpression::Event(Event::#event_ident))
            }
            RequirementExpression::OptionEnabled { option, enabled } => {
                let opt_ident = Ident::new(&option.to_snek_case(), Span::call_site());
                quote!(RequirementExpression::Option(|options| options.#opt_ident == #enabled))
            }
            RequirementExpression::OptionIs { option, value, not } => {
                let opt_ident = Ident::new(&option.to_snek_case(), Span::call_site());
                let opt_enum_ident = Ident::new(&option.to_pascal_case(), Span::call_site());
                let value_ident =
                    Ident::new(&convert_to_upper_camel_case(value), Span::call_site());
                if *not {
                    quote!(RequirementExpression::Option(|options| options.#opt_ident != #opt_enum_ident::#value_ident))
                } else {
                    quote!(RequirementExpression::Option(|options| options.#opt_ident == #opt_enum_ident::#value_ident))
                }
            }
            RequirementExpression::Ref(rf) => rf.dump(ctx),
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
            _ => {} // RequirementExpression::Option(fun) => fun(options),
        }
        Ok(())
    }

    pub fn debug(&'a self, ctx: &'a LogicContext) -> ReqDisplay<'a> {
        ReqDisplay { req: self, ctx }
    }

    pub fn owned(&self) -> RequirementExpression<'static> {
        match self {
            RequirementExpression::And(reqs) => {
                RequirementExpression::And(reqs.iter().map(|req| req.owned()).collect())
            }
            RequirementExpression::Or(reqs) => {
                RequirementExpression::Or(reqs.iter().map(|req| req.owned()).collect())
            }
            RequirementExpression::Item(item_id, count) => {
                RequirementExpression::Item(*item_id, *count)
            }
            RequirementExpression::Event(event_id) => RequirementExpression::Event(*event_id),
            RequirementExpression::Area(area, tod) => RequirementExpression::Area(*area, *tod),
            RequirementExpression::Fixed(value) => RequirementExpression::Fixed(*value),
            RequirementExpression::OptionEnabled { option, enabled } => {
                RequirementExpression::OptionEnabled {
                    option: option.clone(),
                    enabled: *enabled,
                }
            }
            RequirementExpression::OptionIs { option, value, not } => {
                RequirementExpression::OptionIs {
                    option: option.clone(),
                    value: value.clone(),
                    not: *not,
                }
            }
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
        if s.contains("Trick") {
            // return Ok(RequirementExpression::Option(s.to_string()));
            return Ok(RequirementExpression::Fixed(true)); // TODO
        }
        if s.contains("Option") {
            let option_enabled_re = Regex::new("Option \"([a-z-]+)\" Enabled").unwrap();
            let option_is_re =
                Regex::new("Option \"([a-z-]+)\" Is( Not)? \"([A-Za-z-]+)\"").unwrap();
            if let Some(enabled) = option_enabled_re.captures(s) {
                let opt = enabled.get(1).unwrap();
                return Ok(RequirementExpression::OptionEnabled {
                    option: opt.as_str().to_string(),
                    enabled: true,
                });
            }
            if let Some(option_is) = option_is_re.captures(s) {
                let option = option_is.get(1).unwrap().as_str().to_string();
                let not = option_is.get(2).is_some();
                let value = option_is.get(3).unwrap().as_str().to_string();
                return Ok(RequirementExpression::OptionIs { option, value, not });
            }
        }
        if let Some(event_id) = event_names.get(s).cloned() {
            return Ok(RequirementExpression::Event(event_id));
        }
        bail!("unknown expression: {s}");
    }
}
