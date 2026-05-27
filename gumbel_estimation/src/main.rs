use gumbel_estimation::{GHLL, ICDFGumbel, PadeGumbel, BasicBitHackGumbel, TaylorBitHackGumbel, SplineGumbel, LUTGumbel, GumbelTransform};
use ahash::RandomState;
use std::time::Instant;

fn run_experiment<G: GumbelTransform + Copy>(name: &str, p: u8, n: usize, iterations: usize, builder: RandomState, g: G) {
    let mut sum_geo = 0.0;
    let mut sum_har = 0.0;
    
    let start = Instant::now();
    for i in 0..iterations {
        let mut ghll = GHLL::with_precision(p, builder.clone(), g).unwrap();
        for j in 0..n {
            ghll.add(&(i as u64 * 1000000 + j as u64));
        }
        sum_geo += ghll.count_geo();
        sum_har += ghll.count_har();
    }
    let duration = start.elapsed();
    
    println!("{}:", name);
    println!("  Time:      {:?}", duration);
    println!("  Geo Mean:  {}", sum_geo / iterations as f64);
    println!("  Har Mean:  {}", sum_har / iterations as f64);
    println!("  Geo Ratio: {}", (sum_geo / iterations as f64) / n as f64);
    println!("  Har Ratio: {}", (sum_har / iterations as f64) / n as f64);
}

fn main() {
    let p = 12;
    let n = 100000;
    let iterations = 10;
    let builder = RandomState::with_seeds(0, 0, 0, 0);
    
    run_experiment("Standard ICDF (f32::ln)", p, n, iterations, builder.clone(), ICDFGumbel::new());
    println!();
    run_experiment("Padé Approximant (Rational + LN_TABLE)", p, n, iterations, builder.clone(), PadeGumbel::new());
    println!();
    run_experiment("Basic Bit-Hack", p, n, iterations, builder.clone(), BasicBitHackGumbel::new());
    println!();
    run_experiment("Taylor Bit-Hack (Branching)", p, n, iterations, builder.clone(), TaylorBitHackGumbel::new());
    println!();
    run_experiment("Linear Spline (1024-entry LUT + FMA)", p, n, iterations, builder.clone(), SplineGumbel::new());
    println!();
    run_experiment("LUT (256-byte i8 table)", p, n, iterations, builder.clone(), LUTGumbel::new());
}
