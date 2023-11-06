use crate::metrics::Metrics;
use hipparchus_geo::{LatLon, WGS84, Ellipsoid};

#[repr(i32)]
#[derive(Clone,PartialEq,Debug)]
pub enum EarthDistance
{
    /// Haversine distance
    Haversine = 1,

    /// Vincenty distance
    Vincenty(f64) = 2,
}

impl Metrics<&LatLon, f64> for EarthDistance
{
    fn measure(self, c1:&LatLon, c2:&LatLon) -> f64
    {
        match self
        {
            EarthDistance::Haversine => WGS84::haversine(c1, c2),
            EarthDistance::Vincenty(p) => WGS84::vincenty(c1, c2, p),
        }
    }
}

// TODO: Find a better data-driven test framework for lat/lon testing
/* 
#[cfg(test)]
mod tests 
{
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_haversine()
    {
        let x = LatLon { latitude: 39.904690, longitude: 116.407170 };
        let y = LatLon { latitude: 31.230370, longitude: 121.473700 };
        let expected =  1067436.5235444377;
        assert_approx_eq!(f64, expected, EarthDistance::Haversine.measure(&x, &y));
        assert_approx_eq!(f64, expected, EarthDistance::Haversine.measure(&y, &x));
    }

    #[test] 
    fn test_haversine_semicircle()
    {
        let x = LatLon { latitude: 90.0, longitude: 0.0 };
        let y = LatLon { latitude: -90.0, longitude: 0.0 };
        // let expected = PI * EarthDistance::R; 1065905.3363660865
        let expected = 1065906.3363660865;
        assert_approx_eq!(f64, expected, EarthDistance::Haversine.measure(&x, &y));
        assert_approx_eq!(f64, expected, EarthDistance::Haversine.measure(&y, &x));
    }

    #[test]
    fn test_vincenty()
    {
        let x = LatLon { latitude: 39.904690, longitude: 116.407170 };
        let y = LatLon { latitude: 31.230370, longitude: 121.473700 };
        let expected =  1065905.3363660865;
        assert_approx_eq!(f64, expected, EarthDistance::Vincenty(1e-6).measure(&x, &y));
        assert_approx_eq!(f64, expected, EarthDistance::Vincenty(1e-6).measure(&y, &x));
    }

    #[test]
    fn test_distance_overlap()
    {
        let x = LatLon { latitude: 35.0, longitude: 37.0 };
        assert_approx_eq!(f64, 0.0, EarthDistance::Haversine.measure(&x, &x));
        assert_approx_eq!(f64, 0.0, EarthDistance::Vincenty(1e-6).measure(&x, &x));
    }
}
 */