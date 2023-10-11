use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

use cached::proc_macro::cached;

use rand::seq::SliceRandom;
use rand::thread_rng;

use integer_partitions::Partitions;
use itertools::Itertools;

use weighted_rand::builder::*;

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

        let mut profile_rotator = profile.clone();
        profile_rotator = profile_rotator.into_iter().rev().collect::<VecDeque<_>>();

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

#[cached]
pub fn num_cdts_in_profile(volume_profile: VolumeProfile) -> usize {
    let mut count = 0;
    for i in 1..volume_profile.profile.len() {
        let n = volume_profile.profile[i];
        let m = volume_profile.profile[i - 1];
        count += utils::choose(n + m, m);
    }
    count
}

pub fn constrained_sum_sample_pos(n: usize, total: usize) -> VecDeque<usize> {
    // This generates an unweighted random partition of n with a total sum of total.
    // however, we need the sum to be weighted by the number of CDTs in each partition.
    let mut rng = thread_rng();
    let mut dividers: Vec<usize> = (1..total).collect();
    dividers.shuffle(&mut rng);
    dividers.truncate(n - 1);
    dividers.sort_unstable();
    dividers.push(total);
    let mut result = vec![0; n];
    let mut prev = 0;
    for (i, &num) in dividers.iter().enumerate() {
        result[i] = num - prev;
        prev = num;
    }
    result.into()
}

pub fn weighted_random_partition(n: usize, total: usize) -> VecDeque<usize> {
    let mut base = vec![1; n];
    let mut weights = Vec::with_capacity(n);
    for _ in 0..total - n {
        weights.clear();

        for i in 0..n {
            let vp = VolumeProfile {
                profile: base.clone().into(),
                id: 0,
            };
            let value = num_cdts_in_profile(vp);
            weights.push((value * base[i]) as u32);
        }

        let builder = WalkerTableBuilder::new(&weights);
        let wa_table = builder.build();
        let r = wa_table.next();
        base[r] += 1;
    }

    base.into()
}

pub fn random_volume_profile(volume: usize, time_size: usize) -> VolumeProfile {
    let partition = weighted_random_partition(time_size, volume / 2);
    VolumeProfile::new(partition)
}
