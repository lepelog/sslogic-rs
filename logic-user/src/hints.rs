use std::collections::HashMap;

use logic_core::{
    options::Options, Explorer, Item, Location, Placement, RequirementExpression, RequirementKey,
    Requirements,
};

pub fn calc_sots_path(requirements: &Requirements, placement: &Placement, options: &Options) {
    let result = calc_required_for(
        requirements,
        placement,
        options,
        &[
            Location::SealedGrounds_ZeldasBlessing.requirement_key(),
            Location::Skyview_GhirahimHeartContainer.requirement_key(),
            Location::EarthTemple_ScalderaHeartContainer.requirement_key(),
        ],
    );
    for (req, requirements) in result.iter() {
        println!("{req:?}:");
        for (loc, item) in requirements {
            println!("  {loc:?}: {item:?}");
        }
    }
}

pub fn calc_required_for(
    requirements: &Requirements,
    placement: &Placement,
    options: &Options,
    to_check: &[RequirementKey],
) -> HashMap<RequirementKey, Vec<(Location, Item)>> {
    let mut result: HashMap<_, Vec<(Location, Item)>> =
        to_check.iter().map(|k| (*k, vec![])).collect();
    for (loc, item) in placement.locations.iter() {
        let mut reqs_without_loc = requirements.create_layer();
        reqs_without_loc
            .set_requirement(loc.requirement_key(), RequirementExpression::Fixed(false));
        let mut explorer = Explorer::create(&reqs_without_loc, placement, options);
        for check in to_check {
            if !explorer.can_reach_requirement(*check) {
                result.get_mut(check).unwrap().push((*loc, *item));
            }
        }
    }
    result
}
