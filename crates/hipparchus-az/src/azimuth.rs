use hipparchus_mean::Fp;

/// To leverage y/x representation of an angle to acquire better precision & performance in regular situations.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Azimuth<T> where T:Fp
{
    /// y component of the angle (sine of the angle when normailzed)
    y: T,

    /// x component of the angle (cosine of the angle when normailzed)
    x: T,
}

impl<T> Azimuth<T> where T:Fp
{
    /// Creates a new angle with the given y & x components
    pub fn new(y:T, x:T) -> Self { Self { y, x } }

    /// Get a NaN angle
    pub fn nan() -> Self { Self::new(T::nan(), T::nan()) }

    /// Get the y component of the angle
    pub fn y(&self) -> T { self.y }

    /// Get the x component of the angle
    pub fn x(&self) -> T { self.x }

    /// Set the y & x components of the angle
    pub fn set(&mut self, y:T, x:T) { self.y = y; self.x = x; }

    /// Returns true if the angle is NaN
    pub fn is_nan(&self) -> bool
    {
        false
        || self.y.is_nan() || self.x.is_nan() 
        || (self.x.is_zero() && self.y.is_zero())
        || (self.x.is_infinite() && self.y.is_infinite())
    }

    /// Create an angle from the given radians
    pub fn with_radians(r:T) -> Self
    {
        Self
        {
            y: r.sin(),
            x: r.cos(),
        }
    }

    /// Create an angle from the given degrees
    pub fn with_degrees(d:T) -> Self
    {
        if d.is_infinite() || d.is_nan()
        {
            Self::nan()
        }
        else
        {
            let r = d.to_radians();
            let y = r.sin();
            let x = r.cos();
            Self{ y, x}
        }
    }

    /// Get the angle in degrees
    pub fn degrees(&self) -> T
    {
        if self.is_nan() { T::nan() } else { T::atan2(self.y, self.x).to_degrees() }
    }

    /// Get the angle in radians
    pub fn radians(&self) -> T 
    {
        T::atan2(self.y, self.x)
    }

    /// Returns the hypotenuse of the angle
    pub fn hypot(&self) -> T
    {
        T::hypot(self.y, self.x)
    }

    pub fn scale(&self, rhs:T) -> Self
    {
        Self
        {
            y: self.y * rhs,
            x: self.x * rhs,
        }
    }

    pub fn scale_assign(&mut self, rhs:T)
    {
        self.y *= rhs;
        self.x *= rhs;
    }

