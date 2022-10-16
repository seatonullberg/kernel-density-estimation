//! Epanechnikov kernel function.

use crate::internal::Float;
use crate::kernel::Kernel;

/// Epanechnikov kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Epanechnikov;

impl Kernel for Epanechnikov {
    fn pdf(&self, x: Float) -> Float {
        let term = 1.0 - x.powi(2);
        if term > 0.0 {
            (3.0 / 4.0) * term
        } else {
            0.0
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
