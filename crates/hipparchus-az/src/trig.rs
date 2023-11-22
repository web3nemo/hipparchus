use hipparchus_mean::Fp;

/// Trait for trigonometric functions
pub trait Trigonometric<T> where T: Fp
{
    /// Returns the sine of the angle
    fn sin(&self) -> T;

    /// Returns the cosine of the angle
    fn cos(&self) -> T;

    /// Returns the tangent of the angle
    fn tan(&self) -> T;

    /// Returns the sine & cosine of the angle
    fn sincos(&self) -> (T, T);

    /// Returns the cotangent of the angle
    #[inline]
    fn cot(&self) -> T { self.tan().recip() }
    
    /// Return the secant of the angle
    #[inline]
    fn sec(&self) -> T { self.sin().recip() }

    /// Return the cosecant of the angle
    #[inline]
    fn csc(&self) -> T { self.cos().recip() }

    /// Return the versine of the angle
    #[inline]
    fn vsin(&self) -> T { T::one() - self.cos() }

    /// Return the vercosine of the angle
    #[inline]
    fn vcos(&self) -> T { T::one() - self.sin() }
}

impl<T> Trigonometric<T> for T where T: Fp
{
    fn sin(&self) -> Self
    {
        T::sin(*self)
    }
    
    fn cos(&self) -> Self
    {
        T::cos(*self)
    }
    
    fn tan(&self) -> Self
    {
        T::tan(*self)
    }
    
    fn sincos(&self) -> (Self, Self)
    {
        T::sin_cos(*self)
    }
}

/// Trait for inverse trigonometric functions
pub trait InverseTrigonometric<T> where T: Fp, Self: Sized
{
    /// Returns the arc sine of the angle
    fn asin(v:T) -> Self;

    /// Returns the arc cosine of the angle
    fn acos(v:T) -> Self;

    /// Returns the arc tangent of the angle, where value is in the range [-PI/2, PI/2] radians
    fn atan(v:T) -> Self;

    /// Returns the arc tangent of the angle, where value is in the range [-PI, PI) radians
    fn atan2(y:T, x:T) -> Self;

    /// Returns the cotangent of the angle
    fn acot(v:T) -> Self { Self::atan(v.recip()) }
    
    /// Returns the cotangent of the angle
    #[inline]
    fn acot2(y:T, x:T) -> Self { Self::atan2(x, y) }
    
    /// Return the secant of the angle
    #[inline]
    fn asec(v:T) -> Self { Self::asin(v.recip()) }

    /// Return the cosecant of the angle
    #[inline]
    fn acsc(v:T) -> Self { Self::acos(v.recip()) }

    /// Return the versine of the angle
    #[inline]
    fn avsin(v:T) -> Self { Self::acos(T::one() - v) }

    /// Return the vercosine of the angle
    #[inline]
    fn avcos(v:T) -> Self { Self::asin(T::one() - v) }
}

impl<T> InverseTrigonometric<T> for T where T: Fp
{
    fn asin(v:T) -> Self { v.asin() }
    
    fn acos(v:T) -> Self { v.acos() }

    fn atan(v:T) -> Self { v.atan() }
    
    fn atan2(y:T, x:T) -> Self { T::atan2(y, x) }
}

/// Trait for hyperbolic functions
pub trait Hyperbolic<T>
{
    fn sinh(self) -> T;
    fn cosh(self) -> T;
    fn tanh(self) -> T;
}

/// Trait for inverse hyperbolic functions
pub trait InverseHyperbolic<T>
{
    fn asinh(self) -> T;
    fn acosh(self) -> T;
    fn atanh(self) -> T;
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(1.0, std::f64::consts::FRAC_PI_4)]
    fn test_trig_tan(#[case] v:f64, #[case] r: f64)
    {
        let atan = v.atan();
        assert_approx_eq!(f64, r, atan);
        let tan = r.tan();
        assert_approx_eq!(f64, v, tan);
        let cot = r.cot();
        assert_approx_eq!(f64, 1.0, tan * cot);
    }

    #[rstest]
    #[case(std::f64::consts::PI, 0.0, 1.0)]
    fn test_trig_sincos(#[case] r: f64, #[case] s:f64, #[case] c:f64)
    {
        let sin = r.sin();
        let cos = r.cos();
        assert_approx_eq!(f64, s, sin);
        assert_approx_eq!(f64, c, cos);
        let (s, c) = r.sincos();
        assert_approx_eq!(f64, s, sin);
        assert_approx_eq!(f64, c, cos);
    }

    #[rstest]
    #[case(-1.0, -std::f64::consts::FRAC_PI_2)]
    fn test_trig_asin(#[case] v: f64, #[case] r: f64)
    {
        assert_approx_eq!(f64, r, v.asin());
        let sin = r.sin();
        assert_approx_eq!(f64, v, sin);
        let sec = r.sec();
        assert_approx_eq!(f64, 1.0, sec * sin);
        let vcos = r.vcos();
        assert_approx_eq!(f64, 1.0, sin + vcos);
    }

    #[rstest]
    #[case( -1.0, std::f64::consts::PI)]
    fn test_trig_cos(#[case] v: f64, #[case] r: f64)
    {
        assert_approx_eq!(f64, r, v.acos());
        let cos = r.cos();
        assert_approx_eq!(f64, v, cos);
        let csc = r.csc();
        assert_approx_eq!(f64, 1.0, cos * csc);
        let vsin = r.vsin();
        assert_approx_eq!(f64, 1.0, cos + vsin);
    }

    #[rstest]
    #[case(1.0, 1.0, std::f64::consts::FRAC_PI_4)]
    fn test_trig_atan2(#[case] y:f64, #[case] x:f64, #[case] r: f64)
    {
        let atan2 = f64::atan2(y, x);
        assert_approx_eq!(f64, r, atan2);
        let tan = r.tan();
        assert_approx_eq!(f64, y/x, tan);
        let cot = r.cot();
        assert_approx_eq!(f64, 1.0, tan * cot);
    }
}
