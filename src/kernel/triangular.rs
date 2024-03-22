//! Triangular kernel function.

use crate::float::KDEFloat;
use crate::kernel::Kernel;

/// Triangular kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Triangular;

impl<F: KDEFloat> Kernel<F> for Triangular {
    fn pdf(&self, x: F) -> F {
        let abs_x = x.abs();
        if abs_x <= F::one() {
            F::one() - abs_x
        } else {
            F::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Triangular;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn triangular() {
        let kernel = Triangular;

        let x = -1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0);

        let x = 0.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 1.0);

        let x = 1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0);
    }
}
