#![allow(non_snake_case)]
use crate::geodesic::core::Geodesic;
use crate::geodesic::caps::Caps;

/// Place a second point, given the first point, an azimuth, and a distance.
///
/// # Arguments
///   - lat1 - Latitude of 1st point [degrees] [-90.,90.]
///   - lon1 - Longitude of 1st point [degrees] [-180., 180.]
///   - azi1 - Azimuth at 1st point [degrees] [-180., 180.]
///   - s12 - Distance from 1st to 2nd point [meters] Value may be negative
///
/// # Returns
///
/// There are a variety of outputs associated with this calculation. We save computation by
/// only calculating the outputs you need. See the following impls which return different subsets of
/// the following outputs:
///
///  - lat2 latitude of point 2 (degrees).
///  - lon2 longitude of point 2 (degrees).
///  - azi2 (forward) azimuth at point 2 (degrees).
///  - m12 reduced length of geodesic (meters).
///  - M12 geodesic scale of point 2 relative to point 1 (dimensionless).
///  - M21 geodesic scale of point 1 relative to point 2 (dimensionless).
///  - S12 area under the geodesic (meters<sup>2</sup>).
///  - a12 arc length of between point 1 and point 2 (degrees).
///
///  If either point is at a pole, the azimuth is defined by keeping the
///  longitude fixed, writing lat = ±(90° − ε), and taking the limit ε → 0+.
///  An arc length greater that 180° signifies a geodesic which is not a
///  shortest path. (For a prolate ellipsoid, an additional condition is
///  necessary for a shortest path: the longitudinal extent must not
///  exceed of 180°.)
pub trait DirectGeodesic<T> 
{
    fn direct(&self, lat1: f64, lon1: f64, azi1: f64, s12: f64) -> T;
}

impl DirectGeodesic<(f64, f64)> for Geodesic 
{
    /// See the documentation for the DirectGeodesic trait.
    ///
    /// # Returns
    ///  - lat2 latitude of point 2 (degrees).
    ///  - lon2 longitude of point 2 (degrees).
    fn direct(&self, lat1: f64, lon1: f64, azi1: f64, s12: f64) -> (f64, f64) 
    {
        let capabilities = Caps::LATITUDE | Caps::LONGITUDE;
        let (_a12, lat2, lon2, _azi2, _s12, _m12, _M12, _M21, _S12) =
            self._gen_direct(lat1, lon1, azi1, false, s12, capabilities);

        (lat2, lon2)
    }
}

impl DirectGeodesic<(f64, f64, f64)> for Geodesic 
{
    /// See the documentation for the DirectGeodesic trait.
    ///
    /// # Returns
    ///  - lat2 latitude of point 2 (degrees).
    ///  - lon2 longitude of point 2 (degrees).
    ///  - azi2 (forward) azimuth at point 2 (degrees).
    fn direct(&self, lat1: f64, lon1: f64, azi1: f64, s12: f64) -> (f64, f64, f64) 
    {
        let capabilities = Caps::LATITUDE | Caps::LONGITUDE | Caps::AZIMUTH;
        let (_a12, lat2, lon2, azi2, _s12, _m12, _M12, _M21, _S12) =
            self._gen_direct(lat1, lon1, azi1, false, s12, capabilities);

        (lat2, lon2, azi2)
    }
}

impl DirectGeodesic<(f64, f64, f64, f64)> for Geodesic 
{
    /// See the documentation for the DirectGeodesic trait.
    ///
    /// # Returns
    ///  - lat2 latitude of point 2 (degrees).
    ///  - lon2 longitude of point 2 (degrees).
    ///  - azi2 (forward) azimuth at point 2 (degrees).
    ///  - m12 reduced length of geodesic (meters).
    fn direct(&self, lat1: f64, lon1: f64, azi1: f64, s12: f64) -> (f64, f64, f64, f64) 
    {
        let capabilities = Caps::LATITUDE | Caps::LONGITUDE | Caps::AZIMUTH | Caps::REDUCEDLENGTH;
        let (_a12, lat2, lon2, azi2, _s12, m12, _M12, _M21, _S12) =
            self._gen_direct(lat1, lon1, azi1, false, s12, capabilities);

        (lat2, lon2, azi2, m12)
    }
}

impl DirectGeodesic<(f64, f64, f64, f64, f64)> for Geodesic 
{
    /// See the documentation for the DirectGeodesic trait.
    ///
    /// # Returns
    ///  - lat2 latitude of point 2 (degrees).
    ///  - lon2 longitude of point 2 (degrees).
    ///  - azi2 (forward) azimuth at point 2 (degrees).
    ///  - M12 geodesic scale of point 2 relative to point 1 (dimensionless).
    ///  - M21 geodesic scale of point 1 relative to point 2 (dimensionless).
    fn direct(&self, lat1: f64, lon1: f64, azi1: f64, s12: f64) -> (f64, f64, f64, f64, f64) 
    {
        let capabilities = Caps::LATITUDE | Caps::LONGITUDE | Caps::AZIMUTH | Caps::GEODESICSCALE;
        let (_a12, lat2, lon2, azi2, _s12, _m12, M12, M21, _S12) =
            self._gen_direct(lat1, lon1, azi1, false, s12, capabilities);

        (lat2, lon2, azi2, M12, M21)
    }
}

