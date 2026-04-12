// this file can be edited to change the dataset sizes used for benchmarks and comparisons

use gumbel_estimation::{GumbelTransform, ICDFGumbel, BitHackGumbel};

// create a const array from a start value and step
const fn array_from_range<const K: usize>(begin: usize, step: usize) -> [usize; K] {
    // create the array
    let mut result = [0; K];

    // Fill the array with values
    let mut curr = begin;
    let mut index = 0;
    while index < K {
        result[index] = curr;
        curr += step;
        index += 1;
    }

    result   
}

#[derive(Debug, Copy, Clone)]
pub enum Transform {
    ICDF,
    BitHack,
}

impl GumbelTransform for Transform {
    fn quantile(&self, q: f32) -> f32 {
        match self {
            Transform::ICDF => ICDFGumbel.quantile(q),
            Transform::BitHack => BitHackGumbel.quantile(q),
        }
    }
}

pub const TRANSFORMS: &[(Transform, &str, bool)] = &[
    (Transform::ICDF, "ICDF", true),
    (Transform::BitHack, "BitHack", false),
];

// cardinalities of the underlying multisets
//pub const CARDINALITIES: &[usize] = &array_from_range::<50>(10_000, 10_000); // large datasets
//pub const CARDINALITIES: &[usize] = &array_from_range::<10>(10_000, 10_000); // large datasets
pub const CARDINALITIES: &[usize] = &[50_000]; // large datasets
//pub const CARDINALITIES: &[usize] = &array_from_range::<2000>(1, 1); // small datasets

// dataset size multiplies; the size of the dataset
// is calculated as `cardinality * data_size_multiply`
pub const DATA_SIZE_MULTIPLIES: &[usize] = &[100]; // large datasets
//pub const DATA_SIZE_MULTIPLIES: &[usize] = &[10]; // small datasets

// precisions to use for the HyperLogLog and Gumbel estimators;
// the number of registers used is equal to `2^precision`
//pub const PRECISIONS: &[u8] = &[4, 8, 12, 16];
pub const PRECISIONS: &[u8] = &[8];

// the number of iterations per single dataset
pub const ITERATIONS: usize = 100;
