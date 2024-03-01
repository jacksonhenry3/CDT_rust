use core::panic;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::{self, Write};

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::utils;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct VolumeProfile {
    pub profile: Vec<usize>,
}

impl VolumeProfile {
    pub fn new(profile: Vec<usize>) -> VolumeProfile {
        VolumeProfile { profile }
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

    VolumeProfile { profile }
}

pub fn acceptance_function(
    old_profile: VolumeProfile,
    new_profile: VolumeProfile,
) -> VolumeProfile {
    let log_old_num_cdts = log_num_cdts_in_profile(&old_profile, 10.0);
    let log_new_num_cdts = log_num_cdts_in_profile(&new_profile, 10.0);

    let acceptance = (log_new_num_cdts - log_old_num_cdts).exp();

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

pub fn volume_profile_samples(
    initial_state: VolumeProfile,
    num_steps: usize,
    num_samples: usize,
) -> Vec<VolumeProfile> {
    let progress_counter = AtomicUsize::new(0);

    println!("Generating samples");
    let samples: Vec<VolumeProfile> = (0..num_samples)
        .collect::<Vec<_>>()
        .par_chunks(rayon::current_num_threads())
        .map(|chunk| {
            let mut current_state = initial_state.clone();
            let mut states = Vec::new();

            for _sim_index in chunk {
                let progress = progress_counter.fetch_add(1, Ordering::SeqCst);
                let progress_percent = 100.0 * progress as f64 / num_samples as f64;
                io::stdout().flush().unwrap();
                print!("\r{:.2}%", progress_percent);
                for _ in 0..num_steps {
                    let proposed_vp = step(&current_state);
                    current_state = acceptance_function(current_state, proposed_vp);
                }

                states.push(current_state.clone());
            }

            states
        })
        .flatten()
        .collect();

    println!();

    samples
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
        final_result.insert(VolumeProfile::new(result));
    }

    // println!("{:?} profiles", final_result);
    final_result
}

// #[cached]
pub fn log_num_cdts_in_profile(volume_profile: &VolumeProfile, scale_factor: f64) -> f64 {
    let mut log_count_sum = 0f64;
    let len = volume_profile.profile.len();
    for i in 0..(len - 1) {
        let n = volume_profile.profile[i];
        let m = volume_profile.profile[i + 1];
        log_count_sum += utils::log_choose(n + m, m);
    }
    log_count_sum - scale_factor.ln()
}

pub fn num_cdts_in_profile(volume_profile: &VolumeProfile) -> u128 {
    let mut count = 1u128;
    let len = volume_profile.profile.len();
    for i in 0..len - 1 {
        let n = volume_profile.profile[i];
        let m = volume_profile.profile[i + 1];
        count = match count.checked_mul(utils::choose(n + m, m) as u128) {
            Some(x) => x,
            None => panic!("Overflow"),
        };
    }
    count
}
