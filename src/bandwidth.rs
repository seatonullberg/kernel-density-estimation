use crate::internal::{interquartile_range, variance};

pub enum Bandwidth {
    Custom(Box<dyn Fn(&[f64]) -> f64>),
    Scott,
    Silverman,
}

impl Bandwidth {
    pub fn bandwidth(&self, data: &[f64]) -> f64 {
        match self {
            Bandwidth::Custom(func) => {
                func(data)
            },
            Bandwidth::Scott => {
                let prefactor = 1.06;
                let n = data.len() as f64;
                let var = variance(data);
                (prefactor * f64::sqrt(var)) / n.powf(1.0 / 5.0)
            },
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
    fn scott() {
        let data = vec![1.0, 1.5, 2.0, 2.5, 3.0];
        let bw = Bandwidth::Scott;
        let res = bw.bandwidth(data.as_slice());
        assert_relative_eq!(res, 0.60736, epsilon=1.0e-5);
    }

    #[test]
    fn silverman() {
        let data = vec![1.0, 1.5, 2.0, 2.5, 3.0];
        let bw = Bandwidth::Silverman;
        let res = bw.bandwidth(data.as_slice());
        assert_relative_eq!(res, 0.51568, epsilon = 1.0e-5);
    }
}
