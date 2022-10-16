pub mod bandwidth;
mod internal;
pub mod kernel;

use crate::bandwidth::Bandwidth;
use crate::internal::{cumsum, Float};
use crate::kernel::Kernel;

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

    pub fn cdf(&self, dataset: &[Float]) -> Vec<Float> {
        let pdf = self.pdf(dataset);
        let sum = cumsum(&pdf);
        let max = sum[sum.len() - 1];
        sum.iter().map(|x| x / max).collect()
    }

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
    pub use crate::KernelDensityEstimator;

    pub use crate::bandwidth::scott::Scott;
    pub use crate::bandwidth::silverman::Silverman;
    pub use crate::bandwidth::Bandwidth;

    pub use crate::kernel::epanechnikov::Epanechnikov;
    pub use crate::kernel::normal::Normal;
    pub use crate::kernel::uniform::Uniform;
    pub use crate::kernel::Kernel;
}
