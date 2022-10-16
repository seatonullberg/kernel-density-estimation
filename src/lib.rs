//! Kernel density estimation in Rust.
//!
//! Kernel density estimation (KDE) is a non-parametric method to estimate the probability
//! density function of a random variable by taking the summation of kernel functions centered
//! on each data point. This crate serves three major purposes based on this idea:
//! 1) Evaluate the probability density function of a random variable.
//! 2) Evaluate the cumulative distribution function of a random variable.
//! 3) Sample data points from the probability density function.
//!
//! An excellent technical description of the method is available
//! [here](https://bookdown.org/egarpor/NP-UC3M/kde-i.html)[^citation].
//!
//! [^citation]: García-Portugués, E. (2022). Notes for Nonparametric Statistics.
//! Version 6.5.9. ISBN 978-84-09-29537-1.

#[warn(missing_docs)]
pub mod bandwidth;
mod internal;
pub mod kernel;

use crate::bandwidth::Bandwidth;
use crate::internal::{cumsum, Float};
use crate::kernel::Kernel;

/// Representation of a kernel density estimate with custom bandwith selector and kernel function.
#[derive(Clone, Debug)]
pub struct KernelDensityEstimator<B, K> {
    observations: Vec<Float>,
    bandwidth: B,
    kernel: K,
}

impl<B, K> KernelDensityEstimator<B, K>
where
    B: Bandwidth,
    K: Kernel,
{
    /// Returns an initialized `KernelDensityEstimator`.
    pub fn new<T>(observations: T, bandwidth: B, kernel: K) -> Self
    where
        T: Into<Vec<Float>>,
    {
        let observations: Vec<Float> = observations.into();
        KernelDensityEstimator {
            observations,
            bandwidth,
            kernel,
        }
    }

    /// Returns the estimated probability density function evaluated at the points in `dataset`.
    pub fn pdf(&self, dataset: &[Float]) -> Vec<Float> {
        let n = self.observations.len() as Float;
        let h = self.bandwidth.bandwidth(&self.observations);
        let prefactor = 1. / (n * h);
        self.observations
            .iter()
            .fold(vec![0.0; dataset.len()], |mut acc, x| {
                dataset.iter().enumerate().for_each(|(i, xi)| {
                    let kernel_arg = (x - xi) / h;
                    acc[i] += prefactor * self.kernel.pdf(kernel_arg);
                });
                acc
            })
    }

    /// Returns the estimated cumulative distribution function evaluated at the points in `dataset`.
    pub fn cdf(&self, dataset: &[Float]) -> Vec<Float> {
        let pdf = self.pdf(dataset);
        let sum = cumsum(&pdf);
        let max = sum[sum.len() - 1];
        sum.iter().map(|x| x / max).collect()
    }

    /// Generates samples from the estimated probability density function.
    pub fn sample(&self, dataset: &[Float], n_samples: usize) -> Vec<Float> {
        let rng = fastrand::Rng::new();
        let cdf = self.cdf(dataset);
        (0..n_samples)
            .into_iter()
            .map(|_| {
                #[cfg(feature = "f64")]
                let rand = rng.f64();
                #[cfg(not(feature = "f64"))]
                let rand = rng.f32();
                let mut lower_index = 0;
                let mut upper_index = 0;
                for (j, elem) in cdf.iter().enumerate() {
                    if elem < &rand {
                        lower_index = j;
                    } else {
                        upper_index = j;
                        break;
                    }
                }
                let xmin = dataset[lower_index];
                let xmax = dataset[upper_index];
                let xrange = xmax - xmin;
                let ymin = cdf[lower_index];
                let ymax = cdf[upper_index];
                let yrange = ymax - ymin;
                let yrange_rand = rand - ymin;
                let yratio = yrange_rand / yrange;
                (xrange * yratio) + xmin
            })
            .collect()
    }
}

pub mod prelude {
    //! `use kernel_density_estimation::prelude::*;` to import all common functionality.

    pub use crate::KernelDensityEstimator;

    pub use crate::bandwidth::scott::Scott;
    pub use crate::bandwidth::silverman::Silverman;
    pub use crate::bandwidth::Bandwidth;

    pub use crate::kernel::epanechnikov::Epanechnikov;
    pub use crate::kernel::normal::Normal;
    pub use crate::kernel::uniform::Uniform;
    pub use crate::kernel::Kernel;
}
