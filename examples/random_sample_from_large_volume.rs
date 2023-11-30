///
use cdt_rust::volume_profiles::{self, volume_profiles};
// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::{self, utils, volume_profiles::weighted_random_partition};
use itertools::Itertools;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    let a: Vec<_> = (0..10_000)
        .map(|_| weighted_random_partition(3, 6))
        .collect();

    let mut partitions = Vec::new();

    for sample in a.iter() {
        let volume_profile = volume_profiles::VolumeProfile::new(sample.clone().into());

        for i in 0..(3 / volume_profile.temporal_multiplicity()) {
            partitions.push(sample.clone());
        }
    }

    let a = partitions;

    // save the samples to a file
    let path = format!("data/weighted_random_partition.csv");
    let mut f = std::fs::File::create(path).unwrap();
    let mut w = std::io::BufWriter::new(&mut f);

    for sample in a {
        let sample_str = sample.iter().join(",");

        // add curly braces on either side of the string

        let sample_str = format!("{{{}}},", sample_str);
        // println!("{}", sample_str);

        writeln!(w, "{}", sample_str).unwrap();
    }
}
