mod common;
mod registers;

pub mod gumbel;
pub mod ghll;
pub mod ghll_real;
pub mod ghll_plus;
pub mod ghll_float;

pub use ghll::GHLL;
pub use ghll_real::GHLLReal;
pub use ghll_plus::GHLLPlus;
pub use ghll_float::{GHLLFloat, GHLLPlusFloat};

pub use gumbel::{
    GumbelTransform, 
    NaiveGumbel, 
    PadeGumbel, 
    BasicBitHackGumbel,
    CorrectedBitHackGumbel, 
    SplineGumbel, 
    LUTGumbel
};

use std::hash::{BuildHasher, Hash};

pub trait GumbelEstimator<T: ?Sized> {
    fn add(&mut self, value: &T);
    fn count(&self) -> f64;
}

impl<B: BuildHasher, G: GumbelTransform, T: Hash> GumbelEstimator<T> for GHLL<B, G> {
    fn add(&mut self, value: &T) { self.add(value); }
    fn count(&self) -> f64 { self.count_geo() }
}

impl<B: BuildHasher, G: GumbelTransform, T: Hash> GumbelEstimator<T> for GHLLPlus<B, G> {
    fn add(&mut self, value: &T) { self.add(value); }
    fn count(&self) -> f64 { self.count() }
}

impl<B: BuildHasher, G: GumbelTransform, T: Hash> GumbelEstimator<T> for GHLLPlusFloat<B, G> {
    fn add(&mut self, value: &T) { self.add(value); }
    fn count(&self) -> f64 { self.count() }
}
