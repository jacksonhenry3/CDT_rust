#![allow(unused)]
use cdt_rust::{action, cdt::CDT};
use rayon::prelude::*;



fn get_random_action() -> (CDT, f64){
    let random_cdt = CDT::random(vec![32;32]);
    let s = action(&random_cdt);
    (random_cdt, s)
}

fn main() {

    // println!("Hello, world!");
    // for i in (0..10_000){
    //     get_random_action();
    // }

    println!("Middle");

    //now with rayon
    let a = (0..1_000_000).into_par_iter().map(|_| get_random_action());
    let b = a.collect::<Vec<(CDT, f64)>>();
    
    //find the max of b 
    let mut max = -10000.0;
    let mut max_cdt = CDT::random(vec![32;32]);
    let mut i = 0;
    for (cdt, s) in b.clone(){
        i += 1;
        if i%10000 == 0{
            println!("i: {}", i);
        }
        if s > max{
            max = s;
            max_cdt = cdt;
        }
    }
    

    println!("Max: {}", max);
    println!("Max CDT: {}", max_cdt);

    //and min
    let mut min = 10000.0;
    let mut min_cdt = CDT::random(vec![32;32]);
    let mut i = 0;

    for (cdt, s) in b{
        i += 1;
        if i%10000 == 0{
            println!("i: {}", i);
        }
        if s < min{
            min = s;
            min_cdt = cdt;
        }
    }

    println!("Min: {}", min);
    println!("Min CDT: {}", min_cdt);
    

    println!("End");
    
}
