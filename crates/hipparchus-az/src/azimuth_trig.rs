use hipparchus_mean::Fp;
use crate::{Trigonometric, Azimuth};

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
        (self.x() / h, self.y() / h)
    }
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
        let sin = a.sin();
        let cos = a.cos();
        assert_approx_eq!(f64, y/x, tan);
        assert_approx_eq!(f64, y/h, sin);
        assert_approx_eq!(f64, x/h, cos);
        assert_approx_eq!(f64, a.radians().tan(), tan);
        assert_approx_eq!(f64, a.radians().sin(), sin);
        assert_approx_eq!(f64, a.radians().cos(), cos);
        assert_approx_eq!(f64, tan, sin/cos);
        assert_approx_eq!(f64, 1.0, sin * sin + cos * cos);
    }

    #[rstest]
    #[case(3.0, 4.0, 5.0)]
    fn test_azimuth_sin(#[case] y: f64, #[case] x: f64, #[case] h: f64)
    {
        let a = Azimuth::new(y, x);
        let tan = a.tan();
        let sin = a.sin();
        let cos = a.cos();
        assert_approx_eq!(f64, y/x, tan);
        assert_approx_eq!(f64, y/h, sin);
        assert_approx_eq!(f64, x/h, cos);
        assert_approx_eq!(f64, a.radians().tan(), tan);
        assert_approx_eq!(f64, a.radians().sin(), sin);
        assert_approx_eq!(f64, a.radians().cos(), cos);
        assert_approx_eq!(f64, tan, sin/cos);
        assert_approx_eq!(f64, 1.0, sin * sin + cos * cos);
    }
}
