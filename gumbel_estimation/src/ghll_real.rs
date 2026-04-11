use std::hash::{Hash, BuildHasher};
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

use crate::common::*;
use crate::gumbel::GumbelTransform;

/// A cardinality estimator using the Gumbel distribution
#[derive(Debug, Clone)]
pub struct GHLLReal<B: BuildHasher, G: GumbelTransform> {
    builder: B,
    transform: G,
    precision: u8,
    no_registers: usize,
    registers: Vec<f32>,
}

impl<B: BuildHasher, G: GumbelTransform> GHLLReal<B, G> {
    /// Creates a new GumbelHyperLogLog object with floating point registers for a specified precision and hash builder
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
        let registers: Vec<_> = (0..no_registers).map(|_| {
            let q = rng.sample(unif);
            transform.quantile(q)
        }).collect();

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

        // Update the register to the current max of Gumbel random variables
        self.registers[index] = f32::max(self.registers[index], gumbel_variate);
    }

    /// Returns the approximate cardinality of the stream,
    /// averaging substream results with the geometric mean
    pub fn count_geo(&self) -> f64 {
        // Calculate the geometric mean of the `exp(register)` terms
        let registers_sum = self.registers.iter()
            .map(|&val| val as f64)
            .sum::<f64>();
        let registers_mean = registers_sum / self.no_registers as f64;
        
        self.no_registers as f64 * f64::exp(NEG_GAMMA + registers_mean)
    }
    
    /// Returns the approximate cardinality of the stream,
    /// averaging substream results with the harmonic mean
    pub fn count_har(&self) -> f64 {
        // Calculate the harmonic mean of the `exp(register)` terms
        let registers_sum = self.registers.iter()
            .map(|&val| f64::exp(-val as f64))
            .sum::<f64>();
        let registers_mean = registers_sum / self.no_registers as f64;
        
        self.no_registers as f64 / registers_mean - 1.0
    }
}
