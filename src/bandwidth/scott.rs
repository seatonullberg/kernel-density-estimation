//! Scott's rule for bandwidth selection.

use crate::bandwidth::Bandwidth;
use crate::float::{float, KDEFloat};
use crate::internal::variance;

/// Scott's rule for bandwidth selection.
#[derive(Clone, Copy, Debug)]
pub struct Scott;

impl<F: KDEFloat> Bandwidth<F> for Scott {
    fn bandwidth(&self, data: &[F]) -> F {
        let prefactor = float!(1.06);
        let n = float!(data.len());
        let var = variance(data);
        let numerator = prefactor * var.sqrt();
        let denominator = float!(n.powf(float!(0.2)));
        numerator / denominator
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
