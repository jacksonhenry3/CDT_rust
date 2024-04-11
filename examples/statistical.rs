// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::utils::write_volume_action_to_csv;
use cdt_rust::volume_profiles::{generate_sample_profile, volume_profile_samples, VolumeProfile};
use cdt_rust::{self, cdt};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    // Parameters
    // for time_size in (10..180).step_by(10) {
    let time_size = 32;
    let volume = 32 * 32; // Volume of the CDT
    let num_samples = 100_000; // Number of samples to generate
    for num_iterations in 10..=10 {
        // Number of iterations between samples, it should be a sweep?
        for sample_index in 0..10 {
            println!("Generating initial volume profile");

            // Generate initial volume profile by creating a vec of size time_size with each element equal to volume/time_size, except for the last element to enforce the volume constraint
            let mut initial_volume_profile = vec![1; time_size];
            initial_volume_profile[0] = volume - 2 * (time_size - 2) - 1;

            let initial_volume_profile = VolumeProfile::new(initial_volume_profile);
            // let initial_volume_profile = generate_sample_profile(
            //     VolumeProfile::new(initial_volume_profile.into()),
            //     num_iterations * 5,
            //     1, //initialize with 5 times the number of iterations to make sure we are starting from a random spot. Shouldn't be needed, but makes me feel better.
            // );

            println!("Initial volume profile generated, beginning sample generation with {} steps between samples", num_iterations);

            // Generate volume profile samples
            let mut samples =
                volume_profile_samples(initial_volume_profile, num_iterations, num_samples, 1);

            println!("Volume profile samples generated");

            // Create file for saving results
            let path = format!(
                "data/Statistical_Volume_{volume}_TimeSize_{time_size}_stepsize_{num_iterations}_sample_{sample_index}.csv",
            );

            println!("Calculating actions");

            let progress_counter = AtomicUsize::new(0);
            let length = samples.len();

            // Calculate actions in parallel
            let actions = samples.par_iter_mut().map(|vp| {
                #[cfg(debug_assertions)]
                {
                    let progress = progress_counter.fetch_add(1, Ordering::SeqCst);
                    let progress_percent = 100.0 * progress as f64 / (length as f64);

                    io::stdout().flush().unwrap();
                    print!("\r actions {:.2}% calculated", progress_percent);
                }

                // Generate a random CDT with volumeprofile vp
                let cdt = cdt::CDT::random(vp);
                let action = cdt_rust::r_sqrd_action(&cdt);
                // let action = cdt_rust::eh_action(&cdt);
                // let action = 0.;

                let volume_profile_string = vp.profile.iter().join("_");
                (volume_profile_string, action)
            });

            println!("Saving to file");

            let data = actions.collect::<Vec<_>>();

            let a = write_volume_action_to_csv(data, &path);
        }
    }
}
