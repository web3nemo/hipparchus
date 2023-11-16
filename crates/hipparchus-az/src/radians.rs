use crate::modulo::{Modulo, Remainder};

/// Trait for angle in radians
pub trait Radians
{
    /// The default modulo value for radians
    const MOD_RADIANS: Self;

    /// To normalize an angle in radians with the given remainder algorithm
    fn norm_radians(&self, re: Remainder) -> Self;
}

#[macro_export]
macro_rules! impl_radians
{
    ($ty:ty, $modr:expr) =>
    {
        impl Radians for $ty
        {
            const MOD_RADIANS: Self = $modr;

            fn norm_radians(&self, re: Remainder) -> Self
            {
                match re
                {
                    Remainder::Euclidean => self.umod($modr),
                    Remainder::Symmetry => self.smod($modr),
                    Remainder::InvertedSymmetry => self.smod(-$modr),
                }
            }
        }
    }
}

impl_radians!(f64, std::f64::consts::TAU);
impl_radians!(f32, std::f32::consts::TAU);

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use core::fmt::Debug;
    use float_cmp::{ assert_approx_eq, ApproxEq };

    #[rstest]
    #[case(-std::f64::consts::PI, Remainder::Euclidean, std::f64::consts::PI)]
    #[case(-std::f64::consts::PI, Remainder::Symmetry, -std::f64::consts::PI)]
    #[case(-std::f64::consts::PI, Remainder::InvertedSymmetry, std::f64::consts::PI)]
    #[case(-std::f32::consts::PI, Remainder::Euclidean, std::f32::consts::PI)]
    #[case(-std::f32::consts::PI, Remainder::Symmetry, -std::f32::consts::PI)]
    #[case(-std::f32::consts::PI, Remainder::InvertedSymmetry, std::f32::consts::PI)]
    fn test_radians_norm<T>(#[case] v: T, #[case] re: Remainder, #[case] expected: T) where
        T: Copy + Debug + Radians + ApproxEq
    {
        let actual = v.norm_radians(re);
        assert_approx_eq!(T, expected, actual);
    }
} 
