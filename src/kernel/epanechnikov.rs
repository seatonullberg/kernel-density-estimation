//! Epanechnikov kernel function.

use crate::float::{float, KDEFloat};
use crate::kernel::Kernel;

/// Epanechnikov kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Epanechnikov;

impl<F: KDEFloat> Kernel<F> for Epanechnikov {
    fn pdf(&self, x: F) -> F {
        let term = F::one() - x.powi(2);
        if term > F::zero() {
            float!(3.0 / 4.0) * term
        } else {
            F::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Epanechnikov;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn epanechnikov() {
        let kernel = Epanechnikov;

        let x = 0.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.75);

        let x = -1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0);

        let x = 1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0);
    }
}
