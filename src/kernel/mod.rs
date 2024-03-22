//! Kernel functions.

pub mod cosine;
pub mod epanechnikov;
pub mod logistic;
pub mod normal;
pub mod quartic;
pub mod sigmoid;
pub mod silverman;
pub mod triangular;
pub mod tricube;
pub mod triweight;
pub mod uniform;

use crate::float::KDEFloat;

/// Shared behavior for kernel functions.
///
/// Well-behaved kernel functions exhibit three properties:
/// 1) The function is non-negative and real-valued.
/// 2) The function should integrate to 1.
/// 3) The function should be symmetrical.
pub trait Kernel<F: KDEFloat> {
    /// Returns the probability density function for a value `x`.
    fn pdf(&self, x: F) -> F;
}

impl<T, F> Kernel<F> for T
where
    T: Fn(F) -> F,
    F: KDEFloat,
{
    fn pdf(&self, x: F) -> F {
        self(x)
    }
}
