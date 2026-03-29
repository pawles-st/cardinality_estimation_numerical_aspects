use bitvec::prelude::*;
use std::f64::consts::E;
use std::hash::{Hash, BuildHasher};
use rand::{Rng, thread_rng};
use rand::distributions::{Uniform};

use crate::common::*;
use crate::gen_gumbel;
use crate::registers::Registers;

/// A cardinality estimator using the Gumbel distribution
pub struct GHLLPlus<B: BuildHasher> {
    builder: B,
    precision: u8,
    no_registers: usize,
    registers: Registers,
    free: BitVec,
}

impl<B: BuildHasher> GHLLPlus<B> {
    /// Creates a new `GumbelEstimator` object with a custom precision and hash builder
    ///
    /// # Arguments
    ///
    /// - `precision` - corresponds to the number of registers used by this estimator using the 
    ///   formula `no_registers = 2^precision`; the accepted values lie in the range {4, 5, ..., 16}
    /// - `builder` - this is a hash builder that will be used for hashing provided values
    pub fn with_precision(precision: u8, builder: B) -> Result<Self, GumbelError> {
        // check if the provided precision is within the bounds
        if !(MIN_PRECISION..=MAX_PRECISION).contains(&precision) {
            return Err(GumbelError::InvalidPrecision);
        }

        // calculate the number of registers as `2^precision`
        let no_registers = 1 << precision;

        // create a uniform [0, 1) rng
        let mut rng = thread_rng();
        let unif = Uniform::new(0.0, 1.0);

        // initialise the registers to random gumbel values
        let mut registers = Registers::new(no_registers);
        for i in 0..no_registers {
            let q = rng.sample(unif);
            let c = gen_gumbel::mantissa_to_float(builder.hash_one(i) as u32);
            let gumbel_value = gen_gumbel::quantile_rounded(q, c);
            registers.set(i, gumbel_value);
        }

        // mark all registers as free
        let free = bitvec![1; no_registers];

        // create the estimator object
        Ok(Self {
            builder,
            precision,
            no_registers,
            registers,
            free,
        })
    }

    pub fn add<H: Hash + ?Sized>(&mut self, value: &H) {
        // hash the value and separate the hash into the index and the remainder
        let (index, hash) = hash_value(value, &self.builder, self.precision);

        // mark the register as taken
        self.free.set(index, false);

        // create a gumbel random variable
        let gumbel_value = gen_gumbel::from_bits_rounded(
            hash,
            gen_gumbel::mantissa_to_float(self.builder.hash_one(index) as u32)
        );

        // update the register to the max of the gumbel random variables
        self.registers.set_greater(index, gumbel_value);
    }

    pub fn count(&self) -> f64 {
        // compute the numbers of free registers
        let no_free = self.free.count_ones();

        // apply low-range correction
        if no_free as f64 >= self.no_registers as f64 / E  {
            // perform linear counting
            return self.no_registers as f64 * f64::ln(self.no_registers as f64 / no_free as f64);
        }

        // count the number of occupied registers
        let no_occupied = self.no_registers - no_free;

        // apply the second half of shift rounding
        // and calculate the geometric mean of the `exp(register)` terms
        let registers_sum = self.registers.iter()
            .zip(self.free.iter())
            .enumerate()
            .filter_map(|(i, (val, free))| if *free {
                    None
                } else {
                    Some((val as i32 - gen_gumbel::BIAS) as f64 - gen_gumbel::mantissa_to_float(self.builder.hash_one(i) as u32) as f64)
                }
            )
            .sum::<f64>();
        let registers_mean = registers_sum / no_occupied as f64;
        
        no_occupied as f64 * f64::exp(NEG_GAMMA + 0.5 + registers_mean) - self.no_registers as f64 / 2.0 - 0.5
    }
}
