pub mod epanechnikov;
pub mod normal;

pub trait Kernel {
    fn cdf(&self, x: f64) -> f64;
    fn pdf(&self, x: f64) -> f64;
}
