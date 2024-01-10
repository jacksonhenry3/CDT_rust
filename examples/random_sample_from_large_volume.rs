// run this with cargo r -r --example random_sample_from_large_volume

use cdt_rust::volume_profiles::{
    self, acceptance_function, generate_sample_profile, step, volume_profile_samples,
    volume_profiles, VolumeProfile,
};

use cdt_rust::{self, cdt, utils, volume_profiles::weighted_random_partition};
use itertools::Itertools;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let initial_volume_profile = VolumeProfile::new(vec![3, 3, 4].into());

    println!("{:?}", initial_volume_profile);
    let initial_volume_profile = generate_sample_profile(initial_volume_profile, 1000);

    println!(
        "Initial volume profile generated {:?}",
        initial_volume_profile
    );
    let samples = volume_profile_samples(initial_volume_profile, 10, 63_872);

    println!("Sample generated");

    let vol = 20;

    let path = format!("data/Volume_{}_statistical.csv", vol);
    let mut f = File::create(path).unwrap();
    let mut w = std::io::BufWriter::new(&mut f);

    for mut vp in samples {
        vp.to_canonical_order();
        let vol_prof = vp.profile.make_contiguous().iter().join(":");
        writeln!(w, "{:?},{}", vol_prof, 0).unwrap();
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
