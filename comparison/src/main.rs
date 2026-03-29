use itertools::iproduct;
use std::io::{stdout, Write};
use std::thread;
use std::sync::{Arc, Mutex};

use comparison::gather;
use comparison::constants::{CARDINALITIES, DATA_SIZE_MULTIPLIES, PRECISIONS};

fn main() {
    println!("Gathering results...");

    // take dataset specifications based on all combinations of
    // (cardinality, data_size) using the constants from constants.rs;
    // datasets of size larger than a billion are ignored
    let data_sizes: Vec<_> = iproduct!(CARDINALITIES, DATA_SIZE_MULTIPLIES).filter(|(card, mult)| card * mult <= 1_000_000_000).collect();
    let no_datasets = data_sizes.len();

    // prepare the handles
    let mut handles = Vec::new();

    // create the counter for experiments done or in progress
    let in_progress_all = Arc::new(Mutex::new(0));

    // calculate the number of threads
    let no_threads = PRECISIONS.len();

    // gather the results; split the gatherer into threads based on precision
    for prec in PRECISIONS {
        // total number of experiments
        let total_experiments = no_datasets * no_threads;

        // clone the data iterators
        let data_sizes = data_sizes.clone();

        // get a shared reference to the counter of completed experiments
        let in_progress = Arc::clone(&in_progress_all);

        // create the thread
        let handle = thread::Builder::new()
            .name(format!("Thread prec={}", prec))
            .spawn(move || {
            for (card, mult) in data_sizes {
                // update the datasets-in-progress counter
                {
                    let mut count = in_progress.lock().unwrap();
                    *count += 1;
                    print!("\rin progress: {}/{}; ", count, total_experiments);
                    stdout().flush().unwrap();
                }

                // gather results
                gather(prec, card, card * mult).unwrap_or_else(|e| panic!("Failed gathering data: {}\n", e));
            }
        }).unwrap();

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    println!();
}
