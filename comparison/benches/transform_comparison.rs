use criterion::*;
use itertools::iproduct;

use comparison::constants::{CARDINALITIES, DATA_SIZE_MULTIPLIES};
use comparison::{Transform, get_transform_name, load_data};
use gumbel_estimation::{ICDFGumbel, PadeGumbel, SimpleBitHackGumbel, TaylorBitHackGumbel, SplineGumbel, LUTGumbel};

mod common;

use crate::common::{bench_hll, bench_ghllplus};

fn benchmark(c: &mut Criterion) {
    let data_sizes: Vec<_> = iproduct!(CARDINALITIES, DATA_SIZE_MULTIPLIES)
        .filter(|(card, mult)| card * mult <= 1_000_000_000)
        .collect();

    let transforms = vec![
        Transform::Icdf,
        Transform::Pade,
        Transform::SimpleBithack,
        Transform::TaylorBithack,
        Transform::Spline,
        Transform::LUT,
    ];

    let mut group = c.benchmark_group("1. Gumbel Transform Comparison (GHLL+)");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Linear));

    for (card, mult) in &data_sizes {
        let total_size = *card * *mult;
        group.throughput(Throughput::Elements(total_size as u64));
        let data: Vec<u64> = load_data(*card, total_size).unwrap_or_else(|_| (0..total_size as u64).collect());

        // Standard HLL Baseline
        bench_hll(&mut group, *card, &data);

        for &t_enum in &transforms {
            let t_name = get_transform_name(t_enum);
            match t_enum {
                Transform::Icdf => { bench_ghllplus(&mut group, t_name, *card, &data, ICDFGumbel::default()); }
                Transform::Pade => { bench_ghllplus(&mut group, t_name, *card, &data, PadeGumbel::default()); }
                Transform::SimpleBithack => { bench_ghllplus(&mut group, t_name, *card, &data, SimpleBitHackGumbel::default()); }
                Transform::TaylorBithack => { bench_ghllplus(&mut group, t_name, *card, &data, TaylorBitHackGumbel::default()); }
                Transform::Spline => { bench_ghllplus(&mut group, t_name, *card, &data, SplineGumbel::default()); }
                Transform::LUT => { bench_ghllplus(&mut group, t_name, *card, &data, LUTGumbel::default()); }
            }
        }
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
