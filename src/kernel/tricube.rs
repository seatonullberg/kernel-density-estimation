//! Tricube kernel function.

use crate::float::{float, KDEFloat};
use crate::kernel::Kernel;

/// Tricube kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Tricube;

impl<F: KDEFloat> Kernel<F> for Tricube {
    fn pdf(&self, x: F) -> F {
        let abs_x = x.abs();
        if abs_x <= F::one() {
            float!(70.0 / 81.0) * (F::one() - abs_x.powi(3)).powi(3)
        } else {
            F::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tricube;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn tricube() {
        let kernel = Tricube;

        let x = 0.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.86419, epsilon = 1.0e-5);

        let x = -1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0, epsilon = 1.0e-5);

        let x = 1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0, epsilon = 1.0e-5);

        let x = 0.5;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.57895, epsilon = 1.0e-5);
    }
}
