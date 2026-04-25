use gumbel_estimation::{GHLL, TaylorIntegerGumbel};
use ahash::RandomState;

fn main() {
    let p = 12;
    let n = 10000;
    let iterations = 100;
    let builder = RandomState::with_seeds(0, 0, 0, 0);
    
    let mut sum_geo = 0.0;
    let mut sum_har = 0.0;
    
    for i in 0..iterations {
        let mut ghll = GHLL::with_precision(p, builder.clone(), TaylorIntegerGumbel::new()).unwrap();
        for j in 0..n {
            ghll.add(&(i as u64 * 1000000 + j as u64));
        }
        sum_geo += ghll.count_geo();
        sum_har += ghll.count_har();
    }
    
    println!("Taylor Integer (256-byte i8 LUT):");
    println!("  Geo Mean: {}", sum_geo / iterations as f64);
    println!("  Har Mean: {}", sum_har / iterations as f64);
    println!("  Geo Ratio: {}", (sum_geo / iterations as f64) / n as f64);
    println!("  Har Ratio: {}", (sum_har / iterations as f64) / n as f64);
}
