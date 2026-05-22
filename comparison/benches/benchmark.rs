use criterion::*;
use itertools::iproduct;

use comparison::constants::{CARDINALITIES, DATA_SIZE_MULTIPLIES, PRECISIONS};
use comparison::{Algorithm, Transform, get_transform_name, load_data};
use gumbel_estimation::{ICDFGumbel, SimpleBitHackGumbel, TaylorBitHackGumbel, TaylorIntegerGumbel};

mod common;

use crate::common::{bench_hll, bench_ghll, bench_ghll_real, bench_ghllplus};

fn benchmark(c: &mut Criterion) {
    let data_sizes: Vec<_> = iproduct!(CARDINALITIES, DATA_SIZE_MULTIPLIES)
        .filter(|(card, mult)| card * mult <= 1_000_000_000)
        .collect();

    // Algorithms and Transforms to test
    // To match the boxplot changes, we benchmark only a single GHLL variant (e.g., GhllPlus)
    // against all transforms.
    let algorithms = vec![Algorithm::Hll, Algorithm::GhllPlus];
    let transforms = vec![Transform::Icdf, Transform::SimpleBithack, Transform::TaylorBithack, Transform::TaylorInteger];

    for prec in PRECISIONS {
        let mut group = c.benchmark_group("Cardinality Estimation");

        for (card, mult) in &data_sizes {
            group.throughput(Throughput::Elements((*card) as u64 * (*mult) as u64));

            let data: Vec<u64> = load_data(*card, card * mult)
                .unwrap_or_else(|e| panic!("{}", e));

            // Run HLL
            if algorithms.contains(&Algorithm::Hll) {
                bench_hll(&mut group, prec, *card, &data);
            }

            // Run Gumbel variants
            for &t_enum in &transforms {
                let t_name = get_transform_name(t_enum);

                macro_rules! run_variants {
                    ($transform:expr) => {
                        let t = $transform;
                        if algorithms.contains(&Algorithm::Ghll) {
                            bench_ghll(&mut group, t_name, prec, *card, &data, t);
                        }
                        if algorithms.contains(&Algorithm::GhllReal) {
                            bench_ghll_real(&mut group, t_name, prec, *card, &data, t);
                        }
                        if algorithms.contains(&Algorithm::GhllPlus) {
                            bench_ghllplus(&mut group, t_name, prec, *card, &data, t);
                        }
                    };
                }

                match t_enum {
                    Transform::Icdf => { run_variants!(ICDFGumbel::default()); }
                    Transform::SimpleBithack => { run_variants!(SimpleBitHackGumbel::default()); }
                    Transform::TaylorBithack => { run_variants!(TaylorBitHackGumbel::default()); }
                    Transform::TaylorInteger => { run_variants!(TaylorIntegerGumbel::default()); }
                }
            }
        }

        group.finish();
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
