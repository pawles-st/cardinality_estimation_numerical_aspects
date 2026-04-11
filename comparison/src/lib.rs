use ahash::random_state::RandomState;
use gumbel_estimation::{GHLL, GHLLPlus, GHLLReal, GumbelTransform};
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

pub mod constants;

use constants::ITERATIONS;

pub fn prepare_outfile(alg: &str, prec: u8, card: usize, size: usize) -> Result<File, io::Error> {
    let outpath = format!("../results/{}_{}_{}_{}.txt", alg, prec, card, size);
    let out = File::create(outpath)?;

    Ok(out)
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

pub fn gather<G: GumbelTransform + Copy>(
    prec: u8,
    card: usize,
    size: usize,
    data: &[u64],
    transform: G,
    transform_name: &str,
    include_hll: bool,
) -> Result<(), io::Error> {
    // Crepare the output files
    let mut ghll_geo_out = prepare_outfile(&format!("GHLLGeo_{}", transform_name), prec, card, size)?;
    let mut ghll_har_out = prepare_outfile(&format!("GHLLHar_{}", transform_name), prec, card, size)?;
    let mut ghllr_geo_out = prepare_outfile(&format!("GHLLRealGeo_{}", transform_name), prec, card, size)?;
    let mut ghllr_har_out = prepare_outfile(&format!("GHLLRealHar_{}", transform_name), prec, card, size)?;
    let mut ghllp_out = prepare_outfile(&format!("GHLLPlus_{}", transform_name), prec, card, size)?;
    
    // Create independent hash builders for each iteration
    let builders: Vec<_> = (0..ITERATIONS).map(|_| RandomState::new()).collect();

    // HLL
    if include_hll {
        let mut hll_out = prepare_outfile("HLL", prec, card, size)?;
        for i in 0..ITERATIONS {
            let mut estimator = HyperLogLogPF::<u64, _>::new(prec, builders[i].clone()).unwrap();
            for &value in data {
                estimator.insert(&value)
            }
            writeln!(hll_out, "{}", estimator.count())?;
        }
    }

    // GHLL

    for i in 0..ITERATIONS {
        let mut estimator = GHLL::<_, _>::with_precision(prec, builders[i].clone(), transform).unwrap();
        for &value in data {
            estimator.add(&value)
        }
        writeln!(ghll_geo_out, "{}", estimator.count_geo())?;
        writeln!(ghll_har_out, "{}", estimator.count_har())?;
    }
    
    // GHLLReal

    for i in 0..ITERATIONS {
        let mut estimator = GHLLReal::<_, _>::with_precision(prec, builders[i].clone(), transform).unwrap();
        for &value in data {
            estimator.add(&value)
        }
        writeln!(ghllr_geo_out, "{}", estimator.count_geo())?;
        writeln!(ghllr_har_out, "{}", estimator.count_har())?;
    }
    
    // GHLLPlus

    for i in 0..ITERATIONS {
        let mut estimator = GHLLPlus::<_, _>::with_precision(prec, builders[i].clone(), transform).unwrap();
        for &value in data {
            estimator.add(&value)
        }
        writeln!(ghllp_out, "{}", estimator.count())?;
    }

    Ok(())
}

