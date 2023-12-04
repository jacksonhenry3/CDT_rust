///
use cdt_rust::volume_profiles::{self, volume_profiles};
// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::{self, cdt, utils, volume_profiles::weighted_random_partition};
use itertools::Itertools;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let cdts = cdt::random_sample(32, 32 * 32, 1000);
    println!("{} cdts", cdts.len());
    let cdts = cdts.iter().map(|cdt| {
        let action = cdt_rust::rsqrd_action(&cdt);
        (cdt.volume_profile(), action)
    });

    let path = format!("data/Volume_{}.csv", 32 * 32);
    let mut f = File::create(path).unwrap();
    let mut w = std::io::BufWriter::new(&mut f);

    for (vp, s) in cdts {
        let vol_prof = vp.iter().join(":");
        writeln!(w, "{:?},{}", vol_prof, s).unwrap();
    }
}
