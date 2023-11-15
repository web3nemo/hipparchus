use num::{Float, FromPrimitive, Zero, One};
use num::traits::Inv;

/// Trait for floating point types.
pub trait Fp: Float + FromPrimitive + Zero + One + Inv<Output=Self>
{}

impl<T> Fp for T where T: Float + FromPrimitive + Zero + One + Inv<Output=Self>,
{}
