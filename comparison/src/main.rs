use itertools::iproduct;
use rayon::prelude::*;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use gumbel_estimation::{ICDFGumbel, BitHackGumbel};

use comparison::{gather, load_data};
use comparison::constants::{CARDINALITIES, DATA_SIZE_MULTIPLIES, PRECISIONS};

fn main() {
    println!("Gathering results...");

    // Filter all specified data sizes where total number of elements is at most 1 billion
    let data_sizes: Vec<_> = iproduct!(CARDINALITIES, DATA_SIZE_MULTIPLIES)
        .filter(|(card, mult)| card * mult <= 1_000_000_000)
        .collect();

    // Prepare task progress counter info
    let total_tasks = data_sizes.len();
    let completed_tasks = Arc::new(Mutex::new(0));

    for (card, mult) in data_sizes {
        // Load a dataset
        let size = card * mult;
        let data = load_data(card, size).unwrap_or_else(|e| panic!("Failed loading data: {}", e));

        PRECISIONS.par_iter().for_each(|&prec| {
            // Run ICDF (with HLL)
            gather(
                prec, card, size,
                &data,
                ICDFGumbel::default(), "ICDF",
                true,
            ).unwrap_or_else(|e| panic!("Failed evaluating ICDF (prec {}): {}", prec, e));
            
            // Run BitHack (without HLL)
            gather(
                prec, card, size,
                &data,
                BitHackGumbel::default(), "BitHack",
                false,
            ).unwrap_or_else(|e| panic!("Failed evaluating BitHack (prec {}): {}", prec, e));

        });
        
        // Update progress
        let mut count = completed_tasks.lock().unwrap();
        *count += 1;
        print!("\rCompleted: {}/{}", count, total_tasks);
        let _ = stdout().flush();
    }

    println!("\nDone!");
}
