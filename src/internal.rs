pub(crate) fn variance(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let squares_sum = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>();
    squares_sum / (n - 1.0)
}

pub(crate) fn quantile(data: &[f64], tau: f64) -> f64 {
    assert!(0.0 <= tau && tau <= 1.0);
    let mut data: Vec<f64> = data.clone().to_owned();
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = data.len() as f64;
    let index = f64::round(tau * (n + 1.0)) as usize;
    data[index - 1]
}

pub(crate) fn interquartile_range(data: &[f64]) -> f64 {
    quantile(data, 0.75) - quantile(data, 0.25)
}

pub(crate) fn cumsum(data: &[f64]) -> Vec<f64> {
    let mut res: Vec<f64> = vec![0.0; data.len()];
    let mut sum: f64;
    for i in 0..data.len() {
        sum = data[..i+1].iter().sum();
        res[i] = sum;
    }
    res
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::{interquartile_range, quantile, variance, cumsum};

    #[test]
    fn test_variance() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res = variance(data.as_slice());
        assert_relative_eq!(res, 2.5);
    }

    #[test]
    fn test_quantile() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res = quantile(data.as_slice(), 0.5);
        assert_relative_eq!(res, 3.0);
    }

    #[test]
    fn test_interquartile_range() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res = interquartile_range(data.as_slice());
        assert_relative_eq!(res, 3.0);
    }

    #[test]
    fn test_cumsum() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let res = cumsum(data.as_slice());
        assert_eq!(res[0], 1.0);
        assert_eq!(res[1], 3.0);
        assert_eq!(res[2], 6.0);
        assert_eq!(res[3], 10.0);
        assert_eq!(res[4], 15.0);
    }
}
