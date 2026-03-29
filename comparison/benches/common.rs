use ahash::random_state::RandomState;
use criterion::*;
use criterion::measurement::Measurement;
use gumbel_estimation::{GHLL, GHLLPlus};
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::{BufRead, BufReader};

pub fn load_data(card: usize, size: usize) -> Result<Vec<u64>, io::Error>
{
    let file = File::open(format!("../data/data_{}_{}.txt", card, size))?;
    let reader = BufReader::new(file);

    reader.lines().map(|l| {
        l.and_then(|l| l.trim().parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        )
    }).collect()
}

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

pub fn bench_ghll<T, M>(g: &mut BenchmarkGroup<M>, prec: u8, card: usize, data: &[T])
where
    T: Hash,
    M: Measurement,
{
    g.bench_with_input(BenchmarkId::new("GumbelHyperLogLog", format!("{}/{}/{}", prec, card, data.len())), data, |b, data| b.iter(|| {
        let mut estimator = GHLL::<_>::with_precision(prec, RandomState::new()).unwrap();
        for d in data {
            estimator.add(d);
        }
        let _estimate = estimator.count_geo();
    }));
}

pub fn bench_ghllplus<T, M>(g: &mut BenchmarkGroup<M>, prec: u8, card: usize, data: &[T])
where
    T: Hash,
    M: Measurement,
{
    g.bench_with_input(BenchmarkId::new("GumbelHyperLogLog+", format!("{}/{}/{}", prec, card, data.len())), data, |b, data| b.iter(|| {
        let mut estimator = GHLLPlus::<_>::with_precision(prec, RandomState::new()).unwrap();
        for d in data {
            estimator.add(d);
        }
        let _estimate = estimator.count();
    }));
}
