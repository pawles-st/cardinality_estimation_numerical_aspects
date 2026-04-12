use ahash::random_state::RandomState;
use criterion::*;

use gumbel_estimation::{GHLL, ICDFGumbel, BitHackGumbel};

use comparison::load_data;

fn profile_ghll(c: &mut Criterion) {
    let prec = 8;
    let card = 50_000;
    let size = 500_000;

    let data = load_data(card, size).unwrap_or_else(|_| vec![0u64; size]);
    let builder = RandomState::new();
    let transform = BitHackGumbel::new();

    let mut group = c.benchmark_group("Profile");

    group.bench_function("GHLL", |b| {
        b.iter_batched(
            || GHLL::with_precision(prec, builder.clone(), transform).unwrap(),
            |mut estimator| {
                for d in &data {
                    estimator.add(d);
                }
            },
            BatchSize::LargeInput
        )
    });
    group.finish();
}

criterion_group!(benches, profile_ghll);
criterion_main!(benches);
