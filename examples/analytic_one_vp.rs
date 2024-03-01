// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::volume_profiles::{num_cdts_in_profile, VolumeProfile};
use cdt_rust::{self};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    // Parameters
    let volume_profile = VolumeProfile::new(vec![4, 4, 4]);
    let volume_profile_string = volume_profile.profile.iter().join("_");

    // Create file for saving results
    let path = format!("data/Volume_{:?}_statistical.csv", volume_profile.profile);
    let mut f = File::create(path).unwrap();
    let mut w = std::io::BufWriter::new(&mut f);

    println!("Calculating actions");

    let progress_counter = AtomicUsize::new(0);

    let cdts = cdt_rust::cdt_iterator(volume_profile.profile.clone());

    println!("calculating the length of the iterator");
    let length = num_cdts_in_profile(&volume_profile);
    println!("length of the iterator is {}", length);

    // Calculate actions in parallel
    let actions = cdts.par_bridge().map(|cdt| {
        let progress = progress_counter.fetch_add(1, Ordering::SeqCst);
        let progress_percent = 100.0 * progress as f64 / (length as f64);
        print!("\r{:.2}%", progress_percent);

        // Generate a random CDT with volumeprofile vp

        cdt_rust::r_sqrd_action(&cdt)
    });

    println!("Saving to file");

    // Write results to file
    for action in actions.collect::<Vec<_>>() {
        writeln!(w, "{:?},{}", volume_profile_string, action).unwrap();
    }
}
