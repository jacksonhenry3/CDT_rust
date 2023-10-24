// run this with cargo r -r --example random_sample_from_large_volume
use cdt_rust::{self, utils, volume_profiles::weighted_random_partition};

fn main() {
    let a: Vec<_> = (0..10_000)
        .map(|_| weighted_random_partition(4, 10))
        .collect();

    fcv
}
