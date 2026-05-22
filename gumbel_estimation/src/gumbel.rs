use crate::common::mantissa_to_float;

const INV_MANTISSA: f32 = 1.0 / (1 << 23) as f32;
const LN_2: f32 = 0.69314718_f32;

// Merged bit-hack constants
const C1: f32 = 8.262958288192749e-8; // LN_2 / 2^23
const C2: f32 = -87.9899710891601;   // SIGMA - (127.0 * LN_2)

// Generic trait for Gumbel variate creation from either a [0, 1) f32 or its binary representation
pub trait GumbelTransform {
    // Gumbel distribution inverse CDF;
    // Generates a Gumbel value from a [0, 1) float
    fn quantile(&self, q: f32) -> f32;

    // Creates a Gumbel value from a mantissa bit representation of a [0, 1) float
    #[inline]
    fn from_bits(&self, bits: u32) -> f32 {
        self.quantile(mantissa_to_float(bits))
    }

    // Creates a Gumbel value in Q16.16 fixed-point format from a mantissa bit representation
    #[inline]
    fn from_bits_fixed(&self, bits: u32) -> i32 {
        (self.from_bits(bits) * 65536.0) as i32
    }
}

// Standard inverse CDF (quantile function) double-log Gumbel transform
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ICDFGumbel;
impl ICDFGumbel {
    pub fn new() -> Self {
        Self{}
    }
}
impl GumbelTransform for ICDFGumbel {
    #[inline]
    fn quantile(&self, q: f32) -> f32 {
        -f32::ln(-f32::ln(q))
    }
}

// Simple bit-hack Gumbel transform
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SimpleBitHackGumbel;
impl SimpleBitHackGumbel {
    pub fn new() -> Self{
        Self{}
    }

    #[inline(always)]
    fn bit_hack_ln(&self, x: f32) -> f32 {
        const C2_SIMPLE: f32 = -88.02969186; // -(127.0 * LN_2), no SIGMA shift to prevent log(negative)
        (x.to_bits() as f32).mul_add(C1, C2_SIMPLE)
    }
}
impl GumbelTransform for SimpleBitHackGumbel {
    #[inline]
    fn quantile(&self, q: f32) -> f32 {
        let ln_q = self.bit_hack_ln(q);
        -self.bit_hack_ln(-ln_q)
    }
}

// Taylor branching bit-hack Gumbel transform
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TaylorBitHackGumbel;
impl TaylorBitHackGumbel {
    pub fn new() -> Self {
        Self{}
    }

    #[inline(always)]
    fn bit_hack_ln(&self, x: f32) -> f32 {
        (x.to_bits() as f32).mul_add(C1, C2)
    }
}
impl GumbelTransform for TaylorBitHackGumbel {
    #[inline(always)]
    fn quantile(&self, q: f32) -> f32 {
        if q < 0.5 {
            let log_q = -self.bit_hack_ln(q);
            -self.bit_hack_ln(log_q)
        } else {
            let v = 1.0 - q;
            let base_y = -self.bit_hack_ln(v);
            let taylor = v * v.mul_add(0.20833333, 0.5);
            base_y - taylor
        }
    }

    #[inline(always)]
    fn from_bits(&self, bits: u32) -> f32 {
        let m = bits >> 9;
        let q = mantissa_to_float(bits);
        
        if m < (1 << 22) { // q < 0.5
            let log_q = -self.bit_hack_ln(q);
            -self.bit_hack_ln(log_q)
        } else {
            let v = 1.0 - q;
            let base_y = -self.bit_hack_ln(v);
            let taylor = v * v.mul_add(0.20833333, 0.5);
            base_y - taylor
        }
    }
}

// Taylor Integer Gumbel transform using fixed-point arithmetic and static LUT
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TaylorIntegerGumbel;

