// TODO: Refactor to move into hipparchus-geo crate
use crate::metrics::Metrics;
use hipparchus_geo::{LatLon, WGS84, Metrics as GeoMetrics};

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

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(39.904690, 116.407170, 31.230370, 121.473700, 1067370.0, 1e-6)]
    fn test_haversine
    (
        #[case] lat1: f64, #[case] lon1: f64, 
        #[case] lat2: f64, #[case] lon2: f64, 
        #[case] expected: f64, #[case] epsilon :f64
    )
    {
        let x = LatLon::new(lat1, lon1);
        let y = LatLon::new(lat2, lon2);
        assert_approx_eq!(f64, expected, EarthDistance::Haversine.measure(&x, &y), epsilon=expected * epsilon);
        assert_approx_eq!(f64, expected, EarthDistance::Haversine.measure(&y, &x), epsilon=expected * epsilon);
    }

    #[rstest]
    #[case(39.904690, 116.407170, 31.230370, 121.473700, 1065906.0, 1e-6)]
    fn test_vincenty
    (
        #[case] lat1: f64, #[case] lon1: f64, 
        #[case] lat2: f64, #[case] lon2: f64, 
        #[case] expected: f64, #[case] epsilon :f64
    )
    {
        let x = LatLon::new(lat1, lon1);
        let y = LatLon::new(lat2, lon2);
        assert_approx_eq!(f64, expected, EarthDistance::Vincenty(1e-6).measure(&x, &y), epsilon=expected * epsilon);
        assert_approx_eq!(f64, expected, EarthDistance::Vincenty(1e-6).measure(&y, &x), epsilon=expected * epsilon);
    }
}
