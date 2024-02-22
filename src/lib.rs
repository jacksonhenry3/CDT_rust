pub mod cdt;
pub mod slab;
pub mod utils;
pub mod volume_profiles;
use cdt::CDT;

pub use slab::Slab;

use crate::slab::all_slabs;

#[derive(Debug, Eq, PartialOrd, Ord, Clone, PartialEq, Hash, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub fn number_of_triangles_around_a_node(
    cdt: &CDT,
    time_index: usize,
    space_index: usize,
    direction: Direction,
) -> i32 {
    // all nodes have at least one spatial edge
    let mut result = 1;

    let triangle_value = cdt[time_index][space_index];

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
                if space_index < space_size - 1 {
                    Some(space_index + 1)
                } else {
                    None
                }
            }
        }
    };

    //count the number of spatial neighbors to the right are of a different type
    let space_size = cdt.slabs[time_index].len();
    let mut next_space_index_option = next_index(space_index, space_size);

    while let Some(next_space_index) = next_space_index_option {
        // println!("next_space_index: {} {}", time_index, next_space_index);

        result += 1;
        if cdt[time_index][next_space_index] == triangle_value {
            break;
        }
        next_space_index_option = next_index(next_space_index, space_size);
    }

    // if next_space_index_option.is_none() {
    //     result += 1;
    // }

    //get the temporal pair of the node
    if let Some((other_time_index, other_space_index)) =
        cdt.get_temporal_pair(time_index, space_index)
    {
        result += 1;
        let other_space_size = cdt.slabs[other_time_index].len();
        let mut other_next_space_index_option = next_index(other_space_index, other_space_size);

        while let Some(other_next_space_index) = other_next_space_index_option {
            // println!("next_space_index: {} {}", other_time_index, other_next_space_index);

            result += 1;
            if cdt[other_time_index][other_next_space_index]
                == cdt[other_time_index][other_space_index]
            {
                break;
            }
            other_next_space_index_option = next_index(other_next_space_index, other_space_size);
        }
    }

    // below is needed if thinking about number of edges instead of number of triangles.
    // if boundary add one
    // let is_space_boundary = cdt.slabs[time_index].is_boundary(space_index, direction);
    // let is_time_boundary = (time_index == 0 && triangle_value)
    //     || (time_index == cdt.time_size() - 1 && !triangle_value);

    // let is_boundary = is_space_boundary || is_time_boundary;

    result
}

// deficite angle
pub fn deficite_angle(cdt: &CDT, time_index: usize, space_index: usize, side: Direction) -> f64 {
    let number_of_edges = number_of_triangles_around_a_node(cdt, time_index, space_index, side);

    //figure out if the node is on spatial or temporal boundary
    let is_spatial_boundary = cdt.slabs[time_index].is_boundary(space_index, side);
    let triangle_value = cdt[time_index][space_index];

    let is_temporal_boundary = (time_index == 0 && triangle_value)
        || (time_index == cdt.time_size() - 1 && !triangle_value);

    let expected_number_of_triangles = match (is_spatial_boundary, is_temporal_boundary) {
        (true, true) => 6.,
        (true, false) => 6.,
        (false, true) => 6.,
        (false, false) => 6.,
    };

    // match (is_spatial_boundary, is_spatial_boundary) {
    //     (true, true) => println!("is spatial and temporal boundary"),
    //     (true, false) => println!("is spatial boundary"),
    //     (false, true) => println!("is temporal boundary"),
    //     (false, false) => println!("is not boundary"),
    // }

    // println!("{} {}", number_of_edges, expected_number_of_triangles);
    (number_of_edges as f64) - expected_number_of_triangles // * std::f64::consts::PI / 3.0
}

//create a cdt iterator that iterates over all possible cdt with a given volume profile using slab_iterator
pub fn cdt_iterator(volume_profile: Vec<usize>) -> impl Iterator<Item = CDT> {
    //assert that every element in the volume profile is less than 128 and greater than 3
    assert!(
        volume_profile.iter().all(|x| *x <= 128),
        "volume profile must be less than 128"
    );

    //create a vector of slab iterators where number of zeros is the next element in the volume profile
    let mut slab_iterators = vec![];

    for (i, num_zeros) in volume_profile.iter().enumerate() {
        let num_ones = volume_profile[(i + 1).rem_euclid(volume_profile.len())];
        slab_iterators.push(all_slabs(*num_zeros, num_ones));
    }

    let mut current_slabs = vec![];
    for slab in &mut slab_iterators {
        current_slabs.push(slab.next().unwrap());
    }

    slab_iterators[0] = all_slabs(volume_profile[0], volume_profile[1]);

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
                    volume_profile[i],
                    volume_profile[(i + 1).rem_euclid(volume_profile.len())],
                );
                current_slabs[i] = slab_iterators[i].next().unwrap();
            }
        }

        Some(CDT::new(current_slabs.clone()))
    })
}

pub fn eh_action(cdt: &CDT) -> f64 {
    //calculate the einstien hilbert action of the cdt
    let mut result = 0f64;
    let lambda = 0f64;
    //sum the deficite angles of all nodes, all nodes are here identified as all lower right nodes of true triangles
    let nodes = cdt
        .triangles()
        .into_iter()
        .filter(|(_x, _t, value)| *value)
        .collect::<Vec<_>>();

    // add in the top row of false triangles to nodes
    let time_size: usize = cdt.len() - 1;
    let a = cdt[time_size]
        .into_iter()
        .enumerate()
        .map(|(a, b)| (time_size, a, *b))
        .filter(|(_, _, value)| !*value)
        .collect::<Vec<_>>();

    for (time_index, space_index, _value) in nodes.into_iter().chain(a.into_iter()) {
        let num_adj_tris =
            number_of_triangles_around_a_node(cdt, time_index, space_index, Direction::Right);

        // println!("{} {} {} {}", time_index, space_index, _value, num_adj_tris);

        // if cdt.slabs[time_index].is_boundary(space_index, Direction::Right) {
        //     num_adj_tris -= 1;
        // }
        let area = num_adj_tris as f64 / 3.0;

        result +=
            area * (deficite_angle(cdt, time_index, space_index, Direction::Right) / area - lambda);

        let triangle_index = cdt.get_triangle_index(time_index, space_index);
        if triangle_index == 0 {
            let num_adj_tris =
                number_of_triangles_around_a_node(cdt, time_index, space_index, Direction::Left)
                    as f32;
            let area = num_adj_tris as f64 / 3.0;

            result += area
                * (deficite_angle(cdt, time_index, space_index, Direction::Left) / area - lambda);
        }
    }

    result
}
