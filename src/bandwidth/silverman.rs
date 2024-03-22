//! Silverman's rule for bandwidth selection.

use crate::bandwidth::Bandwidth;
use crate::float::{float, KDEFloat};
use crate::internal::{interquartile_range, variance};

/// Silverman's rule for bandwidth selection.
#[derive(Clone, Copy, Debug)]
pub struct Silverman;

impl<F: KDEFloat> Bandwidth<F> for Silverman {
    fn bandwidth(&self, data: &[F]) -> F {
        let var = variance(data);
        let var_term = var.sqrt();
        let iqr = interquartile_range(data);
        let iqr_term = iqr / float!(1.349);
        let n = float!(data.len());
        let m = if var_term < iqr_term {
            var_term
        } else {
            iqr_term
        };
        let numerator = float!(0.9) * m;
        let denominator = n.powf(float!(0.2));
        numerator / denominator
    }
}

#[cfg(test)]
mod tests {
    use super::{Bandwidth, Silverman};
    use approx::*;

    #[test]
    fn silverman() {
        let data = vec![1.0, 1.5, 2.0, 2.5, 3.0];
        let res = Silverman.bandwidth(&data);
        assert_relative_eq!(res, 0.51568, epsilon = 1.0e-5);
    }
}
