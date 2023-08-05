//! Logistic kernel function.

use crate::internal::Float;
use crate::kernel::Kernel;

#[cfg(not(feature = "f64"))]
use std::f32::consts::E;
#[cfg(feature = "f64")]
use std::f64::consts::E;

/// Logistic kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Logistic;

impl Kernel for Logistic {
    fn pdf(&self, x: Float) -> Float {
        let exp_neg_x = E.powf(-x);
        let exp_x = E.powf(x);
        1.0 / (exp_neg_x + 2.0 + exp_x)
    }
}

#[cfg(test)]
mod tests {
    use super::Logistic;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn logistic() {
        let kernel = Logistic;

        let x = 0.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.25, epsilon = 1.0e-5);

        let x = -1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.19661, epsilon = 1.0e-5);

        let x = 1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.19661, epsilon = 1.0e-5);

        let x = 0.5;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.23500, epsilon = 1.0e-5);
    }
}
