// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::volume_profiles::{self, VolumeProfile};
use cdt_rust::{self, cdt};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    // Parameters

    let num_samples = 10_000; // Number of samples to generate
                              // let volume_profile = VolumeProfile::new(vec![32; 32]);

    // now create a volume profile with all ones except for the first value which is 32*31
    let mut volume_profile = vec![1; 31];
    volume_profile.append(&mut vec![32 * 32 - 31]);

    let volume_profile = VolumeProfile::new(volume_profile);

    let volume_profile_string = volume_profile.profile.iter().join("_");

    // Create file for saving results
    let path = format!("data/Volume_{:?}_statistical.csv", volume_profile.profile);
    println!("{:?}", path);
    let mut f = File::create(path).unwrap();
    let mut w = std::io::BufWriter::new(&mut f);

    println!("Calculating actions");

    let progress_counter = AtomicUsize::new(0);
    let length = num_samples;

    // Calculate actions in parallel
    let actions = (0..num_samples).into_par_iter().map(|_i| {
        #[cfg(debug_assertions)]
        {
            let progress = progress_counter.fetch_add(1, Ordering::SeqCst);
            let progress_percent = 100.0 * progress as f64 / (length as f64);
            print!("\r{:.2}%", progress_percent);
        }

        // Generate a random CDT with volumeprofile vp
        let cdt = cdt::CDT::random(&volume_profile);

        // let vol_prof = vp.profile.make_contiguous().iter().join(":");
        cdt_rust::r_sqrd_action(&cdt)
    });

    println!("Saving to file");

    // Write results to file
    for action in actions.collect::<Vec<_>>() {
        writeln!(w, "{:?},{}", volume_profile_string, action).unwrap();
    }
}
