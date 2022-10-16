use crate::bandwidth::Bandwidth;
use crate::internal::{variance, Float};

pub struct Scott;

impl Bandwidth for Scott {
    fn bandwidth(&self, data: &[Float]) -> Float {
        let prefactor = 1.06;
        let n = data.len() as Float;
        let var = variance(data);
        (prefactor * Float::sqrt(var)) / n.powf(1. / 5.)
    }
}

#[cfg(test)]
mod tests {
    use super::{Bandwidth, Scott};
    use approx::*;

    #[test]
    fn scott() {
        let data = vec![1.0, 1.5, 2.0, 2.5, 3.0];
        let res = Scott.bandwidth(&data);
        assert_relative_eq!(res, 0.60736, epsilon = 1.0e-5);
    }
}
