#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// import the crate im in for  benchmarking

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

fn benchmark_count(c: &mut Criterion) {
    //use rayon to count the elements in the iterator
    c.bench_function("count", |b| {
        b.iter(|| {
            let mut a = cdt_rust::cdt_iterator(vec![3; 4]);
            let count = a.filter(|x| x.volume_profile()[0] == 3).count();
        })
    });
}

criterion_group!(benches, benchmark_count);

criterion_main!(benches);
