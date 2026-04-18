use ahash::random_state::RandomState;
use criterion::*;
use hyperloglogplus::{HyperLogLog, HyperLogLogPF};
use gumbel_estimation::*;
use gumbel_estimation::gumbel::GumbelTransform;

fn bench_hll_vs_ghll_steps(c: &mut Criterion) {
    let prec = 12;
    let size = 500_000;
    let data: Vec<u64> = (0..size as u64).collect();
    let builder = RandomState::new();
    let bit_hack = BitHackGumbel::new();
    let optimal = OptimalGumbel::new();
    let fast = FastGumbel::new();

    let mut group = c.benchmark_group("HLL_vs_GHLL_Steps");

    // 1. Standard HLL Baseline
    group.bench_function("1_HLL_Standard", |b| {
        b.iter_batched(
            || HyperLogLogPF::<u64, _>::new(prec, builder.clone()).unwrap(),
            |mut hll| {
                for d in &data {
                    hll.insert(d);
                }
            },
            BatchSize::LargeInput
        )
    });

    // 2. GHLL Infrastructure (Single Hash + Integer Packing)
    group.bench_function("2_GHLL_Int_Packing", |b| {
        let mut registers = Registers::new(1 << prec);
        b.iter(|| {
            for d in &data {
                let h = builder.hash_one(d);
                let idx = (h >> (64 - prec)) as usize;
                let val = (h as u32).leading_zeros();
                registers.set_greater(idx, val);
            }
        })
    });

    // 3. GHLL + Double Hashing (HLL + get_shift overhead)
    group.bench_function("3_GHLL_Double_Hash_Int", |b| {
        let mut registers = Registers::new(1 << prec);
        b.iter(|| {
            for d in &data {
                let h1 = builder.hash_one(d);
                let idx = (h1 >> (64 - prec)) as usize;
                
                // Second hash call
                let s_h = builder.hash_one(&idx) as u32;
                let _s = mantissa_to_float(s_h);
                
                let val = (h1 as u32).leading_zeros();
                registers.set_greater(idx, val);
            }
        })
    });

    // 4. GHLL + Float Conversion (The FPU transition cost)
    group.bench_function("4_GHLL_Float_Conv", |b| {
        let mut registers = Registers::new(1 << prec);
        b.iter(|| {
            for d in &data {
                let h = builder.hash_one(d) as u32;
                let idx = (h >> (32 - prec)) as usize;
                let f = mantissa_to_float(h);
                // Simple encode logic
                let val = ((f32::floor(f + 0.5) as i32) + 16) as u32;
                registers.set_greater(idx, val);
            }
        })
    });

    // 5. GHLL + BitHack Gumbel (The Gumbel math cost)
    group.bench_function("5_GHLL_BitHack_Math", |b| {
        let mut registers = Registers::new(1 << prec);
        b.iter(|| {
            for d in &data {
                let h = builder.hash_one(d) as u32;
                let idx = (h >> (32 - prec)) as usize;
                let g = bit_hack.from_bits(h);
                let val = Registers::encode(g, 0.5);
                registers.set_greater(idx, val);
            }
        })
    });

    // 5b. GHLL + Optimal Gumbel (The Gumbel math cost)
    group.bench_function("5b_GHLL_Optimal_Math", |b| {
        let mut registers = Registers::new(1 << prec);
        b.iter(|| {
            for d in &data {
                let h = builder.hash_one(d) as u32;
                let idx = (h >> (32 - prec)) as usize;
                let g = optimal.from_bits(h);
                let val = Registers::encode(g, 0.5);
                registers.set_greater(idx, val);
            }
        })
    });

    // 5c. GHLL + Fast Gumbel (The Gumbel math cost)
    group.bench_function("5c_GHLL_Fast_Math", |b| {
        let mut registers = Registers::new(1 << prec);
        b.iter(|| {
            for d in &data {
                let h = builder.hash_one(d) as u32;
                let idx = (h >> (32 - prec)) as usize;
                let g = fast.from_bits(h);
                let val = Registers::encode(g, 0.5);
                registers.set_greater(idx, val);
            }
        })
    });

    // 6. Full GHLL (BitHack + Double Hash)
    group.bench_function("6_GHLL_Full", |b| {
        let mut registers = Registers::new(1 << prec);
        b.iter(|| {
            for d in &data {
                let h = builder.hash_one(d) as u32;
                let idx = (h >> (32 - prec)) as usize;
                let g = bit_hack.from_bits(h);
                let shift = get_shift(&builder, idx);
                let val = Registers::encode(g, shift);
                registers.set_greater(idx, val);
            }
        })
    });

    // 6b. Full GHLL (Optimal + Double Hash)
    group.bench_function("6b_GHLL_Full_Optimal", |b| {
        let mut registers = Registers::new(1 << prec);
        b.iter(|| {
            for d in &data {
                let h = builder.hash_one(d) as u32;
                let idx = (h >> (32 - prec)) as usize;
                let g = optimal.from_bits(h);
                let shift = get_shift(&builder, idx);
                let val = Registers::encode(g, shift);
                registers.set_greater(idx, val);
            }
        })
    });

    // 6c. Full GHLL (Fast + Double Hash)
    group.bench_function("6c_GHLL_Full_Fast", |b| {
        let mut registers = Registers::new(1 << prec);
        b.iter(|| {
            for d in &data {
                let h = builder.hash_one(d) as u32;
                let idx = (h >> (32 - prec)) as usize;
                let g = fast.from_bits(h);
                let shift = get_shift(&builder, idx);
                let val = Registers::encode(g, shift);
                registers.set_greater(idx, val);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_hll_vs_ghll_steps);
criterion_main!(benches);
