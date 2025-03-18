mod nauty;
mod algorithm;
mod infection;
mod graph;

use std::ptr;
use std::os::raw::c_int;

use algorithm::Algorithm;
use graph::Graph;

fn main() {
    // P_3: 0-1-2
    let edges = vec![(0, 1), (1, 2)];
    let graph = Graph::new(3, &edges);
    let mut algo = Algorithm::init(graph);
    algo.run();
    println!("{:?}", algo.orbit_reps.len());

    }
