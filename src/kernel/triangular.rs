//! Triangular kernel function.

use crate::internal::Float;
use crate::kernel::Kernel;

/// Triangular kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Triangular;

impl Kernel for Triangular {
    fn pdf(&self, x: Float) -> Float {
        let abs_x = x.abs();
        if abs_x <= 1.0 {
            1.0 - abs_x
        } else {
            0.0
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
