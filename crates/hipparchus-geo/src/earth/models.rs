use crate::earth::ellipsoid::Ellipsoid;

/// Ellipsoid Model
pub trait Model
{
    /// The equatorial radius (semi-major axis)
    const A:f64;

    /// flattening
    const F_INV:f64;

    /// The 1st flattening of the ellipsoid.
    /// - zero: sphere
    /// - positive: oblate ellipsoid
    /// - negative: prolate ellipsoid
    const F:f64 = 1.0 / Self::F_INV;

    /// The 2nd flattening
    const M:f64 = Self::F / (1.0 - Self::F);

    /// The 3rd flattening
    const N:f64 = Self::F / (2.0 - Self::F);

    /// The polar radius (semi-minor axis)
    const B:f64 = Self::A * (1.0 - Self::F);

    /// The radius ratio: Q = B / A
    const Q:f64 = 1.0 - Self::F;

    /// Meridian radius of curvature
    const C:f64 = Self::A * Self::A / Self::B;

    /// E => E1^2, square of the 1st eccentricity
    const E1SQ:f64 = Self::F * (2.0 - Self::F);

    /// E' => E2^2, square of the 2nd eccentricity
    const E2SQ:f64 = (Self::A * Self::A - Self::B * Self::B) / (Self::B * Self::B);

    /// E" => E3^2, square of the 3rd eccentricity
    const E3SQ:f64 = (Self::A * Self::A - Self::B * Self::B) / (Self::A * Self::A + Self::B * Self::B);

    /// E1, the 1st eccentricity
    fn e1() ->f64
    {
        f64::sqrt(Self::E1SQ)
    }

    /// E2, the 2nd eccentricity
    fn e2() -> f64
    {
        f64::sqrt(Self::E2SQ)
    }
    
    /// E2, the 3rd eccentricity
    fn e3() -> f64
    {
        f64::sqrt(Self::E3SQ)
    }

    fn elps() -> Ellipsoid
    {
        Ellipsoid::new(Self::A, Self::F_INV)
    }

    fn flattening(index: usize) -> f64
    {
        match index
        {
            1 => Self::F,
            2 => Self::M,
            3 => Self::N,
            _ => panic!("flattening index must be 1, 2 or 3"),
        }
    }

    fn eccentricity(index: usize) -> f64
    {
        match index
        {
            1 => Self::e1(),
            2 => Self::e2(),
            3 => Self::e3(),
            _ => panic!("eccentricity index must be 1, 2 or 3"),
        }
    }

    fn eccentricity_square(index: usize) -> f64
    {
        match index
        {
            1 => Self::E1SQ,
            2 => Self::E2SQ,
            3 => Self::E3SQ,
            _ => panic!("eccentricity index must be 1, 2 or 3"),
        }
    }
}

pub struct WGS84 { }
impl Model for WGS84
{
    const A:f64 = 6_378_137.0;
    const F_INV:f64 = 298.257_223_563;
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_wgs84_eccentricity()
    {
        let e1 = WGS84::e1();
        let e2 = WGS84::e2();
        assert_approx_eq!(f64, (WGS84::A + WGS84::B) * (WGS84::A - WGS84::B), e1 * e2 * WGS84::A * WGS84::B, epsilon=1e-2);
    }
}