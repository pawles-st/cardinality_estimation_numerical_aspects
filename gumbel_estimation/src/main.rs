use gumbel_estimation::{GumbelTransform, TaylorIntegerGumbel, ICDFGumbel, GHLL};
use ahash::RandomState;
use std::time::Instant;

fn main() {
    let iterations: u32 = 10_000_000;
    let s = RandomState::new();
    
    let t_int = TaylorIntegerGumbel::new();
    let t_icdf = ICDFGumbel::new();
    
    let mut ghll_int = GHLL::with_precision(12, s.clone(), t_int).unwrap();
    let mut ghll_icdf = GHLL::with_precision(12, s.clone(), t_icdf).unwrap();
    
    // 1. Measure GHLL + TaylorInteger (Fixed-point path)
    let start = Instant::now();
    for i in 0..iterations {
        ghll_int.add(&i);
    }
    println!("GHLL TaylorInteger (LUT Fixed): {:?}", start.elapsed());

    // 2. Measure GHLL + ICDF (FPU path)
    let start = Instant::now();
    for i in 0..iterations {
        ghll_icdf.add(&i);
    }
    println!("GHLL ICDF (FPU):                {:?}", start.elapsed());
}
