use std::{collections::HashSet, hash::Hash};

use integer_partitions::Partitions;
use itertools::Itertools;

//derive eq
#[derive(Debug, Clone)]
pub struct volume_profile {
    profile: Vec<usize>,
    volume: usize,
    product: usize,
    id: usize,
}

impl volume_profile {
    pub fn new(profile: Vec<usize>) -> volume_profile {
        let volume = profile.iter().fold(0, |acc, x| acc + x);
        let product = profile.iter().fold(1, |acc, x| acc * x);

        let total = profile.iter().fold(1, |acc, x| acc * x);
        let mut id = 0;
        for (index, val) in profile.iter().enumerate() {
            let next = profile[(index + 1) % profile.len()];
            id += val * (total / next);
        }
        volume_profile {
            profile,
            volume,
            product,
            id,
        }
    }
}

impl PartialEq for volume_profile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for volume_profile {}

impl Hash for volume_profile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn non_cyclic_permutations(vec: Vec<usize>) -> HashSet<volume_profile> {
    let mut result: HashSet<_> = vec
        .iter()
        .cloned()
        .permutations(vec.len())
        .map(|x| volume_profile::new(x))
        .collect();

    result
}

pub fn volume_profiles(volume: usize) -> impl Iterator<Item = HashSet<volume_profile>> {
    //assert div 2, with message
    assert_eq!(volume % 2, 0, "volume must be divisible by 2");

    let mut a = Partitions::new(volume / 2);

    std::iter::from_fn(move || match a.next() {
        Some(profile) => Some(non_cyclic_permutations(profile.to_vec())),
        _ => None,
    })
}
