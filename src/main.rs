#![allow(unused)]
use std::collections::{HashMap, HashSet};
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

fn main() {
    let volume = 100;
    // let a = constrained_sum_sample_pos(32, 32 * 32);
    //generate a million constrained sum samples using rayon
    let now = Instant::now();
    let samples: Vec<_> = (0..100)
        .into_par_iter()
        .map(|_| random_volume_profile(128 * 128, 128))
        .collect();

    let elapsed_time = now.elapsed();
    println!(" {} ms.", elapsed_time.as_millis());

    // printem all
    //make a hashset of samples
    let mut sample_set: HashSet<VolumeProfile> = samples.into_par_iter().collect();
    for sample in &sample_set {
        println!("{:?}", sample);
    }
    println!("Number of samples: {}", sample_set.len());

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
