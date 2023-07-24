

use cdt_rust::{cdt::CDT, deficite_angle, number_of_triangles_arround_a_node};
use cdt_rust::r#move::{DecreaseMove, IncreaseMove, ParityMove,Move};
use grafferous;

use rand::{thread_rng, Rng};

fn main() {
    let mut cdt = CDT::new_flat(10, 10);
    let g = cdt.to_graph();
    println!();
    println!();
    println!();
    graph_to_mathematica_format(g);
}

fn tuple_to_mathematica_format(t:(i32,i32)) -> String {
    format!("{{{},{}}}",t.0,t.1)
}



fn graph_to_mathematica_format(graph:grafferous::Graph<(i32,i32),()>) -> () {
    let mut result = String::new();
    result.push_str("Graph[{");
    
    for (from,tos) in graph.edges {
        for to in tos {
            result.push_str(&format!("{}->{},",tuple_to_mathematica_format(from),tuple_to_mathematica_format(to)));
        }
    }
    //remove the last comma
    result.pop();

    result.push_str("}]");
    println!("{}",result);
}