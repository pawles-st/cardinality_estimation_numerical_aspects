use ahash::random_state::RandomState;
use criterion::*;
use criterion::measurement::Measurement;
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
use std::hash::Hash;

use gumbel_estimation::{GHLL, GHLLReal, GHLLPlus, GumbelTransform, ICDFGumbel, BitHackGumbel};

pub use comparison::load_data;

pub fn bench_hll<T, M>(
    g: &mut BenchmarkGroup<M>,
    prec: u8,
    card: usize,
    data: &[T],
)
where
    T: Hash,
    M: Measurement,
{
    g.bench_with_input(BenchmarkId::new("HyperLogLog", format!("{}/{}/{}", prec, card, data.len())), data, |b, data| {
        b.iter_batched(
            || HyperLogLogPF::<T, _>::new(prec, RandomState::new()).unwrap(),
            |mut estimator| {
                for d in data {
                    estimator.insert(d);
                }
                let _estimate = estimator.count();
            },
            BatchSize::SmallInput
        )
    });
}

pub fn bench_ghll<T, M, G>(
    g: &mut BenchmarkGroup<M>,
    prec: u8,
    card: usize,
    data: &[T],
    transform: G,
    label: &str,
)
where
    T: Hash,
    M: Measurement,
    G: GumbelTransform + Copy,
{
    g.bench_with_input(
        BenchmarkId::new(label, format!("{}/{}/{}", prec, card, data.len())),
        data,
        |b, data| {
            b.iter_batched(
                || GHLL::<_, G>::with_precision(prec, RandomState::new(), transform).unwrap(),
                |mut estimator| {
                    for d in data {
                        estimator.add(d);
                    }
                    let _estimate = estimator.count_geo();
                },
                BatchSize::SmallInput
            )
        }
    );
}

pub fn bench_ghllreal<T, M, G>(
    g: &mut BenchmarkGroup<M>,
    prec: u8,
    card: usize,
    data: &[T],
    transform: G,
    label: &str,
)
where
    T: Hash,
    M: Measurement,
    G: GumbelTransform + Copy,
{
    g.bench_with_input(
        BenchmarkId::new(label, format!("{}/{}/{}", prec, card, data.len())),
        data,
        |b, data| {
            b.iter_batched(
                || GHLLReal::<_, G>::with_precision(prec, RandomState::new(), transform).unwrap(),
                |mut estimator| {
                    for d in data {
                        estimator.add(d);
                    }
                    let _estimate = estimator.count_geo();
                },
                BatchSize::SmallInput
            )
        }
    );
}

pub fn bench_ghllplus<T, M, G>(
    g: &mut BenchmarkGroup<M>,
    prec: u8,
    card: usize,
    data: &[T],
    transform: G,
    label: &str,
)
where
    T: Hash,
    M: Measurement,
    G: GumbelTransform + Copy,
{
    g.bench_with_input(
        BenchmarkId::new(label, format!("{}/{}/{}", prec, card, data.len())),
        data,
        |b, data| {
            b.iter_batched(
                || GHLLPlus::<_, G>::with_precision(prec, RandomState::new(), transform).unwrap(),
                |mut estimator| {
                    for d in data {
                        estimator.add(d);
                    }
                    let _estimate = estimator.count();
                },
                BatchSize::SmallInput
            )
        }
    );
}
