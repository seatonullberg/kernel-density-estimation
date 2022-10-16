pub mod scott;
pub mod silverman;

use crate::internal::Float;

pub trait Bandwidth {
    fn bandwidth(&self, data: &[Float]) -> Float;
}

impl<T> Bandwidth for T
where
    T: Fn(&[Float]) -> Float,
{
    fn bandwidth(&self, data: &[Float]) -> Float {
        self(data)
    }
}
