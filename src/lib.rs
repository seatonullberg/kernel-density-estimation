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
pub mod kde;
pub mod kernel;

pub mod prelude {
    //! `use kernel_density_estimation::prelude::*;` to import all common functionality.

    pub use crate::bandwidth::scott::Scott;
    pub use crate::bandwidth::silverman::Silverman;
    pub use crate::bandwidth::Bandwidth;

    pub use crate::kde::univariate::UnivariateKDE;
    pub use crate::kde::KernelDensityEstimator;

    pub use crate::kernel::epanechnikov::Epanechnikov;
    pub use crate::kernel::normal::Normal;
    pub use crate::kernel::uniform::Uniform;
    pub use crate::kernel::triangular::Triangular;
    pub use crate::kernel::quartic::Quartic;
    pub use crate::kernel::triweight::Triweight;
    pub use crate::kernel::tricube::Tricube;
    pub use crate::kernel::cosine::Cosine;
    pub use crate::kernel::logistic::Logistic;
    pub use crate::kernel::sigmoid::Sigmoid;
    pub use crate::kernel::Kernel;
}
