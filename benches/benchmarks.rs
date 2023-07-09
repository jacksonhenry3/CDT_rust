#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// import the crate im in for  benchmarking
use cdt_rust;

fn benchmark_slab(c: &mut Criterion) {
    let a = cdt_rust::Slab::new(vec![true; 128]);

    //set
    c.bench_function("slab_set", |b| {
        b.iter(|| {
            let mut a = a.clone();
            a.set(black_box(0), black_box(true));
        })
    });

    //get
    c.bench_function("slab_get", |b| {
        b.iter(|| {
            let mut a = a.clone();
            a[black_box(0)];
        })
    });
}

criterion_group!(benches, benchmark_slab);

criterion_main!(benches);
