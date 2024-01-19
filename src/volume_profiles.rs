use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use xxhash_rust::xxh3::xxh3_64;

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
        let mut id: u64 = 0;

        for _ in 0..(profile.len()) {
            profile_rotator.rotate_right(1);
            let hash_order = xxh3_64(profile_rotator.iter().join(":").as_bytes());
            id = id.wrapping_add(hash_order);
        }

        let mut profile_rotator = profile.clone();
        profile_rotator.make_contiguous().reverse();
        for _ in 0..(profile.len()) {
            profile_rotator.rotate_right(1);
            let hash_order = xxh3_64(profile_rotator.iter().join(":").as_bytes());
            id = id.wrapping_add(hash_order);
        }

        VolumeProfile { profile, id }
    }

    pub fn temporal_multiplicity(&self) -> usize {
        // rotate the profile to the right untill its the same as the original
        let mut profile_rotator = self.profile.clone();
        profile_rotator.rotate_right(1);
        let mut count = 1;
        while profile_rotator != self.profile {
            profile_rotator.rotate_right(1);
            count += 1;
        }

        // the *2 comes from temporal reversal.
        // if the profile is the same forward and backward, then the we dont multiply by 2

        // check if the profile is the same forward and backward
        let mut profile_rotator = self.profile.clone();
        profile_rotator.make_contiguous().reverse();

        // check if any rotations are the same as the original
        for _ in 0..(self.profile.len()) {
            profile_rotator.rotate_right(1);
            if profile_rotator == self.profile {
                return count;
            }
        }

        count * 2
    }

    pub fn to_canonical_order(&mut self) {
        //rotate the the profile so that it is in the smallest alphanumeric order
        let mut profile_rotator = self.profile.clone();
        let mut min_profile = self.profile.clone();
        for _ in 0..(self.profile.len()) {
            profile_rotator.rotate_right(1);
            if profile_rotator < min_profile {
                min_profile = profile_rotator.clone();
            }
        }

        let mut profile_rotator = min_profile.clone();
        profile_rotator.make_contiguous().reverse();
        for _ in 0..(self.profile.len()) {
            profile_rotator.rotate_right(1);
            if profile_rotator < min_profile {
                min_profile = profile_rotator.clone();
            }
        }

        self.profile = min_profile;
    }
}

pub fn step(volume_profile: &VolumeProfile) -> VolumeProfile {
    let mut profile = volume_profile.profile.clone();

    // first create a list of integers 1-len and permute them randomly
    let mut rng = thread_rng();
    let mut indices: Vec<usize> = (0..volume_profile.profile.len()).collect();
    indices.shuffle(&mut rng);

    // pair each index with the next index
    let pairs = indices.chunks(2);

    for pair in pairs {
        if pair.len() == 1 {
            break;
        }
        let i = pair[0];
        let j = pair[1];

        // if the perturbation amount is greater than or equal to the size of profile[i] reduce the perturbation to the size of profile[i]-1
        let perturbation_amount = 1.min(profile[i] - 1);

        profile[i] -= perturbation_amount;
        profile[j] += perturbation_amount;
    }

    VolumeProfile::new(profile)
    // VolumeProfile { profile, id: 0 } // we shouldnt need to calculate ID here, and this speeds things up.
}

pub fn acceptance_function(
    old_profile: VolumeProfile,
    new_profile: VolumeProfile,
) -> VolumeProfile {
    // let old_ln_num_cdts = ln_num_cdts_in_profile(&old_profile);
    // let new_ln_num_cdts = ln_num_cdts_in_profile(&new_profile);

    // let ln_acceptance = new_ln_num_cdts - old_ln_num_cdts;
    // let acceptance = ln_acceptance.exp() * (old_profile.temporal_multiplicity() as f64)
    //     / (new_profile.temporal_multiplicity() as f64);

    let old_num_cdts = num_cdts_in_profile(&old_profile);
    let new_num_cdts = num_cdts_in_profile(&new_profile);

    let acceptance = new_num_cdts as f64 / old_num_cdts as f64;
    let acceptance = acceptance * (old_profile.temporal_multiplicity() as f64)
        / (new_profile.temporal_multiplicity() as f64);

    //random number between 0 and 1
    let random_number = rand::thread_rng().gen_range(0.0..1.0);

    if acceptance > random_number {
        new_profile
    } else {
        old_profile
    }
}

pub fn generate_sample_profile(initial_state: VolumeProfile, num_steps: usize) -> VolumeProfile {
    let mut current_state = initial_state;
    for _ in 0..num_steps {
        let proposed_vp = step(&current_state);
        current_state = acceptance_function(current_state, proposed_vp);
    }
    current_state
}

