use std::ops::AddAssign;
use num::{Zero, One};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Angle
{
    y: f64,
    x: f64,
}

impl Angle
{
    pub fn new(y:f64, x:f64) -> Self { Self { y, x } }

    pub fn y(&self) -> f64 { self.y }

    pub fn x(&self) -> f64 { self.x }

    pub fn tan(&self) -> f64 { self.y / self.x }

    pub fn nan() -> Self { Self::new(f64::NAN, f64::NAN) }

    pub fn is_nan(&self) -> bool
    {
        false
        || self.y.is_nan() || self.x.is_nan() 
        || (self.x.is_zero() && self.y.is_zero()) 
        || (self.x.is_infinite() && self.y.is_infinite())
    }

    pub fn with_radians(r:f64) -> Self
    {
        Self
        {
            y: r.sin(),
            x: r.cos(),
        }
    }

    pub fn with_degrees(d:f64) -> Self
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

    pub fn degrees(&self) -> f64
    {
        if self.is_nan() { f64::NAN } else { f64::atan2(self.y, self.x).to_degrees() }
    }

    pub fn radians(&self) -> f64 
    {
        f64::atan2(self.y, self.x)
    }

    /// Returns the hypotenuse of the angle
    pub fn hypot(&self) -> f64
    {
        f64::hypot(self.y, self.x)
    }

    /// Returns true if the angle is normalized
    pub fn is_normalized(&self) -> bool
    {
        self.hypot().is_one()
    }

    pub fn norm(&mut self)
    { 
        todo!()
    }

    pub fn normalized(&self) -> Self
    {
        if self.is_nan()
        {
            return Self::nan();
        }

        let r = self.hypot();
        if r.is_finite()
        {
            return Self
            {
                y: self.y / r,
                x: self.x / r,
            };
        }

        let max = f64::max(self.y.abs(), self.x.abs());
        if max.is_infinite()
        {
            Self
            {
                y: if self.y.is_finite() { 0.0 } else { 1.0f64.copysign(self.y) },
                x: if self.x.is_finite() { 0.0 } else { 1.0f64.copysign(self.x) },
            }
        }
        else
        {
            Self
            {
                y: self.y / max,
                x: self.x / max,
            }.normalized()
       }
    }

    /*
    pub fn with_lambertian(psi:f64) -> Self { todo!() }

    pub fn with_lambertian_degrees(psid:f64) -> Self { todo!() }

    pub fn lambertian(&self) -> f64 { todo!() }

    pub fn lambertian_degrees(&self) -> f64 { todo!() }
    */
}

