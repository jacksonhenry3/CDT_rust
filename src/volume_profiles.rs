use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};
use xxhash_rust::xxh3::xxh3_64;

use crate::utils;

//derive eq
// probably dont need calc id when running a large sim, consider directly geenerating vp instead of using new.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct VolumeProfile {
    pub profile: Vec<usize>,
}
// Now that there is a canonical form here, a lot can be simplified.

impl VolumeProfile {
    pub fn new(profile: Vec<usize>) -> VolumeProfile {
        VolumeProfile { profile }
    }
}

pub fn step(volume_profile: &VolumeProfile) -> VolumeProfile {
    // WARNING the id of the returned volume profile is not calculated
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

    // VolumeProfile::new(profile)
    VolumeProfile { profile } // we shouldnt need to calculate ID here, and this speeds things up.
}

pub fn acceptance_function(
    old_profile: VolumeProfile,
    new_profile: VolumeProfile,
) -> VolumeProfile {
    let old_num_cdts = num_cdts_in_profile(&old_profile);
    let new_num_cdts = num_cdts_in_profile(&new_profile);

    let acceptance = new_num_cdts as f64 / old_num_cdts as f64;

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

    let mut progress_counter = AtomicUsize::new(0);

    println!("Generating samples");
    let samples: Vec<Vec<VolumeProfile>> = (0..num_threads)
        .into_par_iter()
        .map(|i| {
            let progress = progress_counter.fetch_add(1, Ordering::SeqCst);
            let progress_percent = 100.0 * progress as f64 / (num_threads * chunk_size) as f64;
            print!("\r{:.2}%", progress_percent);

            let start_index = i * chunk_size;
            let end_index = if i == num_threads - 1 {
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
    for thread_sample in samples.into_iter() {
        result.extend(thread_sample);
    }

    result
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
