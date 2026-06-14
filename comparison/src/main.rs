use ahash::random_state::RandomState;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use gumbel_estimation::{NaiveGumbel, PadeGumbel, BasicBitHackGumbel, CorrectedBitHackGumbel, SplineGumbel, LUTGumbel};
use std::io;
use rayon::prelude::*;

use comparison::{run_hll, run_ghll, run_ghllplus, run_ghllplusfloat, get_transform_name, load_data, save_to_file, Algorithm, Transform};

#[derive(Parser, Debug)]
#[command(author, version, about = "Gumbel HLL Thesis Comparison Tool")]
struct Args {
    /// Cardinalities to test (e.g. 10000, 20000, ...;
    /// defaults to 10000 through 100000 with 10000 step)
    #[arg(short, long, value_delimiter = ',', default_values_t = vec![10000, 20000, 30000, 40000, 50000, 60000, 70000, 80000, 90000, 100000])]
    cardinalities: Vec<usize>,

    /// Data size multiplier (e.g. 10 for small datasets, 100 for large; defaults to 100)
    #[arg(short, long, default_value_t = 100)]
    multiplier: usize,

    /// Precisions to test (e.g. 4, 8, 12, 16; defaults to these four values)
    #[arg(short, long, value_delimiter = ',', default_values_t = [4, 8, 12, 16])]
    precisions: Vec<u8>,

    /// Number of independent iterations for each estimator (defaults to 100)
    #[arg(short, long, default_value_t = 100)]
    iterations: usize,

    /// Algorithms to test (e.g. hll, ghll, ghllplus, ghllplusfloat; defaults to all of them)
    #[arg(short, long, value_enum, value_delimiter = ',', default_values_t = vec![Algorithm::Hll, Algorithm::Ghll, Algorithm::GhllPlus, Algorithm::GhllPlusFloat])]
    algorithms: Vec<Algorithm>,

    /// Gumbel Transforms to test (e.g. icdf, pade, basic-bithack, corrected-bithack, spline, lut; defaults to all of them)
    #[arg(short, long, value_enum, value_delimiter = ',', default_values_t = vec![Transform::Icdf, Transform::Pade, Transform::BasicBithack, Transform::CorrectedBithack, Transform::Spline, Transform::LUT])]
    transforms: Vec<Transform>,

}

struct IterationResults {
    hll: Option<f64>,
    ghll: Vec<(Transform, (f64, f64))>,
    ghll_plus: Vec<(Transform, f64)>,
    ghll_plus_float: Vec<(Transform, f64)>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let multi = MultiProgress::new();

    let total_steps = (args.cardinalities.len() * args.precisions.len()) as u64;

    // Prepare metadata
    let main_pb = multi.add(ProgressBar::new(total_steps));
    main_pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan} {pos}/{len} {msg}").unwrap());

    for &card in &args.cardinalities {
        let mult = args.multiplier;
        let size = card * mult;
        let data = load_data(card, size)?;

        for &prec in &args.precisions {
            main_pb.set_message(format!("Processing p={}, n={}...", prec, card));
            
            // Create deterministic hash builders
            let builders: Vec<_> = (0..args.iterations).map(|i| RandomState::with_seeds(i as u64, 0, 0, 0)).collect();

            // UNIFIED PARALLEL LOOP: Run all algorithms for each builder
            let all_results: Vec<IterationResults> = builders.into_par_iter().map(|builder| {
                let mut res = IterationResults {
                    hll: None,
                    ghll: Vec::new(),
                    ghll_plus: Vec::new(),
                    ghll_plus_float: Vec::new(),
                };

                if args.algorithms.contains(&Algorithm::Hll) {
                    res.hll = Some(run_hll(prec, &data, &builder));
                }

                for &t_enum in &args.transforms {
                    macro_rules! run_for_transform {
                        ($t_obj:expr) => {
                            let t = $t_obj;
                            if args.algorithms.contains(&Algorithm::Ghll) {
                                res.ghll.push((t_enum, run_ghll(prec, &data, &builder, t)));
                            }
                            if args.algorithms.contains(&Algorithm::GhllPlus) {
                                res.ghll_plus.push((t_enum, run_ghllplus(prec, &data, &builder, t)));
                            }
                            if args.algorithms.contains(&Algorithm::GhllPlusFloat) {
                                res.ghll_plus_float.push((t_enum, run_ghllplusfloat(prec, &data, &builder, t)));
                            }
                        };
                    }

                    match t_enum {
                        Transform::Icdf => { run_for_transform!(NaiveGumbel::default()); }
                        Transform::Pade => { run_for_transform!(PadeGumbel::default()); }
                        Transform::BasicBithack => { run_for_transform!(BasicBitHackGumbel::default()); }
                        Transform::CorrectedBithack => { run_for_transform!(CorrectedBitHackGumbel::default()); }
                        Transform::Spline => { run_for_transform!(SplineGumbel::default()); }
                        Transform::LUT => { run_for_transform!(LUTGumbel::default()); }
                    }
                }
                res
            }).collect();

            // SAVE RESULTS: Pivot the collected data back to files
            if args.algorithms.contains(&Algorithm::Hll) {
                let hll_vec: Vec<_> = all_results.iter().map(|r| r.hll.unwrap()).collect();
                save_to_file("HLL", prec, card, size, hll_vec)?;
            }

            for &t_enum in &args.transforms {
                let t_name = get_transform_name(t_enum);
                
                if args.algorithms.contains(&Algorithm::Ghll) {
                    let geo: Vec<_> = all_results.iter().map(|r| r.ghll.iter().find(|(te, _)| *te == t_enum).unwrap().1.0).collect();
                    let har: Vec<_> = all_results.iter().map(|r| r.ghll.iter().find(|(te, _)| *te == t_enum).unwrap().1.1).collect();
                    save_to_file(&format!("GHLLGeo_{}", t_name), prec, card, size, geo)?;
                    save_to_file(&format!("GHLLHar_{}", t_name), prec, card, size, har)?;
                }
                if args.algorithms.contains(&Algorithm::GhllPlus) {
                    let vals: Vec<_> = all_results.iter().map(|r| r.ghll_plus.iter().find(|(te, _)| *te == t_enum).unwrap().1).collect();
                    save_to_file(&format!("GHLLPlus_{}", t_name), prec, card, size, vals)?;
                }
                if args.algorithms.contains(&Algorithm::GhllPlusFloat) {
                    let vals: Vec<_> = all_results.iter().map(|r| r.ghll_plus_float.iter().find(|(te, _)| *te == t_enum).unwrap().1).collect();
                    save_to_file(&format!("GHLLPlusFloat_{}", t_name), prec, card, size, vals)?;
                }
            }
            main_pb.inc(1);
        }
    }

    main_pb.finish_with_message("Done!");
    Ok(())
}
