//! Sigmoid kernel function.

use crate::float::{float, primitive, KDEFloat};
use crate::kernel::Kernel;

use std::f64::consts::{E, FRAC_2_PI};

/// Sigmoid kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Sigmoid;

impl<F: KDEFloat> Kernel<F> for Sigmoid {
    fn pdf(&self, x: F) -> F {
        let numerator = float!(FRAC_2_PI);
        let denominator = float!(E.powf(primitive!(x)) + E.powf(primitive!(-x)));
        numerator / denominator
    }
}

#[cfg(test)]
mod tests {
    use super::Sigmoid;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn sigmoid() {
        let kernel = Sigmoid;

        let x = 0.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.31831, epsilon = 1.0e-5);

        let x = -1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.20628, epsilon = 1.0e-5);

        let x = 1.0;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.20628, epsilon = 1.0e-5);

        let x = 0.5;
        let res = kernel.pdf(x);
        assert_relative_eq!(res, 0.28228, epsilon = 1.0e-5);
    }
}
