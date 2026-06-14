use bitvec::prelude::*;
use std::f64::consts::E;
use std::hash::{Hash, BuildHasher};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

use crate::common::*;
use crate::gumbel::GumbelTransform;
use crate::registers::Registers;

/// A GumbelHyperLogLog using floating-point updates with optimized 5-bit registers (Raw)
#[derive(Debug, Clone)]
pub struct GHLLFloat<B: BuildHasher, G: GumbelTransform> {
    builder: B,
    transform: G,
    precision: u8,
    no_registers: usize,
    registers: Registers,
}

impl<B: BuildHasher, G: GumbelTransform> GHLLFloat<B, G> {
    pub fn with_precision(precision: u8, builder: B, transform: G) -> Result<Self, GumbelError> {
        if !(MIN_PRECISION..=MAX_PRECISION).contains(&precision) {
            return Err(GumbelError::InvalidPrecision);
        }

        let no_registers = 1 << precision;
        let mut rng = thread_rng();
        let unif = Uniform::new(0.0, 1.0);

        let mut registers = Registers::new(no_registers);
        for i in 0..no_registers {
            let q = rng.sample(unif);
            let gumbel_variate = transform.quantile(q);
            let shift = get_shift(&builder, i);
            registers.set(i, Registers::encode(gumbel_variate, shift));
        }

        Ok(Self {
            builder,
            transform,
            precision,
            no_registers,
            registers,
        })
    }

    /// Adds a new value using standard FLOATING POINT math before encoding into 5-bit register
    pub fn add<H: Hash + ?Sized>(&mut self, value: &H) {
        let (index, hash) = hash_value(value, &self.builder, self.precision);
        
        // FLOAT PATH
        let gumbel_float = self.transform.from_bits(hash);
        let shift_float = get_shift(&self.builder, index);
        
        // Encode using float logic into the same 5-bit packed register
        self.registers.set_greater(index, Registers::encode(gumbel_float, shift_float));
    }

    pub fn count_geo(&self) -> f64 {
        let registers_sum: f64 = self.registers.iter()
            .enumerate()
            .map(|(i, val)| Registers::decode(val, get_shift(&self.builder, i)))
            .sum();
        let registers_mean = registers_sum / self.no_registers as f64;
        self.no_registers as f64 * f64::exp(NEG_GAMMA + registers_mean)
    }
}

/// A production-ready GHLL using floating-point updates (for comparison)
#[derive(Debug, Clone)]
pub struct GHLLPlusFloat<B: BuildHasher, G: GumbelTransform> {
    builder: B,
    transform: G,
    precision: u8,
    no_registers: usize,
    registers: Registers,
    free: BitVec,
}

impl<B: BuildHasher, G: GumbelTransform> GHLLPlusFloat<B, G> {
    pub fn with_precision(precision: u8, builder: B, transform: G) -> Result<Self, GumbelError> {
        if !(MIN_PRECISION..=MAX_PRECISION).contains(&precision) {
            return Err(GumbelError::InvalidPrecision);
        }

        let no_registers = 1 << precision;
        let mut rng = thread_rng();
        let unif = Uniform::new(0.0, 1.0);

        let mut registers = Registers::new(no_registers);
        for i in 0..no_registers {
            let q = rng.sample(unif);
            let gumbel_variate = transform.quantile(q);
            let shift = get_shift(&builder, i);
            registers.set(i, Registers::encode(gumbel_variate, shift));
        }

        let free = bitvec![1; no_registers];

        Ok(Self {
            builder,
            transform,
            precision,
            no_registers,
            registers,
            free,
        })
    }

    pub fn add<H: Hash + ?Sized>(&mut self, value: &H) {
        let (index, hash) = hash_value(value, &self.builder, self.precision);
        self.free.set(index, false);
        
        let gumbel_float = self.transform.from_bits(hash);
        let shift_float = get_shift(&self.builder, index);
        
        self.registers.set_greater(index, Registers::encode(gumbel_float, shift_float));
    }

    pub fn count(&self) -> f64 {
        let no_free = self.free.count_ones();
        if no_free as f64 >= self.no_registers as f64 / E  {
            return self.no_registers as f64 * f64::ln(self.no_registers as f64 / no_free as f64);
        }
        let no_occupied = self.no_registers - no_free;
        let registers_sum = self.registers.iter()
            .zip(self.free.iter())
            .enumerate()
            .filter_map(|(i, (val, free))| if *free {
                    None
                } else {
                    Some(Registers::decode(val, get_shift(&self.builder, i)))
                }
            )
            .sum::<f64>();
        let registers_mean = registers_sum / no_occupied as f64;
        no_occupied as f64 * f64::exp(NEG_GAMMA + registers_mean) - self.no_registers as f64 / 2.0 - 0.5
    }
}
