use criterion::*;
use itertools::iproduct;

use comparison::constants::{CARDINALITIES, DATA_SIZE_MULTIPLIES, PRECISIONS};

mod common;

use crate::common::{bench_hll, bench_ghll, bench_ghllplus, load_data};

fn benchmark(c: &mut Criterion) {
    
    let data_sizes: Vec<_> = iproduct!(CARDINALITIES, DATA_SIZE_MULTIPLIES).filter(|(card, mult)| card * mult <= 1_000_000_000).collect();

    for prec in PRECISIONS { 
        let mut group = c.benchmark_group("Cardinality Estimation");

        for (card, mult) in &data_sizes {

            group.throughput(Throughput::Elements((*card) as u64 * (*mult) as u64));

            // read data from file

            let data: Vec<u64> = load_data(*card, card * mult)
                .unwrap_or_else(|e| panic!("{}", e));

            // perform Hyperloglog benchmark

            bench_hll(&mut group, prec, *card, &data);

            // perform Gumbel benchmark

            bench_ghll(&mut group, prec, *card, &data);

            // perform GumbelLazy benchmark

            bench_ghllplus(&mut group, prec, *card, &data);
        }

        group.finish();
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