    pub fn is_zero_family(&self) -> bool
    {
        let k = self.y /self.x;
        k.is_zero()
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(1.0, -1.0)]
    #[case(-1.0, 1.0)]
    #[case(-1.0, -1.0)]
    fn test_azimuth(#[case] y: f64, #[case] x: f64)
    {
        let a = Azimuth::new(y, x);
        assert_approx_eq!(f64, y, a.y());
        assert_approx_eq!(f64, x, a.x());
        assert_eq!(false, a.is_nan());
    }

    #[rstest]
    #[case(0.0, 1.0)]
    #[case(0.0, -1.0)]
    #[case(0.0, f64::INFINITY)]
    #[case(0.0, f64::NEG_INFINITY)]
    fn test_azimuth_zero(#[case] y: f64, #[case] x: f64)
    {
        let a = Azimuth::new(y, x);
        assert_approx_eq!(f64, y, a.y());
        assert_approx_eq!(f64, x, a.x());
        assert_eq!(false, a.is_nan());
    }

    #[rstest]
    #[case(1.0, 0.0)]
    #[case(f64::INFINITY, 0.0)]
    #[case(-1.0, 0.0)]
    #[case(f64::NEG_INFINITY, 0.0)]
    fn test_azimuth_inf(#[case] y: f64, #[case] x: f64)
    {
        let a = Azimuth::new(y, x);
        assert_approx_eq!(f64, y, a.y());
        assert_approx_eq!(f64, x, a.x());
        assert_eq!(false, a.is_nan());
    }

    // (0, 0) => nan
    // (+/-inf, +/-inf) => nan
    // (nan, any) & (any, nan) => nan
    // (nan, nan) => nan
    #[rstest]
    #[case(0.0, 0.0)]
    #[case(f64::INFINITY, f64::INFINITY)]
    #[case(f64::INFINITY, f64::NEG_INFINITY)]
    #[case(f64::NEG_INFINITY, f64::INFINITY)]
    #[case(f64::NEG_INFINITY, f64::NEG_INFINITY)]
    #[case(f64::NAN, 0.0)]
    #[case(f64::NAN, 1.0)]
    #[case(0.0, f64::NAN)]
    #[case(1.0, f64::NAN)]
    #[case(f64::NAN, f64::NAN)]
    fn test_azimuth_nan(#[case] y: f64, #[case] x: f64)
    {
        let a = Azimuth::new(y, x);
        assert_approx_eq!(f64, y, a.y());
        assert_approx_eq!(f64, x, a.x());
        assert_eq!(true, a.is_nan());
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
    fn test_azimuth_degrees(#[case] d: f64)
    {
        let r = d.to_radians();
        let y = r.sin();
        let x = r.cos();

        let a = Azimuth::with_degrees(d);
        assert_approx_eq!(f64, y, a.y);
        assert_approx_eq!(f64, x, a.x);
        assert_approx_eq!(f64, d, a.degrees());
        assert_approx_eq!(f64, r, a.radians());
    }

    #[rstest]
    #[case(f64::INFINITY)]
    #[case(f64::NEG_INFINITY)]
    #[case(f64::NAN)]
    fn test_azimuth_with_degrees_nan(#[case] d: f64)
    {
        let az = Azimuth::with_degrees(d);
        assert_eq!(true, az.is_nan());
    }

    #[rstest]
    #[case(f64::INFINITY)]
    #[case(f64::NEG_INFINITY)]
    #[case(f64::NAN)]
    fn test_azimuth_with_radians_nan(#[case] r: f64)
    {
        let az = Azimuth::with_radians(r);
        assert_eq!(true, az.is_nan());
    }

    #[rstest]
    #[case(std::f64::consts::PI)]
    #[case(std::f64::consts::FRAC_PI_2+std::f64::consts::FRAC_PI_4)]
    #[case(std::f64::consts::FRAC_PI_2)]
    #[case(std::f64::consts::FRAC_PI_4)]
    #[case(0.0)]
    #[case(-std::f64::consts::FRAC_PI_4)]
    #[case(-std::f64::consts::FRAC_PI_2)]
    #[case(-std::f64::consts::FRAC_PI_2-std::f64::consts::FRAC_PI_4)]
    #[case(-std::f64::consts::PI)]
    fn test_azimuth_radians(#[case] r: f64)
    {
        let d = r.to_degrees();
        let y = r.sin();
        let x = r.cos();
        let a = Azimuth::with_radians(r);
        assert_approx_eq!(f64, y, a.y);
        assert_approx_eq!(f64, x, a.x);
        assert_approx_eq!(f64, d, a.degrees());
        assert_approx_eq!(f64, r, a.radians());
    }

    #[rstest]
    #[case(3.0, 4.0)]
    fn test_azimuth_set(#[case] y: f64, #[case] x: f64)
    {
        let mut az = Azimuth::new(0.0, 0.0);
        assert_approx_eq!(f64, 0.0, az.y());
        assert_approx_eq!(f64, 0.0, az.x());
        az.set(y, x);
        assert_approx_eq!(f64, y, az.y());
        assert_approx_eq!(f64, x, az.x());
    }

    #[rstest]
    #[case(3.0, 4.0, 5.0)]
    #[case(-3.0, 4.0, 5.0)]
    #[case(3.0, -4.0, 5.0)]
    #[case(-3.0, -4.0, 5.0)]
    #[case(0.6, 0.8, 1.0)]
    #[case(-0.6, 0.8, 1.0)]
    #[case(0.6, -0.8, 1.0)]
    #[case(-0.6, -0.8, 1.0)]
    fn test_azimuth_hypot(#[case] y: f64, #[case] x: f64, #[case] h: f64)
    {
        let a = Azimuth::new(y, x);
        assert_approx_eq!(f64, y, a.y());
        assert_approx_eq!(f64, x, a.x());
        assert_approx_eq!(f64, h, a.hypot());
    }

    #[rstest]
    #[case(3.0, 4.0, 2.0, 10.0)]
    #[case(1.0, 1.0, f64::sqrt(2.0), 2.0)]
    fn test_azimuth_scale(#[case] y: f64, #[case] x: f64, #[case] rhs: f64, #[case] h: f64)
    {
        let mut az = Azimuth::new(y, x);

        let res = az.scale(rhs);
        assert_approx_eq!(f64, h, res.hypot());
        assert_approx_eq!(f64, az.radians(), res.radians());

        az.scale_assign(rhs);
        assert_approx_eq!(f64, az.hypot(), res.hypot());
        assert_approx_eq!(f64, az.radians(), res.radians());
    }

    #[rstest]
    #[case(3.0, 4.0, false)]
    #[case(0.0, 1.0, true)]
    #[case(0.0, 2.0, true)]
    #[case(0.0, f64::MAX, true)]
    #[case(0.0, f64::INFINITY, true)]
    #[case(0.0, f64::NAN, false)]
    #[case(0.0, 0.0, false)]
    fn test_azimuth_zero_family(#[case] y: f64, #[case] x: f64, #[case] res: bool)
    {
        let az = Azimuth::new(y, x);
        assert_eq!(res, az.is_zero_family());
    }
}