//! Uniform kernel function.

use crate::internal::Float;
use crate::kernel::Kernel;

/// Uniform kernel function.
pub struct Uniform;

impl Kernel for Uniform {
    fn pdf(&self, x: Float) -> Float {
        if x.abs() > 1. {
            0.0
        } else {
            0.5
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
