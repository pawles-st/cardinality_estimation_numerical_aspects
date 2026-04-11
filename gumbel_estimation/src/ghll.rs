use std::hash::{Hash, BuildHasher};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

use crate::common::*;
use crate::gumbel::GumbelTransform;
use crate::registers::Registers;

/// GumbelHyperLogLog - a cardinality estimator using the Gumbel distribution
#[derive(Debug, Clone)]
pub struct GHLL<B: BuildHasher, G: GumbelTransform> {
    builder: B,
    transform: G,
    precision: u8,
    no_registers: usize,
    registers: Registers,
}

impl<B: BuildHasher, G: GumbelTransform> GHLL<B, G> {
    /// Creates a new GumbelHyperLogLog object with discrete registers (5-bits per register) for a specified precision and hash builder
    ///
    /// # Arguments
    ///
    /// - `precision` - corresponds to the number of registers used by this estimator using the 
    ///   formula `no_registers = 2^precision`; the accepted values lie in range {4, 5, ..., 16}
    /// - `builder` - hash builder that will be used for hashing provided elements
    /// - `transform` - object for creating Gumbel variables from [0, 1) floats
    pub fn with_precision(precision: u8, builder: B, transform: G) -> Result<Self, GumbelError> {
        // Check if the provided precision is within the bounds
        if !(MIN_PRECISION..=MAX_PRECISION).contains(&precision) {
            return Err(GumbelError::InvalidPrecision);
        }

        // Calculate the number of registers as `2^precision`
        let no_registers = 1 << precision;

        // Create a uniform [0, 1) rng
        let mut rng = thread_rng();
        let unif = Uniform::new(0.0, 1.0);

        // Initialise the registers to random Gumbel values
        let mut registers = Registers::new(no_registers);
        for i in 0..no_registers {
            // Create a Gumbel random variate
            let q = rng.sample(unif);
            let gumbel_variate = transform.quantile(q);

            // Initialise the register with shift rounding
            let shift = get_shift(&builder, i);
            registers.set(i, Registers::encode(gumbel_variate, shift));
        }

        // Create the estimator object
        Ok(Self {
            builder,
            transform,
            precision,
            no_registers,
            registers,
        })
    }

    /// Adds a new value to the estimor/Observes a new stream element
    pub fn add<H: Hash + ?Sized>(&mut self, value: &H) {
        // Hash the value and separate into the index and the remainder
        let (index, hash) = hash_value(value, &self.builder, self.precision);
        
        // Create a Gumbel random variate
        let gumbel_variate = self.transform.from_bits(hash);

        // Obtain the substream constant
        let shift = get_shift(&self.builder, index);

        // Update the register to the current max of Gumbel random variables
        self.registers.set_greater(index, Registers::encode(gumbel_variate, shift));
    }

    /// Returns the approximate cardinality of the stream,
    /// averaging substream results with the geometric mean
    pub fn count_geo(&self) -> f64 {
        // Calculate the geometric mean of the `exp(register)` terms
        let registers_sum = self.registers.iter()
            .enumerate()
            .map(|(i, val)| Registers::decode(val, get_shift(&self.builder, i)))
            .sum::<f64>();
        let registers_mean = registers_sum / self.no_registers as f64;
        
        self.no_registers as f64 * f64::exp(NEG_GAMMA + registers_mean)
    }
    
    /// Returns the approximate cardinality of the stream,
    /// averaging substream results with the harmonic mean
    pub fn count_har(&self) -> f64 {
        // Calculate the harmonic mean of the `exp(register)` terms
        let registers_sum = self.registers.iter()
            .enumerate()
            .map(|(i, val)| f64::exp(-Registers::decode(val, get_shift(&self.builder, i))))
            .sum::<f64>();
        let registers_mean = registers_sum / self.no_registers as f64;
        
        self.no_registers as f64 / registers_mean - 1.0
    }
}
