use std::hash::{BuildHasher, Hash};

mod common;
mod registers;

pub use registers::Registers;
pub use common::{get_shift, mantissa_to_float, GumbelError, MIN_PRECISION, MAX_PRECISION, NEG_GAMMA};
pub mod gumbel;
pub mod ghll;
pub mod ghll_real;
pub mod ghll_plus;

pub use gumbel::{GumbelTransform, ICDFGumbel, BitHackGumbel, PadeGumbel, OptimalGumbel, FastGumbel};
pub use ghll::GHLL;
pub use ghll_real::GHLLReal;
pub use ghll_plus::GHLLPlus;

pub trait GumbelEstimator<T: ?Sized> {
    fn add(&mut self, value: &T);
    fn count(&self) -> f64;
}

pub struct GHLLGeo<B: BuildHasher, G: GumbelTransform>(GHLL<B, G>);
impl<B: BuildHasher, G: GumbelTransform, T: Hash> GumbelEstimator<T> for GHLLGeo<B, G> {
    fn add(&mut self, value: &T) { self.0.add(value); }
    fn count(&self) -> f64 { self.0.count_geo() }
}

pub struct GHLLHar<B: BuildHasher, G: GumbelTransform>(GHLL<B, G>);
impl<B: BuildHasher, G: GumbelTransform, T: Hash> GumbelEstimator<T> for GHLLHar<B, G> {
    fn add(&mut self, value: &T) { self.0.add(value); }
    fn count(&self) -> f64 { self.0.count_har() }
}

pub struct GHLLRealGeo<B: BuildHasher, G: GumbelTransform>(GHLLReal<B, G>);
impl<B: BuildHasher, G: GumbelTransform, T: Hash> GumbelEstimator<T> for GHLLRealGeo<B, G> {
    fn add(&mut self, value: &T) { self.0.add(value); }
    fn count(&self) -> f64 { self.0.count_geo() }
}

pub struct GHLLRealHar<B: BuildHasher, G: GumbelTransform>(GHLLReal<B, G>);
impl<B: BuildHasher, G: GumbelTransform, T: Hash> GumbelEstimator<T> for GHLLRealHar<B, G> {
    fn add(&mut self, value: &T) { self.0.add(value); }
    fn count(&self) -> f64 { self.0.count_har() }
}

impl<B: BuildHasher, G: GumbelTransform, T: Hash> GumbelEstimator<T> for GHLLPlus<B, G> {
    fn add(&mut self, value: &T) { self.add(value); }
    fn count(&self) -> f64 { self.count() }
}
