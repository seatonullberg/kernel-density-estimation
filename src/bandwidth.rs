use crate::internal::{interquartile_range, variance};

pub enum Bandwidth {
    Custom(f64),
    Silverman,
}

impl Bandwidth {
    pub fn bandwidth(&self, data: &[f64]) -> f64 {
        match self {
            Bandwidth::Custom(h) => *h,
            Bandwidth::Silverman => {
                let var = variance(data);
                let var_term = f64::sqrt(var);
                let iqr = interquartile_range(data);
                let iqr_term = iqr / 1.349;
                let n = data.len() as f64;
                let m: f64;
                if var_term < iqr_term {
                    m = var_term
                } else {
                    m = iqr_term
                }
                (0.9 * m) / n.powf(1.0 / 5.0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Bandwidth;
    use approx::*;

    #[test]
    fn silverman() {
        let mut data = vec![1.0, 1.5, 2.0, 2.5, 3.0];
        let bw = Bandwidth::Silverman;
        let res = bw.bandwidth(&mut data);
        assert_relative_eq!(res, 0.51568, epsilon = 1.0e-5);
    }
}
