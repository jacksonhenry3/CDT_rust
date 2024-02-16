// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::volume_profiles::{generate_sample_profile, volume_profile_samples, VolumeProfile};
use cdt_rust::{self, cdt};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let cdt = cdt::CDT::random(&VolumeProfile::new(vec![10, 10, 3, 10, 10].into()));

    println!("{}", cdt);
    println!("{:?}", cdt_rust::eh_action(&cdt));
}
