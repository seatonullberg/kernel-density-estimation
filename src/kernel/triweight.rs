//! Triweight kernel function.

use crate::internal::Float;
use crate::kernel::Kernel;

/// Triweight kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Triweight;

impl Kernel for Triweight {
    fn pdf(&self, x: Float) -> Float {
        let abs_x = x.abs();
        if abs_x <= 1.0 {
            (35.0 / 32.0) * (1.0 - x.powi(2)).powi(3)
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Triweight;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn triweight() {
        let kernel = Triweight;

        let x = 0.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 1.09375, epsilon = 1.0e-5);

        let x = -1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0, epsilon = 1.0e-5);

        let x = 1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0, epsilon = 1.0e-5);

        let x = 0.5;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.46143, epsilon = 1.0e-5);
    }
}
