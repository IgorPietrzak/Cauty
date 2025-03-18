use crate::infection::infection_set;
use std::os::raw::c_int;
use crate::nauty;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::Graph;

pub struct Algorithm {
    pub orbit_reps: Vec<Vec<usize>>,      // All orbit representatives
    pub current_reps: Vec<Vec<usize>>,    // Reps at current level
    pub n: usize,
    infection_set: Vec<Vec<usize>>,       // Colourings to process
    pub infected: usize,                  // Current number of 1s
    pub graph: Graph,
}

impl Algorithm {
    pub fn init(graph: Graph) -> Self {
        let n = graph.n;
        let current_reps: Vec<Vec<usize>> = vec![vec![0; n]];
        let orbit_reps: Vec<Vec<usize>> = vec![vec![0; n]]; // Start with all-0s
        let infection_set = infection_set(&current_reps); // Fixed typo from your original
        let infected = 1;

        Self {
            orbit_reps,
            current_reps,
            n,
            infection_set,
            infected,
            graph,
        }
    }

    pub fn run(&mut self) -> usize {
        while (self.infected as f32) <= ((self.n / 2) as f32).floor() {
            self.run_level();
        }
        self.invert_colourings();
        self.orbit_reps.len() // Return total number of orbits
    }

    pub fn run_level(&mut self) {
        println!("Level {}: {} infection candidates {:?}", 
                 self.infected, self.infection_set.len(), self.infection_set);
        self.get_current_reps();
        self.orbit_reps.extend(self.current_reps.clone());
        self.reset_level();
    }

    fn reset_level(&mut self) {
        self.infection_set = infection_set(&self.current_reps);
        self.current_reps = Vec::new();
        self.infected += 1;
    }

    fn get_current_reps(&mut self) {
        let mut canon_map: HashMap<Vec<nauty::setword>, Vec<usize>> = HashMap::new();
        
        for coloring in &self.infection_set {
            let colors: Vec<c_int> = coloring.iter().map(|&x| x as c_int).collect();
            let (canon, _stats) = self.graph.run_nauty_with_coloring(&colors);
            canon_map.entry(canon).or_insert_with(|| coloring.clone());
        }

        self.current_reps = canon_map.into_values().collect();
        println!("Added {} reps at level {}: {:?}", 
                 self.current_reps.len(), self.infected, self.current_reps);
    }

    fn invert_colourings(&mut self) {
        let mut flipped_colours: Vec<Vec<usize>> = Vec::new();
        let mut seen_colours = HashSet::new();

        for row in self.orbit_reps.iter() {
            seen_colours.insert(row.clone());
        }

        for row in self.orbit_reps.iter() {
            let flipped_row: Vec<usize> = row.iter().map(|&value| value ^ 1).collect();
            if !seen_colours.contains(&flipped_row) {
                flipped_colours.push(flipped_row.clone());
                seen_colours.insert(flipped_row);
            }
        }

        self.orbit_reps.extend(flipped_colours);
    }
}



