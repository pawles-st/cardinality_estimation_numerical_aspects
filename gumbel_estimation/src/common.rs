use std::hash::{Hash, BuildHasher};

/// Negative value of the gamma constant
pub const NEG_GAMMA: f64 = -0.577_215_664_901_532_9_f64;

/// The minumal accepted precision
pub const MIN_PRECISION: u8 = 4;

/// The maximmal accepted precision
pub const MAX_PRECISION: u8 = 16;

#[derive(Debug)]
pub enum GumbelError {
    InvalidPrecision,
}

#[inline(always)]
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
