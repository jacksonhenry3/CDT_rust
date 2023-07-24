pub mod cdt;
pub mod r#move;
pub mod slab;

use cdt::CDT;

pub use slab::Slab;

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
