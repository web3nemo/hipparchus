/// Trait for floating point types.
pub trait Fp: num::Float
    + num::FromPrimitive + num::Zero + num::One
    + std::ops::MulAssign + std::ops::DivAssign + std::ops::AddAssign + std::ops::SubAssign
    + num::traits::Inv<Output=Self>
    + float_cmp::ApproxEq
{}

impl<T> Fp for T where T: num::Float
    + num::FromPrimitive + num::Zero + num::One 
    + std::ops::MulAssign + std::ops::DivAssign + std::ops::AddAssign + std::ops::SubAssign
    + num::traits::MulAdd + num::traits::Inv<Output=Self>
    + float_cmp::ApproxEq
{}
