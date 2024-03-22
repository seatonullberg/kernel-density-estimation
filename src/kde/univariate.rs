use crate::bandwidth::Bandwidth;
use crate::float::{float, KDEFloat};
use crate::internal::{cumsum, Sealed};
use crate::kde::KernelDensityEstimator;
use crate::kernel::Kernel;

pub trait UnivariateKDE<B, K, F: KDEFloat>: Sealed {
    fn new<T: Into<Vec<F>>>(observations: T, bandwidth: B, kernel: K) -> Self;
    fn pdf(&self, dataset: &[F]) -> Vec<F>;
    fn cdf(&self, dataset: &[F]) -> Vec<F>;
    fn sample(&self, dataset: &[F], n_samples: usize) -> Vec<F>;
}

impl<B, K, F: KDEFloat> UnivariateKDE<B, K, F> for KernelDensityEstimator<Vec<F>, B, K>
where
    B: Bandwidth<F>,
    K: Kernel<F>,
{
    /// Returns an initialized `KernelDensityEstimator`.
    fn new<T: Into<Vec<F>>>(observations: T, bandwidth: B, kernel: K) -> Self {
        let observations: Vec<F> = observations.into();
        KernelDensityEstimator {
            observations,
            bandwidth,
            kernel,
        }
    }

    /// Returns the estimated probability density function evaluated at the points in `dataset`.
    fn pdf(&self, dataset: &[F]) -> Vec<F> {
        let n = float!(self.observations.len());
        let h = self.bandwidth.bandwidth(&self.observations);
        let prefactor = F::one() / (n * h);
        self.observations
            .iter()
            .fold(vec![F::zero(); dataset.len()], |mut acc, &x| {
                dataset.iter().enumerate().for_each(|(i, &xi)| {
                    let kernel_arg = (x - xi) / h;
                    acc[i] += prefactor * self.kernel.pdf(kernel_arg);
                });
                acc
            })
    }

    /// Returns the estimated cumulative distribution function evaluated at the points in `dataset`.
    fn cdf(&self, dataset: &[F]) -> Vec<F> {
        let pdf = self.pdf(dataset);
        let sum = cumsum(&pdf);
        let max = sum[sum.len() - 1];
        sum.iter().map(|&x| x / max).collect()
    }

    /// Generates samples from the estimated probability density function.
    fn sample(&self, dataset: &[F], n_samples: usize) -> Vec<F> {
        let rng = fastrand::Rng::new();
        let cdf = self.cdf(dataset);
        (0..n_samples)
            .map(|_| {
                let rand = float!(rng.f64());
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
