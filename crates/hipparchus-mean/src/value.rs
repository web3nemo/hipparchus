use num::{Float, FromPrimitive, Zero, One};
use num::traits::Inv;

pub trait Fp: Float + FromPrimitive + Zero + One + Inv
{}

impl<T> Fp for T where T: Float + FromPrimitive + Zero + One + Inv,
{}

