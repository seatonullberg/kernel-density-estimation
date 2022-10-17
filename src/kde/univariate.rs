use crate::bandwidth::Bandwidth;
use crate::internal::{cumsum, Float, Sealed};
use crate::kde::KernelDensityEstimator;
use crate::kernel::Kernel;

pub trait UnivariateKDE<B, K>: Sealed {
    fn new<T: Into<Vec<Float>>>(observations: T, bandwidth: B, kernel: K) -> Self;
    fn pdf(&self, dataset: &[Float]) -> Vec<Float>;
    fn cdf(&self, dataset: &[Float]) -> Vec<Float>;
    fn sample(&self, dataset: &[Float], n_samples: usize) -> Vec<Float>;
}

impl<B, K> UnivariateKDE<B, K> for KernelDensityEstimator<Vec<Float>, B, K>
where
    B: Bandwidth,
    K: Kernel,
{
    /// Returns an initialized `KernelDensityEstimator`.
    fn new<T: Into<Vec<Float>>>(observations: T, bandwidth: B, kernel: K) -> Self {
        let observations: Vec<Float> = observations.into();
        KernelDensityEstimator {
            observations,
            bandwidth,
            kernel,
        }
    }

    /// Returns the estimated probability density function evaluated at the points in `dataset`.
    fn pdf(&self, dataset: &[Float]) -> Vec<Float> {
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
    fn cdf(&self, dataset: &[Float]) -> Vec<Float> {
        let pdf = self.pdf(dataset);
        let sum = cumsum(&pdf);
        let max = sum[sum.len() - 1];
        sum.iter().map(|x| x / max).collect()
    }

    /// Generates samples from the estimated probability density function.
    fn sample(&self, dataset: &[Float], n_samples: usize) -> Vec<Float> {
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
