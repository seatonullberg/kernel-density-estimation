//! Cosine kernel function.

use crate::float::{float, primitive, KDEFloat};
use crate::kernel::Kernel;

use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

/// Cosine kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Cosine;

impl<F: KDEFloat> Kernel<F> for Cosine {
    fn pdf(&self, x: F) -> F {
        let abs_x = x.abs();
        if abs_x <= F::one() {
            let res = FRAC_PI_4 * (FRAC_PI_2 * primitive!(abs_x)).cos();
            float!(res)
        } else {
            F::zero()
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
