//! Sigmoid kernel function.

use crate::internal::Float;
use crate::kernel::Kernel;

#[cfg(not(feature = "f64"))]
use std::f32::consts::{E, FRAC_2_PI};
#[cfg(feature = "f64")]
use std::f64::consts::{E, FRAC_2_PI};

/// Sigmoid kernel function.
#[derive(Clone, Copy, Debug)]
pub struct Sigmoid;

impl Kernel for Sigmoid {
    fn pdf(&self, x: Float) -> Float {
        FRAC_2_PI / (E.powf(x) + E.powf(-x))
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
