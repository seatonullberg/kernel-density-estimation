//! Quartic (biweight) kernel function.

use crate::float::{float, KDEFloat};
use crate::kernel::Kernel;

/// Quartic (biweight) kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Quartic;

impl<F: KDEFloat> Kernel<F> for Quartic {
    fn pdf(&self, x: F) -> F {
        let abs_x = x.abs();
        if abs_x <= F::one() {
            float!(15.0 / 16.0) * (F::one() - x.powi(2)).powi(2)
        } else {
            F::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Quartic;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn quartic() {
        let kernel = Quartic;

        let x = 0.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.93750, epsilon = 1.0e-5);

        let x = -1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0, epsilon = 1.0e-5);

        let x = 1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0, epsilon = 1.0e-5);

        let x = 0.5;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.52734, epsilon = 1.0e-5);
    }
}
