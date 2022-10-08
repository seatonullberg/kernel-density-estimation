pub mod bandwidth;
mod internal;
pub mod kernel;

use bandwidth::Bandwidth;
use kernel::Kernel;

pub struct KernelDensityEstimator {
    samples: Vec<f64>,
}

impl KernelDensityEstimator {
    pub fn new(samples: Vec<f64>) -> Self {
        KernelDensityEstimator { samples }
    }

    pub fn pdf<B, K>(&self, dataset: Vec<f64>, bandwidth: B, kernel: K) -> Vec<f64>
    where
        B: Bandwidth,
        K: Kernel,
    {
        let n = self.samples.len() as f64;
        let h = bandwidth.bandwidth(self.samples.as_slice());
        let prefactor = 1. / (n * h);
        let mut kernel_arg: f64;
        let mut res: Vec<f64> = vec![0.0; dataset.len()];
        for x in self.samples.iter() {
            for (i, xi) in dataset.iter().enumerate() {
                kernel_arg = (x - xi) / h;
                res[i] += kernel.pdf(kernel_arg);
            }
        }
        res.iter().map(|x| x * prefactor).collect()
    }
}

pub mod prelude {
    pub use crate::KernelDensityEstimator;

    pub use crate::bandwidth::scott::Scott;
    pub use crate::bandwidth::silverman::Silverman;
    pub use crate::bandwidth::Bandwidth;

    pub use crate::kernel::epanechnikov::Epanechnikov;
    pub use crate::kernel::normal::Normal;
    pub use crate::kernel::Kernel;
}
