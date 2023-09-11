//ignor unusued
#![allow(dead_code)]

use crate::cdt::CDT;

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

fn graph_to_mathematica_format(graph: grafferous::Graph<(i32, i32), ()>) {
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

pub fn choose(n: usize, k: usize) -> usize {
    if k == 0 {
        return 1;
    }
    if k > n {
        return 0;
    }
    choose(n - 1, k - 1) + choose(n - 1, k)
}
