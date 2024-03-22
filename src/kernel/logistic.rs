//! Logistic kernel function.

use crate::float::{float, primitive, KDEFloat};
use crate::kernel::Kernel;

use std::f64::consts::E;

/// Logistic kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Logistic;

impl<F: KDEFloat> Kernel<F> for Logistic {
    fn pdf(&self, x: F) -> F {
        let exp_neg_x = E.powf(primitive!(-x));
        let exp_x = E.powf(primitive!(x));
        float!(1.0 / (exp_neg_x + 2.0 + exp_x))
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
