use std::ops::{Rem, Sub, Add, Div, Neg, Mul};
use num::{FromPrimitive, Zero};
use crate::value::Two;

/// Trait for signed & unsigned mod operation
/// The default implementtion of % (or fmod) on nagative value and negative base are not well defined as standard. 
/// It is highly depends on programming language implementation. In Rust/C++, fmod (or %) are running in truncated mode. 
/// See in https://en.wikipedia.org/wiki/Modulo for details.
pub trait Modulo
{
    /// Returns the signed remainder of the division of self by base.
    /// - If base is positive, the outout value will be in range [-base/2, base/2)
    /// - If base is negative, the outout value will be in range (base/2, -base/2]
    /// - If base is zero, the output value should be self
    fn smod(self, base: Self) -> Self;

    /// Returns the unsigned remainder of the division of self by base.
    /// - If base is positive, the outout value will be in range [0, base)
    /// - If base is negative, the outout value will be in range (0, -base]
    /// - If base is zero, the output value should be self
    fn umod(self, base: Self) -> Self;
}

impl<T> Modulo for T where T: Copy + PartialOrd + FromPrimitive + Zero + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self> + Neg<Output=Self>,
{
    fn smod(self, base: Self) -> Self
    {
        if base > T::zero()
        {
            let r = self % base;
            let h = base.half();
            if r >= h
            {
                r - base
            }
            else if r < -h
            {
                r + base
            }
            else
            {
                r
            }
        }
        else if base < T::zero()
        {
            let r = self % base;
            let h = base.half();
            if r <= h
            {
                r - base
            }
            else if r > -h
            {
                r + base
            }
            else
            {
                r
            }
        }
        else
        {
            self
        }
    }

    fn umod(self, base: Self) -> Self
    {
        if base > T::zero()
        {
            let r = self % base;
            if r < T::zero()
            {
                r + base
            }
            else
            {
                r
            }
        }
        else if base < T::zero()
        {
            let r = self % base;
            if r <= T::zero()
            {
                r - base
            }
            else
            {
                r
            }
        }
        else
        {
            self
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(179.9, 360.0)]
    #[case(90.0, 360.0)]
    #[case(0.0, 360.0)]
    #[case(-90.0, 360.0)]
    #[case(-180.0, 360.0)]
    #[case(180.0, -360.0)]
    #[case(90.0, -360.0)]
    #[case(0.0, -360.0)]
    #[case(-90.0, -360.0)]
    #[case(-179.9, -360.0)]
    fn test_smod(#[case] value: f64, #[case] base: f64)
    {
        assert_approx_eq!(f64, value, value.smod(base));
        assert_approx_eq!(f64, value, (value+base).smod(base));
        assert_approx_eq!(f64, value, (value-base).smod(base));
    }

    #[rstest]
    #[case(180.0, 360.0, -180.0)]
    #[case(-180.0, 360.0, -180.0)]
    #[case(180.0, -360.0, 180.0)]
    #[case(-180.0, -360.0, 180.0)]
    fn test_smod_special(#[case] value: f64, #[case] base: f64, #[case] expected: f64)
    {
        let actual =value.smod(base);
        assert_approx_eq!(f64, expected, actual);
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(0.0f64, 360.0)]
    #[case(-0.0f64, 360.0)]
    #[case(0.0f64, -360.0)]
    #[case(-0.0f64, -360.0)]
    #[case(f64::MAX, 0.0f64)]
    #[case(f64::MIN, 0.0f64)]
    #[case(f64::MAX, -0.0f64)]
    #[case(f64::MIN, -0.0f64)]
    #[case(0.0f64, 0.0f64)]
    #[case(-0.0f64, 0.0f64)]
    #[case(0.0f64, -0.0f64)]
    #[case(-0.0f64, -0.0f64)]
    fn test_smod_zero(#[case] value: f64, #[case] base: f64)
    {
        let actual =value.smod(base);
        assert_approx_eq!(f64, value, actual);
        assert_eq!(value, actual);
        assert_eq!(value.to_bits(), actual.to_bits());
    }

    #[rstest]
    #[case(179, 360)]
    #[case(90, 360)]
    #[case(0, 360)]
    #[case(-90, 360)]
    #[case(-180, 360)]
    #[case(180, -360)]
    #[case(90, -360)]
    #[case(0, -360)]
    #[case(-90, -360)]
    #[case(-179, -360)]
    fn test_smod_i32(#[case] value: i32, #[case] base: i32)
    {
        assert_eq!(value, value.smod(base));
        assert_eq!(value, (value+base).smod(base));
        assert_eq!(value, (value-base).smod(base));
    }

    #[rstest]
    #[case(359.0, 360.0)]
    #[case(270.0, 360.0)]
    #[case(180.9, 360.0)]
    #[case(90.0, 360.0)]
    #[case(0.0, 360.0)]
    #[case(360.0, -360.0)]
    #[case(270.0, -360.0)]
    #[case(180.9, -360.0)]
    #[case(90.0, -360.0)]
    #[case(1.0, -360.0)]
    fn test_umod(#[case] value: f64, #[case] base: f64)
    {
        assert_approx_eq!(f64, value, value.umod(base));
        assert_approx_eq!(f64, value, (value+base).umod(base));
        assert_approx_eq!(f64, value, (value-base).umod(base));
    }

    #[rstest]
    #[case(359, 360)]
    #[case(270, 360)]
    #[case(180, 360)]
    #[case(90, 360)]
    #[case(0, 360)]
    #[case(360, -360)]
    #[case(270, -360)]
    #[case(180, -360)]
    #[case(90, -360)]
    #[case(1, -360)]      // Default ULPS is not enough
    fn test_umod_i32(#[case] value: i32, #[case] base: i32)
    {
        assert_eq!(value, value.umod(base));
        assert_eq!(value, (value+base).umod(base));
        assert_eq!(value, (value-base).umod(base));
    }
}
