//! Float constraints for generic math.

use num_traits;

// Custom Float implementation based on argmin crate:
// https://docs.rs/argmin/latest/argmin/core/trait.ArgminFloat.html

/// An alias for float types (f32, f64) which
/// combines many commonly required traits.
/// It is automatically implemented for all 
/// types which fulfill the trait bounds.
pub trait KDEFloat:
    'static
    + num_traits::Float
    + num_traits::FloatConst
    + num_traits::FromPrimitive
    + num_traits::ToPrimitive
    + std::fmt::Debug
    + std::fmt::Display
    + std::iter::Sum
    + std::ops::AddAssign
{
}

/// `KDEFloat` is automatically implemented for all 
/// types which fulfill the trait bounds.
impl<T> KDEFloat for T where
    T: 'static
        + num_traits::Float
        + num_traits::FloatConst
        + num_traits::FromPrimitive
        + num_traits::ToPrimitive
        + std::fmt::Debug
        + std::fmt::Display
        + std::iter::Sum
        + std::ops::AddAssign
{
}

// Macro to simplify primitive to Float conversion.
// Note that the name of the generic parameter MUST be F.
macro_rules! float {
    ($x:expr) => {
        F::from($x).unwrap()
    };
}
pub(crate) use float;

// Macro to simplify Float to f64 conversion.
// Note that the argument MUST implement Float.
macro_rules! primitive {
    ($x:expr) => {
        $x.to_f64().unwrap()
    };
}
pub(crate) use primitive;
