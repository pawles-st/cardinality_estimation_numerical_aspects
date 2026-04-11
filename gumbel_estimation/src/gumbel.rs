use crate::common::mantissa_to_float;

const INV_MANTISSA: f32 = 1.0 / (1 << 23) as f32;
const LN_2: f32 = 0.69314718_f32;
const SIGMA: f32 = 0.0430357_f32;

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

// Bit-hack Gumbel generator
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BitHackGumbel;
impl BitHackGumbel {
    pub fn new() -> Self{
        Self{}
    }

    // Fast linear approximation of ln(x) for x > 0
    // For x = 2^e (1 + m), the approximation is
    // ln(x) \approx e ln(2) + m
    #[inline]
    fn bit_hack_ln(&self, x: f32) -> f32 {
        let bits = x.to_bits();

        // Bits 30-23 hold the biased exponent; subtracting 127
        // gives the actual power of 2
        let e = ((bits >> 23) & 0xFF) as i32 - 127;

        // Bits 22-0 hold the mantissa; multiply by 2^-23
        // to normalise to [0, 1)
        let m = (bits & 0x7FFFFF) as f32 * INV_MANTISSA;

        // Return the linear approximation e ln(2) + m
        (e as f32 * LN_2) + m
    }
}
impl GumbelTransform for BitHackGumbel {
    #[inline]
    fn quantile(&self, q: f32) -> f32 {
        let ln_q = self.bit_hack_ln(q);
        -self.bit_hack_ln(-ln_q)
    }
}
