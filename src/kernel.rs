use std::f64::consts::PI;

pub enum Kernel {
    Normal,
    Epanechnikov,
}

impl Kernel {
    pub fn eval(&self, x: f64) -> f64 {
        match self {
            Kernel::Normal => {
                let frac_sqrt2pi = 1.0 / f64::sqrt(2.0 * PI);
                let exponent = (-1.0 / 2.0) * x.powi(2);
                frac_sqrt2pi * exponent.exp()
            }
            Kernel::Epanechnikov => {
                let term = 1.0 - x.powi(2);
                if term > 0.0 {
                    return (3.0 / 4.0) * term;
                } else {
                    return 0.0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Kernel;
    use approx::*;

    #[test]
    fn normal() {
        let mut x: f64;
        let mut res: f64;
        let kernel = Kernel::Normal;

        x = 0.0;
        res = kernel.eval(x);
        assert_relative_eq!(res, 0.39894, epsilon = 1.0e-5);

        x = -1.0;
        res = kernel.eval(x);
        assert_relative_eq!(res, 0.24197, epsilon = 1.0e-5);

        x = 1.0;
        res = kernel.eval(x);
        assert_relative_eq!(res, 0.24197, epsilon = 1.0e-5);
    }

    #[test]
    fn epanechnikov() {
        let mut x: f64;
        let mut res: f64;
        let kernel = Kernel::Epanechnikov;

        x = 0.0;
        res = kernel.eval(x);
        assert_relative_eq!(res, 0.75);

        x = -1.0;
        res = kernel.eval(x);
        assert_relative_eq!(res, 0.0);

        x = 1.0;
        res = kernel.eval(x);
        assert_relative_eq!(res, 0.0);
    }
}
