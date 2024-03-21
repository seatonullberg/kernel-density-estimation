//! Tricube kernel function.

use crate::internal::Float;
use crate::kernel::Kernel;

/// Tricube kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Tricube;

impl Kernel for Tricube {
    fn pdf(&self, x: Float) -> Float {
        let abs_x = x.abs();
        if abs_x <= 1.0 {
            (70.0 / 81.0) * (1.0 - abs_x.powi(3)).powi(3)
        } else {
            0.0
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