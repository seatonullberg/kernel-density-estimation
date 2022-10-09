pub mod epanechnikov;
pub mod normal;
pub mod uniform;

pub trait Kernel {
    fn pdf(&self, x: f64) -> f64;
}

impl<T> Kernel for T
where
    T: Fn(f64) -> f64,
{
    fn pdf(&self, x: f64) -> f64 {
        self(x)
    }
}
