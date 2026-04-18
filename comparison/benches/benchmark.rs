use criterion::*;
use itertools::iproduct;

use gumbel_estimation::{ICDFGumbel, BitHackGumbel, PadeGumbel, OptimalGumbel, FastGumbel};

use comparison::constants::{CARDINALITIES, DATA_SIZE_MULTIPLIES, PRECISIONS};

mod common;

use crate::common::{bench_hll, bench_ghll, bench_ghllreal, bench_ghllplus, load_data};

fn benchmark(c: &mut Criterion) {
    let data_sizes: Vec<_> = iproduct!(CARDINALITIES.iter().copied(), DATA_SIZE_MULTIPLIES.iter().copied())
        .filter(|(card, mult)| card * mult <= 1_000_000_000).collect();

    for &prec in PRECISIONS {
        let mut group = c.benchmark_group("Cardinality Estimation");

        for &(card, mult) in &data_sizes {

            group.throughput(Throughput::Elements(card as u64 * mult as u64));

            // read data from file

            let data: Vec<u64> = load_data(card, card * mult)
                .unwrap_or_else(|e| panic!("{}", e));

            // perform Hyperloglog benchmark

            bench_hll(&mut group, prec, card, &data);

            // perform GumbelHyperLogLog benchmark

            bench_ghll(&mut group, prec, card, &data, ICDFGumbel::default(), "GHLL (ICDF)");
            bench_ghll(&mut group, prec, card, &data, BitHackGumbel::default(), "GHLL (BitHack)");
            bench_ghll(&mut group, prec, card, &data, PadeGumbel::default(), "GHLL (Pade)");
            bench_ghll(&mut group, prec, card, &data, OptimalGumbel::default(), "GHLL (Optimal)");
            bench_ghll(&mut group, prec, card, &data, FastGumbel::default(), "GHLL (Fast)");

            // perform GumbelHyperLogLogReal benchmark

            bench_ghllreal(&mut group, prec, card, &data, ICDFGumbel::default(), "GHLLReal (ICDF)");
            bench_ghllreal(&mut group, prec, card, &data, BitHackGumbel::default(), "GHLLReal (BitHack)");
            bench_ghllreal(&mut group, prec, card, &data, PadeGumbel::default(), "GHLLReal (Pade)");
            bench_ghllreal(&mut group, prec, card, &data, OptimalGumbel::default(), "GHLLReal (Optimal)");
            bench_ghllreal(&mut group, prec, card, &data, FastGumbel::default(), "GHLLReal (Fast)");

            // perform GumbelHyperLogLog+ benchmark

            bench_ghllplus(&mut group, prec, card, &data, ICDFGumbel::default(), "GHLL+ (ICDF)");
            bench_ghllplus(&mut group, prec, card, &data, BitHackGumbel::default(), "GHLL+ (BitHack)");
            bench_ghllplus(&mut group, prec, card, &data, PadeGumbel::default(), "GHLL+ (Pade)");
            bench_ghllplus(&mut group, prec, card, &data, OptimalGumbel::default(), "GHLL+ (Optimal)");
            bench_ghllplus(&mut group, prec, card, &data, FastGumbel::default(), "GHLL+ (Fast)");
        }

        group.finish();
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