impl AddAssign for Angle
{
    fn add_assign(&mut self, rhs: Self) 
    {
        if !rhs.tan().is_zero()
        {
            let x = self.x * rhs.x - self.y * rhs.y;
            let y = self.y * rhs.x + self.x * rhs.y;
            self.x = x;
            self.y = y;
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

    #[rstest]
    #[case(1.0, 1.0, 1.0)]
    #[case(1.0, -1.0, -1.0)]
    #[case(-1.0, 1.0, -1.0)]
    #[case(-1.0, -1.0, 1.0)]
    #[case(2.0, 1.0, 2.0)]
    #[case(2.0, -1.0, -2.0)]
    #[case(-2.0, 1.0, -2.0)]
    #[case(-2.0, -1.0, 2.0)]
    fn test_angle(#[case] y: f64, #[case] x: f64, #[case] expected: f64)
    {
        let a = Angle::new(y, x);
        assert_approx_eq!(f64, y, a.y());
        assert_approx_eq!(f64, x, a.x());
        assert_approx_eq!(f64, expected, a.tan());
        assert_eq!(false, a.is_nan());
    }

    #[rstest]
    #[case(0.0, 1.0)]
    #[case(0.0, -1.0)]
    #[case(0.0, f64::INFINITY)]
    #[case(0.0, f64::NEG_INFINITY)]
    fn test_angle_zero(#[case] y: f64, #[case] x: f64)
    {
        let a = Angle::new(y, x);
        assert_approx_eq!(f64, y, a.y());
        assert_approx_eq!(f64, x, a.x());
        assert_eq!(true, a.tan().is_zero(), "expected zero, got {}", a.tan());
        assert_eq!(false, a.is_nan());
    }

    #[rstest]
    #[case(1.0, 0.0, f64::INFINITY)]
    #[case(f64::INFINITY, 0.0, f64::INFINITY)]
    #[case(-1.0, 0.0, f64::NEG_INFINITY)]
    #[case(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY)]
    fn test_angle_inf(#[case] y: f64, #[case] x: f64, #[case] expected: f64)
    {
        let a = Angle::new(y, x);
        assert_approx_eq!(f64, y, a.y());
        assert_approx_eq!(f64, x, a.x());
        assert_approx_eq!(f64, expected, a.tan());
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
    fn test_angle_nan(#[case] y: f64, #[case] x: f64)
    {
        let a = Angle::new(y, x);
        assert_approx_eq!(f64, y, a.y());
        assert_approx_eq!(f64, x, a.x());
        assert_eq!(true, a.tan().is_nan(), "expected NAN, got {}", a.tan());
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
    fn test_angle_degrees(#[case] d: f64)
    {
        let r = d.to_radians();
        let y = r.sin();
        let x = r.cos();

        let a = Angle::with_degrees(d);
        assert_approx_eq!(f64, y, a.y);
        assert_approx_eq!(f64, x, a.x);
        assert_approx_eq!(f64, d, a.degrees());
        assert_approx_eq!(f64, r, a.radians());
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
    fn test_angle_radians(#[case] r: f64)
    {
        let d = r.to_degrees();
        let y = r.sin();
        let x = r.cos();
        let a = Angle::with_radians(r);
        assert_approx_eq!(f64, y, a.y);
        assert_approx_eq!(f64, x, a.x);
        assert_approx_eq!(f64, d, a.degrees());
        assert_approx_eq!(f64, r, a.radians());
    }

    #[rstest]
    #[case(3.0, 4.0, 5.0, false)]
    #[case(-3.0, 4.0, 5.0, false)]
    #[case(3.0, -4.0, 5.0, false)]
    #[case(-3.0, -4.0, 5.0, false)]
    #[case(0.6, 0.8, 1.0, true)]
    #[case(-0.6, 0.8, 1.0, true)]
    #[case(0.6, -0.8, 1.0, true)]
    #[case(-0.6, -0.8, 1.0, true)]
    fn test_angle_hypot(#[case] y: f64, #[case] x: f64, #[case] h: f64, #[case] n: bool)
    {
        let a = Angle::new(y, x);
        assert_approx_eq!(f64, y, a.y());
        assert_approx_eq!(f64, x, a.x());
        assert_approx_eq!(f64, h, a.hypot());
        assert_eq!(n, a.is_normalized());
    }

    #[rstest]
    #[case(0.0, 1.0)]
    #[case(0.0, -1.0)]
    #[case(1.0, 0.0)]
    #[case(-1.0, 0.0)]
    fn test_angle_norm_same_axis(#[case] y: f64, #[case] x: f64)
    {
        let a = Angle::new(y, x);
        let n = a.normalized();
        assert_approx_eq!(f64, y, n.y());
        assert_approx_eq!(f64, x, n.x());
        assert_approx_eq!(f64, 1.0, n.hypot());
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
    fn test_angle_norm_same_degree(#[case] d: f64)
    {
        let r = d.to_radians();
        let y = r.sin();
        let x = r.cos();
        let a = Angle::new(y, x);
        let n = a.normalized();
        assert_approx_eq!(f64, y, n.y());
        assert_approx_eq!(f64, x, n.x());
        assert_approx_eq!(f64, 1.0, n.hypot());
    }

    #[rstest]
    #[case(f64::MAX, f64::MAX, Angle::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2))]
    #[case(f64::MIN, f64::MIN, Angle::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2))]
    #[case(f64::MAX, f64::MIN, Angle::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2))]
    #[case(f64::MIN, f64::MAX, Angle::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2))]
    #[case(1.0, f64::MAX, Angle::new(0.0, 1.0))]
    #[case(1.0, f64::MIN, Angle::new(0.0, -1.0))]
    #[case(f64::MAX, 1.0, Angle::new(1.0, 0.0))]
    #[case(f64::MIN, 1.0, Angle::new(-1.0, 0.0))]
    fn test_angle_norm_big(#[case] y: f64, #[case] x: f64, #[case] expected: Angle)
    {
        let a = Angle::new(y, x);
        let n = a.normalized();
        assert_approx_eq!(f64, expected.y, n.y());
        assert_approx_eq!(f64, expected.x, n.x());
        assert_approx_eq!(f64, 1.0, n.hypot());
    }

    #[rstest]
    #[case(0.0, f64::INFINITY, Angle::new(0.0, 1.0))]
    #[case(0.0, f64::NEG_INFINITY, Angle::new(0.0, -1.0))]
    #[case(f64::INFINITY, 0.0, Angle::new(1.0, 0.0))]
    #[case(f64::NEG_INFINITY, 0.0, Angle::new(-1.0, 0.0))]
    fn test_angle_norm_inf(#[case] y: f64, #[case] x: f64, #[case] expected: Angle)
    {
        let a = Angle::new(y, x);
        let n = a.normalized();
        assert_approx_eq!(f64, expected.y, n.y());
        assert_approx_eq!(f64, expected.x, n.x());
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
    fn test_angle_norm_nan(#[case] y: f64, #[case] x: f64)
    {
        let a = Angle::new(y, x);
        let n = a.normalized();
        assert_eq!(true, n.y().is_nan(), "n.y() expected NAN, got {}", n.y());
        assert_eq!(true, n.x().is_nan(), "n.x() expected NAN, got {}", n.x());
        assert_eq!(true, n.is_nan());
    }
}