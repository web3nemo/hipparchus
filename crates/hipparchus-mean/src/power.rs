use num::FromPrimitive;
use num::traits::Pow;

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

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(2.0, 4.0)]
    #[case(-2.0, 4.0)]
    fn test_power_sq(#[case] value: f64, #[case] expected: f64)
    {
        let actual = value.sq();
        assert_approx_eq!(f64, expected, actual);
    }

    #[rstest]
    #[case(2.0, 8.0)]
    #[case(-2.0, -8.0)]
    fn test_power_cb(#[case] value: f64, #[case] expected: f64)
    {
        let actual = value.cu();
        assert_approx_eq!(f64, expected, actual);
    }
}