use gumbel_estimation::{
    GumbelTransform, 
    NaiveGumbel, 
    PadeGumbel, 
    BasicBitHackGumbel, 
    CorrectedBitHackGumbel, 
    SplineGumbel, 
    LUTGumbel
};
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("visualisation/all_errors.csv")?;
    let mut writer = BufWriter::new(file);

    // Header
    writeln!(writer, "q,Naive,Pade,BasicBitHack,CorrectedBitHack,Spline,LUT")?;

    let naive = NaiveGumbel::new();
    let pade = PadeGumbel::new();
    let basic_bh = BasicBitHackGumbel::new();
    let corrected_bh = CorrectedBitHackGumbel::new();
    let spline = SplineGumbel::new();
    let lut = LUTGumbel::new();

    // Sample 10,000 points across the [0, 1] range
    let samples = 10000;
    for i in 1..samples {
        let q = i as f64 / samples as f64;
        let q32 = q as f32;

        // "True" value using high-precision f64
        let true_val = -(-q.ln()).ln();

        // Calculate absolute errors for each transform
        let err_naive = (naive.quantile(q32) as f64 - true_val).abs();
        let err_pade = (pade.quantile(q32) as f64 - true_val).abs();
        let err_basic = (basic_bh.quantile(q32) as f64 - true_val).abs();
        let err_corrected = (corrected_bh.quantile(q32) as f64 - true_val).abs();
        let err_spline = (spline.quantile(q32) as f64 - true_val).abs();
        let err_lut = (lut.quantile(q32) as f64 - true_val).abs();

        writeln!(
            writer,
            "{},{},{},{},{},{},{}",
            q, err_naive, err_pade, err_basic, err_corrected, err_spline, err_lut
        )?;
    }

    println!("Generated visualisation/all_errors.csv");
    Ok(())
}
