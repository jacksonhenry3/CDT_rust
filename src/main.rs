//allow dead code
#![allow(dead_code)]
//allow unused variables
#![allow(unused_variables)]
//allow unused imports
#![allow(unused_imports)]
//allow unused mut
#![allow(unused_mut)]

use cdt_rust;
use rand::{thread_rng, Rng};

fn main() {


    //test adding and removing many triangles
    let mut cdt = cdt_rust::CDT::new_flat(10,10);
    let refrence_cdt = cdt.clone();
    let mut rng = thread_rng();

    for i in 0..100 {
        let (time_index, space_index) = cdt.random_triangle();
        cdt.increase_move(time_index, space_index);

        let (time_index, space_index) = cdt.random_triangle();
        cdt.decrease_move(time_index, space_index);

    }

}
