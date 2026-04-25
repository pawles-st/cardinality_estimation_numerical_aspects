use std::hash::{Hash, BuildHasher};

pub const NEG_GAMMA: f64 = -0.577_215_664_901_532_9_f64;
pub const MIN_PRECISION: u8 = 4;
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
    let hash = builder.hash_one(value);
    // Use top bits for index, bottom 32 bits for remainder to avoid shift-induced zeros
    let index = (hash >> (64 - precision)) as usize;
    let remainder = hash as u32;
    (index, remainder)
}

#[inline]
pub fn mantissa_to_float(bits: u32) -> f32 {
    let exponent_bits = 127 << 23;
    let mantissa_bits = bits >> 9;
    f32::from_bits(exponent_bits | mantissa_bits) - 1.0
}

#[inline]
pub fn get_shift<B: BuildHasher>(builder: &B, index: usize) -> f32 {
    let hash = builder.hash_one(&index) as u32;
    mantissa_to_float(hash)
}

#[inline]
pub fn get_shift_fixed<B: BuildHasher>(builder: &B, index: usize) -> i32 {
    let hash = builder.hash_one(&index) as u32;
    (hash >> 16) as i32
}
