// run this with cargo r -r --example random_sample_from_large_volume

use cdt_rust::volume_profiles::{generate_sample_profile, volume_profile_samples, VolumeProfile};
use cdt_rust::{self, cdt};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;

fn main() {
    // Parameters
    let side_length = 256; // Length of each side of the volume
    let num_samples = 100_000; // Number of samples to generate
    let num_iterations = 1000; // Number of iterations to generate the initial volume profile

    // Generate initial volume profile
    let initial_volume_profile = VolumeProfile::new(vec![side_length; side_length].into());
    let initial_volume_profile = generate_sample_profile(initial_volume_profile, num_iterations);

    // Generate volume profile samples
    let mut samples = volume_profile_samples(initial_volume_profile, side_length * 2, num_samples);

    println!("Sample generated");

    // Calculate volume
    let vol = side_length * side_length * 2;

    // Create file for saving results
    let path = format!("data/Volume_{}_statistical.csv", vol);
    let mut f = File::create(path).unwrap();
    let mut w = std::io::BufWriter::new(&mut f);

    println!("Calculating actions");

    // Calculate actions in parallel
    let actions = samples.par_iter_mut().map(|vp| {
        vp.to_canonical_order();

        // Generate a random CDT with volumeprofile vp
        let cdt = cdt::CDT::random(vp.clone());
        let action = cdt_rust::rsqrd_action(&cdt);

        let vol_prof = vp.profile.make_contiguous().iter().join(":");
        (vol_prof, action)
    });

    println!("Saving to file");

    // Write results to file
    for (vol_prof, action) in actions.collect::<Vec<_>>() {
        writeln!(w, "{:?},{}", vol_prof, action).unwrap();
    }
}
