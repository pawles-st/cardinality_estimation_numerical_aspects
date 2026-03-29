use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn generate(out: &mut File, card: usize, size: usize) -> io::Result<()> {
    let mut rng = thread_rng();
    let unif_elem = Uniform::new(0, u64::MAX);
    let unif_index = Uniform::new(0, card);

    let mut universe = HashSet::new();
    while universe.len() < card {
        let elem = rng.sample(unif_elem);
        universe.insert(elem);
    }

    universe.iter().try_for_each(|elem| {
        writeln!(out, "{}", elem)
    })?;

    let universe_vec: Vec<u64> = universe.into_iter().collect();
    let no_duplicates = size - card;
    (0..no_duplicates).try_for_each(|_| {
        let index = rng.sample(unif_index);
        let elem = universe_vec[index];
        writeln!(out, "{}", elem)
    })?;

    Ok(())
}