// use rayon par_chunk to generate samples in parallel better
pub fn volume_profile_samples(
    initial_state: VolumeProfile,
    num_steps: usize,
    num_samples: usize,
) -> Vec<VolumeProfile> {
    // use rayon to get the ideal number of threads
    let num_threads = rayon::current_num_threads();
    let chunk_size = num_samples / num_threads;

    let samples: Vec<Vec<VolumeProfile>> = (0..num_threads)
        .into_par_iter()
        .map(|i| {
            let start_index = i * chunk_size;
            let end_index = if i == 9 {
                num_samples
            } else {
                (i + 1) * chunk_size
            };

            let mut thread_samples = Vec::with_capacity(end_index - start_index);

            let mut current_state = initial_state.clone();

            for sim_index in start_index..end_index {
                // print the thread index and sample progress as a percentage
                if i == 0 {
                    print!(
                        "\rThread {} {:.2}%",
                        i,
                        100.0 * (sim_index - start_index) as f64 / (end_index - start_index) as f64
                    );
                }
                for _ in 0..num_steps {
                    let proposed_vp = step(&current_state);
                    current_state = acceptance_function(current_state, proposed_vp);
                }
                thread_samples.push(current_state.clone());
            }

            thread_samples
        })
        .collect();

    // explaoin
    println!();
    println!("Combining samples");
    let num_samples_actual = samples.len();
    let mut result = Vec::with_capacity(num_samples_actual);
    for (i, thread_sample) in samples.into_iter().enumerate() {
        print!("\rCombining thread {} samples", i / num_samples_actual);
        result.extend(thread_sample);
    }

    result
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

    // println!("{:?} profiles", final_result);
    final_result
}

// #[cached]
pub fn num_cdts_in_profile(volume_profile: &VolumeProfile) -> u128 {
    let mut count = 1u128;
    let len = volume_profile.profile.len();
    for i in 0..len {
        let n = volume_profile.profile[i];
        let m = volume_profile.profile[(i + 1) % len];
        count = match count.checked_mul(utils::choose(n + m, m) as u128) {
            Some(x) => x,
            None => u128::MAX,
        };
    }
    count
}

pub fn ln_num_cdts_in_profile(volume_profile: &VolumeProfile) -> f64 {
    let mut count = 0.0;
    let len = volume_profile.profile.len();
    for i in 0..len {
        let n = volume_profile.profile[i] as u64;
        let m = volume_profile.profile[(i + 1) % len] as u64;
        count += utils::ln_choose(n + m, m);
    }
    count
}
// pub fn ln_num_cdts_in_profile(volume_profile: &VolumeProfile) -> f64 {
//     let prof_vec: Vec<_> = volume_profile.profile.clone().into();
//     let a = prof_vec.windows(2).par_bridge().map(|window| {
//         let n = window[1] as u64;
//         let m = window[0] as u64;
//         utils::ln_choose(n + m, m)
//     });

//     a.sum::<f64>()
// }

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
    let mut weights = vec![0.0; n];

    for INDEX in 0..total - n {
        let mut total = 0.0;
        let proportionality = num_cdts_in_profile(&VolumeProfile {
            profile: base.clone().into(),
            id: 0,
        }) as f64;

        for i in 0..n {
            let mut vp: VolumeProfile = VolumeProfile {
                profile: base.clone().into(),
                id: 0,
            };
            vp.profile[i] += 1;
            let ln_value = ln_num_cdts_in_profile(&vp);
            let value = ((ln_value - f64::ln(proportionality)).exp() as f32) * (base[i] as f32);
            weights[i] = value;
            total += value;
        }
        let max_value = weights.iter().cloned().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(max) => Some(x.max(max)),
        });

        let max_value = max_value.unwrap();
        println!("{:?}", max_value);
        println!("{:?}", proportionality);

        weights = weights.iter().map(|x| x / max_value).collect();

        println!("{:?}", weights);

        let builder = WalkerTableBuilder::new(&weights);
        let wa_table = builder.build();
        let r = wa_table.next();
        base[r] += 1;
    }

    base.into()
}

pub fn random_volume_profile(volume: usize, time_size: usize) -> VolumeProfile {
    // let partition = weighted_random_partition(time_size, volume / 2);
    // THIS IS WRONG BECOUSE THE PARTITION DOESNT INCLUDE PARITY CHANGES (it includes 123, but not 132)
    let partition = constrained_sum_sample_pos(time_size, volume / 2);

    // not calculculating the id leads to a ~10% speedup (IF COLLECTING IN TO A HASH TABLE)
    VolumeProfile::new(partition)
    // VolumeProfile {
    //     profile: partition,
    //     id: 0,
    // }
}

pub fn random_sample(volume: usize, time_size: usize, num_samples: usize) -> Vec<VolumeProfile> {
    let a: Vec<_> = (0..num_samples)
        // .into_par_iter()
        .map(|x| {
            println!("{:?}", x);
            weighted_random_partition(time_size, 2 * volume)
        })
        .collect();

    let mut partitions = Vec::new();

    let maximum_temporal_multiplicity = 2 * time_size;

    for sample in a.iter() {
        let volume_profile = VolumeProfile::new(sample.clone().into());

        for _ in 0..(maximum_temporal_multiplicity / &volume_profile.temporal_multiplicity()) {
            partitions.push(volume_profile.clone());
        }
    }
    partitions
}
