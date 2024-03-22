//! Bandwidth selection strategies.

pub mod scott;
pub mod silverman;

use crate::float::KDEFloat;

/// Shared behavior for bandwidth selection strategies.
pub trait Bandwidth<F: KDEFloat> {
    /// Returns a bandwidth value estimated from the points in `data`.
    fn bandwidth(&self, data: &[F]) -> F;
}

impl<T, F> Bandwidth<F> for T
where
    T: Fn(&[F]) -> F,
    F: KDEFloat,
{
    fn bandwidth(&self, data: &[F]) -> F {
        self(data)
    }
}
