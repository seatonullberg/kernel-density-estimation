//! Uniform kernel function.

use crate::float::{float, KDEFloat};
use crate::kernel::Kernel;

/// Uniform kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Uniform;

impl<F: KDEFloat> Kernel<F> for Uniform {
    fn pdf(&self, x: F) -> F {
        if x.abs() > F::one() {
            F::zero()
        } else {
            float!(0.5)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Uniform;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn uniform() {
        let kernel = Uniform;

        let x = 0.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.5);

        let x = 2.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0);

        let x = -2.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0);
    }
}