// 256-byte symmetric LUT (Q6.2) covering full u32 range
pub const GUMBEL_LUT: [i8; 256] = [
  -1,   0,   0,   1,  -3,  -2,  -2,  -1,
  -4,  -4,  -3,  -3,  -5,  -5,  -4,  -4,
  -6,  -5,  -5,  -5,  -6,  -6,  -6,  -6,
  -7,  -7,  -6,  -6,  -7,  -7,  -7,  -7,
  -8,  -8,  -7,  -7,  -8,  -8,  -8,  -8,
  -8,  -8,  -8,  -8,  -9,  -9,  -9,  -9,
  -9,  -9,  -9,  -9,  -9,  -9,  -9,  -9,
 -10, -10,  -9,  -9, -10, -10, -10, -10,
 -10, -10, -10, -10, -10, -10, -10, -10,
 -10, -10, -10, -10, -11, -11, -11, -11,
 -11, -11, -11, -11, -11, -11, -11, -11,
 -11, -11, -11, -11, -11, -11, -11, -11,
 -12, -11, -11, -11, -12, -12, -12, -12,
 -12, -12, -12, -12, -12, -12, -12, -12,
 -12, -12, -12, -12, -12, -12, -12, -12,
 -12, -12, -12, -12, -12, -12, -12, -12,
   4,   3,   3,   2,   8,   7,   6,   5,
  10,  10,   9,   8,  13,  13,  12,  11,
  16,  15,  15,  14,  19,  18,  17,  17,
  22,  21,  20,  20,  24,  24,  23,  22,
  27,  26,  26,  25,  30,  29,  29,  28,
  33,  32,  31,  31,  36,  35,  34,  34,
  38,  38,  37,  36,  41,  40,  40,  39,
  44,  43,  42,  42,  47,  46,  45,  45,
  49,  49,  48,  47,  52,  51,  51,  50,
  55,  54,  54,  53,  58,  57,  56,  56,
  61,  60,  59,  58,  63,  62,  62,  61,
  66,  65,  65,  64,  69,  68,  67,  67,
  72,  71,  70,  70,  74,  74,  73,  72,
  77,  76,  76,  75,  80,  79,  78,  78,
  83,  82,  81,  81,  85,  85,  84,  83,
  88,  87,  87,  86,  91,  90,  90,  89,
];

impl TaylorIntegerGumbel {
    pub fn new() -> Self {
        Self {}
    }
}

impl GumbelTransform for TaylorIntegerGumbel {
    #[inline]
    fn quantile(&self, q: f32) -> f32 {
        -f32::ln(-f32::ln(q))
    }

    #[inline(always)]
    fn from_bits_fixed(&self, bits: u32) -> i32 {
        if bits == 0 {
            return -1048576; // -16.0 in Q16.16
        }

        let is_high = bits >> 31;
        let target = bits ^ (0u32.wrapping_sub(is_high));
        // k is number of leading zeros (0..32)
        let k = target.leading_zeros().max(1).min(32);
        // Extract 2-bit mantissa: skip leading bit
        let m = (target.wrapping_shl(k.wrapping_add(1))) >> 30;
        
        // Expand Q8.2 to Q16.16: << 14
        (GUMBEL_LUT[((is_high << 7) | ((k - 1) << 2) | (m & 0x03)) as usize] as i32) << 14
    }

    #[inline(always)]
    fn from_bits(&self, bits: u32) -> f32 {
        self.from_bits_fixed(bits) as f32 * (1.0 / 65536.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taylor_integer_accuracy() {
        let taylor_int = TaylorIntegerGumbel::new();
        let mut test_cases = Vec::new();
        for i in 1..32 {
            test_cases.push(1 << i);
            test_cases.push(!(1 << i));
        }
        for bits in test_cases {
            if bits == 0 || bits == 0xFFFFFFFF { continue; }
            let u = bits as f64 / 4294967296.0;
            let expected = -(-u.ln()).ln();
            let actual = taylor_int.from_bits(bits) as f64;
            let diff = (expected - actual).abs();
            if u > 0.001 && u < 0.999 {
                assert!(diff < 0.5, "Failed at bits={:08x}, u={}, expected={}, actual={}, diff={}", bits, u, expected, actual, diff);
            }
        }
    }

    #[test]
    fn test_taylor_integer_bias() {
        let icdf = ICDFGumbel::new();
        let taylor_int = TaylorIntegerGumbel::new();
        let mut sum_diff = 0.0;
        let n = 100000;
        for i in 0..n {
            let bits = (i as u64 * 0xFFFFFFFF / n as u64) as u32;
            let expected = icdf.quantile((bits as f32 + 0.5) / 4294967296.0);
            let actual = taylor_int.from_bits(bits);
            sum_diff += actual - expected;
        }
        let avg_bias = sum_diff / n as f32;
        println!("TaylorIntegerGumbel Avg Bias: {}", avg_bias);
        assert!(avg_bias.abs() < 0.02);
    }
}
