#![allow(unused)]
use core::panic;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use cdt_rust::{
    action, cdt::CDT, cdt_iterator, slab::sum_binary_digit_range, utils::choose,
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
    let a = volume_profiles(20).flatten();

    // hasmap of volume profiles
    let mut results = HashMap::new();
    for volume_profile in a {
        let mut count = 1;

        for (i, layer_size) in volume_profile.profile.iter().enumerate() {
            let next_layer_size = volume_profile.profile[(i + 1) % volume_profile.profile.len()];

            // use the choose function (binomial coefecient) of layer_size and next_layer_size
            count *= choose(*layer_size + next_layer_size, next_layer_size);
        }

        results.insert(volume_profile.profile, count);
    }

    println!("{:?}", results);
}
