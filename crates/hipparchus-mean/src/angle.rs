use crate::modulo::{Modulo, Remainder};

/// Trait for angle in degrees
pub trait Degrees
{
    const MOD_DEGREES: Self;
    fn norm_degrees(&self, re: Remainder) -> Self;
}

#[macro_export]
macro_rules! impl_degrees_signed
{
    ($ty:ty, $modd:expr) =>
    {
        impl Degrees for $ty
        {
            const MOD_DEGREES: Self = $modd;

            fn norm_degrees(&self, re: Remainder) -> Self
            {
                match re
                {
                    Remainder::Euclidean => self.umod($modd),
                    Remainder::Symmetry => self.smod($modd),
                    Remainder::InvertedSymmetry => self.smod(-$modd),
                }
            }
        }
    }
}

impl_degrees_signed!(f64, 360.0);
impl_degrees_signed!(f32, 360.0);
impl_degrees_signed!(i128, 360);
impl_degrees_signed!(i64, 360);
impl_degrees_signed!(i32, 360);
impl_degrees_signed!(i16, 360);

#[macro_export]
macro_rules! impl_degrees_unsigned
{
    ($ty:ty, $modd:expr) =>
    {
        impl Degrees for $ty
        {
            const MOD_DEGREES: Self = $modd;

            fn norm_degrees(&self, re: Remainder) -> Self
            {
                match re
                {
                    Remainder::Euclidean => self.rem_euclid($modd),
                    Remainder::Symmetry => panic!("Invalid modulo for degrees in Symmetry mode"),
                    Remainder::InvertedSymmetry  => panic!("Invalid modulo for degrees in InvertedSymmetry mode"),
                }
            }
        }
    }
}

impl_degrees_unsigned!(u128, 360);
impl_degrees_unsigned!(u64, 360);
impl_degrees_unsigned!(u32, 360);
impl_degrees_unsigned!(u16, 360);

// std::f64::consts::TAU
/// Trait for angle in radians
pub trait Radians
{
    const MOD_RADIANS: Self;
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
    fn test_degrees_norm_signed<T>(#[case] v: T, #[case] re: Remainder, #[case] expected: T) where
        T: Copy + Debug + Degrees + PartialEq
    {
        let actual = v.norm_degrees(re);
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(360u128, Remainder::Euclidean, 0u128)]
    #[case(360u64, Remainder::Euclidean, 0u64)]
    #[case(360u32, Remainder::Euclidean, 0u32)]
    #[case(360u16, Remainder::Euclidean, 0u16)]
    fn test_degrees_norm_unsigned<T>(#[case] v: T, #[case] re: Remainder, #[case] expected: T) where
        T: Copy + Debug + Degrees + PartialEq
    {
        let actual = v.norm_degrees(re);
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(360u128, Remainder::Symmetry, 0u128)]
    #[case(360u64, Remainder::Symmetry, 0u64)]
    #[case(360u32, Remainder::Symmetry, 0u32)]
    #[case(360u16, Remainder::Symmetry, 0u16)]
    #[case(360u128, Remainder::InvertedSymmetry, 0u128)]
    #[case(360u64, Remainder::InvertedSymmetry, 0u64)]
    #[case(360u32, Remainder::InvertedSymmetry, 0u32)]
    #[case(360u16, Remainder::InvertedSymmetry, 0u16)]
    #[should_panic]
    fn test_degrees_norm_panic<T>(#[case] v: T, #[case] re: Remainder, #[case] expected: T) where
        T: Copy + Debug + Degrees + PartialEq
    {
        let actual = v.norm_degrees(re);
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(-180.0f64, Remainder::Euclidean, 180.0f64)]
    #[case(-180.0f64, Remainder::Symmetry, -180.0f64)]
    #[case(-180.0f64, Remainder::InvertedSymmetry, 180.0f64)]
    #[case(-180.0f32, Remainder::Euclidean, 180.0f32)]
    #[case(-180.0f32, Remainder::Symmetry, -180.0f32)]
    #[case(-180.0f32, Remainder::InvertedSymmetry, 180.0f32)]
    fn test_degrees_norm_fp<T>(#[case] v: T, #[case] re: Remainder, #[case] expected: T) where
        T: Copy + Debug + Degrees + ApproxEq
    {
        let actual = v.norm_degrees(re);
        assert_approx_eq!(T, expected, actual);
    }

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
