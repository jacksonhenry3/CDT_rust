#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cdt_rust::{cdt::CDT, volume_profiles, Slab};

fn becnhmark_random_cdt(c: &mut Criterion) {
    c.bench_function("random_cdt", |b| {
        // put the volume profile in a black box
        let volume_profile = black_box(volume_profiles::VolumeProfile::new(vec![20; 20]));
        b.iter(|| {
            let cdt = CDT::random(&volume_profile);
        })
    });
}

fn benchmark_vp_step(c: &mut Criterion) {
    c.bench_function("vp_step", |b| {
        // put the volume profile in a black box
        let volume_profile = black_box(volume_profiles::VolumeProfile::new(vec![20; 20]));
        let cdt = CDT::random(&volume_profile);
        b.iter(|| {
            let new_profile = volume_profiles::step(&volume_profile);
        });
    });
}

// fn becnchmark_vp_acceptance_function(c: &mut Criterion) {
//     c.bench_function("vp_acceptance_function", |b| {
//         // put the volume profile in a black box
//         let old_profile = black_box(volume_profiles::VolumeProfile::new(vec![20; 20]));

//         let mut new_profile = vec![10; 10];
//         new_profile.extend(vec![30; 10]);

//         let new_profile = black_box(volume_profiles::VolumeProfile::new(new_profile));
//         let old_profile_multiplicity =
//             black_box(volume_profiles::log_num_cdts_in_profile(old_profile.clone()).1);
//         b.iter(|| {
//             let result = volume_profiles::acceptance_function(
//                 old_profile.clone(),
//                 old_profile_multiplicity,
//                 new_profile.clone(),
//             );
//         });
//     });
// }

fn benchmark_log_num_cdts_in_profile(c: &mut Criterion) {
    c.bench_function("log_num_cdts_in_profile", |b| {
        // put the volume profile in a black box
        let volume_profile = black_box(volume_profiles::VolumeProfile::new(vec![20; 20]));
        b.iter(|| {
            let result = volume_profiles::log_num_cdts_in_profile(volume_profile.clone());
        });
    });
}

// benchmark addition for refrence
fn bench_addition(c: &mut Criterion) {
    c.bench_function("addition", |b| b.iter(|| 2 + 2));
}

// benchmark volume_profile_samples
fn benchmark_volume_profile_samples(c: &mut Criterion) {
    c.bench_function("volume_profile_samples", |b| {
        // put the volume profile in a black box
        let volume_profile = black_box(volume_profiles::VolumeProfile::new(vec![20; 20]));
        let num_iterations = 100;
        let num_samples = 100;
        b.iter(|| {
            let result =
                volume_profiles::generate_sample_profile(volume_profile.clone(), num_iterations);
        });
    });
}

criterion_group!(
    benches,
    // becnhmark_random_cdt,
    // bench_addition,
    // benchmark_vp_step,
    // becnchmark_vp_acceptance_function,
    // benchmark_log_num_cdts_in_profile,
    benchmark_volume_profile_samples
);

criterion_main!(benches);
