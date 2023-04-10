use logic_core::{Requirements, Placement, Options, Location, Item, Explorer, get_requirements, Entrance, Exit, TimeOfDay};
use rand::{Rng, seq::SliceRandom, SeedableRng, random};
use rand_pcg::Pcg64;

pub fn assumed_fill<R: Rng>(rng: &mut R, requirements: &Requirements<'_>, placement: &mut Placement, options: &Options, locations: &mut Vec<Location>, items: &mut Vec<Item>) {
    locations.shuffle(rng);
    items.shuffle(rng);
    'outer: while let Some(item) = items.pop() {
        let mut explorer = Explorer::create(&requirements, placement, options);
        for unplaced_item in items.iter() {
            explorer.insert_item(*unplaced_item);
        }
        let mut loc_idx = 0;
        while let Some(location) = locations.get(loc_idx) {
            println!("checking {item:?} at {location:?}");
            // println!("{:?}", explorer.inventory);
            if explorer.can_reach(*location) {
                println!("placing {item:?} at {location:?}");
                placement.set_location(*location, item);
                locations.swap_remove(loc_idx);
                continue 'outer;
            }
            loc_idx += 1;
        }
        panic!("cannot place {item:?}, no locations reachable");
    }
}

fn main() {
    let seed = random();
    // let seed = 6766802355250029389;
    println!("seed: {seed}");
    let mut rng = Pcg64::seed_from_u64(seed);
    let requirements = Requirements::new_from_map(get_requirements());
    let options = Options::new();
    let mut placement = Placement::new();
    placement.initial_entrance = Some((Entrance::KnightAcademy_From_Skyloft_UpperDoors, TimeOfDay::Day));
    placement.connect(Entrance::Skyloft_From_KnightAcademy_LowerDoors, Exit::KnightAcademy_To_Skyloft_LowerDoors);
    placement.connect(Entrance::Skyloft_From_KnightAcademy_UpperDoors, Exit::KnightAcademy_To_Skyloft_UpperDoors);
    placement.connect(Entrance::KnightAcademy_From_Skyloft_Chimney, Exit::Skyloft_To_KnightAcademy_Chimney);
    let mut locations = vec![
        Location::KnightAcademy_CawlinsLetter,
        Location::KnightAcademy_ChestNearGoddessStatue,
        Location::KnightAcademy_FledgesGift,
        Location::KnightAcademy_OwlansGift,
        Location::CentralSkyloft_ItemInBirdNest,
        Location::KnightAcademy_InZeldasCloset,
    ];
    let mut items = vec![
        Item::GoddessHarp,
        Item::Clawshots,
        Item::GustBellows,
        Item::RubyTablet
    ];
    assumed_fill(&mut rng, &requirements, &mut placement, &options, &mut locations, &mut items);
}
