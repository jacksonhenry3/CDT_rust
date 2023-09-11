use std::{collections::HashSet, hash::Hash};

use integer_partitions::Partitions;
use itertools::Itertools;

//derive eq
#[derive(Debug, Clone)]
pub struct VolumeProfile {
    pub profile: Vec<usize>,
    pub id: usize,
}

impl VolumeProfile {
    pub fn new(profile: Vec<usize>) -> VolumeProfile {
        let product = profile.iter().fold(1, |acc, x| acc * x);

        let mut id = 0;
        for (index, val) in profile.iter().enumerate() {
            let next = profile[(index + 1) % profile.len()];
            id += val * (product / next);
        }
        VolumeProfile { profile, id }
    }
}

impl PartialEq for VolumeProfile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for VolumeProfile {}

impl Hash for VolumeProfile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn non_cyclic_permutations(vec: Vec<usize>) -> HashSet<VolumeProfile> {
    let result: HashSet<_> = vec
        .iter()
        .cloned()
        .permutations(vec.len())
        .map(|x| VolumeProfile::new(x))
        .collect();

    result
}

pub fn volume_profiles(volume: usize) -> impl Iterator<Item = HashSet<VolumeProfile>> {
    //assert div 2, with message
    assert_eq!(volume % 2, 0, "volume must be divisible by 2");

    let mut a = Partitions::new(volume / 2);

    std::iter::from_fn(move || match a.next() {
        Some(profile) => Some(non_cyclic_permutations(profile.to_vec())),
        _ => None,
    })
}