impl DirectGeodesic<(f64, f64, f64, f64, f64, f64)> for Geodesic 
{
    /// See the documentation for the DirectGeodesic trait.
    ///
    /// # Returns
    ///  - lat2 latitude of point 2 (degrees).
    ///  - lon2 longitude of point 2 (degrees).
    ///  - azi2 (forward) azimuth at point 2 (degrees).
    ///  - m12 reduced length of geodesic (meters).
    ///  - M12 geodesic scale of point 2 relative to point 1 (dimensionless).
    ///  - M21 geodesic scale of point 1 relative to point 2 (dimensionless).
    fn direct(&self, lat1: f64, lon1: f64, azi1: f64, s12: f64) -> (f64, f64, f64, f64, f64, f64) 
    {
        let capabilities = Caps::LATITUDE
            | Caps::LONGITUDE
            | Caps::AZIMUTH
            | Caps::REDUCEDLENGTH
            | Caps::GEODESICSCALE;
        let (_a12, lat2, lon2, azi2, _s12, m12, M12, M21, _S12) =
            self._gen_direct(lat1, lon1, azi1, false, s12, capabilities);

        (lat2, lon2, azi2, m12, M12, M21)
    }
}

impl DirectGeodesic<(f64, f64, f64, f64, f64, f64, f64, f64)> for Geodesic 
{
    /// See the documentation for the DirectGeodesic trait.
    ///
    /// # Returns
    ///  - lat2 latitude of point 2 (degrees).
    ///  - lon2 longitude of point 2 (degrees).
    ///  - azi2 (forward) azimuth at point 2 (degrees).
    ///  - m12 reduced length of geodesic (meters).
    ///  - M12 geodesic scale of point 2 relative to point 1 (dimensionless).
    ///  - M21 geodesic scale of point 1 relative to point 2 (dimensionless).
    ///  - S12 area under the geodesic (meters<sup>2</sup>).
    ///  - a12 arc length of between point 1 and point 2 (degrees).
    fn direct(
        &self,
        lat1: f64,
        lon1: f64,
        azi1: f64,
        s12: f64,
    ) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let capabilities = Caps::LATITUDE
            | Caps::LONGITUDE
            | Caps::AZIMUTH
            | Caps::REDUCEDLENGTH
            | Caps::GEODESICSCALE
            | Caps::AREA;
        let (a12, lat2, lon2, azi2, _s12, m12, M12, M21, S12) =
            self._gen_direct(lat1, lon1, azi1, false, s12, capabilities);

        (lat2, lon2, azi2, m12, M12, M21, S12, a12)
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use crate::earth::models::WGS84;
    use crate::geodesic::line::GeodesicLine;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_std_geodesic_geodsolve1() 
    {
        let geod = Geodesic::model::<WGS84>();
        let (lat2, lon2, azi2) = geod.direct(40.63972222, -73.77888889, 53.5, 5850e3);
        assert_approx_eq!(f64, lat2, 49.01467, epsilon = 0.5e-5);
        assert_approx_eq!(f64, lon2, 2.56106, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2, 111.62947, epsilon = 0.5e-5);
    }

    #[test]
    fn test_std_geodesic_geodsolve5() 
    {
        // Check fix for point2=pole bug found 2010-05-03
        let geod = Geodesic::model::<WGS84>();
        let (lat2, lon2, azi2) = geod.direct(0.01777745589997, 30.0, 0.0, 10e6);
        assert_approx_eq!(f64, lat2, 90.0, epsilon = 0.5e-5);
        if lon2 < 0.0 
        {
            assert_approx_eq!(f64, lon2, -150.0, epsilon = 0.5e-5);
            assert_approx_eq!(f64, azi2.abs(), 180.0, epsilon = 0.5e-5);
        }
        else
        {
            assert_approx_eq!(f64, lon2, 30.0, epsilon = 0.5e-5);
            assert_approx_eq!(f64, azi2, 0.0, epsilon = 0.5e-5);
        }
    }
    
