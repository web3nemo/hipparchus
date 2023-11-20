use hipparchus_mean::Fp;
use crate::Azimuth;

pub trait Norm
{
    /// Returns true if the instance is normalized
    fn is_normalized(&self) -> bool;

    /// Normalize the mutable instance
    fn norm(&mut self);

    /// Returns a normalized copy of the instance
    fn normalized(&self) -> Self;
}

fn azimuth_normalize<T>(x: &mut T, y: &mut T) where T:Fp
{
    if y.is_infinite() || x.is_infinite()
    {
        // If one of the components is infinite, so we can:
        // - Replace the infinite component with 1.0 or -1.0
        // - Replace the finite component with 0.0
        *y = if y.is_finite() { T::zero() } else { T::one().copysign(*y) };
        *x = if x.is_finite() { T::zero() } else { T::one().copysign(*x) };
    }
    else
    {
        let mut r = T::hypot(*y, *x);
        if r.is_infinite()
        {
            // None of the components are infinite but hypot is infinite, so we can:
            // - Devide by the max value of the absoulte values of the components
            // - Recalculate the hypot with finite components (already shrinked by the max value)
            let max = T::max(y.abs(), x.abs());
            *y /= max;
            *x /= max;
            r = T::hypot(*y, *x);
        }
        *y /= r;
        *x /= r;
    }
}

impl<T> Norm for Azimuth<T> where T:Fp
{
    fn is_normalized(&self) -> bool
    {
        // Due to round-off error, it is a bit risk to use the equal judgment statement below:
        // self.hypot().is_one()
        T::one().approx_eq(self.hypot(), T::Margin::default())
    }

    fn norm(&mut self)
    {
        if self.is_nan()
        {
            self.set(T::nan(), T::nan());
        }
        else
        {
            let mut y = self.y();
            let mut x = self.x();
            azimuth_normalize(&mut y, &mut x);
            self.set(y, x);
        }
    }

    fn normalized(&self) -> Self
    {
        if self.is_nan()
        {
            Self::nan()
        }
        else
        {
            let mut y = self.y();
            let mut x = self.x();
            azimuth_normalize(&mut y, &mut x);
            Self::new(y, x)
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;
    use std::f64::consts::FRAC_1_SQRT_2;

    fn assert_azimuth_norm(y:f64, x:f64, yn:f64, xn:f64)
    {
        let mut az = Azimuth::new(y, x);

        let n = az.normalized();
        assert_eq!(true, n.is_normalized());
        assert_approx_eq!(f64, yn, n.y());
        assert_approx_eq!(f64, xn, n.x());
        assert_approx_eq!(f64, 1.0, n.hypot());

        az.norm();
        assert_eq!(true, az.is_normalized());
        assert_approx_eq!(f64, yn, az.y());
        assert_approx_eq!(f64, xn, az.x());
        assert_approx_eq!(f64, 1.0, az.hypot());
    }

    #[rstest]
    #[case(0.0, 1.0)]
    #[case(0.0, -1.0)]
    #[case(1.0, 0.0)]
    #[case(-1.0, 0.0)]
    fn test_azimuth_norm_same_axis(#[case] y: f64, #[case] x: f64)
    {
        assert_azimuth_norm(y, x, y, x);
    }

    #[rstest]
    #[case(180.0)]
    #[case(135.0)]
    #[case(90.0)]
    #[case(45.0)]
    #[case(0.0)]
    #[case(-45.0)]
    #[case(-90.0)]
    #[case(-135.0)]
    #[case(-180.0)]
    fn test_azimuth_norm_same_degree(#[case] d: f64)
    {
        let r = d.to_radians();
        let y = r.sin();
        let x = r.cos();
        assert_azimuth_norm(y, x, y, x);
    }

    #[rstest]
    #[case(f64::MAX, f64::MAX, FRAC_1_SQRT_2, FRAC_1_SQRT_2)]
    #[case(f64::MIN, f64::MIN, -FRAC_1_SQRT_2, -FRAC_1_SQRT_2)]
    #[case(f64::MAX, f64::MIN, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)]
    #[case(f64::MIN, f64::MAX, -FRAC_1_SQRT_2, FRAC_1_SQRT_2)]
    #[case(1.0, f64::MAX, 0.0, 1.0)]
    #[case(1.0, f64::MIN, 0.0, -1.0)]
    #[case(f64::MAX, 1.0, 1.0, 0.0)]
    #[case(f64::MIN, 1.0, -1.0, 0.0)]
    fn test_azimuth_norm_big(#[case] y: f64, #[case] x: f64, #[case] yn: f64, #[case] xn: f64)
    {
        assert_azimuth_norm(y, x, yn, xn);
    }

    #[rstest]
    #[case(0.0, f64::INFINITY, 0.0, 1.0)]
    #[case(0.0, f64::NEG_INFINITY, 0.0, -1.0)]
    #[case(f64::INFINITY, 0.0, 1.0, 0.0)]
    #[case(f64::NEG_INFINITY, 0.0, -1.0, 0.0)]
    fn test_azimuth_norm_inf(#[case] y: f64, #[case] x: f64, #[case] yn: f64, #[case] xn: f64)
    {
        assert_azimuth_norm(y, x, yn, xn);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(f64::INFINITY, f64::INFINITY)]
    #[case(f64::NEG_INFINITY, f64::INFINITY)]
    #[case(f64::INFINITY, f64::NEG_INFINITY)]
    #[case(f64::NEG_INFINITY, f64::NEG_INFINITY)]
    #[case(f64::NAN, f64::NAN)]
    #[case(f64::NAN, 0.0)]
    #[case(0.0, f64::NAN)]
    fn test_azimuth_norm_nan(#[case] y: f64, #[case] x: f64)
    {
        let mut az = Azimuth::new(y, x);
        let n = az.normalized();
        assert_eq!(true, n.y().is_nan(), "n.y() expected NAN, got {}", n.y());
        assert_eq!(true, n.x().is_nan(), "n.x() expected NAN, got {}", n.x());
        assert_eq!(true, n.is_nan());

        az.norm();
        assert_eq!(true, az.y().is_nan(), "az.y() expected NAN, got {}", az.y());
        assert_eq!(true, az.x().is_nan(), "az.x() expected NAN, got {}", az.x());
        assert_eq!(true, az.is_nan());
    }
}
