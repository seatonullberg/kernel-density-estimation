//! Silverman kernel function.

use crate::float::{float, primitive, KDEFloat};
use crate::kernel::Kernel;

use std::f64::consts::{E, FRAC_PI_4, SQRT_2};

/// Silverman kernel function.
#[derive(Clone, Copy, Debug)]
pub struct SilvermanKernel;

impl<F: KDEFloat> Kernel<F> for SilvermanKernel {
    fn pdf(&self, x: F) -> F {
        let abs_x = x.abs();
        if abs_x <= F::one() {
            let term_a = 0.5 * E.powf(-primitive!(x.abs()) / SQRT_2);
            let term_b = ((primitive!(x.abs()) / SQRT_2) + FRAC_PI_4).sin();
            float!(term_a * term_b)
        } else {
            F::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SilvermanKernel;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn silverman_kernel() {
        let kernel = SilvermanKernel;

        let x = 0.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.35355, epsilon = 1.0e-5);

        let x = -1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.24578, epsilon = 1.0e-5);

        let x = 1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.24578, epsilon = 1.0e-5);

        let x = 0.5;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.31886, epsilon = 1.0e-5);
    }
}
