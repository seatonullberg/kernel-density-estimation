//! Normal (Gaussian) kernel function.

use crate::float::{float, KDEFloat};
use crate::kernel::Kernel;

use std::f64::consts::PI;

/// Normal (Gaussian) kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Normal;

impl<F: KDEFloat> Kernel<F> for Normal {
    fn pdf(&self, x: F) -> F {
        let frac_sqrt2pi = F::one() / F::sqrt(float!(2.0) * float!(PI));
        let exponent = float!(-1.0 / 2.0) * x.powi(2);
        frac_sqrt2pi * exponent.exp()
    }
}

#[cfg(test)]
mod tests {
    use super::Normal;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn normal() {
        let kernel = Normal;

        let x = 0.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.39894, epsilon = 1.0e-5);

        let x = -1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.24197, epsilon = 1.0e-5);

        let x = 1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.24197, epsilon = 1.0e-5);
    }
}
