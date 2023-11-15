use std::ops::{Div, Mul};
use num::FromPrimitive;

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

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_two_consts()
    {
        let onehalf = f32::onehalf();
        assert_approx_eq!(f32, 0.5, onehalf);
        let two = f32::two();
        assert_approx_eq!(f32, 2.0, two);
    }

    #[rstest]
    #[case(1.0, 0.5)]
    #[case(-1.0, -0.5)]
    fn test_two_half(#[case] value: f64, #[case] expected: f64)
    {
        let actual = value.half();
        assert_approx_eq!(f64, expected, actual);
        assert_approx_eq!(f64, value, actual.twice());
    }

    #[rstest]
    #[case(1.0, 2.0)]
    #[case(-1.0, -2.0)]
    fn test_two_twice(#[case] value: f64, #[case] expected: f64)
    {
        let actual = value.twice();
        assert_approx_eq!(f64, expected, actual);
        assert_approx_eq!(f64, value, actual.half());
    }
}