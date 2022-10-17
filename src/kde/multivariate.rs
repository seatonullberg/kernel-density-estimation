use crate::internal::{Float, Sealed};

pub type Matrix2D = nalgebra::DMatrix<Float>;

pub trait MultivariateKDE<B, K>: Sealed {
    fn new<T: Into<Matrix2D>>(observations: T, bandwidth: B, kernel: K) -> Self;
    fn pdf(&self, dataset: &Matrix2D) -> Vec<Float>;
    fn cdf(&self, dataset: &Matrix2D) -> Vec<Float>;
    fn sample(&self, dataset: &Matrix2D, n_samples: usize) -> Matrix2D;
}
