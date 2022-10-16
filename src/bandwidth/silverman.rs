//! Silverman's rule for bandwidth selection.

use crate::bandwidth::Bandwidth;
use crate::internal::{interquartile_range, variance, Float};

/// Silverman's rule for bandwidth selection.
#[derive(Clone, Copy, Debug)]
pub struct Silverman;

impl Bandwidth for Silverman {
    fn bandwidth(&self, data: &[Float]) -> Float {
        let var = variance(data);
        let var_term = Float::sqrt(var);
        let iqr = interquartile_range(data);
        let iqr_term = iqr / 1.349;
        let n = data.len() as Float;
        let m: Float = if var_term < iqr_term {
            var_term
        } else {
            iqr_term
        };
        (0.9 * m) / n.powf(1.0 / 5.0)
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
