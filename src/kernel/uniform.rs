use crate::kernel::Kernel;

pub struct Uniform;

impl Kernel for Uniform {
    fn pdf(&self, x: f64) -> f64 {
        if x.abs() > 1. {
            return 0.;
        } else {
            return 0.5;
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
        let mut x: f64;
        let mut res: f64;
        let kernel = Uniform;

        x = 0.0;
        res = kernel.pdf(x);
        assert_relative_eq!(res, 0.5);

        x = 2.0;
        res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0);

        x = -2.0;
        res = kernel.pdf(x);
        assert_relative_eq!(res, 0.0);
    }
}
