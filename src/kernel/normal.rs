use crate::kernel::Kernel;

use std::f64::consts::PI;

pub struct Normal;

impl Kernel for Normal {
    fn cdf(&self, x: f64) -> f64 {
        unimplemented!();
    }

    fn pdf(&self, x: f64) -> f64 {
        let frac_sqrt2pi = 1.0 / f64::sqrt(2.0 * PI);
        let exponent = (-1.0 / 2.0) * x.powi(2);
        frac_sqrt2pi * exponent.exp()
    }
}

#[cfg(test)]
mod tests {
    use super::Normal;
    use crate::kernel::Kernel;
    use approx::*;

    #[test]
    fn normal() {
        let mut x: f64;
        let mut res: f64;
        let kernel = Normal;

        x = 0.0;
        res = kernel.pdf(x);
        assert_relative_eq!(res, 0.39894, epsilon = 1.0e-5);

        x = -1.0;
        res = kernel.pdf(x);
        assert_relative_eq!(res, 0.24197, epsilon = 1.0e-5);

        x = 1.0;
        res = kernel.pdf(x);
        assert_relative_eq!(res, 0.24197, epsilon = 1.0e-5);
    }
}
