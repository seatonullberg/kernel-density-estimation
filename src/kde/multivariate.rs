use crate::float::KDEFloat;
use crate::internal::Sealed;

pub type Matrix2D<F> = nalgebra::DMatrix<F>;

pub trait MultivariateKDE<B, K, F: KDEFloat>: Sealed {
    fn new<T: Into<Matrix2D<F>>>(observations: T, bandwidth: B, kernel: K) -> Self;
    fn pdf(&self, dataset: &Matrix2D<F>) -> Vec<F>;
    fn cdf(&self, dataset: &Matrix2D<F>) -> Vec<F>;
    fn sample(&self, dataset: &Matrix2D<F>, n_samples: usize) -> Matrix2D<F>;
}
