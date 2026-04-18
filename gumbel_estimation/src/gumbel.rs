use crate::common::mantissa_to_float;

const INV_MANTISSA: f32 = 1.0 / (1 << 23) as f32;
const LN_2: f32 = 0.69314718_f32;
const SIGMA: f32 = 0.0430357_f32;
//const SIGMA: f32 = 0.057305_f32;

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

// The Optimal Gumbel Algorithm
// This version uses a direct float-to-int cast for the inner log2 extraction
// and a single bit-hack ln for the outer log, significantly reducing operations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OptimalGumbel;

impl OptimalGumbel {
    pub fn new() -> Self {
        Self
    }
}

impl GumbelTransform for OptimalGumbel {
    #[inline]
    fn quantile(&self, q: f32) -> f32 {
        let bits_u = q.to_bits();

        // 1. Inner log2 Extraction & 2. Fast Absolute Value
        // Since q in (0, 1), log2(u) is negative.
        // We compute v = -log2(u) directly.
        let v = 127.0 - (bits_u as f32 * INV_MANTISSA);

        // 3. Outer ln Evaluation
        let bits_v = v.to_bits();
        // Since v > 0, bit 31 is 0, so (bits_v >> 23) has bit 8 as 0.
        // We can safely skip & 0xFF.
        let ev = (bits_v >> 23) as i32 - 127;
        let mv = (bits_v & 0x7FFFFF) as f32 * INV_MANTISSA;
        let w = (ev as f32 * LN_2) + mv;

        // 4. Constant Correction
        // X \approx -w - ln(ln(2)). 
        // ln(ln(2)) \approx -0.36651292.
        const NEG_LN_LN_2: f32 = 0.36651292;
        -w + NEG_LN_LN_2
    }
}

// High-throughput approximation of -ln(x) via IEEE-754 type-punning.
// Uses a global linear approximation of the floating point bit representation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FastGumbel;

impl FastGumbel {
    pub fn new() -> Self {
        Self
    }

    #[inline(always)]
    fn fast_neg_log(x: f32) -> f32 {
        let i = x.to_bits();
        // Reduced baseline bias from +0.07 to 0.00
        -8.262958e-8 * (i as f32) + 87.990
    }
}

impl GumbelTransform for FastGumbel {
    #[inline]
    fn quantile(&self, q: f32) -> f32 {
        // Shifted singularity for Regime B
        let v = 1.0 - q;

        // Evaluate Regime A inner log
        let log_q = Self::fast_neg_log(q);

        // Branchless hardware select (compiles to cmov or blend)
        let z = if q < 0.5 { log_q } else { v };

        // Compute outer log
        let base_y = Self::fast_neg_log(z);

        // Branchless higher-order Taylor terms for Regime B: v/2 + 5v^2/24
        // 5/24 \approx 0.20833333
        let taylor_expansion = v * (0.5 + 0.20833333 * v);
        let residual = if q >= 0.5 { taylor_expansion } else { 0.0 };

        base_y - residual
    }
}

// Padé approximant based Gumbel generator
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PadeGumbel;

impl PadeGumbel {
    pub fn new() -> Self {
        Self
    }

    #[inline(always)]
    fn get_ln_m(m: usize) -> f32 {
        const LN_TABLE: [f32; 128] = [
            0.00000000, 0.00000000, 0.69314718, 1.09861229, 1.38629436, 1.60943791, 1.79175947, 1.94591015,
            2.07944154, 2.19722458, 2.30258509, 2.39789527, 2.48490665, 2.56494936, 2.63905733, 2.70805020,
            2.77258872, 2.83321334, 2.89037176, 2.94443898, 2.99573227, 3.04452244, 3.09104245, 3.13549422,
            3.17805383, 3.21887582, 3.25809654, 3.29583687, 3.33220451, 3.36729583, 3.40119738, 3.43398720,
            3.46573590, 3.49650756, 3.52636051, 3.55534806, 3.58351894, 3.61091791, 3.63758616, 3.66356165,
            3.68887945, 3.71357207, 3.73766962, 3.76120012, 3.78418963, 3.80666249, 3.82864140, 3.85014760,
            3.87120101, 3.89182030, 3.91202301, 3.93182563, 3.95124372, 3.97029191, 3.98898405, 4.00733319,
            4.02535169, 4.04305127, 4.06044301, 4.07753744, 4.09434456, 4.11087386, 4.12713439, 4.14313473,
            4.15888308, 4.17438727, 4.18965474, 4.20469262, 4.21950771, 4.23410650, 4.24849524, 4.26267988,
            4.27666612, 4.29045944, 4.30406509, 4.31748811, 4.33073334, 4.34380542, 4.35670883, 4.36944785,
            4.38202663, 4.39444915, 4.40671925, 4.41884061, 4.43081680, 4.44265126, 4.45434730, 4.46590812,
            4.47733681, 4.48863637, 4.49980967, 4.51085951, 4.52178858, 4.53259949, 4.54329478, 4.55387689,
            4.56434819, 4.57471098, 4.58496748, 4.59511985, 4.60517019, 4.61512052, 4.62497281, 4.63472899,
            4.64439089, 4.65396035, 4.66343909, 4.67282883, 4.68213123, 4.69134788, 4.70048037, 4.70953020,
            4.71849887, 4.72738782, 4.73619845, 4.74493213, 4.75359019, 4.76217393, 4.77068462, 4.77912349,
            4.78749174, 4.79579055, 4.80402104, 4.81218422, 4.82028157, 4.82831374, 4.83628191, 4.84418709,
        ];
        LN_TABLE[m & 0x7F]
    }
}

impl GumbelTransform for PadeGumbel {
    #[inline]
    fn quantile(&self, q: f32) -> f32 {
        let u = q.to_bits();
        let exp_bits = (u >> 23) & 0xFF;
        let m = 127 - exp_bits as i32;

        const C1: f32 = 0.69314718; // ln(2)
        const C2: f32 = -0.36651292; // ln(ln(2))

        if m == 1 {
            let v = 1.0 - q;
            let w = v / (2.0 - v);
            let z = w * w;
            let num_l = (-8.0f32 * z).mul_add(w, 30.0 * w);
            let den_l = (-9.0f32).mul_add(z, 15.0);
            let y = num_l / den_l;
            
            // For the m=1 branch, we follow the C implementation's lead:
            // use a manual inner log approximation but call the standard library 
            // for the final outer log to avoid the overhead of a second bit-hack chain.
            return -y.ln();
        }

        let t_bits = (u & 0x7FFFFF) | 0x3F800000;
        let m_mantissa = f32::from_bits(t_bits);

        let w = (m_mantissa - 1.0) / (m_mantissa + 1.0);
        let z = w * w;
        let num_l = (-8.0 * z).mul_add(w, 30.0 * w);
        let den_l = (-9.0f32).mul_add(z, 15.0);
        let l = num_l / den_l;

        let u_val = l / (m as f32 * C1);
        let num_k = (3.0 * u_val).mul_add(u_val, -6.0 * u_val);
        let den_k = u_val.mul_add(u_val, (-6.0f32).mul_add(u_val, 6.0));
        let k = num_k / den_k;

        -(Self::get_ln_m(m as usize) + C2 + k)
    }
}
