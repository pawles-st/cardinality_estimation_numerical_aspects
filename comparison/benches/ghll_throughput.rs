use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gumbel_estimation::{GHLLPlus, GHLLPlusFloat, NaiveGumbel, CorrectedBitHackGumbel, BasicBitHackGumbel, LUTGumbel, SplineGumbel, PadeGumbel};
use ahash::RandomState;
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};

fn bench_micro_throughput(c: &mut Criterion) {
    let precision = 12;
    let builder = RandomState::with_seeds(0, 0, 0, 0);
    
    // 1. HLL Baseline (Pure Fixed for stability)
    let mut group_hll = c.benchmark_group("HLL");
    group_hll.bench_function("Standard", |b| {
        let mut hll: HyperLogLogPF<u64, RandomState> = HyperLogLogPF::new(precision, builder.clone()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            hll.insert(black_box(&i));
            i = i.wrapping_add(1);
        })
    });
    group_hll.finish();

    // 2. GHLLPlus (Production Q16.16)
    let mut group_q = c.benchmark_group("GHLL");
    
    group_q.bench_function("Naive", |b| {
        let mut ghll = GHLLPlus::with_precision(precision, builder.clone(), NaiveGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_q.bench_function("Pade", |b| {
        let mut ghll = GHLLPlus::with_precision(precision, builder.clone(), PadeGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_q.bench_function("CorrectedBitHack", |b| {
        let mut ghll = GHLLPlus::with_precision(precision, builder.clone(), CorrectedBitHackGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_q.bench_function("BasicBitHack", |b| {
        let mut ghll = GHLLPlus::with_precision(precision, builder.clone(), BasicBitHackGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_q.bench_function("LUT", |b| {
        let mut ghll = GHLLPlus::with_precision(precision, builder.clone(), LUTGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_q.bench_function("Spline", |b| {
        let mut ghll = GHLLPlus::with_precision(precision, builder.clone(), SplineGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_q.finish();

    // 3. GHLLPlusFloat (Production f32 Baseline)
    let mut group_f = c.benchmark_group("GHLLFloat");
    
    group_f.bench_function("Naive", |b| {
        let mut ghll = GHLLPlusFloat::with_precision(precision, builder.clone(), NaiveGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_f.bench_function("Pade", |b| {
        let mut ghll = GHLLPlusFloat::with_precision(precision, builder.clone(), PadeGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_f.bench_function("CorrectedBitHack", |b| {
        let mut ghll = GHLLPlusFloat::with_precision(precision, builder.clone(), CorrectedBitHackGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_f.bench_function("BasicBitHack", |b| {
        let mut ghll = GHLLPlusFloat::with_precision(precision, builder.clone(), BasicBitHackGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_f.bench_function("LUT", |b| {
        let mut ghll = GHLLPlusFloat::with_precision(precision, builder.clone(), LUTGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_f.bench_function("Spline", |b| {
        let mut ghll = GHLLPlusFloat::with_precision(precision, builder.clone(), SplineGumbel::new()).unwrap();
        let mut i = 0u64;
        b.iter(|| {
            ghll.add(black_box(&i));
            i = i.wrapping_add(1);
        })
    });

    group_f.finish();
}

criterion_group!(benches, bench_micro_throughput);
criterion_main!(benches);
