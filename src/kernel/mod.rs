pub mod epanechnikov;
pub mod normal;
pub mod uniform;

use crate::internal::Float;

pub trait Kernel {
    fn pdf(&self, x: Float) -> Float;
}

impl<T> Kernel for T
where
    T: Fn(Float) -> Float,
{
    fn pdf(&self, x: Float) -> Float {
        self(x)
    }
}
