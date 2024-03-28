use cached::proc_macro::cached;
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

pub fn step(volume_profile: &VolumeProfile, step_scale: usize) -> VolumeProfile {
    let mut profile = volume_profile.profile.clone();
    let mut rng = thread_rng();

    let mut indices: Vec<usize> = (0..volume_profile.profile.len()).collect();
    indices.shuffle(&mut rng);

    for window in indices.chunks(2) {
        if let [i, j] = window {
            // let perturbation_amount = rng.gen_range(0..=step_scale.min((profile[*i] - 1)));
            // let disp = rng.gen_range(0..=perturbation_amount);
            // let perturbation_amount = perturbation_amount - disp;
            let perturbation_amount = step_scale.min(profile[*i] - 1);
            profile[*i] -= perturbation_amount;
            profile[*j] += perturbation_amount;
        }
    }

    VolumeProfile { profile }
}

pub fn acceptance_function(
    old_profile: VolumeProfile,
    log_old_num_cdts: f64,
    new_profile: VolumeProfile,
) -> (VolumeProfile, f64, bool) {
    let (new_profile, log_new_num_cdts) = log_num_cdts_in_profile(new_profile);

    let acceptance = (log_new_num_cdts - log_old_num_cdts).exp();

    //random number between 0 and 1
    let random_number = rand::thread_rng().gen_range(0.0..1.0);

    if acceptance > random_number {
        (new_profile, log_new_num_cdts, true)
    } else {
        (old_profile, log_old_num_cdts, false)
    }
}

pub fn generate_sample_profile(
    initial_state: VolumeProfile,
    num_steps: usize,
    step_scale: usize,
) -> VolumeProfile {
    let mut current_state = initial_state;
    let mut log_current_multiplicty = log_num_cdts_in_profile(current_state.clone()).1;

    for _ in 0..num_steps {
        let proposed_vp = step(&current_state, step_scale);
        (current_state, log_current_multiplicty, _) =
            acceptance_function(current_state, log_current_multiplicty, proposed_vp);
    }
    current_state
}

pub fn volume_profile_samples(
    initial_state: VolumeProfile,
    num_steps: usize,
    num_samples: usize,
    step_scale: usize,
) -> Vec<VolumeProfile> {
    let progress_counter = AtomicUsize::new(0);
    let accepted_counter = AtomicUsize::new(0);

    let samples: Vec<VolumeProfile> = (0..num_samples)
        .collect::<Vec<_>>()
        .chunks(num_samples / rayon::current_num_threads())
        .map(|chunk| {
            let mut current_state = initial_state.clone();
            let mut log_current_multiplicity = log_num_cdts_in_profile(current_state.clone()).1;
            let mut states: Vec<VolumeProfile> = Vec::new();
            for _ in chunk {
                for _ in 0..num_steps {
                    let proposed_vp = step(&current_state, step_scale);
                    let was_accepted;
                    (current_state, log_current_multiplicity, was_accepted) =
                        acceptance_function(current_state, log_current_multiplicity, proposed_vp);

                    #[cfg(debug_assertions)]
                    {
                        if was_accepted {
                            accepted_counter.fetch_add(1, Ordering::Relaxed);
                        }

                        let progress = progress_counter.fetch_add(1, Ordering::Relaxed);
                        let accepted = accepted_counter.load(Ordering::Relaxed);

                        let progress_percent =
                            100.0 * progress as f64 / (num_samples * num_steps) as f64;

                        let acceptance_ratio = 100.0 * accepted as f64 / ((progress) as f64);
                        io::stdout().flush().unwrap();
                        print!(
                            " \r{:.2}% complete, with acceptance ratio {:.2}% {} ",
                            progress_percent, acceptance_ratio, progress
                        );
                    }
                }

                states.push(current_state.clone());
            }

            states
        })
        .flatten()
        .collect();
    #[cfg(debug_assertions)]
    {
        println!();
    }

    samples
}

pub fn volume_profiles(volume: usize, time_size: usize) -> HashSet<VolumeProfile> {
    let total_spatial_length = volume;

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

#[cached]
pub fn log_num_cdts_in_profile(volume_profile: VolumeProfile) -> (VolumeProfile, f64) {
    let scale_factor = 0.001 as f64;
    let mut log_count_sum = 0f64;
    for window in volume_profile.profile.windows(2) {
        let n = window[0];
        let m = window[1];
        log_count_sum += utils::log_choose(n + m, m);
    }
    (volume_profile, log_count_sum - scale_factor.ln())
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
