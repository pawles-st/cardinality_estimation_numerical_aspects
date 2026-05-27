use criterion::*;
use itertools::iproduct;

use comparison::constants::{CARDINALITIES, DATA_SIZE_MULTIPLIES};
use comparison::{load_data};
use gumbel_estimation::{LUTGumbel};

mod common;

use crate::common::{bench_hll, bench_ghll, bench_ghllplus};

fn benchmark(c: &mut Criterion) {
    let data_sizes: Vec<_> = iproduct!(CARDINALITIES, DATA_SIZE_MULTIPLIES)
        .filter(|(card, mult)| card * mult <= 1_000_000_000)
        .collect();

    let mut group = c.benchmark_group("2. Estimator Overhead (LUT Baseline)");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Linear));

    for (card, mult) in &data_sizes {
        let total_size = *card * *mult;
        group.throughput(Throughput::Elements(total_size as u64));
        let data: Vec<u64> = load_data(*card, total_size).unwrap_or_else(|_| (0..total_size as u64).collect());

        // 1. Standard HLL
        bench_hll(&mut group, *card, &data);

        // 2. Raw GHLL (Fastest Gumbel)
        bench_ghll(&mut group, "LUT", *card, &data, LUTGumbel::default());

        // 3. GHLL+ (Gumbel with BitVec tracking)
        bench_ghllplus(&mut group, "LUT", *card, &data, LUTGumbel::default());
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
