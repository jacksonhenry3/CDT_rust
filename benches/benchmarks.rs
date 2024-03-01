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

fn becnchmark_vp_acceptance_function(c: &mut Criterion) {
    c.bench_function("vp_acceptance_function", |b| {
        // put the volume profile in a black box
        let old_profile = black_box(volume_profiles::VolumeProfile::new(vec![20; 20]));

        let mut new_profile = vec![10; 10];
        new_profile.extend(vec![30; 10]);

        let new_profile = black_box(volume_profiles::VolumeProfile::new(new_profile));
        b.iter(|| {
            let result =
                volume_profiles::acceptance_function(old_profile.clone(), new_profile.clone());
        });
    });
}

criterion_group!(
    benches,
    becnhmark_random_cdt,
    benchmark_vp_step,
    becnchmark_vp_acceptance_function
);

criterion_main!(benches);
