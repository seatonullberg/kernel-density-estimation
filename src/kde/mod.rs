pub mod multivariate;
pub mod univariate;

use crate::internal::Sealed;

/// Representation of a kernel density estimate with custom bandwith selector and kernel function.
#[derive(Clone, Debug)]
pub struct KernelDensityEstimator<T, B, K> {
    observations: T,
    bandwidth: B,
    kernel: K,
}

impl<T, B, K> Sealed for KernelDensityEstimator<T, B, K> {}
