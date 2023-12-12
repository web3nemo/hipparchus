use hipparchus_nt::{impl_newtype, op_arithmatic};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Degrees<T>(T);
impl_newtype!(Degrees<T>);
impl_newtype_from!(Degrees<T>: <f32>, <f64>);
op_arithmatic!(Degrees<T> => All);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Radians<T>(T);
impl_newtype!(Radians<T>);
impl_newtype_from!(Radians<T>: <f32>, <f64>);
op_arithmatic!(Radians<T> => All);

impl<T: std::fmt::Debug> std::fmt::Debug for Degrees<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        self.0.fmt(f)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Degrees<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        self.0.fmt(f)
    }
}

pub trait Angle
{
    /// 360°, full circle (τ = 2π)
    const TAU: Self;

    /// 180°, straight angle (π)
    const PI: Self;

    /// 90°, right angle (π/2)
    const HALF_PI: Self;

    /// complementary angle (π/2 - θ)
    fn complementary(&self) -> Self;

    /// supplementary angle (π - θ)
    fn supplementary(&self) -> Self;

    /// coterminal angle (2π - θ)
    fn coterminal(&self) -> Self;
}

#[macro_export]
macro_rules! impl_primitive_angle
{
    ($tt:tt, $ty:ty, $pi:expr, $two:expr) =>
    {
        impl Angle for $tt<$ty>
        {
            const TAU: Self = Self($pi * $two);
            const PI: Self = Self($pi);
            const HALF_PI: Self = Self($pi / $two);

            #[inline]
            fn complementary(&self) -> Self { Self(Self::HALF_PI.unwrap() - self.unwrap()) }

            #[inline]
            fn supplementary(&self) -> Self { Self(Self::PI.unwrap() - self.unwrap()) }

            #[inline]
            fn coterminal(&self) -> Self { Self(Self::TAU.unwrap() - self.unwrap()) }
       }
    }
}

impl_primitive_angle!(Degrees, f64, 180.0, 2.0);
impl_primitive_angle!(Degrees, f32, 180.0, 2.0);

impl_primitive_angle!(Degrees, i128, 180, 2);
impl_primitive_angle!(Degrees, i64, 180, 2);
impl_primitive_angle!(Degrees, i32, 180, 2);
impl_primitive_angle!(Degrees, i16, 180, 2);

impl_primitive_angle!(Degrees, u128, 180, 2);
impl_primitive_angle!(Degrees, u64, 180, 2);
impl_primitive_angle!(Degrees, u32, 180, 2);
impl_primitive_angle!(Degrees, u16, 180, 2);

impl_primitive_angle!(Degrees, isize, 180, 2);
impl_primitive_angle!(Degrees, usize, 180, 2);

impl_primitive_angle!(Radians, f64, std::f64::consts::PI, 2.0);
impl_primitive_angle!(Radians, f32, std::f32::consts::PI, 2.0);
