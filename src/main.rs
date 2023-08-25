#![allow(unused)]
use cdt_rust::{action, cdt::CDT, cdt_iterator, slab::sum_binary_digit_range, volume_profiles};
use rayon::prelude::*;

fn measure_boundaries(cdt:&CDT) -> usize {
    let mut max = f64::NAN;
    let transition_triangles = cdt.all_transition_triangles();
    transition_triangles.iter().count()
}

fn main() {
    
    let cdts= cdt_iterator(vec![128;128]);

    println!("done");

    

    //get the index of the cdt with the most transition triangles
    let max = cdts
        .enumerate()
        .map(|(i, cdt)| (action(&cdt), cdt))
        .max().unwrap();

    let cdts= cdt_iterator(vec![4,4,4]);
    
    let min = cdts
        .enumerate()
        .map(|(i, cdt)| (action(&cdt), cdt))
        .min().unwrap();

    println!("max: {}", max.0);
    println!("{}", max.1);
    
    println!("min: {}", min.0);
    println!("{}", min.1);


    let cdts= cdt_iterator(vec![4,1,1,2,4]);

    println!("done");

    

    //get the index of the cdt with the most transition triangles
    let max = cdts
        .enumerate()
        .map(|(i, cdt)| (action(&cdt), cdt))
        .max().unwrap();

    let cdts= cdt_iterator(vec![4,1,1,2,4]);
    
    let min = cdts
        .enumerate()
        .map(|(i, cdt)| (action(&cdt), cdt))
        .min().unwrap();

    println!("max: {}", max.0);
    println!("{}", max.1);
    
    println!("min: {}", min.0);
    println!("{}", min.1);

    

    //to mathematica format

}
