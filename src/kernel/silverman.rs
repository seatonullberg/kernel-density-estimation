//! Silverman kernel function.

use crate::internal::Float;
use crate::kernel::Kernel;

#[cfg(not(feature = "f64"))]
use std::f32::consts::{E, FRAC_PI_4, SQRT_2};
#[cfg(feature = "f64")]
use std::f64::consts::{E, FRAC_PI_4, SQRT_2};

/// Silverman kernel function.
#[derive(Clone, Copy, Debug)]
pub struct SilvermanKernel;

impl Kernel for SilvermanKernel {
    fn pdf(&self, x: Float) -> Float {
        let abs_x = x.abs();
        if abs_x <= 1.0 {
            0.5 * E.powf(- x.abs() / SQRT_2) * ((x.abs()/SQRT_2) + FRAC_PI_4).sin()
        } else {
            0.0
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