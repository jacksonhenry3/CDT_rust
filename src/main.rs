//allow dead code
#![allow(dead_code)]
//allow unused variables
#![allow(unused_variables)]
//allow unused imports
#![allow(unused_imports)]
//allow unused mut
#![allow(unused_mut)]

use cdt_rust::cdt::CDT;
use rand::{thread_rng, Rng};

fn main() {
    //test adding and removing many triangles
    // let mut cdt = cdt_rust::CDT::new_flat(10, 10);
    // let refrence_cdt = cdt.clone();
    // let mut rng = thread_rng();
    let mut random_cdt = CDT::random(vec![10,10,10,10,10]);


    for i in 0..100_000 {
        //check if random is valid

        //check if random is valid
        assert!(random_cdt.is_valid());
        
        //get all transition triangles
        let transition_triangles = random_cdt.all_transition_triangles();

        //check if all transition triangles are valid
        for (time_index, space_index) in transition_triangles {
            let triangle = random_cdt[time_index][space_index];
            let (other_time_index, other_space_index) = random_cdt.get_temporal_pair(time_index, space_index);
            let other_triangle = random_cdt[other_time_index][other_space_index];

            assert_ne!(triangle, other_triangle);
        }

        let (time_index,space_index) = random_cdt.random_transisition_triangle();

        random_cdt.parity_move(time_index, space_index);

        
    }
}
