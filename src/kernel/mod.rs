//! Kernel functions.

pub mod epanechnikov;
pub mod normal;
pub mod uniform;
pub mod triangular;
pub mod quartic;
pub mod triweight;
pub mod tricube;
pub mod cosine;
pub mod logistic;
pub mod sigmoid;
pub mod silverman;

use crate::internal::Float;

/// Shared behavior for kernel functions.
///
/// Well-behaved kernel functions exhibit three properties:
/// 1) The function is non-negative and real-valued.
/// 2) The function should integrate to 1.
/// 3) The function should be symmetrical.
pub trait Kernel {
    /// Returns the probability density function for a value `x`.
    fn pdf(&self, x: Float) -> Float;
}

impl<T> Kernel for T
where
    T: Fn(Float) -> Float,
{
    fn pdf(&self, x: Float) -> Float {
        self(x)
    }
}
