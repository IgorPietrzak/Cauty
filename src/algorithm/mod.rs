use crate::infection::infection_set;
use std::os::raw::c_int;
use crate::nauty;
use std::collections::HashMap;
use crate::Graph;

pub struct Cauty {
    pub orbit_count: usize,
    seen_canons: HashMap<usize, Vec<Vec<nauty::setword>>>, // Dedup per level
    pub current_reps: Vec<Vec<usize>>,   // Temp storage for level processing
    pub n: usize,
    infection_set: Vec<Vec<usize>>,
    pub infected: usize,
    pub graph: Graph,
}

impl Cauty {
    pub fn init(graph: Graph) -> Self {
        let n = graph.n;
        let initial_rep = vec![0; n];
        let flipped_initial: Vec<usize> = initial_rep.iter().map(|&x| x ^ 1).collect();
        let colors_init: Vec<c_int> = initial_rep.iter().map(|&x| x as c_int).collect();
        let colors_flip: Vec<c_int> = flipped_initial.iter().map(|&x| x as c_int).collect();
        let (canon_init, _) = graph.run_nauty_with_coloring(&colors_init);
        let (canon_flip, _) = graph.run_nauty_with_coloring(&colors_flip);
        
        let mut seen_canons: HashMap<usize, Vec<Vec<nauty::setword>>> = HashMap::new();
        seen_canons.insert(0, vec![canon_init]);
        seen_canons.insert(n, vec![canon_flip]);
        let orbit_count = 2; 

        let current_reps: Vec<Vec<usize>> = vec![initial_rep];
        let infection_set = infection_set(&current_reps);
        let infected = 1;

        Self {
            orbit_count,
            seen_canons,
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
        self.orbit_count
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

        let reps = self.seen_canons.entry(count).or_insert_with(Vec::new);
        let is_new = !reps.iter().any(|rep_canon| rep_canon == &canon);

        if is_new {
            reps.push(canon);
            self.orbit_count += 1;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::Graph;
    use std::os::raw::c_int;

    #[test]
    fn test_colourings_chemistry() {
        // Define the 2,3-dichlorobutane graph (8 vertices, 7 edges)
        let vertices = 8;
        let edges = vec![
            (0, 1), // C1-C2
            (1, 2), // C2-C3
            (2, 3), // C3-C4
            (1, 4), // C2-Cl1
            (2, 5), // C3-Cl2
            (1, 6), // C2-H1
            (2, 7), // C3-H2
        ];
        let graph = Graph::new(vertices, &edges);

        // Define the four colourings for vertices 4 and 5 (Cl1, Cl2)
        let colourings: Vec<Vec<c_int>> = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0], // (4, 5) = (0, 0) -> (R,R)
            vec![0, 0, 0, 0, 1, 1, 0, 0], // (4, 5) = (1, 1) -> (S,S)
            vec![0, 0, 0, 0, 0, 1, 0, 0], // (4, 5) = (0, 1) -> Meso (R,S)
            vec![0, 0, 0, 0, 1, 0, 0, 0], // (4, 5) = (1, 0) -> Meso (S,R)
        ];

        // Run nauty for each colouring and collect canonical forms
        let mut canonical_forms = Vec::new();
        for coloring in &colourings {
            let (canon, _stats) = graph.run_nauty_with_coloring(coloring);
            canonical_forms.push((canon, coloring[4], coloring[5]));
        }

        println!("{:?}", canonical_forms);

    }
}
