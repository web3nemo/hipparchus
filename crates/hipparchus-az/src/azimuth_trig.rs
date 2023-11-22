use hipparchus_mean::Fp;
use crate::{Azimuth, Trigonometric, InverseTrigonometric};

impl<T> Trigonometric<T> for Azimuth<T> where T:Fp
{   
    /// Get the tangent of the angle
    fn tan(&self) -> T { self.y() / self.x() }

    /// Get the sine of the angle
    fn sin(&self) -> T { self.y() / self.hypot() }

    /// Get the cosine of the angle
    fn cos(&self) -> T { self.x() / self.hypot() }

    /// Get the sine & cosine of the angle
    fn sincos(&self) -> (T, T)
    {
        let h = self.hypot();
        (self.y() / h, self.x() / h)
    }
}

impl<T> InverseTrigonometric<T> for Azimuth<T> where T:Fp
{
    /// Returns the arc sine of the angle
    fn asin(v:T) -> Self { Self::with_radians(v.asin()) }

    /// Returns the arc cosine of the angle
    fn acos(v:T) -> Self { Self::with_radians(v.acos()) }

    /// Returns the arc tangent of the angle, where value is in the range [-PI/2, PI/2] radians
    fn atan(v:T) -> Self { Self::with_radians(v.atan()) }

    /// Returns the arc tangent of the angle, where value is in the range [-PI, PI) radians
    fn atan2(y:T, x:T) -> Self { Self::with_radians(T::atan2(y, x)) }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(3.0, 4.0, 5.0)]
    fn test_azimuth_trig_tan(#[case] y: f64, #[case] x: f64, #[case] h: f64)
    {
        let a = Azimuth::new(y, x);
        let tan = a.tan();
        let cot = a.cot();
        assert_approx_eq!(f64, y/x, tan);
        assert_approx_eq!(f64, x/y, cot);
        assert_approx_eq!(f64, a.radians().tan(), tan);
        assert_approx_eq!(f64, a.radians().cot(), cot);
        let sin = a.sin();
        let cos = a.cos();
        assert_approx_eq!(f64, y/h, sin);
        assert_approx_eq!(f64, x/h, cos);
        assert_approx_eq!(f64, tan, sin/cos);
        assert_approx_eq!(f64, cot, cos/sin);
        assert_approx_eq!(f64, 1.0, tan * cot);
    }

    #[rstest]
    #[case(3.0, 4.0, 5.0)]
    fn test_azimuth_trig_sincos(#[case] y: f64, #[case] x: f64, #[case] h: f64)
    {
        let a = Azimuth::new(y, x);
        let (sin, cos) = a.sincos();
        assert_approx_eq!(f64, y/h, sin);
        assert_approx_eq!(f64, x/h, cos);
        assert_approx_eq!(f64, a.sin(), sin);
        assert_approx_eq!(f64, a.cos(), cos);
        assert_approx_eq!(f64, a.radians().sin(), sin);
        assert_approx_eq!(f64, a.radians().cos(), cos);
        assert_approx_eq!(f64, 1.0, sin * sin + cos * cos);
    }

    #[rstest]
    #[case(3.0, 4.0, 5.0)]
    fn test_azimuth_sin(#[case] y: f64, #[case] x: f64, #[case] h: f64)
    {
        let a = Azimuth::new(y, x);
        let sin = a.sin();
        let sec = a.sec();
        let vcos = a.vcos();
        assert_approx_eq!(f64, y/h, sin);
        assert_approx_eq!(f64, h/y, sec);
        assert_approx_eq!(f64, 1.0, sec * sin);
        assert_approx_eq!(f64, 1.0, vcos + sin);
        assert_approx_eq!(f64, a.radians().sin(), sin);
        assert_approx_eq!(f64, a.radians().sec(), sec);
        assert_approx_eq!(f64, a.radians().vcos(), vcos);
    }

    #[rstest]
    #[case(3.0, 4.0, 5.0)]
    fn test_azimuth_cos(#[case] y: f64, #[case] x: f64, #[case] h: f64)
    {
        let a = Azimuth::new(y, x);
        let cos = a.cos();
        let csc = a.csc();
        let vsin = a.vsin();
        assert_approx_eq!(f64, x/h, cos);
        assert_approx_eq!(f64, h/x, csc);
        assert_approx_eq!(f64, 1.0, csc * cos);
        assert_approx_eq!(f64, 1.0, vsin + cos);
        assert_approx_eq!(f64, a.radians().cos(), cos);
        assert_approx_eq!(f64, a.radians().csc(), csc);
        assert_approx_eq!(f64, a.radians().vsin(), vsin);
    }
}
