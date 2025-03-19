mod nauty;
mod algorithm;
mod infection;
mod graph;

use std::ptr;
use std::os::raw::c_int;
use algorithm::Cauty;
use graph::Graph;

fn main() {
    let tests = [
        ("K_2", 2, vec![(0, 1)], 3),
        ("P_3", 3, vec![(0, 1), (1, 2)], 6),
        ("P_4", 4, vec![(0, 1), (1, 2), (2, 3)], 10),
        ("C_4", 4, vec![(0, 1), (1, 2), (2, 3), (3, 0)], 6),
        ("K_3", 3, vec![(0, 1), (1, 2), (2, 0)], 4),
        ("S_4", 4, vec![(0, 1), (0, 2), (0, 3)], 8),
        ("K_{2,2}", 4, vec![(0, 2), (0, 3), (1, 2), (1, 3)], 6),
        ("2K_2", 4, vec![(0, 1), (2, 3)], 6),
        ("P_5", 5, vec![(0, 1), (1, 2), (2, 3), (3, 4)], 20),
        ("K_4", 4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)], 5),
    ];

    for (name, n, edges, expected) in tests.iter() {
        println!("Testing {} with n={} and edges {:?}", name, n, edges);
        let graph = Graph::new(*n, edges);
        let mut algo = Cauty::init(graph);
        let orbit_count = algo.run();
        println!("Computed orbits: {}, Expected: {}, {}", orbit_count, expected, 
                 if orbit_count == *expected { "PASS" } else { "FAIL" });
        println!("---");
    }
}
