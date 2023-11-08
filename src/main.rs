#![allow(unused)]
use cdt_rust::slab;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::time::{self, Instant};
use weighted_rand::builder::*;

use cdt_rust::volume_profiles::{
    constrained_sum_sample_pos, num_cdts_in_profile, random_volume_profile, VolumeProfile,
};
use cdt_rust::{
    cdt::CDT, cdt_iterator, eh_action, slab::sum_binary_digit_range, utils::choose,
    volume_profiles::non_cyclic_permutations, volume_profiles::volume_profiles,
};
use itertools::Itertools;
use rayon::prelude::*;

fn measure_boundaries(cdt: &CDT) -> usize {
    let mut max = f64::NAN;
    let transition_triangles = cdt.all_transition_triangles();
    transition_triangles.iter().count()
}

fn write_data(vol: usize) {
    let path = format!("Volume_{}.csv", vol);
    let mut f = File::create(path).unwrap();
    let mut w = BufWriter::new(&mut f);
    for time in 2..=vol {
        for profile in volume_profiles(vol, time) {
            let profile_vec = profile.profile.iter().map(|&x| x as u32).collect();
            let profile_id = profile.id;
            let vol_prof = profile
                .profile
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(",");
            let num_cdts = num_cdts_in_profile(profile.clone());
            for (i, cdt) in cdt_iterator(profile_vec).enumerate() {
                let id = format!("{}{}", profile_id, i);
                let bin_cdt = cdt
                    .slabs
                    .iter()
                    .zip(&profile.profile)
                    .map(|(slab, width)| format!("{:0>w$b}", slab.data, w = 2 * width))
                    .collect::<Vec<_>>()
                    .join(" ");
                writeln!(
                    w,
                    "{},\"{}\",{},{},{}",
                    id,
                    vol_prof,
                    num_cdts,
                    eh_action(&cdt),
                    bin_cdt
                )
                .unwrap();
            }
        }
    }
}

fn main() {
    let volume = 12;
    write_data(volume);

    // let a = constrained_sum_sample_pos(32, 32 * 32);
    //generate a million constrained sum samples using rayon
    let now = Instant::now();
    let samples = (0..1_000)
        .into_par_iter()
        .map(|_| random_volume_profile(32 * 32, 32));

    samples.count();

    let elapsed_time = now.elapsed();
    println!(" {} ms.", elapsed_time.as_millis());

    // let profiles: Vec<_> = volume_profiles(volume).into_iter().flatten().collect();

    // let mut counts = vec![];
    // for profile in (&profiles).into_iter() {
    //     println!("{:?}", &profile);
    //     let count = num_cdts_in_profile(&profile);
    //     counts.push(count as u32);
    // }

    // let builder = WalkerTableBuilder::new(&counts);
    // let wa_table = builder.build();

    // for i in (0..10).map(|_| wa_table.next()) {
    //     println!("{:?}", &profiles[i]);
    // }

    // let profiles = volume_profiles(volume);

    // println!("Number of profiles: {}", profiles.count());

    // for profile in profiles {
    //     println!("{:?}", profile);
    // }
}
