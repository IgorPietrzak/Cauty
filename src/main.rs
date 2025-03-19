mod nauty;
mod algorithm;
mod infection;
mod graph;

use algorithm::Cauty;
use graph::Graph;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GraphInput {
    name: String,
    vertices: usize,
    edges: Vec<(usize, usize)>,
}

#[derive(Serialize)]
struct ResultOutput {
    name: String,
    vertices: usize,
    orbits: usize,
    runtime_ms: u128,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <graph_input.json>", args[0]);
        return;
    }

    let filename = &args[1];
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let graph_input: GraphInput = serde_json::from_str(&contents).expect("Invalid JSON format");

    let graph = Graph::new(graph_input.vertices, &graph_input.edges);
    let mut algo = Cauty::init(graph);
    let start = Instant::now();
    let orbit_count = algo.run();
    let duration = start.elapsed().as_millis();

    let result = ResultOutput {
        name: graph_input.name,
        vertices: graph_input.vertices,
        orbits: orbit_count,
        runtime_ms: duration,
    };

    let output = serde_json::to_string(&result).expect("Failed to serialize output");
    println!("{}", output);
}

