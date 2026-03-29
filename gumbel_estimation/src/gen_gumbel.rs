const MIN_REGISTER_VALUE: f32 = -16.0;
const MAX_REGISTER_VALUE: f32 = 15.0;

pub const BIAS: i32 = 16;

// create a [0, 1) float from its mantissa bit represenations
pub fn mantissa_to_float(bits: u32) -> f32 {
    // create the exponent and mantissa bits
    let exponent_bits = 127 << 23;
    let mantissa_bits = bits >> 9;

    // combine the bits
    let bits = exponent_bits | mantissa_bits;

    f32::from_bits(bits) - 1.0
}

// create a gumbel random value from a mantissa bit representation of a [0, 1) float
#[inline(always)]
pub fn from_bits(bits: u32) -> f32 {
    // create a random [0, 1) float
    let random_unif = mantissa_to_float(bits);

    // create a gumbel random variable
    quantile(random_unif)
}

// create a gumbel random value from a [0, 1) float
#[inline(always)]
pub fn quantile(q: f32) -> f32 {
    -f32::ln(-f32::ln(q))
}

// create a gumbel random value from a bit representation of a [0, 1) float,
// but round the result to an integer with the rounding value of `c`
#[inline(always)]
pub fn from_bits_rounded(hash: u32, c: f32) -> u32 {
    let gumbel_value = from_bits(hash);
    shift_round(gumbel_value, c)
}

// create a gumbel random value from a [0, 1) float
// but round the result to an integer with the rounding value of `c`
#[inline(always)]
pub fn quantile_rounded(q: f32, c: f32) -> u32 {
    let gumbel_value = quantile(q);
    shift_round(gumbel_value, c)
}

// perform shift rounding of a value using the rounding value of `c`
#[inline(always)]
pub fn shift_round(value: f32, c: f32) -> u32 {
    ((f32::floor(value + c).clamp(MIN_REGISTER_VALUE, MAX_REGISTER_VALUE) as i32) + BIAS) as u32
}
