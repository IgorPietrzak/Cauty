use rayon::prelude::*; //multithreading lib

pub fn infection_set(current_reps: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut infection_set = Vec::new();
    for colouring in current_reps {
        infection_set.extend(build_infection(colouring));
    }
    infection_set
}

fn build_infection(colouring: &Vec<usize>) -> Vec<Vec<usize>> {
    let parallel_colourings: Vec<Vec<usize>> = (0..colouring.len())
        .into_par_iter() // creates parallel iterator.
        .filter_map(|i| {
            if colouring[i] == 0 {
                let mut new_colouring = colouring.clone(); // We only clone when we need to flip 0 to 1.
                new_colouring[i] = 1;
                Some(new_colouring)
            } else {
                None // Do nothing when encounter a 1.
            }
        })
        .collect();

    parallel_colourings
}

#[cfg(test)]
mod test {
    use super::infection_set;
    #[test]
    fn test_infection() {
        let current_reps: Vec<Vec<usize>> = vec![vec![0,0,0]];
        let infection_set = infection_set(&current_reps);
        println!("{:?}",infection_set);
    } 


}


















