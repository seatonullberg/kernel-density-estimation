use crate::float::{float, primitive, KDEFloat};

pub trait Sealed {}

pub(crate) fn variance<F: KDEFloat>(data: &[F]) -> F {
    let n = float!(data.len());
    let mean = data.iter().copied().sum::<F>() / n;
    let squares_sum = data.iter().map(|&x| (x - mean).powi(2)).sum::<F>();
    squares_sum / (n - F::one())
}

pub(crate) fn quantile<F: KDEFloat>(data: &[F], tau: F) -> F {
    assert!((0.0..=1.0).contains(&primitive!(tau)));
    let mut data = data.to_owned();
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = float!(data.len());
    let index = F::round(tau * (n + F::one())).to_usize().unwrap();
    data[index - 1]
}

pub(crate) fn interquartile_range<F: KDEFloat>(data: &[F]) -> F {
    let upper = float!(0.75);
    let lower = float!(0.25);
    quantile(data, upper) - quantile(data, lower)
}

pub(crate) fn cumsum<F: KDEFloat>(data: &[F]) -> Vec<F> {
    (0..data.len())
        .map(|i| data[..i + 1].iter().copied().sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::{cumsum, interquartile_range, quantile, variance};

    #[test]
    fn test_variance() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res = variance(&data);
        assert_relative_eq!(res, 2.5);
    }

    #[test]
    fn test_quantile() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res = quantile(&data, 0.5);
        assert_relative_eq!(res, 3.0);
    }

    #[test]
    fn test_interquartile_range() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res = interquartile_range(&data);
        assert_relative_eq!(res, 3.0);
    }

    #[test]
    fn test_cumsum() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res = cumsum(&data);
        let target = vec![1.0, 3.0, 6.0, 10.0, 15.0];
        assert_eq!(res, target);
    }
}
