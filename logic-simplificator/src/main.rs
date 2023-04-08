use std::collections::{HashMap, HashSet};

pub enum Type {
    Or,
    And,
}

#[derive(Debug, Clone)]
pub struct CountMap(HashMap<&'static str, usize>);

#[derive(Debug, Clone)]
pub struct BitsetDisjunktion {
    possibilities: Vec<CountMap>,
}

impl CountMap {
    pub fn from_list(list: &[&'static str]) -> Self {
        let mut map = HashMap::new();
        for &item in list {
            *map.entry(item).or_default() += 1;
        }
        Self(map)
    }
    pub fn is_subset(&self, other: &CountMap) -> bool {
        for (&k, &v) in &self.0 {
            let bv = other.0.get(k).copied().unwrap_or(0);
            if bv < v {
                return false;
            }
        }
        true
    }

    pub fn is_superset(&self, other: &CountMap) -> bool {
        other.is_subset(self)
    }

    pub fn merge_update(&mut self, other: &CountMap) {
        for (&k, &count) in &other.0 {
            let old_count = self.0.entry(k).or_default();
            *old_count = (*old_count).max(count);
        }
    }

    pub fn merge(&self, other: &CountMap) -> CountMap {
        let mut new_map = self.clone();
        new_map.merge_update(other);
        new_map
    }
}

impl BitsetDisjunktion {
    pub fn new() -> Self {
        Self {
            possibilities: Vec::new(),
        }
    }
    pub fn remove_duplicates(&mut self) {
        let mut left_idx = 0;
        'left_loop: while left_idx < self.possibilities.len() {
            let mut right_idx = left_idx + 1;
            while right_idx < self.possibilities.len() {
                let left = &self.possibilities[left_idx];
                let right = &self.possibilities[right_idx];

                // check if left is included in right
                if left.is_subset(right) {
                    // right can be deleted
                    self.possibilities.swap_remove(right_idx);
                    continue;
                }

                // check if right is included in left
                if right.is_subset(left) {
                    // left can be deleted
                    self.possibilities.swap_remove(left_idx);
                    continue 'left_loop;
                }

                right_idx += 1;
            }
            left_idx += 1;
        }
    }

    pub fn add_or_requirement(&mut self, req: CountMap) {
        for possibility in &mut self.possibilities {
            if possibility.is_subset(&req) {
                // don't add the requirement
                return;
            }
            if req.is_subset(possibility) {
                // the original is worse that the new one
                *possibility = req;
                return;
            }
        }
        self.possibilities.push(req);
    }

    pub fn add_and_requirement(&mut self, req: &CountMap) {
        // needs to be merged into *every* possibility
        for possibility in &mut self.possibilities {
            possibility.merge_update(req);
        }
    }

    pub fn add_or_disj(&mut self, other: BitsetDisjunktion) {
        self.possibilities.extend(other.possibilities.into_iter());
    }

    pub fn add_and_disj(&mut self, other: BitsetDisjunktion) {
        let mut new_disj = BitsetDisjunktion {
            possibilities: Vec::with_capacity(self.possibilities.len() * other.possibilities.len()),
        };
        for poss_a in &self.possibilities {
            for poss_b in &other.possibilities {
                new_disj.add_or_requirement(poss_a.merge(poss_b));
            }
        }
        *self = new_disj
    }
}

enum RequirementExpression {
    And(Vec<RequirementExpression>),
    Or(Vec<RequirementExpression>),
    CountedItem(&'static str, usize),
    Fixed(bool),
}

impl RequirementExpression {
    pub fn get_bitset_disj(&self) -> BitsetDisjunktion {
        match self {
            RequirementExpression::Fixed(false) => BitsetDisjunktion::new(),
            RequirementExpression::Fixed(true) => BitsetDisjunktion {
                possibilities: vec![CountMap(HashMap::new())],
            },
            RequirementExpression::CountedItem(item, count) => BitsetDisjunktion {
                possibilities: vec![CountMap(HashMap::from([(*item, *count)]))],
            },
            RequirementExpression::And(requirements) => {
                let mut disj = BitsetDisjunktion {
                    possibilities: vec![CountMap(HashMap::new())],
                };
                for req in requirements {
                    disj.add_and_disj(req.get_bitset_disj());
                }
                disj
            }
            RequirementExpression::Or(requirements) => {
                let mut disj = BitsetDisjunktion::new();
                for req in requirements {
                    disj.add_or_disj(req.get_bitset_disj());
                }
                disj
            }
        }
    }
}

fn get_example_req() -> RequirementExpression {
    use RequirementExpression::*;
    And(vec![
        CountedItem("Gust Bellows", 1),
        Or(vec![
            CountedItem("Clawshots", 1),
            CountedItem("Progressive Beetle", 1),
            And(vec![
                CountedItem("Whip", 1),
                Or(vec![
                    CountedItem("Progressive Beetle", 1),
                    CountedItem("Progressive Slingshot", 1),
                    CountedItem("Bomb Bag", 1),
                    CountedItem("Clawshots", 1),
                ]),
            ]),
        ]),
    ])
}

pub fn main() {
    // let poss1 = CountMap::from_list(&["Clawshots", "Ruby Tablet", "Progressive Mitts"]);
    // let poss2 = CountMap::from_list(&["Clawshots", "Ruby Tablet"]);

    // let mut disj = BitsetDisjunktion::new();
    // disj.add_or_requirement(poss1);
    // disj.add_or_requirement(poss2);

    // println!("{:?}", disj);

    // disj.remove_duplicates();
    // println!("{:?}", disj);
    let req = get_example_req();
    println!("{:?}", req.get_bitset_disj());
}