    #[test]
    fn test_std_geodesic_geodsolve15() 
    {
        // Initial implementation of Math::eatanhe was wrong for e^2 < 0.  This
        // checks that this is fixed.
        let geod = Geodesic::new(6.4e6, -1f64 / 150.0);
        let (_lat2, _lon2, _azi2, _m12, _M12, _M21, S12, _a12) = geod.direct(1.0, 2.0, 3.0, 4.0);
        assert_approx_eq!(f64, S12, 23700.0, epsilon = 0.5);
    }
    
        
    #[test]
    fn test_std_geodesic_geodsolve17() 
    {
        // Check fix for LONG_UNROLL bug found on 2015-05-07
        let geod = Geodesic::new(6.4e6, -1f64 / 150.0);
        let (_a12, lat2, lon2, azi2, _s12, _m12, _M12, _M21, _S12) = geod._gen_direct(
            40.0,
            -75.0,
            -10.0,
            false,
            2e7,
            Caps::STANDARD | Caps::LONG_UNROLL,
        );
        assert_approx_eq!(f64, lat2, -39.0, epsilon = 1.0);
        assert_approx_eq!(f64, lon2, -254.0, epsilon = 1.0);
        assert_approx_eq!(f64, azi2, -170.0, epsilon = 1.0);

        let line = GeodesicLine::new(&geod, 40.0, -75.0, -10.0, None, None, None);
        let (_a12, lat2, lon2, azi2, _s12, _m12, _M12, _M21, _S12) =
            line._gen_position(false, 2e7, Caps::STANDARD | Caps::LONG_UNROLL);
        assert_approx_eq!(f64, lat2, -39.0, epsilon = 1.0);
        assert_approx_eq!(f64, lon2, -254.0, epsilon = 1.0);
        assert_approx_eq!(f64, azi2, -170.0, epsilon = 1.0);

        let (lat2, lon2, azi2) = geod.direct(40.0, -75.0, -10.0, 2e7);
        assert_approx_eq!(f64, lat2, -39.0, epsilon = 1.0);
        assert_approx_eq!(f64, lon2, 105.0, epsilon = 1.0);
        assert_approx_eq!(f64, azi2, -170.0, epsilon = 1.0);

        let (_a12, lat2, lon2, azi2, _s12, _m12, _M12, _M21, _S12) =
            line._gen_position(false, 2e7, Caps::STANDARD);
        assert_approx_eq!(f64, lat2, -39.0, epsilon = 1.0);
        assert_approx_eq!(f64, lon2, 105.0, epsilon = 1.0);
        assert_approx_eq!(f64, azi2, -170.0, epsilon = 1.0);
    }

    #[test]
    fn test_std_geodesic_geodsolve73() 
    {
        // Check for backwards from the pole bug reported by Anon on 2016-02-13.
        // This only affected the Java implementation.  It was introduced in Java
        // version 1.44 and fixed in 1.46-SNAPSHOT on 2016-01-17.
        // Also the + sign on azi2 is a check on the normalizing of azimuths
        // (converting -0.0 to +0.0).
        let geod = Geodesic::model::<WGS84>();
        let (lat2, lon2, azi2) = geod.direct(90.0, 10.0, 180.0, -1e6);
        assert_approx_eq!(f64, lat2, 81.04623, epsilon = 0.5e-5);
        assert_approx_eq!(f64, lon2, -170.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2, 0.0, epsilon = 0.5e-5);
        assert!(azi2.is_sign_positive());
    }

    #[test]
    fn test_std_geodesic_geodsolve84() 
    {
        // Tests for python implementation to check fix for range errors with
        // {fmod,sin,cos}(inf) (includes GeodSolve84 - GeodSolve91).
        let geod = Geodesic::model::<WGS84>();
        let (lat2, lon2, azi2) = geod.direct(0.0, 0.0, 90.0, f64::INFINITY);
        assert!(lat2.is_nan());
        assert!(lon2.is_nan());
        assert!(azi2.is_nan());
        let (lat2, lon2, azi2) = geod.direct(0.0, 0.0, 90.0, f64::NAN);
        assert!(lat2.is_nan());
        assert!(lon2.is_nan());
        assert!(azi2.is_nan());
        let (lat2, lon2, azi2) = geod.direct(0.0, 0.0, f64::INFINITY, 1000.0);
        assert!(lat2.is_nan());
        assert!(lon2.is_nan());
        assert!(azi2.is_nan());
        let (lat2, lon2, azi2) = geod.direct(0.0, 0.0, f64::NAN, 1000.0);
        assert!(lat2.is_nan());
        assert!(lon2.is_nan());
        assert!(azi2.is_nan());
        let (lat2, lon2, azi2) = geod.direct(0.0, f64::INFINITY, 90.0, 1000.0);
        assert_eq!(lat2, 0.0);
        assert!(lon2.is_nan());
        assert_eq!(azi2, 90.0);
        let (lat2, lon2, azi2) = geod.direct(0.0, f64::NAN, 90.0, 1000.0);
        assert_eq!(lat2, 0.0);
        assert!(lon2.is_nan());
        assert_eq!(azi2, 90.0);
        let (lat2, lon2, azi2) = geod.direct(f64::INFINITY, 0.0, 90.0, 1000.0);
        assert!(lat2.is_nan());
        assert!(lon2.is_nan());
        assert!(azi2.is_nan());
        let (lat2, lon2, azi2) = geod.direct(f64::NAN, 0.0, 90.0, 1000.0);
        assert!(lat2.is_nan());
        assert!(lon2.is_nan());
        assert!(azi2.is_nan());
    }
}