// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::volume_profiles::{generate_sample_profile, volume_profile_samples, VolumeProfile};
use cdt_rust::{self, cdt};
use itertools::Itertools;
use rand::Rng;
use rayon::{prelude::*, vec};
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};


fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..1 {
        let v: Vec<_> = (0..5).map(|_| rng.gen_range(3..41)).collect();
        // let v = vec![6,10, 7, 4, 6];

        let cdt = cdt::CDT::random(&VolumeProfile::new(v.clone()));

        let s = cdt_rust::eh_action(&cdt);
        // if s.abs() > 0.000001 {
        println!("{}", s);
        // }
    }
}
