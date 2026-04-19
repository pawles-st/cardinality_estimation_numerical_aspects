use ahash::random_state::RandomState;
use criterion::*;
use criterion::measurement::Measurement;
use gumbel_estimation::{GHLL, GHLLPlus, GHLLReal, GumbelTransform};
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
use std::hash::Hash;

pub fn bench_hll<T, M>(g: &mut BenchmarkGroup<M>, prec: u8, card: usize, data: &[T])
where
    T: Hash,
    M: Measurement,
{
    g.bench_with_input(BenchmarkId::new("HyperLogLog", format!("{}/{}/{}", prec, card, data.len())), data, |b, data| b.iter(|| {
        let mut estimator = HyperLogLogPF::<T, _>::new(prec, RandomState::new()).unwrap();
        for d in data {
            estimator.insert(d);
        }
        let _estimate = estimator.count();
    }));
}

pub fn bench_ghll<T, M, G>(g: &mut BenchmarkGroup<M>, t_name: &str, prec: u8, card: usize, data: &[T], transform: G)
where
    T: Hash,
    M: Measurement,
    G: GumbelTransform + Copy,
{
    g.bench_with_input(BenchmarkId::new(format!("GHLL_{}", t_name), format!("{}/{}/{}", prec, card, data.len())), data, |b, data| b.iter(|| {
        let mut estimator = GHLL::<_, G>::with_precision(prec, RandomState::new(), transform).unwrap();
        for d in data {
            estimator.add(d);
        }
        let _estimate_geo = estimator.count_geo();
        let _estimate_har = estimator.count_har();
    }));
}

pub fn bench_ghll_real<T, M, G>(g: &mut BenchmarkGroup<M>, t_name: &str, prec: u8, card: usize, data: &[T], transform: G)
where
    T: Hash,
    M: Measurement,
    G: GumbelTransform + Copy,
{
    g.bench_with_input(BenchmarkId::new(format!("GHLLReal_{}", t_name), format!("{}/{}/{}", prec, card, data.len())), data, |b, data| b.iter(|| {
        let mut estimator = GHLLReal::<_, G>::with_precision(prec, RandomState::new(), transform).unwrap();
        for d in data {
            estimator.add(d);
        }
        let _estimate_geo = estimator.count_geo();
        let _estimate_har = estimator.count_har();
    }));
}

pub fn bench_ghllplus<T, M, G>(g: &mut BenchmarkGroup<M>, t_name: &str, prec: u8, card: usize, data: &[T], transform: G)
where
    T: Hash,
    M: Measurement,
    G: GumbelTransform + Copy,
{
    g.bench_with_input(BenchmarkId::new(format!("GHLLPlus_{}", t_name), format!("{}/{}/{}", prec, card, data.len())), data, |b, data| b.iter(|| {
        let mut estimator = GHLLPlus::<_, G>::with_precision(prec, RandomState::new(), transform).unwrap();
        for d in data {
            estimator.add(d);
        }
        let _estimate = estimator.count();
    }));
}
