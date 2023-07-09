use cdt_rust;
use rand::{thread_rng, Rng};

fn main() {
    //create a flat cdt
    let cdt = cdt_rust::CDT::new_flat(10, 10);

    let time_index = 3;
    let space_index = 5;

    //get the pair of the triangle at time_index, space_index
    let pair = cdt.get_pair(time_index, space_index);

    println!(
        "pair of ({},{}) is ({},{})",
        time_index, space_index, pair.0, pair.1
    );

    println!(
        "triangle index of ({},{}) is {}",
        time_index,
        space_index,
        cdt.get_triangle_index(time_index, space_index)
    );

    println!(
        "triangle index of ({},{}) is {}",
        pair.0,
        pair.1,
        cdt.get_triangle_index(pair.0, pair.1)
    );
    
}
