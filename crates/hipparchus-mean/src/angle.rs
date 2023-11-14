use crate::modulo::Modulo;

/// Trait for angle 
pub trait Angle
{
    const M: Self;
    fn norm(self) -> Self;
    fn norm_unbounded(self) -> Self;
    fn norm_symmetry(self) -> Self;
    fn norm_symmetry_unbounded(self) -> Self;
}

impl Angle for f64
{
    const M: Self = 360.0;
    fn norm(self) -> Self { self.umod(Self::M) }
    fn norm_unbounded(self) -> Self { self.umod(-Self::M) }
    fn norm_symmetry(self) -> Self { self.smod(Self::M) }
    fn norm_symmetry_unbounded(self) -> Self { self.smod(-Self::M) }
}

impl Angle for f32
{
    const M: Self = 360.0;
    fn norm(self) -> Self { self.umod(Self::M) }
    fn norm_unbounded(self) -> Self { self.umod(-Self::M) }
    fn norm_symmetry(self) -> Self { self.smod(Self::M) }
    fn norm_symmetry_unbounded(self) -> Self { self.smod(-Self::M) }
}

impl Angle for i128
{
    const M: Self = 360;
    fn norm(self) -> Self { self.umod(Self::M) }
    fn norm_unbounded(self) -> Self { self.umod(-Self::M) }
    fn norm_symmetry(self) -> Self { self.smod(Self::M) }
    fn norm_symmetry_unbounded(self) -> Self { self.smod(-Self::M) }
}

impl Angle for i64
{
    const M: Self = 360;
    fn norm(self) -> Self { self.umod(Self::M) }
    fn norm_unbounded(self) -> Self { self.umod(-Self::M) }
    fn norm_symmetry(self) -> Self { self.smod(Self::M) }
    fn norm_symmetry_unbounded(self) -> Self { self.smod(-Self::M) }
}

impl Angle for i32
{
    const M: Self = 360;
    fn norm(self) -> Self { self.umod(Self::M) }
    fn norm_unbounded(self) -> Self { self.umod(-Self::M) }
    fn norm_symmetry(self) -> Self { self.smod(Self::M) }
    fn norm_symmetry_unbounded(self) -> Self { self.smod(-Self::M) }
}

impl Angle for i16
{
    const M: Self = 360;
    fn norm(self) -> Self { self.umod(Self::M) }
    fn norm_unbounded(self) -> Self { self.umod(-Self::M) }
    fn norm_symmetry(self) -> Self { self.smod(Self::M) }
    fn norm_symmetry_unbounded(self) -> Self { self.smod(-Self::M) }
}

/*
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

}
*/