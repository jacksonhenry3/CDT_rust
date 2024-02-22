// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::volume_profiles::VolumeProfile;
use cdt_rust::{self, cdt};

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    for i in 1..10e5 as usize {
        // print out every multiple of 1000
        if i % 1000 == 0 {
            println!("{}", i);
        }

        let v: Vec<_> = (0..5).map(|_| rng.gen_range(3..41)).collect();
        let temporal_boundary = v[0] + v[v.len() - 1];
        let space_boundary = (v.len()) * 2;

        let cdt = cdt::CDT::random(&VolumeProfile::new(v.clone()));
        // let cdt = cdt::CDT::new_flat(space_size, time_size);

        let s = cdt_rust::eh_action(&cdt);
        let volume = cdt.volume_profile().profile.iter().sum::<usize>() as f64;
        let action = s + (space_boundary as f64) * 3.0 + (temporal_boundary as f64) * 3.0;
        if action.abs() > 1e-10 {
            println!(
                "Volume = {}, vp = {:?},action = {}",
                volume,
                cdt.volume_profile(),
                s + (space_boundary as f64) * 3.0 + (temporal_boundary as f64) * 3.0
            );
        }
    }
}
