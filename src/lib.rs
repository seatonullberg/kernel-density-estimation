pub mod bandwidth;
mod internal;
pub mod kernel;

use bandwidth::Bandwidth;
use internal::cumsum;
use kernel::Kernel;

use fastrand::Rng;

pub struct KernelDensityEstimator<B, K> {
    observations: Vec<f64>,
    bandwidth: B,
    kernel: K,
    rng: Rng,
}

impl<B, K> KernelDensityEstimator<B, K>
where
    B: Bandwidth,
    K: Kernel,
{
    pub fn new(observations: Vec<f64>, bandwidth: B, kernel: K) -> Self {
        let rng = Rng::new();
        KernelDensityEstimator {
            observations,
            bandwidth,
            kernel,
            rng,
        }
    }

    pub fn pdf(&self, dataset: &[f64]) -> Vec<f64> {
        let n = self.observations.len() as f64;
        let h = self.bandwidth.bandwidth(self.observations.as_slice());
        let prefactor = 1. / (n * h);
        let mut kernel_arg: f64;
        let mut res: Vec<f64> = vec![0.0; dataset.len()];
        for x in self.observations.iter() {
            for (i, xi) in dataset.iter().enumerate() {
                kernel_arg = (x - xi) / h;
                res[i] += prefactor * self.kernel.pdf(kernel_arg);
            }
        }
        res
    }

    pub fn cdf(&self, dataset: &[f64]) -> Vec<f64> {
        let pdf = self.pdf(dataset);
        let sum = cumsum(pdf.as_slice());
        let max = sum[sum.len() - 1];
        sum.iter().map(|x| x / max).collect()
    }

    pub fn sample(&self, dataset: &[f64], n_samples: usize) -> Vec<f64> {
        let mut res: Vec<f64> = vec![0.0; n_samples];
        let cdf = self.cdf(dataset);
        // Declare loop variables.
        let mut rand: f64;
        let mut lower_index: usize;
        let mut upper_index: usize;
        let mut xrange: f64;
        let mut xmin: f64;
        let mut xmax: f64;
        let mut yrange: f64;
        let mut ymin: f64;
        let mut ymax: f64;
        let mut yrange_rand: f64;
        let mut yratio: f64;
        for i in 0..n_samples {
            rand = self.rng.f64();
            lower_index = 0;
            upper_index = 0;
            for j in 0..cdf.len() {
                if cdf[j] < rand {
                    lower_index = j;
                } else {
                    upper_index = j;
                    break;
                }
            }
            xmin = dataset[lower_index];
            xmax = dataset[upper_index];
            xrange = xmax - xmin;
            ymin = cdf[lower_index];
            ymax = cdf[upper_index];
            yrange = ymax - ymin;
            yrange_rand = rand - ymin;
            yratio = yrange_rand / yrange;
            res[i] = (xrange * yratio) + xmin;
        }
        res
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
