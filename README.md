# Streaming Algorithms Comparison

This repository provides a numerical comparison suite for streaming algorithms for the count-distinct problem. The project focuses on the **GumbelHyperLogLog** algorithm and its variants, specifically exploring different methods for calculating the Gumbel transform.

The algorithms tested are:

- **`HyperLogLog (HLL)`**, see: Flajolet, Philippe et al. “HyperLogLog: the analysis of a near-optimal cardinality estimation algorithm.” Discrete Mathematics & Theoretical Computer Science (2007): 137-156.
- **`GumbelHyperLogLog (GHLL)`**, see: Lukasiewicz, Aleksander and Przemyslaw Uzna'nski. “Cardinality estimation using Gumbel distribution.” Embedded Systems and Applications (2020).
- **`GumbelHyperLogLog+ (HLL)`**: a modification to the `GumbelHyperLogLog` algorithm that improves accuracy and handling of small cardinalities, from an engineering thesis "Efficiency of algorithms for counting unique elements in the data stream" (2025).

## Gumbel Transform Methods

The performance and accuracy of the GHLL algorithm depend heavily on the method used to compute the Gumbel transform (the inverse CDF of the Gumbel distribution: $G^{-1}(q) = -\ln(-\ln(q))$). This repository implements and compares several approaches:

| Method | Description | Characteristics |
| :--- | :--- | :--- |
| **Naive** | Standard implementation using two `ln()` calls. | **High Accuracy**, Low Performance |
| **Padé** | Uses Padé approximants to estimate logarithms. | **Good Accuracy**, Low Performance |
| **Basic Bit-hack** | Simple bit-manipulation trick to approximate $\ln(x)$. | Low Accuracy, **High Performance** |
| **Corrected Bit-hack** | Bit-hack with Taylor expansion correction for $q \approx 1$. | Moderate Accuracy, Moderate Performance |
| **Spline** | Piecewise linear spline with 256 segments. | **High Accuracy**, **High Performance** |
| **LUT** | Small 256-byte static lookup table with fixed-point math. | Moderate Accuracy, **Ultra Performance** |

The GHLL and GHLL+ algorithms are implemented in Rust and located in the `/gumbel_estimation` directory, with specific Gumbel transform methods found in `gumbel_estimation/src/gumbel.rs`.

## Prerequisites

To run the experiments and generate plots, you need:

