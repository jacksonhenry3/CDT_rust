pub mod cdt;
pub mod r#move;
pub mod slab;

use cdt::CDT;

pub use slab::Slab;

use crate::slab::all_slabs;

pub fn number_of_triangles_arround_a_node(
    cdt: &CDT,
    time_index: usize,
    space_index: usize,
) -> usize {
    let mut result = 4;
    //get the temporal pair of the node
    let (other_time_index, other_space_index) = cdt.get_temporal_pair(time_index, space_index);

    //count the number of spatial neighbors to the right are of a different type
    let mut next_space_index = (space_index + 1) % cdt.slabs[time_index].length;
    while cdt[time_index][next_space_index] != cdt[time_index][space_index] {
        result += 1;
        next_space_index = (next_space_index + 1) % cdt.slabs[time_index].length;
    }

    let mut next_space_index = (other_space_index + 1) % cdt.slabs[other_time_index].length;
    while cdt[other_time_index][next_space_index] != cdt[other_time_index][other_space_index] {
        result += 1;
        next_space_index = (next_space_index + 1) % cdt.slabs[other_time_index].length;
    }

    result
}

// deficite angle
pub fn deficite_angle(cdt: &CDT, time_index: usize, space_index: usize) -> f64 {
    let number_of_edges = number_of_triangles_arround_a_node(cdt, time_index, space_index);
    ((number_of_edges as i64 - 6) as f64) * std::f64::consts::PI / 3.0
}

//create a cdt iterator that iterates over all possible cdt with a given volume profile using slab_iterator
pub fn cdt_iterator(volume_profile: Vec<u32>) -> impl Iterator<Item = CDT> {
    //assert that every element in the volume profile is less than 128 and greater than 3
    assert!(
        volume_profile.iter().all(|x| *x <= 128 && *x >= 3),
        "volume profile must be between 3 and 128"
    );

    //create a vector of slab iterators where number of zeros is the next element in the volume profile
    let mut slab_iterators = vec![];

    for (i, num_zeros) in volume_profile.iter().rev().enumerate() {
        slab_iterators.push(all_slabs(
            volume_profile[(i + 1).rem_euclid(volume_profile.len())],
            *num_zeros,
        ));
    }

    let mut current_slabs = vec![];
    for i in 0..volume_profile.len() {
        current_slabs.push(slab_iterators[i].next().unwrap());
    }

    slab_iterators[0] = all_slabs(
        volume_profile[volume_profile.len() - 1],
        volume_profile[0],
    );


    std::iter::from_fn(move || {
        for i in 0..volume_profile.len() {
            let slab = slab_iterators[i].next();


            if slab.is_some() {
                current_slabs[i] = slab.unwrap();
                break;
            } else {
                if i == volume_profile.len() - 1 {
                    return None;
                }
                slab_iterators[i] = all_slabs(
                    volume_profile[(i + 1).rem_euclid(volume_profile.len())],
                    volume_profile[i],
                );
                current_slabs[i] = slab_iterators[i].next().unwrap();
            }
        }

        Some(CDT::new(current_slabs.clone()))
    })
}


pub fn action(cdt:&CDT) -> f64 {
    //calculate the einstien hilbert action of the cdt 
    let mut result = 0.0;

    //sum the deficite angles of all nodes
    for time_index in 0..cdt.slabs.len() {
        for space_index in 0..cdt.slabs[time_index].length {
            result += deficite_angle(&cdt, time_index, space_index);
        }
    }

    result
}