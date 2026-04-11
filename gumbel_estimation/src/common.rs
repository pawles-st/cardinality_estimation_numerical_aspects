use std::hash::{Hash, BuildHasher};

/// Negative value of Euler gamma constant
pub const NEG_GAMMA: f64 = -0.577_215_664_901_532_9_f64;

/// The minimal accepted precision
pub const MIN_PRECISION: u8 = 4;

/// The maximal accepted precision
pub const MAX_PRECISION: u8 = 16;

#[derive(Debug)]
pub enum GumbelError {
    InvalidPrecision,
}

#[inline]
pub fn hash_value<H, B>(value: &H, builder: &B, precision: u8) -> (usize, u32) 
where
    H: Hash + ?Sized,
    B: BuildHasher,
{
    // obtain the value's hash
    let mut hash = builder.hash_one(value) as u32;
    
    // choose a register based on the first `precision` bits
    let index: usize = (hash >> (32 - precision)) as usize;

    // discard the above bits from the hash
    hash <<= precision;

    (index, hash)
}

// Create a [0, 1) float from its binary represenation
#[inline]
pub fn mantissa_to_float(bits: u32) -> f32 {
    // Bits 30-23 hold the biased exponent; the value of 127
    // corresponds to the unbiased exponent being equal to 0
    let exponent_bits = 127 << 23;

    // Bits 22-0 hold the mantissa;
    let mantissa_bits = bits >> 9;

    // Combine the bits
    let bits = exponent_bits | mantissa_bits;

    // Created float is of the form (1 + m), m \in [0, 1);
    // subtract one to shift to [0, 1) range
    f32::from_bits(bits) - 1.0
}

/// Obtains a shift rounding value for a give register index
#[inline]
pub fn get_shift<B: BuildHasher>(builder: &B, index: usize) -> f32 {
    let hash = builder.hash_one(&index) as u32;
    mantissa_to_float(hash)
}
