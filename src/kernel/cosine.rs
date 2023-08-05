///! Cosine kernel function.

use crate::internal::Float;
use crate::kernel::Kernel;

#[cfg(not(feature = "f64"))]
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
#[cfg(feature = "f64")]
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

/// Cosine kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Cosine;

impl Kernel for Cosine {
    fn pdf(&self, x: Float) -> Float {
        let abs_x = x.abs();
        if abs_x <= 1.0 {
            FRAC_PI_4 * (FRAC_PI_2 * abs_x).cos()
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cosine;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn cosine() {
        let kernel = Cosine;

        let x = 0.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.78539, epsilon = 1.0e-5);

        let x = -1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0, epsilon = 1.0e-5);

        let x = 1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0, epsilon = 1.0e-5);

        let x = 0.5;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.55536, epsilon = 1.0e-5);
    }
}
