// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::volume_profiles::volume_profiles;
use cdt_rust::{self, cdt_iterator};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    // Parameters
    let time_size = 5;
    let volume = 20;

    println!("Generating initial volume profile");

    // this could be simplified by changing volume_profiles to output a Vec. Hashset is being weird with par_iter_mut
    let volume_profile_set = volume_profiles(volume, time_size);
    let mut volume_profiles: Vec<_> = volume_profile_set.iter().collect();

    let length = volume_profiles.len();

    println!("Sample generated");

    // Create file for saving results
    let base_path = format!("data/Analytic_Volume_{}_TimeSize_{}/", volume, time_size);
    // Create directory if it doesn't exist
    std::fs::create_dir_all(&base_path).unwrap();

    println!("Calculating actions");

    let progress_counter = AtomicUsize::new(0);

    // Calculate actions in parallel
    volume_profiles.par_iter_mut().for_each(|vp| {
        let progress = progress_counter.fetch_add(1, Ordering::SeqCst);
        let progress_percent = 100.0 * progress as f64 / (length as f64);

        io::stdout().flush().unwrap();
        print!("\r{:.2}%", progress_percent);

        let vol_prof = vp.profile.iter().join("_");
        let path = format!("{}{}.csv", base_path, vol_prof);

        let mut f = File::create(path).unwrap();
        let mut w = std::io::BufWriter::new(&mut f);

        // Generate all CDTs with the given volume profile
        let cdts = cdt_iterator(vp.profile.clone());
        for cdt in cdts {
            let action = cdt_rust::r_sqrd_action(&cdt);
            writeln!(w, "{:?},{}", vol_prof, action).unwrap();
        }
    });
}
