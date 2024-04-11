#![allow(unused)]

use cdt_rust::eh_action;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::time::{self, Instant};
use weighted_rand::builder::*;

use cdt_rust::volume_profiles::{log_num_cdts_in_profile, num_cdts_in_profile, VolumeProfile};
use cdt_rust::{cdt::CDT, cdt_iterator, utils::choose, volume_profiles::volume_profiles};
use itertools::Itertools;
use rayon::prelude::*;

// This file can be used for testing if it's easier, but final work should be done in the tests folder

// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::utils::write_volume_action_to_csv;
use cdt_rust::volume_profiles::{generate_sample_profile, volume_profile_samples};
use cdt_rust::{self, cdt};
// use itertools::Itertools;
use rayon::prelude::*;
// use std::fs::File;
use std::io::{self};
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let vp1 = VolumeProfile::new(vec![1, 6, 1]);
    let vp2 = VolumeProfile::new(vec![6, 1, 6]);

    println!("{}", log_num_cdts_in_profile(vp1).1);
    println!("{}", log_num_cdts_in_profile(vp2).1);
}
