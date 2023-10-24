use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

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

pub fn volume_profiles(volume: usize, time_size: usize) -> HashSet<VolumeProfile> {
    assert!(volume % 2 == 0, "Volume must be even");
    let total_spatial_length = volume / 2;

    let all_dividers = (1..total_spatial_length).combinations(time_size - 1);
    let mut final_result = HashSet::new();
    for mut dividers in all_dividers {
        dividers.push(total_spatial_length);

        let mut result = vec![0; time_size];
        let mut prev = 0;
        for (i, &num) in dividers.iter().enumerate() {
            result[i] = num - prev;
            prev = num;
        }
        final_result.insert(VolumeProfile::new(result.into()));
    }

    final_result
}

// #[cached]
pub fn num_cdts_in_profile(volume_profile: &VolumeProfile) -> u128 {
    let mut count = 1;
    for i in 1..volume_profile.profile.len() {
        let n = volume_profile.profile[i];
        let m = volume_profile.profile[i - 1];
        count *= (utils::choose(n + m, m) as u128);
    }
    count
}
pub fn num_cdts_in_profile_proportional(
    volume_profile: &VolumeProfile,
    proportionality: f64,
) -> f64 {
    let prof_vec: Vec<_> = volume_profile.profile.clone().into();
    let a = prof_vec.windows(2).par_bridge().map(|window| {
        let n = window[1];
        let m = window[0];
        utils::proportional_choose(n + m, m, proportionality)
    });

    // let values = a.clone().collect::<Vec<_>>();

    // println!("{:?}", values);
    a.reduce(|| 1f64, |a, b| a * b)
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
            let value = num_cdts_in_profile_proportional(&vp, 10e-29) as usize;
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
    // let partition = weighted_random_partition(time_size, volume / 2);
    let partition = constrained_sum_sample_pos(time_size, volume / 2);

    // not calculculating the id leads to a ~10% speedup (IF COLLECTING IN TO A HASH TABLE)
    VolumeProfile::new(partition)
    // VolumeProfile {
    //     profile: partition,
    //     id: 0,
    // }
}
