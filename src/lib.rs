pub mod cdt;
pub mod slab;
pub mod utils;
use cdt::CDT;

pub use slab::Slab;

use crate::slab::all_slabs;

#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash,Copy)]
pub enum Direction {
    Left,
    Right,
}

pub fn number_of_triangles_arround_a_node(
    cdt: &CDT,
    time_index: usize,
    space_index: usize,
    direction: Direction,

) -> usize {
    //TODO this should probs change.
    let mut result = 2;

    let next_index = |space_index: usize, space_size: usize| -> Option<usize> {
        //return if the next space index is less than or equal to space size
        match direction {
            Direction::Left => {
                if space_index > 0 {
                    Some(space_index - 1)
                } else {
                    None
                }
            }
            Direction::Right => {
                if space_index < space_size-1 {
                    Some(space_index + 1)
                } else {
                    None
                }
            }
        }
    };
    


    //count the number of spatial neighbors to the right are of a different type
    let space_size = cdt.slabs[time_index].length;
    let mut next_space_index_option = next_index(space_index, space_size);

    while let Some(next_space_index) = next_space_index_option {
        // println!("next_space_index: {} {}", time_index, next_space_index);
       

        result += 1;
        if cdt[time_index][next_space_index] == cdt[time_index][space_index] {
            break;
        }
        next_space_index_option = next_index(next_space_index, space_size);
    }

    

    //get the temporal pair of the node
    let (other_time_index, other_space_index) = cdt.get_temporal_pair(time_index, space_index);

    let other_space_size = cdt.slabs[other_time_index].length;
    let mut other_next_space_index_option = next_index(other_space_index, other_space_size);

    while let Some(other_next_space_index) = other_next_space_index_option {
        // println!("next_space_index: {} {}", other_time_index, other_next_space_index);
        
        result += 1;
        if cdt[other_time_index][other_next_space_index] == cdt[other_time_index][other_space_index]
        {
            break;
        }
        other_next_space_index_option = next_index(other_next_space_index, other_space_size);
    }

    result
}

// deficite angle
pub fn deficite_angle(cdt: &CDT, time_index: usize, space_index: usize, side:Direction) -> f64 {
    let number_of_edges = number_of_triangles_arround_a_node(cdt, time_index, space_index,side);
    let mut expected_number_of_edges = 6;
    //figure out if the node is on spatial or temporal boundary
    let is_spatial_boundary = cdt.slabs[time_index].is_boundary(space_index,side);

    if is_spatial_boundary {
        expected_number_of_edges = 3;
    }

    println!("{} {} {:?}", number_of_edges, is_spatial_boundary, side);

    ((number_of_edges as i64 - expected_number_of_edges) as f64) // * std::f64::consts::PI / 3.0
}

//create a cdt iterator that iterates over all possible cdt with a given volume profile using slab_iterator
pub fn cdt_iterator(volume_profile: Vec<u32>) -> impl Iterator<Item = CDT> {
    //assert that every element in the volume profile is less than 128 and greater than 3
    assert!(
        volume_profile.iter().all(|x| *x <= 128 && *x >= 0),
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
    for slab in &mut slab_iterators {
        current_slabs.push(slab.next().unwrap());
    }

    slab_iterators[0] = all_slabs(volume_profile[volume_profile.len() - 1], volume_profile[0]);

    std::iter::from_fn(move || {
        for i in 0..volume_profile.len() {
            let slab_option = slab_iterators[i].next();

            if let Some(slab) = slab_option {
                current_slabs[i] = slab;
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

pub fn action(cdt: &CDT) -> f64 {
    //calculate the einstien hilbert action of the cdt
    let mut result = 0.0;

    //sum the deficite angles of all nodes
    for (time_index, space_index, _value) in cdt
        .triangles()
        .into_iter()
        .filter(|(_x, _t, value)| *value == true)
    {
        result += deficite_angle(cdt, time_index, space_index, Direction::Right); 

        let triangle_index = cdt.get_triangle_index(time_index, space_index);
        if triangle_index == 0 {
            result += deficite_angle(cdt, time_index, space_index, Direction::Left);
        }
    }

    result
}
