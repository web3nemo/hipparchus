use std::ops::{Div, Mul};
use num::{Float, FromPrimitive, Zero, One};
use num::traits::{Inv, Pow};

/// Trait for floating point types.
pub trait Fp: Float + FromPrimitive + Zero + One + Inv<Output=Self>
{}

impl<T> Fp for T where T: Float + FromPrimitive + Zero + One + Inv<Output=Self>,
{}

/// Trait for floating point types with calculation with two
pub trait Two
{
    /// Returns two
    fn two() -> Self;

    /// Returns one half
    fn onehalf() -> Self;

    /// Returns twice self
    fn twice(self) -> Self;

    /// Returns half self
    fn half(self) -> Self;
}

impl<T> Two for T where T: FromPrimitive + Div<Output=T> + Mul<Output=T>
{
    fn two() -> Self
    {
        T::from_i32(2).unwrap()
    }

    fn onehalf() -> Self
    {
        T::from_i32(1).unwrap() / T::from_i32(2).unwrap()
    }
    
    fn twice(self) -> Self
    {
        self * Self::two()
    }
    
    fn half(self) -> Self
    {
        self / Self::two()
    }
}

/// Trait for square and cube calculation
pub trait Power
{
    fn sq(self) -> Self;
    fn cu(self) -> Self;
}

impl<T> Power for T where T: FromPrimitive + Pow<i32, Output=T>
{
    fn sq(self) -> Self
    {
        self.pow(2)
    }

    fn cu(self) -> Self
    {
        self.pow(3)
    }
}
