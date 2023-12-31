use std::ops::{Mul, MulAssign};
use hipparchus_mean::Fp;
use crate::Azimuth;

/// angle add + hypot multiply 
impl<T> std::ops::Add for Azimuth<T> where T:Fp
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self
    {
        if rhs.is_zero_family()
        {
            self
        }
        else
        {
            let x = self.x() * rhs.x() - self.y() * rhs.y();
            let y = self.y() * rhs.x() + self.x() * rhs.y();
            Self::new(y, x)
        }
    }
}

/// angle add + hypot multiply 
impl<T> std::ops::AddAssign for Azimuth<T> where T:Fp
{
    fn add_assign(&mut self, rhs: Self) 
    {
        if !rhs.is_zero_family()
        {
            let x = self.x() * rhs.x() - self.y() * rhs.y();
            let y = self.y() * rhs.x() + self.x() * rhs.y();
            self.set(y, x);
        }
    }
}

/// Unit element for addition where y = 0 and x = 1
impl<T> num::Zero for Azimuth<T> where T:Fp
{
    fn zero() -> Self
    {
        Self::new(T::zero(), T::one())
    }
    
    fn is_zero(&self) -> bool
    {
        // Due to round-off error, it is a bit risk to use the equal judgment statement below:
        // self.y().is_zero() && self.x().is_one()
        true
        && T::zero().approx_eq(self.y(), T::Margin::default()) 
        && T::one().approx_eq(self.x(), T::Margin::default())
    }
}

/// Additive Inverse
impl<T> std::ops::Neg for Azimuth<T> where T:Fp
{
    type Output = Self;

    fn neg(self) -> Self::Output
    {
        // x' = x / (r * r)
        // y' = -y / (r * r)
        let h = self.hypot();
        Self::new(-self.y()/(h*h), self.x()/(h*h))
    }
}

/// angle subtract + hypot devide 
impl<T> std::ops::Sub for Azimuth<T> where T:Fp
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output 
    {
        if rhs.is_zero_family()
        {
            self
        }
        else
        {
            let h = rhs.hypot();
            let x = self.x() * rhs.x() + self.y() * rhs.y();
            let y = self.y() * rhs.x() - self.x() * rhs.y();
            let hh = h * h;
            Self::new(y/hh, x/hh)
        }
    }
}

/// angle subtract + hypot devide 
impl<T> std::ops::SubAssign for Azimuth<T> where T:Fp
{
    fn sub_assign(&mut self, rhs: Self) 
    {
        if !rhs.is_zero_family()
        {
            let h = rhs.hypot();
            let x = self.x() * rhs.x() + self.y() * rhs.y();
            let y = self.y() * rhs.x() - self.x() * rhs.y();
            let hh = h * h;
            self.set(y/hh, x/hh)
        }
    }
}

/// angle multiply + hypot power
impl<T> std::ops::Mul<T> for Azimuth<T> where T:Fp
{
    type Output = Self;
    
    fn mul(self, rhs: T) -> Self::Output 
    {
        let h = self.hypot().powf(rhs);
        let rad = self.radians() * rhs;
        let mut az = Self::with_radians(rad);
        az.scale_assign(h);
        az
    }
}

/// angle multiply + hypot power
impl<T> std::ops::MulAssign<T> for Azimuth<T> where T:Fp
{
    fn mul_assign(&mut self, rhs: T)
    {
        let h = self.hypot().powf(rhs);
        let rad = self.radians() * rhs;
        let mut az = Self::with_radians(rad);
        az.scale_assign(h);
        self.set(az.y(), az.x());
    }
}

/// angle devide + hypot root
impl<T> std::ops::Div<T> for Azimuth<T> where T:Fp
{
    type Output = Self;
    
    fn div(self, rhs: T) -> Self::Output 
    {
        self.mul(rhs.recip())
    }
}

