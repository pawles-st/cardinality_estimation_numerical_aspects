pub mod constants;
use ahash::random_state::RandomState;
use gumbel_estimation::{GHLL, GHLLPlus, GHLLReal, GumbelTransform};
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Copy, Eq, Hash)]
pub enum Algorithm {
    Hll,
    Ghll,
    GhllReal,
    GhllPlus,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Copy, Eq, Hash)]
pub enum Transform {
    Icdf,
    SimpleBithack,
    TaylorBithack,
}

pub fn get_transform_name(t: Transform) -> &'static str {
    match t {
        Transform::Icdf => "ICDF",
        Transform::SimpleBithack => "SimpleBitHack",
        Transform::TaylorBithack => "TaylorBitHack",
    }
}

pub fn save_to_file<T: std::fmt::Display>(
    alg_name: &str,
    prec: u8,
    card: usize,
    size: usize,
    results: Vec<T>,
) -> std::io::Result<()> {
    let outpath = format!("../results/{}_{}_{}_{}.txt", alg_name, prec, card, size);
    let file = File::create(outpath)?;
    let mut writer = BufWriter::new(file);

    for res in results {
        writeln!(writer, "{}", res)?;
    }

    writer.flush()?;
    Ok(())
}

pub fn load_data(card: usize, size: usize) -> Result<Vec<u64>, io::Error> {
    let file = File::open(format!("../data/data_{}_{}.txt", card, size))?;
    let reader = BufReader::new(file);

    reader.lines().map(|l| {
        l.and_then(|l| l.trim().parse::<u64>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        )
    }).collect()
}

// Run experiments for HLL
pub fn gather_hll(
    prec: u8,
    data: &[u64],
    builders: &[RandomState],
) -> Vec<f64> {
    builders.into_par_iter().map(|builder| {
        let mut estimator = HyperLogLogPF::<u64, _>::new(prec, builder.clone()).unwrap();
        for &value in data { estimator.insert(&value) }
        estimator.count()
    }).collect()
}

// Run experiments for GHLL (Geo + Har)
pub fn gather_ghll<G: GumbelTransform + Copy + Send + Sync>(
    prec: u8,
    data: &[u64],
    builders: &[RandomState],
    transform: G,
) -> (Vec<f64>, Vec<f64>) {
    builders.into_par_iter().map(|builder| {
        let mut estimator = GHLL::with_precision(prec, builder.clone(), transform).unwrap();
        for &value in data { estimator.add(&value) }
        (estimator.count_geo(), estimator.count_har())
    }).unzip()
}

// Run experiments for GHLLReal (Geo + Har)
pub fn gather_ghllreal<G: GumbelTransform + Copy + Send + Sync>(
    prec: u8,
    data: &[u64],
    builders: &[RandomState],
    transform: G,
) -> (Vec<f64>, Vec<f64>) {
    builders.into_par_iter().map(|builder| {
        let mut estimator = GHLLReal::with_precision(prec, builder.clone(), transform).unwrap();
        for &value in data { estimator.add(&value) }
        (estimator.count_geo(), estimator.count_har())
    }).unzip()
}

// Run experiments for GHLLPlus
pub fn gather_ghllplus<G: GumbelTransform + Copy + Send + Sync>(
    prec: u8,
    data: &[u64],
    builders: &[RandomState],
    transform: G,
) -> Vec<f64> {
    builders.into_par_iter().map(|builder| {
        let mut estimator = GHLLPlus::with_precision(prec, builder.clone(), transform).unwrap();
        for &value in data { estimator.add(&value) }
        estimator.count()
    }).collect()
}