1.  **Rust** (1.82.0 or newer): [https://rustup.rs/](https://rustup.rs/)
2.  **R**: [https://www.r-project.org/](https://www.r-project.org/)
3.  **R Packages**:
    ```R
    install.packages(c("ggplot2", "tidyr", "dplyr", "jsonlite"))
    ```

## Quick Start (Reproduce All Results)

The easiest way to reproduce all results from the thesis is to run the master reproduction script from the root directory:

```bash
chmod +x reproduce_full.sh
./reproduce_full.sh
```

This script will:
1.  Generate all necessary datasets in the `data/` folder.
2.  Run accuracy experiments for all estimators.
3.  Generate approximation error data for different Gumbel transform methods.
4.  Run throughput benchmarks (both generic and native).
5.  Generate all plots in the `plots/` folder.

**Note:** Generating data and running experiments can take a significant amount of time depending on your hardware.

## Modular Usage

If you want to run components of the pipeline separately, follow these steps in order.

### 1. Data Generation
Generate the raw datasets used for experiments:
```bash
./gen_data.sh
```
This populates the `data/` directory.

### 2. Accuracy Experiments
Run the estimators on the generated data:
```bash
cd comparison
cargo run --release -- --cardinalities 10000,50000 --multiplier 100
cd ..
```
The results (raw estimation values) are saved to `results/`.

### 3. Approximation Error Data
To compare different methods for calculating Gumbel transforms (Naive, Pade, Spline, etc.):
```bash
cargo run --release --bin gen_all_errors --manifest-path="gumbel_estimation/Cargo.toml"
```
This generates `visualisation/all_errors.csv`.

### 4. Throughput Benchmarks (Optional)
Run timing benchmarks to compare estimator speed:
```bash
cd comparison
# Generic run
cargo bench --bench ghll_throughput
# Native run (optimized for current CPU)
RUSTFLAGS="-C target-cpu=native" cargo bench --bench ghll_throughput
cd ..
```
**Note:** The plotting scripts expect benchmark results to be organized into `generic` and `native` directories. If running manually, you may need to rename the `new` folders in `target/criterion/` accordingly (the `reproduce_full.sh` script handles this automatically).

### 5. Plotting
Generate visualisations from the collected `results/`:
```bash
# Run from the project root
Rscript visualisation/boxplot.R
Rscript visualisation/tput.R
Rscript visualisation/scatter.R
Rscript visualisation/approximation_error.R
```
The resulting plots will be saved in the `plots/` folder.

## Numerical Studies

For custom numerical studies, the `/comparison` directory contains the tool for collecting accuracy data and comparing execution times.

### Accuracy Experiments

The tool uses a command-line interface. To run with default parameters:

```bash
cargo run --release --manifest-path="comparison/Cargo.toml"
```

Custom parameters can be provided:
- `--cardinalities`: List of unique elements to test (e.g., `10000,50000,100000`).
- `--multiplier`: Data size multiplier (default: 100).
- `--precisions`: List of precisions to test (e.g., `4,8,12,16`).
- `--iterations`: Number of independent iterations (default: 100).
- `--algorithms`: Algorithms to test (`hll`, `ghll`, `ghll-plus`, `ghll-plus-float`).
- `--transforms`: Gumbel transform methods (`naive`, `pade`, `basic-bithack`, `corrected-bithack`, `spline`, `lut`).

Example:
```bash
cargo run --release --manifest-path="comparison/Cargo.toml" -- --cardinalities 10000,50000 --precisions 12 --iterations 50 --algorithms hll,ghll-plus
```

The results will be saved to the `/results` directory.

### Throughput Benchmarks

To compare execution times, use:

```bash
cargo bench --manifest-path="comparison/Cargo.toml"
```

Or for hardware-optimized results:
```bash
RUSTFLAGS="-C target-cpu=native" cargo bench --manifest-path="comparison/Cargo.toml"
```

Generated reports will be in `target/criterion/report`.

## Project Structure

- `gumbel_estimation/`: Core Rust library containing implementations of GHLL and GHLL+.
    - `ghll.rs`: Standard GumbelHyperLogLog implementation.
    - `ghll_plus.rs`: GHLL+ implementation with improved bias correction.
    - `ghll_float.rs`: Alternative GHLL+ implementation using floating-point registers.
    - `gumbel.rs`: Implementations of various Gumbel transform calculation methods (Pade, Spline, LUT, etc.).
- `comparison/`: Benchmarking and comparison tool for running accuracy and throughput experiments.
- `gen_data/`: Utility for generating random datasets.
- `visualisation/`: R scripts for data analysis and plot generation.

## Data Generation

The programs for performing numerical studies assume that the datasets are located in the `/data` directory in the appropriate files. To generate the data, you can use the `gen_data.sh` script, which should be run from the main directory. The script contains ready-to-use code snippets for generating both large and small datasets.

The data is generated using the program located in the `gen_data` directory. It creates random 64-bit integers and places them in the `/data` folder.

## Visualization

Plot generation takes place in the `/visualisation` directory. It contains the following R programs:

- `boxplot.R`: Generates boxplots comparing the accuracy of different estimators.
- `tput.R`: Generates throughput comparison plots.
- `approximation_error.R`: Analyzes and plots approximation errors for Gumbel transforms.
- `scatter.R`: Creates scatter plots illustrating the spread of results for a single algorithm.

There is also an auxiliary file `common.R` that contains shared helper functions.

The programs should be run from the root directory using the command:

```bash
Rscript visualisation/[filename]
```

The resulting plots will be saved in the `plots/` folder.
