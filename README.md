# Streaming Algorithms Comparison

This repository prodives a numerical comparison suite for streaming algorithms for the count-distinct problem. The algorithms tested are:

- **`HyperLogLog`**, see: Flajolet, Philippe et al. “HyperLogLog: the analysis of a near-optimal cardinality estimation algorithm.” Discrete Mathematics & Theoretical Computer Science (2007): 137-156.
- **`GumbelHyperLogLog`**, see: Lukasiewicz, Aleksander and Przemyslaw Uzna'nski. “Cardinality estimation using Gumbel distribution.” Embedded Systems and Applications (2020).
- **`GumbelHyperLogLog+`**: a modification to the `GumbelHyperLogLog` algorithm

The implementations of the GHLL and GHLL+ algorithms were created in Rust 1.82.0. They are located in the `/gumbel_estimation` directory in the attached archive. Inside the `/src` subdirectory, you will find the source code of the algorithms, specifically:

- `ghll.rs`: contains the GHLL algorithm in the version with discretization;
- `ghll_real.rs`: includes the same algorithm, but with real-number registers;
- `ghll_plus.rs`: contains the implementation of the GHLL+ algorithm;
- Various auxiliary modules, for example, for generating random variables from the Gumbel distribution or defining the structure of 5-bit registers for storing the maximum.

The `gumbel_estimation` directory serves only as a library providing these algorithms; therefore, the code inside does not compile into an executable file.

## Numerical Studies

For numerical studies, the `/comparison` directory is used, which contains both the code for collecting data on the accuracy of the estimators and the code for comparing the execution time of the algorithms. To set the parameters for running the tests, navigate to the `/src` subdirectory and open the `constants.rs` file. This file contains all the parameters used by the program, ready for modification:

- **`CARDINALITIES`**: An array of the number of unique elements for which the studies should be conducted. The file contains predefined values for both large and small data sets. Simply comment/uncomment the appropriate line. If you want to use a different array definition, you can edit the parameters of the `array_from_range` function, which creates an array from three arguments: the starting value `begin`, the step `step`, and the number of elements specified in the variable's type declaration. For example:

    ```rust
    pub const CARDINALITIES: [usize; 25] = array_from_range(100, 10)
    ```

    will create an array of 25 elements: `[100, 110, 120, ..., 330, 340]`.

- **`DATA_SIZE_MULTIPLIES`**: Defines the value by which the number of unique elements is multiplied to obtain the total number of elements in the data set. There are versions available for both types of test sets considered in the thesis. You can also use other values by defining the array accordingly, keeping in mind that its size should be adjusted in the type declaration to match the new definition.

- **`PRECISIONS`**: An array of "precisions" for the algorithms. This is used to determine the number of substreams used by the algorithm according to the formula \(k = 2^{\{precision\}}\). For each value in this array, a separate thread will be created during the experiments to speed up the study of the algorithm's accuracy.

- **`ITERATIONS`**: Defines the number of iterations performed by each algorithm for a single data set.

After setting the parameters, you can run the program to collect the accuracy of the estimations using the command

```bash
cargo run --release
```

The results will be saved to the `/results` directory in the main folder of the archive. To compare execution times, use the command

```bash
cargo bench
```

Generated plots will be located in the `/target/criterion/report` directory. Both commands must be executed in the `/comparison` directory; otherwise, an error may occur due to incorrect file paths.

## Data Generation

The programs for performing numerical studies assume that the data sets are located in the `/data` directory in the appropriate files. To generate the data, you can use the `gen_data.sh` script, which should be run from the main directory of the archive. The script contains ready-to-use code snippets for generating both large and small data sets. For custom configurations, you can modify the script accordingly. 

The data is generated using the program located in the `gen_data` directory, which contains the generator code. This program creates random 64-bit integers and places them in the appropriate file in the `/data` folder.

## Visualization

Plot generation takes place in the `/visualisation` directory. It contains the source code of four programs written in R:

- `boxplot.R`: Used to generate boxplots for the algorithms.
- `means.R`: Generates plots comparing the mean values.
- `real.R`: A separate program for creating plots of the means comparing the versions of the GHLL algorithm with and without discretization.
- `scatter.R`: Creates scatter plots illustrating the spread of results for a single algorithm.

There is also an auxiliary file `common.R` that contains a function to read the collected results. Each of the four scripts contains predefined configurations to generate plots for small and large data. You just need to modify the value of the `data.chosen` variable at the top of the script, which represents the category of test sets: `1` for small data sets, and `2` for large data sets. To use your custom configuration, modify the definitions of lists such as `cardinalities` inside the code.

The programs can be run from the shell using the command

```bash
Rscript [filename]
```

Make sure to run the command from the `/visualisation` directory. The resulting plot will be saved in the same folder.
