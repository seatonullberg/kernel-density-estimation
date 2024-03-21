#[cfg(feature = "f64")]
pub(crate) type Float = f64;

#[cfg(not(feature = "f64"))]
pub(crate) type Float = f32;

pub trait Sealed {}

pub(crate) fn variance(data: &[Float]) -> Float {
    let n = data.len() as Float;
    let mean = data.iter().sum::<Float>() / n;
    let squares_sum = data.iter().map(|x| (x - mean).powi(2)).sum::<Float>();
    squares_sum / (n - 1.0)
}

pub(crate) fn quantile(data: &[Float], tau: Float) -> Float {
    assert!((0.0..=1.0).contains(&tau));
    let mut data = data.to_owned();
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = data.len() as Float;
    let index = Float::round(tau * (n + 1.0)) as usize;
    data[index - 1]
}

pub(crate) fn interquartile_range(data: &[Float]) -> Float {
    quantile(data, 0.75) - quantile(data, 0.25)
}

pub(crate) fn cumsum(data: &[Float]) -> Vec<Float> {
    (0..data.len())
        .map(|i| data[..i + 1].iter().sum())
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
