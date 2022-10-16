//! Normal (Gaussian) kernel function.

use crate::internal::Float;
use crate::kernel::Kernel;

#[cfg(not(feature = "f64"))]
use std::f32::consts::PI;
#[cfg(feature = "f64")]
use std::f64::consts::PI;

/// Normal (Gaussian) kernel function.
pub struct Normal;

impl Kernel for Normal {
    fn pdf(&self, x: Float) -> Float {
        let frac_sqrt2pi = 1.0 / Float::sqrt(2.0 * PI);
        let exponent = (-1.0 / 2.0) * x.powi(2);
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
