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
pub trait InverseTrigonometric
{
    type Output;

    /// Returns the arc sine of the angle
    fn asin(&self) -> Self::Output;

    /// Returns the arc cosine of the angle
    fn acos(&self) -> Self::Output;

    /// Returns the arc tangent of the angle, where value is in the range [-PI/2, PI/2] radians
    fn atan(&self) -> Self::Output;

    /// Returns the arc tangent of the angle, where value is in the range [-PI, PI) radians
    fn atan2(y:(&Self, &Self)) -> Self::Output;
}

impl<T> InverseTrigonometric for T where T: Fp
{
    type Output = T;

    fn asin(&self) -> Self::Output
    {
        T::asin(*self)
    }
    
    fn acos(&self) -> Self::Output
    {
        T::acos(*self)
    }

    fn atan(&self) -> Self::Output
    {
        T::atan(*self)
    }
    
    fn atan2(y:(&Self, &Self)) -> Self::Output
    {
        T::atan2(*y.0, *y.1)
    }
}

pub trait Hyperbolic<T>
{
    fn sinh(self) -> T;
    fn cosh(self) -> T;
    fn tanh(self) -> T;
}

pub trait InverseHyperbolic<T>
{
    fn asinh(self) -> T;
    fn acosh(self) -> T;
    fn atanh(self) -> T;
}