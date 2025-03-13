//pub struct Algorithm {
//    pub orbit_reps: Vec<Vec<usize>>,
//    pub current_reps: Vec<Vec<usize>>,
//    pub n: usize,
//    infection_set: Vec<Vec<usize>>,
//    pub graph: Graph,
//}
//
//impl Algorithm {
//    pub fn print_current_state(&self) {
//        logger::quiet_log(self);
//    }
//
//    pub fn init(graph: Graph) -> Self {
//        let n = graph.vertices.len();
//        let current_reps: Vec<Vec<usize>> = vec![vec![0; n]];
//        let orbit_reps: Vec<Vec<usize>> = vec![vec![0; n]]; // trivial colouring at level 0.
//        let infection_set = infection_set(&current_reps);
//        let infected = 1;
//
//        Self {
//            orbit_reps,
//            current_reps,
//            n,
//            pi,
//            infection_set,
//            infected,
//            graph,
//        }
//    }
//
//    // MAIN_LOOP:
//    pub fn run(&mut self) {
//        while (self.infected as f32) <= ((self.n / 2) as f32).floor() {
//            self.run_level();
//        }
//
//        self.invert_colourings(); // swap 1s and 0s.
//        self.print_current_state();
//    }
//
//    // EACH_LEVEL:
//    pub fn run_level(&mut self) {
//        self.print_current_state();
//        self.test_colourings(); // populate current_reps based off this.
//        self.orbit_reps.extend(self.current_reps.clone()); // add this levels reps to orbit_reps.
//        self.reset_level();
//    }
//
//    fn reset_level(&mut self) {
//        self.infection_set = infection_set(&self.current_reps);
//        self.current_reps = Vec::new();
//        self.infected += 1;
//    }
//
//    fn test_colourings(&mut self) {
//       // NAUTY canonical labelling called from here 
//    }
//
//    fn invert_colourings(&mut self) {
//        let mut flipped_colours: Vec<Vec<usize>> = Vec::new();
//        let mut seen_colours = HashSet::new();
//
//        // Add the original colourings to the HashSet so we can check against them
//        for row in self.orbit_reps.iter() {
//            seen_colours.insert(row.clone()); // Insert original colourings into the HashSet
//        }
//
//        // Iterate over each row in orbit_reps and flip the values
//        for row in self.orbit_reps.iter() {
//            let flipped_row: Vec<usize> = row.iter().map(|&value| value ^ 1).collect(); // XOR flip
//
//            // Check if the flipped colouring is already in orbit_reps
//            if !seen_colours.contains(&flipped_row) {
//                flipped_colours.push(flipped_row.clone()); // Add flipped row to the new collection
//                seen_colours.insert(flipped_row); // Mark it as seen
//            }
//        }
//
//        // Now extend self.orbit_reps with the flipped colourings
//        self.orbit_reps.extend(flipped_colours);
//    }
//}
