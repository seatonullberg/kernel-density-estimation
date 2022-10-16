//! Bandwidth selection strategies.

pub mod scott;
pub mod silverman;

use crate::internal::Float;

/// Shared behavior for bandwidth selection strategies.
pub trait Bandwidth {
    /// Returns a bandwidth value estimated from the points in `data`.
    fn bandwidth(&self, data: &[Float]) -> Float;
}

impl<T> Bandwidth for T
where
    T: Fn(&[Float]) -> Float,
{
    fn bandwidth(&self, data: &[Float]) -> Float {
        self(data)
    }
}
