// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::volume_profiles::{generate_sample_profile, volume_profile_samples, VolumeProfile};
use cdt_rust::{self, cdt};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    // Parameters
    let volume = 100; // Volume of the CDT
    let time_size = 10;
    let num_samples = 1_000_000; // Number of samples to generate

    // Number of iterations between samples, it should be a sweep?
    let num_iterations = 2 * ((volume as f32).sqrt() as usize);

    println!("Generating initial volume profile");

    // Generate initial volume profile by creating a vec of size time_size with each element equal to volume/time_size, except for the last element to enforce the volume constraint
    let mut initial_volume_profile = vec![volume / time_size; time_size];
    initial_volume_profile[time_size - 1] = volume - (volume / time_size) * (time_size - 1);

    let initial_volume_profile = generate_sample_profile(
        VolumeProfile::new(initial_volume_profile),
        num_iterations * 5, //initialize with 5 times the number of iterations to make sure we are starting from a random spot. Shouldn't be needed, but makes me feel better.
    );

    println!("Initial volume profile generated, beginning sample generation");

    // Generate volume profile samples
    let mut samples = volume_profile_samples(initial_volume_profile, num_iterations, num_samples);

    println!("Volume profile samples generated");

    // Create file for saving results
    let path = format!(
        "data/Statistical_Volume_{}_TimeSize_{}.csv",
        volume, time_size
    );

    let mut f = File::create(path).unwrap();
    let mut w = std::io::BufWriter::new(&mut f);

    println!("Calculating actions");

    let progress_counter = AtomicUsize::new(0);
    let length = samples.len();

    // Calculate actions in parallel
    let actions = samples.par_iter_mut().map(|vp| {
        let progress = progress_counter.fetch_add(1, Ordering::SeqCst);
        let progress_percent = 100.0 * progress as f64 / (length as f64);

        io::stdout().flush().unwrap();
        print!("\r{:.2}%", progress_percent);

        // Generate a random CDT with volumeprofile vp
        let cdt = cdt::CDT::random(vp);
        let action = cdt_rust::r_sqrd_action(&cdt);

        let volume_profile_string = vp.profile.iter().join(":");
        (volume_profile_string, action)
    });

    println!("Saving to file");

    // Write results to file
    for (volume_profile_string, action) in actions.collect::<Vec<_>>() {
        writeln!(w, "{:?},{}", volume_profile_string, action).unwrap();
    }
}
