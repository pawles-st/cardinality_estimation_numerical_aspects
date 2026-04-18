use ahash::random_state::RandomState;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use gumbel_estimation::{ICDFGumbel, BitHackGumbel};
use std::io;

use comparison::{gather_hll, gather_ghll, gather_ghllreal, gather_ghllplus, get_transform_name, load_data, save_to_file, Algorithm, Transform};

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

    /// Algorithms to test (e.g. hll, ghll, ghllreal, ghllplus; defaults to all of them)
    #[arg(short, long, value_enum, value_delimiter = ',', default_values_t = vec![Algorithm::Hll, Algorithm::Ghll, Algorithm::GhllReal, Algorithm::GhllPlus])]
    algorithms: Vec<Algorithm>,

    /// Gumbel Transforms to test (e.g. icdf, bithack; defaults to all of them)
    #[arg(short, long, value_enum, value_delimiter = ',', default_values_t = vec![Transform::Icdf, Transform::Bithack])]
    transforms: Vec<Transform>,

}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let multi = MultiProgress::new();

    // Prepare metadata
    let main_pb = multi.add(ProgressBar::new(args.cardinalities.len() as u64));
    main_pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan} {pos}/{len} {msg}").unwrap());

    for &card in &args.cardinalities {
        // Load the dataset
        let mult = args.multiplier;
        let size = card * mult;
        main_pb.set_message(format!("Loading {}...", card));
        let data = load_data(card, size)?;

        for &prec in &args.precisions {
            // Create hash builders for each iteration
            let builders: Vec<_> = (0..args.iterations).map(|_| RandomState::new()).collect();

            // Run HLL
            if args.algorithms.contains(&Algorithm::Hll) {
                let res = gather_hll(prec, &data, &builders);
                save_to_file("HLL", prec, card, size, res)?;
            }

            // Run Gumbel variants
            for &t_enum in &args.transforms {
                let t_name = get_transform_name(t_enum);

                macro_rules! run_variants {
                    ($transform:expr) => {
                        let t = $transform;
                        if args.algorithms.contains(&Algorithm::Ghll) {
                            let (geo, har) = gather_ghll(prec, &data, &builders, t);
                            save_to_file(&format!("GHLLGeo_{}", t_name), prec, card, size, geo)?;
                            save_to_file(&format!("GHLLHar_{}", t_name), prec, card, size, har)?;
                        }
                        if args.algorithms.contains(&Algorithm::GhllReal) {
                            let (geo, har) = gather_ghllreal(prec, &data, &builders, t);
                            save_to_file(&format!("GHLLRealGeo_{}", t_name), prec, card, size, geo)?;
                            save_to_file(&format!("GHLLRealHar_{}", t_name), prec, card, size, har)?;
                        }
                        if args.algorithms.contains(&Algorithm::GhllPlus) {
                            let res = gather_ghllplus(prec, &data, &builders, t);
                            save_to_file(&format!("GHLLPlus_{}", t_name), prec, card, size, res)?;
                        }
                    };
                }

                match t_enum {
                    Transform::Icdf => { run_variants!(ICDFGumbel::default()); }
                    Transform::Bithack => { run_variants!(BitHackGumbel::default()); }
                }
            }
        }
        main_pb.inc(1);
    }

    Ok(())
}
