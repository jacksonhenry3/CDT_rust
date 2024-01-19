//ignor unusued
#![allow(dead_code)]

use cached::proc_macro::cached;


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

#[cached]
pub fn choose(n: usize, k: usize) -> usize {
    if k == 0 {
        return 1;
    }
    if k > n {
        return 0;
    }
    match choose(n - 1, k - 1).checked_add(choose(n - 1, k)) {
        Some(x) => x,
        None => usize::MAX,
    }
}

pub fn binomial_coefficient(n: u64, k: u64) -> u64 {
    let mut dp = vec![vec![0u64; k as usize + 1]; n as usize + 1];

    for i in 0..=n as usize {
        for j in 0..=std::cmp::min(i, k as usize) {
            if j == 0 || j == i {
                dp[i][j] = 1;
            } else {
                dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
            }
        }
    }

    dp[n as usize][k as usize]
}

pub fn proportional_choose(n: usize, k: usize, scale_factor: f64) -> f64 {
    let mut dp = vec![vec![0f64; k + 1]; n + 1];

    for i in 0..=n {
        for j in 0..=std::cmp::min(i, k) {
            if j == 0 || j == i {
                dp[i][j] = scale_factor;
            } else {
                dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
            }
        }
    }

    dp[n][k]
}

pub(crate) fn ln_choose(n: u64, k: u64) -> f64 {
    let mut result = 0.0;
    for i in 0..k {
        result += (n as f64 - i as f64).ln() - (k as f64 - i as f64).ln();
    }
    result
}

// fn histogram(volume: usize) -> HashMap<usize, u32> {
//     let profiles = volume_profiles(volume);
//     let mut counts = HashMap::new();
//     for profile in profiles.into_iter().flatten() {
//         let count = num_cdts_in_profile(profile);
//         *counts.entry(count).or_insert(0) += 1;
//     }
//     counts
// }

// pub fn write_histogram_to_file(volume: usize) {
//     let histogram = histogram(volume);
//     let mut sorted = histogram.into_iter().collect::<Vec<_>>();
//     sorted.sort_by_key(|&(n, _)| n);
//     let _ = std::fs::write(
//         format!("Histogram {volume}"),
//         sorted
//             .into_iter()
//             .map(|(num, freq)| format!("{num}: {freq}"))
//             .collect::<Vec<_>>()
//             .join("\n"),
//     );
// }
