// algorithm.rs
use crate::infection::infection_set;
use std::os::raw::c_int;
use crate::nauty;
use std::collections::HashMap;
use crate::Graph;

pub struct Algorithm {
    pub orbit_reps: HashMap<usize, Vec<Vec<usize>>>,
    pub current_reps: Vec<Vec<usize>>,
    pub n: usize,
    infection_set: Vec<Vec<usize>>,
    pub infected: usize,
    pub graph: Graph,
}

impl Algorithm {
    pub fn init(graph: Graph) -> Self {
        let n = graph.n;
        let initial_rep = vec![0; n];
        let mut orbit_reps: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
        orbit_reps.insert(0, vec![initial_rep.clone()]);
        
        let flipped_initial: Vec<usize> = initial_rep.iter().map(|&x| x ^ 1).collect();
        orbit_reps.insert(n, vec![flipped_initial]);

        let current_reps: Vec<Vec<usize>> = vec![initial_rep];
        let infection_set = infection_set(&current_reps);
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
        let total = self.orbit_reps.values().map(|v| v.len()).sum::<usize>();
        total
    }

    pub fn run_level(&mut self) {
        self.get_current_reps();

        for rep in self.current_reps.clone() {
            let k = self.infected;
            self.add_rep(rep.clone(), k);

            let flipped: Vec<usize> = rep.iter().map(|&x| x ^ 1).collect();
            let flipped_count = self.n - k;
            self.add_rep(flipped, flipped_count);
        }

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
    }

    fn add_rep(&mut self, coloring: Vec<usize>, count: usize) {
        let colors: Vec<c_int> = coloring.iter().map(|&x| x as c_int).collect();
        let (canon, _stats) = self.graph.run_nauty_with_coloring(&colors);

        let reps = self.orbit_reps.entry(count).or_insert_with(Vec::new);
        let is_new = !reps.iter().any(|rep| {
            let rep_colors: Vec<c_int> = rep.iter().map(|&x| x as c_int).collect();
            let (rep_canon, _) = self.graph.run_nauty_with_coloring(&rep_colors);
            rep_canon == canon
        });

        if is_new {
            reps.push(coloring);
        }
    }
}


