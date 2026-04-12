use itertools::iproduct;
use rayon::prelude::*;
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use comparison::{gather, load_data};
use comparison::constants::{CARDINALITIES, DATA_SIZE_MULTIPLIES, PRECISIONS, TRANSFORMS};

fn main() {
    println!("Gathering results...");

    // Filter all specified data sizes where total number of elements is at most 1 billion
    let data_sizes: Vec<_> = iproduct!(CARDINALITIES.iter().copied(), DATA_SIZE_MULTIPLIES.iter().copied())
        .filter(|(card, mult)| card * mult <= 1_000_000_000)
        .collect();

    // Calculate total "work units" (total elements processed across all experiments)
    let total_elements: usize = data_sizes.iter()
        .map(|(c, m)| c * m * PRECISIONS.len() * TRANSFORMS.len())
        .sum();

    let total_tasks = data_sizes.len() * PRECISIONS.len() * TRANSFORMS.len();
    let completed_tasks = AtomicUsize::new(0);
    let processed_elements = AtomicUsize::new(0);
    let start_time = Instant::now();

    for (card, mult) in data_sizes {
        let size = card * mult;
        let data = load_data(card, size).unwrap_or_else(|e| panic!("Failed loading data: {}", e));

        let tasks: Vec<_> = iproduct!(PRECISIONS.iter().copied(), TRANSFORMS.iter()).collect();

        tasks.into_par_iter().for_each(|(prec, &(transform, name, include_hll))| {
            gather(
                prec, card, size,
                &data,
                transform, name,
                include_hll,
            ).unwrap_or_else(|e| panic!("Failed evaluating {} (prec {}): {}", name, prec, e));
            
            // Weighted progress tracking
            let tasks_done = completed_tasks.fetch_add(1, Ordering::SeqCst) + 1;
            let elements_done = processed_elements.fetch_add(size, Ordering::SeqCst) + size;
            
            let elapsed = start_time.elapsed().as_secs_f64();
            // ETA is now based on throughput (elements per second)
            let throughput = elements_done as f64 / elapsed;
            let remaining_elements = total_elements - elements_done;
            let eta = remaining_elements as f64 / throughput;
            
            let bar_width = 25;
            let progress_pct = (elements_done as f64 / total_elements as f64).min(1.0);
            let bar_filled = (progress_pct * bar_width as f64) as usize;
            
            let bar: String = (0..bar_width)
                .map(|i| if i < bar_filled { '=' } else if i == bar_filled { '>' } else { '-' })
                .collect();

            print!(
                "\r[{}] {:>3.1}% | Tasks: {}/{} | Elapsed: {:.0}s | ETA: {:.0}s    ", 
                bar,
                progress_pct * 100.0,
                tasks_done, 
                total_tasks, 
                elapsed, 
                if elements_done == 0 { 0.0 } else { eta }
            );
            let _ = stdout().flush();
        });
    }

    println!("\nDone!");
}
