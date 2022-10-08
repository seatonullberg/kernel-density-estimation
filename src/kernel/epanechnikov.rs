use crate::kernel::Kernel;

pub struct Epanechnikov;

impl Kernel for Epanechnikov {
    fn pdf(&self, x: f64) -> f64 {
        let term = 1.0 - x.powi(2);
        if term > 0.0 {
            return (3.0 / 4.0) * term;
        } else {
            return 0.0;
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
        let mut x: f64;
        let mut res: f64;
        let kernel = Epanechnikov;

        x = 0.0;
        res = kernel.pdf(x);
        assert_relative_eq!(res, 0.75);

        x = -1.0;
        res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0);

        x = 1.0;
        res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0);
    }
}
