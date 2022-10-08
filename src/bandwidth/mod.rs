pub mod scott;
pub mod silverman;

pub trait Bandwidth {
    fn bandwidth(&self, data: &[f64]) -> f64;
}

impl<T> Bandwidth for T where T: Fn(&[f64]) -> f64 {
    fn bandwidth(&self, data: &[f64]) -> f64 {
        self(data)
    }
}
