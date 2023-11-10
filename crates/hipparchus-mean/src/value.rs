use std::ops::Div;
use num::{Float, FromPrimitive, Zero, One};
use num::traits::{Inv, Pow};

/// Trait for floating point types.
pub trait Fp: Float + FromPrimitive + Zero + One + Inv<Output=Self>
{}

impl<T> Fp for T where T: Float + FromPrimitive + Zero + One + Inv<Output=Self>,
{}

pub trait Two
{
    fn two() -> Self;
    fn half(self) -> Self;
}

impl<T> Two for T where T: FromPrimitive + Div<Output=T>
{
    fn two() -> Self
    {
        T::from_i32(2).unwrap()
    }
    
    fn half(self) -> Self
    {
        self / Self::two()
    }
}

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