/// angle devide + hypot root
impl<T> std::ops::DivAssign<T> for Azimuth<T> where T:Fp
{
    fn div_assign(&mut self, rhs: T)
    {
        self.mul_assign(rhs.recip())
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;
    use std::ops::Neg;
    use num::Zero;

    #[rstest]
    #[case(1.0, 1.0, 1.0, 1.0, 2.0, 0.0)]
    #[case(3.0, 4.0, 4.0, 3.0, 25.0, 0.0)]
    fn test_azimuth_add(#[case] y1: f64, #[case] x1: f64, #[case] y2: f64, #[case] x2: f64, #[case] y: f64, #[case] x: f64)
    {
        // AZ = AZ1 + AZ2
        let az = Azimuth::new(y, x);
        let az1 = Azimuth::new(y1, x1);
        let az2 = Azimuth::new(y2, x2);

        let a = az1 + az2;
        assert_approx_eq!(f64, az.y(), a.y());
        assert_approx_eq!(f64, az.x(), a.x());

        let b = az2 + az1;
        assert_approx_eq!(f64, az.y(), b.y());
        assert_approx_eq!(f64, az.x(), b.x());
    }

    #[rstest]
    #[case(1.0, 1.0, 1.0, 1.0, 2.0, 0.0)]
    #[case(3.0, 4.0, 4.0, 3.0, 25.0, 0.0)]
    fn test_azimuth_addassign(#[case] y1: f64, #[case] x1: f64, #[case] y2: f64, #[case] x2: f64, #[case] y: f64, #[case] x: f64)
    {
        // AZ = AZ1 + AZ2
        let az = Azimuth::new(y, x);
        let az1 = Azimuth::new(y1, x1);
        let az2 = Azimuth::new(y2, x2);

        let mut az1m = Azimuth::new(y1, x1);
        az1m += az2;
        assert_approx_eq!(f64, az.y(), az1m.y());
        assert_approx_eq!(f64, az.x(), az1m.x());

        let mut az2m = Azimuth::new(y2, x2);
        az2m += az1;
        assert_approx_eq!(f64, az.y(), az2m.y());
        assert_approx_eq!(f64, az.x(), az2m.x());
    }

    #[rstest]
    #[case(1.0, 1.0, 1.0, 1.0, 2.0, 0.0)]
    #[case(3.0, 4.0, 4.0, 3.0, 25.0, 0.0)]
    fn test_azimuth_sub(#[case] y1: f64, #[case] x1: f64, #[case] y2: f64, #[case] x2: f64, #[case] y: f64, #[case] x: f64)
    {
        // AZ = AZ1 + AZ2
        let az = Azimuth::new(y, x);
        let az1 = Azimuth::new(y1, x1);
        let az2 = Azimuth::new(y2, x2);

        let a = az - az1;
        assert_approx_eq!(f64, az2.y(), a.y());
        assert_approx_eq!(f64, az2.x(), a.x());

        let b = az - az2;
        assert_approx_eq!(f64, az1.y(), b.y());
        assert_approx_eq!(f64, az1.x(), b.x());
    }

    #[rstest]
    #[case(1.0, 1.0, 1.0, 1.0, 2.0, 0.0)]
    #[case(3.0, 4.0, 4.0, 3.0, 25.0, 0.0)]
    fn test_azimuth_subassign(#[case] y1: f64, #[case] x1: f64, #[case] y2: f64, #[case] x2: f64, #[case] y: f64, #[case] x: f64)
    {
        // AZ = AZ1 + AZ2
        let az1 = Azimuth::new(y1, x1);
        let az2 = Azimuth::new(y2, x2);

        let mut az1m = Azimuth::new(y, x);
        az1m -= az1;
        assert_approx_eq!(f64, az2.y(), az1m.y());
        assert_approx_eq!(f64, az2.x(), az1m.x());

        let mut az2m = Azimuth::new(y, x);
        az2m -= az2;
        assert_approx_eq!(f64, az1.y(), az2m.y());
        assert_approx_eq!(f64, az1.x(), az2m.x());
    }

    #[rstest]
    #[case(1.0, 1.0, 1.0, 1.0, 2.0, 0.0)]
    #[case(3.0, 4.0, 4.0, 3.0, 25.0, 0.0)]
    fn test_azimuth_neg_add(#[case] y1: f64, #[case] x1: f64, #[case] y2: f64, #[case] x2: f64, #[case] y: f64, #[case] x: f64)
    {
        // AZ = AZ1 + AZ2
        let az = Azimuth::new(y, x);
        let az1 = Azimuth::new(y1, x1);
        let az2 = Azimuth::new(y2, x2);

        let c = az + az1.neg();
        assert_approx_eq!(f64, az2.y(), c.y());
        assert_approx_eq!(f64, az2.x(), c.x());
        let d = az + az2.neg();
        assert_approx_eq!(f64, az1.y(), d.y());
        assert_approx_eq!(f64, az1.x(), d.x());
    }

    #[rstest]
    #[case(1.0, 1.0, 1.0, 1.0, 2.0, 0.0)]
    #[case(3.0, 4.0, 4.0, 3.0, 25.0, 0.0)]
    fn test_azimuth_neg_sub(#[case] y1: f64, #[case] x1: f64, #[case] y2: f64, #[case] x2: f64, #[case] y: f64, #[case] x: f64)
    {
        // AZ = AZ1 + AZ2
        let az = Azimuth::new(y, x);
        let az1 = Azimuth::new(y1, x1);
        let az2 = Azimuth::new(y2, x2);

        let a = az1 - az2.neg();
        assert_approx_eq!(f64, az.y(), a.y());
        assert_approx_eq!(f64, az.x(), a.x());
        let b = az2 - az1.neg();
        assert_approx_eq!(f64, az.y(), b.y());
        assert_approx_eq!(f64, az.x(), b.x());
    }

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(3.0, 4.0)]
    fn test_azimuth_zero_add(#[case] y: f64, #[case] x: f64)
    {
        let az = Azimuth::new(y, x);
        let zero = Azimuth::zero();
        let neg = az.neg();

        // Az + (-Az) = 0
        let a = az + neg;
        assert!(a.is_zero_family());
        assert!(a.is_zero());
        assert_approx_eq!(f64, zero.y(), a.y());
        assert_approx_eq!(f64, zero.x(), a.x());

        // Az + 0 = az
        let b = az + zero;
        assert_approx_eq!(f64, az.y(), b.y());
        assert_approx_eq!(f64, az.x(), b.x());

        // 0 + Az = az
        let c = zero + az;
        assert_approx_eq!(f64, az.y(), c.y());
        assert_approx_eq!(f64, az.x(), c.x());
    }

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(3.0, 4.0)]
    fn test_azimuth_zero_addassign(#[case] y: f64, #[case] x: f64)
    {
        let mut az = Azimuth::new(y, x);
        az += Azimuth::zero();
        assert_approx_eq!(f64, y, az.y());
        assert_approx_eq!(f64, x, az.x());
    }

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(3.0, 4.0)]
    fn test_azimuth_zero_sub(#[case] y: f64, #[case] x: f64)
    {
        let az = Azimuth::new(y, x);
        let zero = Azimuth::zero();
        
        // Az - Az = 0
        let a = az - az;
        assert!(a.is_zero_family());
        assert!(a.is_zero());
        assert_approx_eq!(f64, zero.y(), a.y());
        assert_approx_eq!(f64, zero.x(), a.x());

        // Az - 0 = az
        let b = az - zero;
        assert_eq!(az, b);
    }

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(3.0, 4.0)]
    fn test_azimuth_zero_subassign(#[case] y: f64, #[case] x: f64)
    {
        let mut az = Azimuth::new(y, x);
        az -= Azimuth::zero();
        assert_approx_eq!(f64, y, az.y());
        assert_approx_eq!(f64, x, az.x());
    }

    #[rstest]
    #[case(1.0, 1.0, 2.0, 2.0, 0.0)]
    fn test_azimuth_mul(#[case] y: f64, #[case] x: f64, #[case] rhs: f64, #[case] resy: f64, #[case] resx: f64)
    {
        // Az' = Az * 2
        let az = Azimuth::new(y, x);
        let res = az * rhs;
        assert_approx_eq!(f64, resy, res.y());
        assert_approx_eq!(f64, resx, res.x());
    }

    #[rstest]
    #[case(1.0, 1.0, 2.0, 2.0, 0.0)]
    fn test_azimuth_mulassign(#[case] y: f64, #[case] x: f64, #[case] rhs: f64, #[case] resy: f64, #[case] resx: f64)
    {
        // Az' = Az * 2
        let mut az = Azimuth::new(y, x);
        az *= rhs;
        assert_approx_eq!(f64, resy, az.y());
        assert_approx_eq!(f64, resx, az.x());
    }

    #[rstest]
    #[case(2.0, 0.0, 2.0, 1.0, 1.0)]
    fn test_azimuth_div(#[case] y: f64, #[case] x: f64, #[case] rhs: f64, #[case] resy: f64, #[case] resx: f64)
    {
        // Az' = Az / 2
        let az = Azimuth::new(y, x);
        let res = az / rhs;
        assert_approx_eq!(f64, resy, res.y());
        assert_approx_eq!(f64, resx, res.x());
    }

    #[rstest]
    #[case(2.0, 0.0, 2.0, 1.0, 1.0)]
    fn test_azimuth_divassign(#[case] y: f64, #[case] x: f64, #[case] rhs: f64, #[case] resy: f64, #[case] resx: f64)
    {
        // Az' = Az * 2
        let mut az = Azimuth::new(y, x);
        az /= rhs;
        assert_approx_eq!(f64, resy, az.y());
        assert_approx_eq!(f64, resx, az.x());
    }

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(3.0, 4.0)]
    fn test_azimuth_one_mul(#[case] y: f64, #[case] x: f64)
    {
        // Az = Az * 1
        let mut az = Azimuth::new(y, x);

        let res = az * 1.0;
        assert_approx_eq!(f64, az.y(), res.y());
        assert_approx_eq!(f64, az.x(), res.x());

        az *= 1.0;
        assert_approx_eq!(f64, az.y(), res.y());
        assert_approx_eq!(f64, az.x(), res.x());
    }

    #[rstest]
    #[case(1.0, 1.0)]
    #[case(3.0, 4.0)]
    fn test_azimuth_one_div(#[case] y: f64, #[case] x: f64)
    {
        // Az = Az / 1
        let mut az = Azimuth::new(y, x);

        let res = az / 1.0;
        assert_approx_eq!(f64, az.y(), res.y());
        assert_approx_eq!(f64, az.x(), res.x());

        az /= 1.0;
        assert_approx_eq!(f64, az.y(), res.y());
        assert_approx_eq!(f64, az.x(), res.x());
    }
}
