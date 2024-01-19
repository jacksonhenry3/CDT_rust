// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::volume_profiles::{generate_sample_profile, volume_profile_samples, VolumeProfile};
use cdt_rust::{self, cdt};
use core::num;
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    // Parameters
    let side_length = 256; // Length of each side of the volume
    let num_samples = 100_000; // Number of samples to generate

    println!("Generating initial volume profile");

    // Generate initial volume profile
    let chosen_volume_profile = VolumeProfile::new(vec![side_length; side_length].into());
    let mut vp = chosen_volume_profile;

    // Calculate volume
    let vol = side_length * side_length * 2;

    // Create file for saving results
    let path = format!("data/Volume_{}_statistical_one_vp.csv", vol);
    let mut f = File::create(path).unwrap();
    let mut w = std::io::BufWriter::new(&mut f);

    println!("Calculating actions");

    let mut progress_counter = AtomicUsize::new(0);
    let length = num_samples;

    // Calculate actions in parallel
    let actions = (0..num_samples).into_par_iter().map(|i| {
        let progress = progress_counter.fetch_add(1, Ordering::SeqCst);
        let progress_percent = 100.0 * progress as f64 / (length as f64);
        print!("\r{:.2}%", progress_percent);

        // Generate a random CDT with volumeprofile vp
        let cdt = cdt::CDT::random(&vp);
        let action = cdt_rust::rsqrd_action(&cdt);

        // let vol_prof = vp.profile.make_contiguous().iter().join(":");
        (action)
    });

    println!("Saving to file");

    // Write results to file
    for (action) in actions.collect::<Vec<_>>() {
        writeln!(w, "{}", action).unwrap();
    }
}
