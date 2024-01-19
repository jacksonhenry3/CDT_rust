// run this with cargo r -r --example random_sample_from_large_volume

use cdt_rust::volume_profiles::{
    self, acceptance_function, generate_sample_profile, step, volume_profile_samples,
    volume_profiles, VolumeProfile,
};

use cdt_rust::{self, cdt, utils, volume_profiles::weighted_random_partition};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let side_length = 16;
    let initial_volume_profile = VolumeProfile::new(vec![side_length; side_length].into());
    let initial_volume_profile = generate_sample_profile(initial_volume_profile, 1000);

    let mut samples = volume_profile_samples(initial_volume_profile, side_length * 2, 100_000);

    println!("Sample generated");

    let vol = side_length * side_length * 2;

    let path = format!("data/Volume_{}_statistical.csv", vol);
    let mut f = File::create(path).unwrap();
    let mut w = std::io::BufWriter::new(&mut f);

    println!("Calculating actions of {}", samples.len());
    let actions = samples.par_iter_mut().enumerate().map(|(i, mut vp)| {
        // println!("calculating action for the {:?} vp", i);
        vp.to_canonical_order();

        //  print the progress
        if i % 1000 == 0 {
            println!("{}", i);
        }

        // generate a random CDT with volumeprofile vp
        let cdt = cdt::CDT::random(vp.clone());
        let action = cdt_rust::rsqrd_action(&cdt);

        let vol_prof = vp.profile.make_contiguous().iter().join(":");
        (vol_prof, action)

        // vp
    });

    println!("Saving to file");
    for (vol_prof, action) in actions.collect::<Vec<_>>() {
        writeln!(w, "{:?},{}", vol_prof, action).unwrap();
    }

    // let cdts = cdt::random_sample(20, 20 * 20, 100);
    // println!("{} cdts", cdts.len());
    // let cdts = cdts.iter().map(|cdt| {
    //     println!("{:?}", cdt.volume_profile());
    //     let action = cdt_rust::rsqrd_action(&cdt);
    //     (cdt.volume_profile(), action)
    // });

    // let path = format!("data/Volume_{}.csv", 20 * 20);
    // let mut f = File::create(path).unwrap();
    // let mut w = std::io::BufWriter::new(&mut f);

    // for (vp, s) in cdts {
    //     let vol_prof = vp.iter().join(":");
    //     writeln!(w, "{:?},{}", vol_prof, s).unwrap();
    // }
}
