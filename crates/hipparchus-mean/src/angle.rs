use crate::modulo::{Modulo, Remainder};
use num::Zero;

/// Trait for angle in degrees
pub trait Angle
{
    const MOD_DEGREES: Self;
    const MOD_RADIANS: Self;

    fn norm_degrees(&self, re: Remainder) -> Self;
    fn norm_radians(&self, re: Remainder) -> Self;
}

#[macro_export]
macro_rules! impl_angle
{
    ($ty:ty, $modd:expr, $modr:expr) =>
    {
        impl Angle for $ty
        {
            const MOD_DEGREES: Self = $modd;
            const MOD_RADIANS: Self = $modr;

            fn norm_degrees(&self, re: Remainder) -> Self
            {
                if $modd.is_zero() 
                {
                    panic!("Invalid modulo with zero MOD_DEGREES");
                }

                match re
                {
                    Remainder::Euclidean => self.umod($modd),
                    Remainder::Symmetry => self.smod($modd),
                    Remainder::InvertedSymmetry => self.smod(-$modd),
                }
            }

            fn norm_radians(&self, re: Remainder) -> Self
            {
                if $modr.is_zero() 
                {
                    panic!("Invalid modulo with zero MOD_RADIANS");
                }

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

impl_angle!(f64, 360.0, std::f64::consts::TAU);
impl_angle!(f32, 360.0, std::f32::consts::TAU);
impl_angle!(i128, 360, 0);
impl_angle!(i64, 360, 0);
impl_angle!(i32, 360, 0);
impl_angle!(i16, 360, 0);

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use core::fmt::Debug;
    use float_cmp::{ assert_approx_eq, ApproxEq };

    #[rstest]
    #[case(-180.0f64, Remainder::Euclidean, 180.0f64)]
    #[case(-180.0f64, Remainder::Symmetry, -180.0f64)]
    #[case(-180.0f64, Remainder::InvertedSymmetry, 180.0f64)]
    #[case(-180.0f32, Remainder::Euclidean, 180.0f32)]
    #[case(-180.0f32, Remainder::Symmetry, -180.0f32)]
    #[case(-180.0f32, Remainder::InvertedSymmetry, 180.0f32)]
    fn test_angle_norm_degrees_fp<T>(#[case] v: T, #[case] re: Remainder, #[case] expected: T) where
        T: Copy + Debug + Angle + ApproxEq
    {
        let actual = v.norm_degrees(re);
        assert_approx_eq!(T, expected, actual);
    }

    #[rstest]
    #[case(-180i128, Remainder::Euclidean, 180i128)]
    #[case(-180i128, Remainder::Symmetry, -180i128)]
    #[case(-180i128, Remainder::InvertedSymmetry, 180i128)]
    #[case(-180i64, Remainder::Euclidean, 180i64)]
    #[case(-180i64, Remainder::Symmetry, -180i64)]
    #[case(-180i64, Remainder::InvertedSymmetry, 180i64)]
    #[case(-180i32, Remainder::Euclidean, 180i32)]
    #[case(-180i32, Remainder::Symmetry, -180i32)]
    #[case(-180i32, Remainder::InvertedSymmetry, 180i32)]
    #[case(-180i16, Remainder::Euclidean, 180i16)]
    #[case(-180i16, Remainder::Symmetry, -180i16)]
    #[case(-180i16, Remainder::InvertedSymmetry, 180i16)]
    fn test_angle_norm_degrees<T>(#[case] v: T, #[case] re: Remainder, #[case] expected: T) where
        T: Copy + Debug + Angle + PartialEq
    {
        let actual = v.norm_degrees(re);
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(-std::f64::consts::PI, Remainder::Euclidean, std::f64::consts::PI)]
    #[case(-std::f64::consts::PI, Remainder::Symmetry, -std::f64::consts::PI)]
    #[case(-std::f64::consts::PI, Remainder::InvertedSymmetry, std::f64::consts::PI)]
    #[case(-std::f32::consts::PI, Remainder::Euclidean, std::f32::consts::PI)]
    #[case(-std::f32::consts::PI, Remainder::Symmetry, -std::f32::consts::PI)]
    #[case(-std::f32::consts::PI, Remainder::InvertedSymmetry, std::f32::consts::PI)]
    fn test_angle_norm_radians<T>(#[case] v: T, #[case] re: Remainder, #[case] expected: T) where
        T: Copy + Debug + Angle + ApproxEq
    {
        let actual = v.norm_radians(re);
        assert_approx_eq!(T, expected, actual);
    }

    #[rstest]
    #[case(-180i128, Remainder::Euclidean)]
    #[case(-180i128, Remainder::Symmetry)]
    #[case(-180i128, Remainder::InvertedSymmetry)]
    #[case(-180i64, Remainder::Euclidean)]
    #[case(-180i64, Remainder::Symmetry)]
    #[case(-180i64, Remainder::InvertedSymmetry)]
    #[case(-180i32, Remainder::Euclidean)]
    #[case(-180i32, Remainder::Symmetry)]
    #[case(-180i32, Remainder::InvertedSymmetry)]
    #[case(-180i16, Remainder::Euclidean)]
    #[case(-180i16, Remainder::Symmetry)]
    #[case(-180i16, Remainder::InvertedSymmetry)]
    #[should_panic]
    fn test_angle_norm_radians_panic(#[case] v: impl Angle, #[case] re: Remainder)
    {
        v.norm_radians(re);
    }
} 
