use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

use integer_partitions::Partitions;
use itertools::Itertools;

use crate::utils;

//derive eq
#[derive(Debug, Clone)]
pub struct VolumeProfile {
    pub profile: VecDeque<usize>,
    pub id: u64,
}

impl VolumeProfile {
    pub fn new(profile: VecDeque<usize>) -> VolumeProfile {
        let mut profile_rotator = profile.clone();

        //sum of profile
        let sum = profile.iter().sum::<usize>();

        let mut hasher = DefaultHasher::new();
        sum.hash(&mut hasher);
        let mut id = hasher.finish();
        for _ in 0..(profile.len()) {
            let mut hasher = DefaultHasher::new();
            profile_rotator.hash(&mut hasher);
            let temp = hasher.finish();
            // println!("{:?} : {} ", profile_rotator, temp);
            id = id.wrapping_add(temp);
            profile_rotator.rotate_left(1);
        }
        // println!("finished");

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
        .map(|x| VolumeProfile::new(x.into()))
        .collect();

    result
}

pub fn volume_profiles(volume: usize) -> impl Iterator<Item = HashSet<VolumeProfile>> {
    assert_eq!(volume % 2, 0, "volume must be divisible by 2");

    let mut a = Partitions::new(volume / 2);

    std::iter::from_fn(move || match a.next() {
        Some(profile) => Some(non_cyclic_permutations(profile.to_vec())),
        _ => None,
    })
}

pub fn num_cdts_in_profile(volume_profile: &VolumeProfile) -> usize {
    let mut count = 0;
    for i in 1..volume_profile.profile.len() {
        let n = volume_profile.profile[i];
        let k = volume_profile.profile[i - 1];
        count += utils::choose(n, k);
    }
    count
}
