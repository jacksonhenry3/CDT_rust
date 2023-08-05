#![allow(unused)]
use cdt_rust::{action, cdt::CDT, cdt_iterator};
use rayon::prelude::*;



fn get_random_action() -> (CDT, f64){
    let random_cdt = CDT::random(vec![32;32]);
    let s = action(&random_cdt);
    (random_cdt, s)
}

fn main() {

    let mut cdt = CDT::new_flat(32,32);
    let mut s = action(&cdt);
    println!("s: {}", s);

}
