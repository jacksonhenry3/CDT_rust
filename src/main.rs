use cdt_rust::r#move::{DecreaseMove, IncreaseMove, Move, ParityMove};
use cdt_rust::slab::{self, all_slabs};
use cdt_rust::{action, cdt_iterator, Slab};
use cdt_rust::{cdt::CDT, deficite_angle, number_of_triangles_arround_a_node};
use grafferous;
use itertools::Itertools;
use rand::prelude::*;
use rand::{thread_rng, Rng};



fn main() {
    let random_cdt = CDT::random(vec![32; 32]);

    let s = action(&random_cdt);
    println!("action: {}", s);
}

fn spatial_multiplicity(cdt: &CDT) -> i32 {
    let g = cdt.to_graph();
    //get all nodes that have 0 time coordinate
    let nodes = g.nodes.iter().filter(|n| n.0 == 0);
    //sum all circuit starting at each node using an iterator map
    nodes
        .map(|n| grafferous::find_circuits(&g, n, 32).len() as i32)
        .sum()
}

fn tuple_to_mathematica_format(t: (i32, i32)) -> String {
    format!("{{{},{}}}", t.0, t.1)
}

fn graph_to_mathematica_format(graph: grafferous::Graph<(i32, i32), ()>) -> () {
    let mut result = String::new();
    result.push_str("Graph[{");

    for (from, tos) in graph.edges {
        for to in tos {
            result.push_str(&format!(
                "{}->{},",
                tuple_to_mathematica_format(from),
                tuple_to_mathematica_format(to)
            ));
        }
    }
    //remove the last comma
    result.pop();

    result.push_str("}]");
    println!("{}", result);
}
