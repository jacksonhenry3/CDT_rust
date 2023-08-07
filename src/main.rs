#![allow(unused)]
use cdt_rust::{action, cdt::CDT, cdt_iterator};
use rayon::prelude::*;

fn get_random_action() -> (CDT, f64) {
    let random_cdt = CDT::random(vec![20; 20]);
    let s = action(&random_cdt);
    (random_cdt, s)
}

fn main() {
    let mut cdts = (0..1_000_000)
        .into_par_iter()
        .map(|_| get_random_action().1);

    println!("done");

    let a = cdts.count();
    println!("{}", a);
}
