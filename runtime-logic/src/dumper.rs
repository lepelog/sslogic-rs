use std::collections::HashMap;

use crate::loader::BooleanOption;
use crate::loader::IntOption;
use crate::loader::OptionEntry;
use crate::loader::OptionVariant;
use crate::loader::SinglechoiceOption;
use crate::requirements::RequirementExpression;
use crate::structure::ConnectionShuffleType;
use crate::structure::ContextLoadable;
use crate::structure::DoorConnection;
use crate::structure::LocationKind;
use crate::structure::LogicContext;
use crate::structure::RequirementKey;
use crate::structure::TimeOfDay;

use heck::ToPascalCase;
use heck::ToSnekCase;
use heck::ToUpperCamelCase;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

// turns each
pub fn convert_to_upper_camel_case(s: &str) -> String {
    let mut result = s
        .chars()
        .filter(|c| *c != '\'')
        .collect::<String>()
        .to_upper_camel_case();
    if result.as_bytes()[0].is_ascii_digit() {
        result.insert(0, 'X');
    }
    result
}

pub fn dump(
    ctx: &LogicContext,
    requirements: &HashMap<RequirementKey, RequirementExpression<'static>>,
    options: &Vec<OptionEntry>,
) -> anyhow::Result<String> {
    let mut out_stream = TokenStream::new();
    out_stream.extend(quote!(
        #![allow(non_camel_case_types)]
        use std::collections::HashMap;
        use super::logic_static::{TimeOfDay, ForceToD, BitSetCompatible, Requirements, RequirementExpression, RequirementKey};
    ));
    // dump the region enum
    let regions: Vec<_> = ctx
        .regions
        .iter()
        .map(|r| Ident::new(&r.ident, Span::call_site()))
        .collect();
    let cases = ctx.regions.iter().map(|r| {
        let ident = Ident::new(&r.ident, Span::call_site());
        let areas = r
            .areas
            .iter()
            .map(|a| Ident::new(&a.ctx(ctx).ident, Span::call_site()));
        let name = &r.name;
        quote!(
            Region::#ident => &RegionDef {
                name: #name,
                areas: &[#(Area::#areas,)*],
            },
        )
    });
    out_stream.extend(quote!(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Region {
            #(#regions,)*
        }

        pub struct RegionDef {
            pub name: &'static str,
            pub areas: &'static [Area],
        }

        impl Into<usize> for Region {
            fn into(self) -> usize {
                self as usize
            }
        }

        impl BitSetCompatible for Region {
            const ALL: &'static [Region] = &[
                #(Region::#regions,)*
            ];
        }

        impl Region {
            pub fn get(&self) -> &'static RegionDef {
                match self {
                    #(#cases)*
                }
            }

            pub fn name(&self) -> &'static str {
                self.get().name
            }

            pub fn areas(&self) -> &'static [Area] {
                self.get().areas
            }
        }
    ));
    // dump the stages
    let stage_idents: Vec<_> = ctx
        .stages
        .iter()
        .map(|s| Ident::new(&s.ident, Span::call_site()))
        .collect();
    let cases = ctx.stages.iter().map(|s| {
        let ident = Ident::new(&s.ident, Span::call_site());
        let areas = s
            .areas
            .iter()
            .map(|a| Ident::new(&a.ctx(ctx).ident, Span::call_site()));
        let name = &s.name;
        quote!(
            Stage::#ident => &StageDef {
                name: #name,
                areas: &[#(Area::#areas,)*],
            },
        )
    });
    out_stream.extend(quote!(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Stage {
            #(#stage_idents,)*
        }

        pub struct StageDef {
            pub name: &'static str,
            pub areas: &'static [Area],
        }

        impl Into<usize> for Stage {
            fn into(self) -> usize {
                self as usize
            }
        }

        impl BitSetCompatible for Stage {
            const ALL: &'static [Stage] = &[
                #(Stage::#stage_idents,)*
            ];
        }

        impl Stage {
            pub fn get(&self) -> &'static StageDef {
                match self {
                    #(#cases)*
                }
            }

            pub fn name(&self) -> &'static str {
                self.get().name
            }

            pub fn areas(&self) -> &'static [Area] {
                self.get().areas
            }
        }
    ));

    // // dump areas
    let area_idents: Vec<_> = ctx
        .areas
        .iter()
        .map(|a| Ident::new(&a.ident, Span::call_site()))
        .collect();
    let cases = ctx.areas.iter().map(|a| {
        let ident = Ident::new(&a.ident, Span::call_site());
        let name = &a.name;
        let full_name = &a.full_name;
        let region = Ident::new(&a.region.ctx(ctx).ident, Span::call_site());
        let stage = Ident::new(&a.stage.ctx(ctx).ident, Span::call_site());
        let time_of_day = match a.time_of_day {
            TimeOfDay::Day => quote!(Some(ForceToD::Day)),
            TimeOfDay::Night => quote!(Some(ForceToD::Night)),
            _ => quote!(None),
        };
        let can_sleep = a.can_sleep;
        let locations = a
            .locations
            .iter()
            .map(|x| Ident::new(&x.ctx(ctx).ident, Span::call_site()));
        let map_exits = a
            .map_exits
            .iter()
            .map(|x| Ident::new(&x.ctx(ctx).ident, Span::call_site()));
        let map_entrances = a
            .map_entrances
            .iter()
            .map(|x| Ident::new(&x.ctx(ctx).ident, Span::call_site()));
        let logic_exits = a
            .logic_exits
            .iter()
            .map(|x| Ident::new(&x.ctx(ctx).ident, Span::call_site()));
        let logic_entrances = a
            .logic_entrances
            .iter()
            .map(|x| Ident::new(&x.ctx(ctx).ident, Span::call_site()));

        quote!(
            Area::#ident => &AreaDef {
                name: #name,
                full_name: #full_name,
                region: Region::#region,
                stage: Stage::#stage,
                time_of_day: #time_of_day,
                can_sleep: #can_sleep,
                locations: &[#(Location::#locations,)*],
                map_exits: &[#(Exit::#map_exits,)*],
                map_entrances: &[#(Entrance::#map_entrances,)*],
                logic_exits: &[#(Area::#logic_exits,)*],
                logic_entrances: &[#(Area::#logic_entrances,)*],
            },
        )
    });
    out_stream.extend(quote!(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Area {
            #(#area_idents,)*
        }

        pub struct AreaDef {
            pub name: &'static str,
            pub full_name: &'static str,
            pub region: Region,
            pub stage: Stage,
            pub time_of_day: Option<ForceToD>,
            pub can_sleep: bool,
            pub locations: &'static [Location],
            pub map_exits: &'static [Exit],
            pub map_entrances: &'static [Entrance],
            pub logic_exits: &'static [Area],
            pub logic_entrances: &'static [Area],
        }

        impl Into<usize> for Area {
            fn into(self) -> usize {
                self as usize
            }
        }

        impl BitSetCompatible for Area {
            const ALL: &'static [Area] = &[
                #(Area::#area_idents,)*
            ];
        }

        impl Area {
            pub fn get(&self) -> &'static AreaDef {
                match self {
                    #(#cases)*
                }
            }

            pub fn name(&self) -> &'static str {
                self.get().name
            }
        }
    ));

    // dump exits
    let exit_idents: Vec<_> = ctx
        .exits
        .iter()
        .map(|e| Ident::new(&e.ident, Span::call_site()))
        .collect();
    let cases = ctx.exits.iter().map(|e| {
        let ident = Ident::new(&e.ident, Span::call_site());
        let area = Ident::new(&e.area.ctx(ctx).ident, Span::call_site());
        let to = Ident::new(&e.to.ctx(ctx).ident, Span::call_site());
        let disambiguation = if let Some(disambig) = e.disambiguation.as_ref() {
            quote!(Some(#disambig))
        } else {
            quote!(None)
        };
        let display_name = &e.display_name;
        let coupled_entrance = if let Some(entr) = e.coupled_entrance {
            let ident = Ident::new(&entr.ctx(ctx).ident, Span::call_site());
            quote!(Some(Entrance::#ident))
        } else {
            quote!(None)
        };
        let vanilla_entrance = if let Some(entr) = e.vanilla_entrance {
            let ident = Ident::new(&entr.ctx(ctx).ident, Span::call_site());
            quote!(Some(Entrance::#ident))
        } else {
            quote!(None)
        };
        let door_connection = match e.door_connection {
            DoorConnection::No => quote!(DoorConnection::No),
            DoorConnection::Left(conn) => {
                let exit_ident = Ident::new(&conn.ctx(ctx).ident, Span::call_site());
                quote!(DoorConnection::Left(Exit::#exit_ident))
            }
            DoorConnection::Right(conn) => {
                let exit_ident = Ident::new(&conn.ctx(ctx).ident, Span::call_site());
                quote!(DoorConnection::Right(Exit::#exit_ident))
            }
        };
        let connection_shuffle_type = match e.connection_shuffle_type {
            ConnectionShuffleType::Never => quote!(ConnectionShuffleType::Never),
            ConnectionShuffleType::Other => quote!(ConnectionShuffleType::Other),
        };
        let exit_patch_info = e.patch_info.iter().map(|patch_info| {
            let stage_name = &patch_info.stage_name;
            let room = patch_info.room;
            let exit_idx = patch_info.exit_idx;
            quote!(ExitPatchInfo{
                stage_name: #stage_name,
                room: #room,
                exit_idx: #exit_idx,
            })
        });
        // TODO: door_connection, patch_info, connection_type

        quote!(
            Exit::#ident => &ExitDef {
                area: Area::#area,
                to: Area::#to,
                disambiguation: #disambiguation,
                display_name: #display_name,
                coupled_entrance: #coupled_entrance,
                vanilla_entrance: #vanilla_entrance,
                door_connection: #door_connection,
                connection_shuffle_type: #connection_shuffle_type,
                patch_info: &[
                    #(#exit_patch_info,)*
                ],
            },
        )
    });

    out_stream.extend(quote!(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum DoubleDoor {
            No,
            Left,
            Right,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

        pub struct ExitPatchInfo {
            // technical details
            pub stage_name: &'static str,
            pub room: u8,
            pub exit_idx: u8,
        }

        pub struct ExitDef {
            pub area: Area,
            pub to: Area,
            pub disambiguation: Option<&'static str>,
            pub display_name: &'static str,
            pub door_connection: DoorConnection<Exit>,
            pub connection_shuffle_type: ConnectionShuffleType,
            pub coupled_entrance: Option<Entrance>,
            pub vanilla_entrance: Option<Entrance>,
            pub patch_info: &'static [ExitPatchInfo],
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Exit {
            #(#exit_idents,)*
        }

        impl Into<usize> for Exit {
            fn into(self) -> usize {
                self as usize
            }
        }

        impl BitSetCompatible for Exit {
            const ALL: &'static [Exit] = &[
                #(Exit::#exit_idents,)*
            ];
        }

        impl Exit {
            pub fn get(&self) -> &'static ExitDef {
                match self {
                    #(#cases)*
                }
            }
        }
    ));

    // dump entrances
    let entrance_idents: Vec<_> = ctx
        .entrance
        .iter()
        .map(|e| Ident::new(&e.ident, Span::call_site()))
        .collect();
    let cases = ctx.entrance.iter().map(|e| {
        let ident = Ident::new(&e.ident, Span::call_site());
        let area = Ident::new(&e.area.ctx(ctx).ident, Span::call_site());
        let from = Ident::new(&e.from.ctx(ctx).ident, Span::call_site());
        let disambiguation = if let Some(disambig) = e.disambiguation.as_ref() {
            quote!(Some(#disambig))
        } else {
            quote!(None)
        };
        let display_name = &e.display_name;
        let patch_info = if let Some(info) = e.patch_info.as_ref() {
            let stage_name = &info.stage_name;
            let room = info.room;
            let layer = info.layer;
            let entrance_id = info.entrance_id;
            quote!(Some(EntrancePatchInfo {
                stage_name: #stage_name,
                room: #room,
                layer: #layer,
                entrance_id: #entrance_id,
            }))
        } else {
            quote!(None)
        };

        quote!(
            Entrance::#ident => &EntranceDef {
                area: Area::#area,
                from: Area::#from,
                disambiguation: #disambiguation,
                display_name: #display_name,
                patch_info: #patch_info,
            },
        )
    });

    out_stream.extend(quote!(

        pub struct EntrancePatchInfo {
            // technical details
            pub stage_name: &'static str,
            pub room: u8,
            pub layer: u8,
            pub entrance_id: u8,
        }

        pub struct EntranceDef {
            pub area: Area,
            pub from: Area,
            pub disambiguation: Option<&'static str>,
            pub display_name: &'static str,
            pub patch_info: Option<EntrancePatchInfo>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Entrance {
            #(#entrance_idents,)*
        }

        impl Into<usize> for Entrance {
            fn into(self) -> usize {
                self as usize
            }
        }

        impl BitSetCompatible for Entrance {
            const ALL: &'static [Entrance] = &[
                #(Entrance::#entrance_idents,)*
            ];
        }

        impl Entrance {
            pub fn get(&self) -> &'static EntranceDef {
                match self {
                    #(#cases)*
                }
            }
        }
    ));

    // dump events
    let event_idents: Vec<_> = ctx
        .events
        .iter()
        .map(|e| Ident::new(&e.ident, Span::call_site()))
        .collect();
    let cases = ctx.events.iter().map(|e| {
        let ident = Ident::new(&e.ident, Span::call_site());
        let name = &e.name;
        quote!(
            Event::#ident => &EventDef {
                name: #name,
            },
        )
    });

    out_stream.extend(quote!(
        pub struct EventDef {
            pub name: &'static str,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Event {
            #(#event_idents,)*
        }

        impl Into<usize> for Event {
            fn into(self) -> usize {
                self as usize
            }
        }

        impl BitSetCompatible for Event {
            const ALL: &'static [Event] = &[
                #(Event::#event_idents,)*
            ];
        }

        impl Event {
            pub fn get(&self) -> &'static EventDef {
                match self {
                    #(#cases)*
                }
            }
        }
    ));

    let mut item_values: Vec<_> = ctx.items.values().collect();
    item_values.sort_unstable_by_key(|item| item.id);

    // dump items
    let item_idents: Vec<_> = item_values.iter()
        .map(|item| Ident::new(&item.ident, Span::call_site()))
        .collect();
    let item_cases = item_values.iter().map(|item| {
        let name = &item.name;
        let id = item.id.0;
        let ident = Ident::new(&item.ident, Span::call_site());
        quote!(Item::#ident => &ItemDef {
            item_id: #id,
            name: #name
        },)
    });

    out_stream.extend(quote!(
        pub struct ItemDef {
            pub name: &'static str,
            pub item_id: u16,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Item {
            #(#item_idents,)*
        }

        impl Into<usize> for Item {
            fn into(self) -> usize {
                self as usize
            }
        }

        impl BitSetCompatible for Item {
            const ALL: &'static [Item] = &[
                #(Item::#item_idents,)*
            ];
        }

        impl Item {
            pub fn get(&self) -> &'static ItemDef {
                match self {
                    #(#item_cases)*
                }
            }
        }
    ));

    // dump locations
    let location_idents: Vec<_> = ctx
        .locations
        .iter()
        .map(|loc| Ident::new(&loc.ident, Span::call_site()))
        .collect();

    let location_cases = ctx.locations.iter().map(|loc| {
        // pub id: LocationId,
        // pub area: AreaId,
        // pub name: String,
        // /// includes region info
        // pub display_name: String,
        // pub ident: String,
        // pub kind: LocationKind,
        let area = Ident::new(&loc.area.ctx(ctx).ident, Span::call_site());
        let name = &loc.name;
        let ident = Ident::new(&loc.ident, Span::call_site());
        let display_name = &loc.display_name;
        let kind = match &loc.kind {
            LocationKind::Check { vanilla_item, .. } => {
                let item = Ident::new(&vanilla_item.ctx(ctx).ident, Span::call_site());
                // TODO: patch targets
                quote!(LocationKind::Check{ vanilla_item: Item::#item })
            }
            LocationKind::GossipStone { text_path } => {
                quote!(LocationKind::GossipStone { text_path: #text_path })
            }
        };
        quote!(Location::#ident => &LocationDef {
            name: #name,
            area: Area::#area,
            display_name: #display_name,
            kind: #kind,
        },)
    });

    out_stream.extend(quote!(
        pub enum LocationKind {
            Check {
                vanilla_item: Item,
            },
            GossipStone {
                text_path: &'static str,
            },
        }

        pub struct LocationDef {
            pub name: &'static str,
            pub area: Area,
            pub display_name: &'static str,
            pub kind: LocationKind,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Location {
            #(#location_idents,)*
        }

        impl Into<usize> for Location {
            fn into(self) -> usize {
                self as usize
            }
        }

        impl BitSetCompatible for Location {
            const ALL: &'static [Location] = &[
                #(Location::#location_idents,)*
            ];
        }

        impl Location {
            pub fn get(&self) -> &'static LocationDef {
                match self {
                    #(#location_cases)*
                }
            }
        }
    ));

    let mut sorted_reqs: Vec<_> = requirements.iter().collect();
    sorted_reqs.sort_unstable_by_key(|(key, _)| *key);

    // dump logic
    let logic_exprs = sorted_reqs
        .iter()
        .map(|(req_key, requirement)| {
            let key_expr = match req_key {
                RequirementKey::Exit(exit) => {
                    let exit_expr = Ident::new(&exit.ctx(ctx).ident, Span::call_site());
                    quote!(RequirementKey::Exit(Exit::#exit_expr))
                }
                RequirementKey::LogicExit { from, to } => {
                    let from_expr = Ident::new(&from.ctx(ctx).ident, Span::call_site());
                    let to_expr = Ident::new(&to.ctx(ctx).ident, Span::call_site());
                    quote!(RequirementKey::LogicExit{from: Area::#from_expr, to: Area::#to_expr})
                }
                RequirementKey::Location(location) => {
                    let location_expr = Ident::new(&location.ctx(ctx).ident, Span::call_site());
                    quote!(RequirementKey::Location(Location::#location_expr))
                }
                RequirementKey::Event(event) => {
                    let event_expr = Ident::new(&event.ctx(ctx).ident, Span::call_site());
                    quote!(RequirementKey::Event(Event::#event_expr))
                }
            };
            let req_expr = requirement.dump(ctx);
            quote!((#key_expr, #req_expr))
        });

    out_stream.extend(quote!(
        pub fn get_logic() -> Requirements<'static> {
            Requirements::new_from_map(HashMap::from([
                #(#logic_exprs,)*
            ]))
        }
    ));

    // create options struct
    let mut option_fields = TokenStream::new();
    let mut default_instances = TokenStream::new();
    let mut variant_structs = TokenStream::new();
    for opt in options.iter().filter(|opt| opt.permalink) {
        let field_name = Ident::new(&opt.command.to_snek_case(), Span::call_site());
        match &opt.variant {
            OptionVariant::Boolean(BooleanOption { default }) => {
                option_fields.extend(quote!(pub #field_name: bool,));
                default_instances.extend(quote!(#field_name: #default,));
            }
            OptionVariant::Int(IntOption { default, .. }) => {
                option_fields.extend(quote!(pub #field_name: usize,));
                default_instances.extend(quote!(#field_name: #default as usize,));
            }
            OptionVariant::Singlechoice(SinglechoiceOption { default, choices }) => {
                let choice_enum_name = Ident::new(&opt.command.to_pascal_case(), Span::call_site());
                let default_ident =
                    Ident::new(&convert_to_upper_camel_case(&default), Span::call_site());
                let variants = choices.iter().map(|choice| {
                    Ident::new(&convert_to_upper_camel_case(choice), Span::call_site())
                });
                variant_structs.extend(quote!(
                    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                    pub enum #choice_enum_name {
                        #(#variants,)*
                    }
                ));
                option_fields.extend(quote!(pub #field_name: #choice_enum_name,));
                default_instances.extend(quote!(#field_name: #choice_enum_name::#default_ident,));
            }
            OptionVariant::Other => {
                // ignore for now
            }
        }
    }

    out_stream.extend(variant_structs);
    out_stream.extend(quote!(
        #[derive(Debug, Clone)]
        pub struct Options {
            #option_fields
        }

        impl Default for Options {
            fn default() -> Self {
                Options {
                    #default_instances
                }
            }
        }
    ));

    // println!("{}", out_stream.to_string());

    let file = syn::parse2(out_stream)?;
    Ok(prettyplease::unparse(&file))
}
