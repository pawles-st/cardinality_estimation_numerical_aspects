#!/bin/bash
# Master Reproduction Script for Gumbel HLL Thesis
set -e

# Ensure output directories exist
mkdir -p data results plots

echo "=== 1. Generating Raw Data ==="
./gen_data.sh

echo "=== 2. Running Estimator Experiments ==="
# Generate comma-separated list of cardinalities: 10000, 20000, ..., 600000
cards=$(seq 10000 10000 600000 | tr '\n' ',' | sed 's/,$//')

cd comparison
cargo run --release -- \
    --cardinalities "$cards" \
    --multiplier 100 \
    --precisions 4,8,12,16 \
    --iterations 100 \
    --algorithms hll,ghll,ghll-plus,ghll-plus-float
cd ..

echo "=== 3. Generating Approximation Error Data ==="
cargo run --release --bin gen_all_errors --manifest-path="gumbel_estimation/Cargo.toml"

echo "=== 4. Running Timing Benchmarks ==="
cd comparison
# Generic run
cargo bench --bench ghll_throughput
for d in ../target/criterion/*/*/; do
    if [ -d "${d}new" ]; then
        rm -rf "${d}generic"
        mv "${d}new" "${d}generic"
    fi
done

# Native run (requires CPU with BMI2/AVX2 for best results)
RUSTFLAGS="-C target-cpu=native" cargo bench --bench ghll_throughput
for d in ../target/criterion/*/*/; do
    if [ -d "${d}new" ]; then
        rm -rf "${d}native"
        mv "${d}new" "${d}native"
    fi
done
cd ..

echo "=== 5. Generating Plots ==="
Rscript visualisation/boxplot.R
Rscript visualisation/scatter.R
Rscript visualisation/approximation_error.R
Rscript visualisation/tput.R

echo "=== Reproduction Complete! ==="
echo "All plots are available in the plots/ folder."
