/// Fast approximation of -ln(-ln(x)) for x in (0, 1) using Padé approximants.
/// This is a translation of the optimized C implementation.
pub fn fast_neg_lnln_pade(val: f32) -> f32 {
    let u = val.to_bits();
    
    // IEEE 754: x = (1 + f) * 2^(E-127). For x < 1, E-127 < 0.
    // Let m = 127 - E. Then x = (1 + f) * 2^-m.
    let exp_bits = (u >> 23) & 0xFF;
    let m = 127 - exp_bits as i32;

    // Constants
    const C1: f32 = 0.69314718; // ln(2)
    const C2: f32 = -0.36651292; // ln(ln(2))

    // 1. Divergence Handling for x in [0.5, 1)
    if m == 1 {
        let v = 1.0 - val;
        // High-precision approach for y = -ln(1-v)
        let w = v / (2.0 - v);
        let z = w * w;
        // Padé [3/2] approximant for 2*artanh(w)
        let num_l = (-8.0 * z).mul_add(w, 30.0 * w);
        let den_l = (-9.0).mul_add(z, 15.0);
        let y = num_l / den_l;
        
        // Return -ln(-ln(x))
        return -y.ln();
    }

    // 2. Core Rational Approximation (For m >= 2)
    // Extract Mantissa M in [1, 2)
    let t_bits = (u & 0x7FFFFF) | 0x3F800000;
    let m_mantissa = f32::from_bits(t_bits);

    // Step 3a: Inner Logarithm Approximation L ≈ ln(M)
    let w = (m_mantissa - 1.0) / (m_mantissa + 1.0);
    let z = w * w;
    let num_l = (-8.0 * z).mul_add(w, 30.0 * w);
    let den_l = (-9.0).mul_add(z, 15.0);
    let l = num_l / den_l;

    // Step 3b: Outer Logarithm Compression K ≈ ln(1-u)
    // where u = ln(M) / (m * ln(2))
    let u_val = l / (m as f32 * C1);
    // Padé [2/2] for ln(1-u)
    let num_k = (3.0 * u_val).mul_add(u_val, -6.0 * u_val);
    let den_k = u_val.mul_add(u_val, (-6.0f32).mul_add(u_val, 6.0));
    let k = num_k / den_k;

    // Step 3c: Final Assembly
    // ln(-ln(x)) ≈ ln(m) + ln(ln(2)) + K
    // We return the negation for -ln(-ln(x))
    -(get_ln_m(m as usize) + C2 + k)
}

/// Precomputed table for ln(m) to avoid runtime log calls.
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
    LN_TABLE[m]
}
