use crate::geodesic::core::Geodesic;
use crate::geodesic::caps::Caps;

/// Measure the distance (and other values) between two points.
///
/// # Arguments
/// - lat1 latitude of point 1 (degrees).
/// - lon1 longitude of point 1 (degrees).
/// - lat2 latitude of point 2 (degrees).
/// - lon2 longitude of point 2 (degrees).
///
/// # Returns
///
/// There are a variety of outputs associated with this calculation. We save computation by
/// only calculating the outputs you need. See the following impls which return different subsets of
/// the following outputs:
///
/// - s12 distance between point 1 and point 2 (meters).
/// - azi1 azimuth at point 1 (degrees).
/// - azi2 (forward) azimuth at point 2 (degrees).
/// - m12 reduced length of geodesic (meters).
/// - M12 geodesic scale of point 2 relative to point 1 (dimensionless).
/// - M21 geodesic scale of point 1 relative to point 2 (dimensionless).
/// - S12 area under the geodesic (meters<sup>2</sup>).
/// - a12 arc length of between point 1 and point 2 (degrees).
///
///  `lat1` and `lat2` should be in the range [&minus;90&deg;, 90&deg;].
///  The values of `azi1` and `azi2` returned are in the range
///  [&minus;180&deg;, 180&deg;].
///
/// If either point is at a pole, the azimuth is defined by keeping the
/// longitude fixed, writing `lat` = &plusmn;(90&deg; &minus; &epsilon;),
/// and taking the limit &epsilon; &rarr; 0+.
///
/// The solution to the inverse problem is found using Newton's method.  If
/// this fails to converge (this is very unlikely in geodetic applications
/// but does occur for very eccentric ellipsoids), then the bisection method
/// is used to refine the solution.
pub trait InverseGeodesic<T> 
{
    fn inverse(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> T;
}

impl InverseGeodesic<f64> for Geodesic 
{
    /// See the documentation for the InverseGeodesic trait.
    ///
    /// # Returns
    /// - s12 distance between point 1 and point 2 (meters).
    fn inverse(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 
    {
        let capabilities = Caps::DISTANCE;
        let (_a12, s12, _azi1, _azi2, _m12, _M12, _M21, _S12) =
            self._gen_inverse_azi(lat1, lon1, lat2, lon2, capabilities);

        s12
    }
}

impl InverseGeodesic<(f64, f64)> for Geodesic 
{
    /// See the documentation for the InverseGeodesic trait.
    ///
    /// # Returns
    /// - s12 distance between point 1 and point 2 (meters).
    /// - a12 arc length of between point 1 and point 2 (degrees).
    fn inverse(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> (f64, f64) 
    {
        let capabilities = Caps::DISTANCE;
        let (a12, s12, _azi1, _azi2, _m12, _M12, _M21, _S12) =
            self._gen_inverse_azi(lat1, lon1, lat2, lon2, capabilities);

        (s12, a12)
    }
}

impl InverseGeodesic<(f64, f64, f64)> for Geodesic 
{
    /// See the documentation for the InverseGeodesic trait.
    ///
    /// # Returns
    /// - azi1 azimuth at point 1 (degrees).
    /// - azi2 (forward) azimuth at point 2 (degrees).
    /// - a12 arc length of between point 1 and point 2 (degrees).
    fn inverse(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> (f64, f64, f64) 
    {
        let capabilities = Caps::AZIMUTH;
        let (a12, _s12, azi1, azi2, _m12, _M12, _M21, _S12) =
            self._gen_inverse_azi(lat1, lon1, lat2, lon2, capabilities);

        (azi1, azi2, a12)
    }
}

impl InverseGeodesic<(f64, f64, f64, f64)> for Geodesic 
{
    /// See the documentation for the InverseGeodesic trait.
    ///
    /// # Returns
    /// - s12 distance between point 1 and point 2 (meters).
    /// - azi1 azimuth at point 1 (degrees).
    /// - azi2 (forward) azimuth at point 2 (degrees).
    /// - a12 arc length of between point 1 and point 2 (degrees).
    fn inverse(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> (f64, f64, f64, f64) {
        let capabilities = Caps::DISTANCE | Caps::AZIMUTH;
        let (a12, s12, azi1, azi2, _m12, _M12, _M21, _S12) =
            self._gen_inverse_azi(lat1, lon1, lat2, lon2, capabilities);

        (s12, azi1, azi2, a12)
    }
}

impl InverseGeodesic<(f64, f64, f64, f64, f64)> for Geodesic 
{
    /// See the documentation for the InverseGeodesic trait.
    ///
    /// # Returns
    /// - s12 distance between point 1 and point 2 (meters).
    /// - azi1 azimuth at point 1 (degrees).
    /// - azi2 (forward) azimuth at point 2 (degrees).
    /// - m12 reduced length of geodesic (meters).
    /// - a12 arc length of between point 1 and point 2 (degrees).
    fn inverse(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> (f64, f64, f64, f64, f64) {
        let capabilities = Caps::DISTANCE | Caps::AZIMUTH | Caps::REDUCEDLENGTH;
        let (a12, s12, azi1, azi2, m12, _M12, _M21, _S12) =
            self._gen_inverse_azi(lat1, lon1, lat2, lon2, capabilities);

        (s12, azi1, azi2, m12, a12)
    }
}

impl InverseGeodesic<(f64, f64, f64, f64, f64, f64)> for Geodesic 
{
    /// See the documentation for the InverseGeodesic trait.
    ///
    /// # Returns
    /// - s12 distance between point 1 and point 2 (meters).
    /// - azi1 azimuth at point 1 (degrees).
    /// - azi2 (forward) azimuth at point 2 (degrees).
    /// - M12 geodesic scale of point 2 relative to point 1 (dimensionless).
    /// - M21 geodesic scale of point 1 relative to point 2 (dimensionless).
    /// - a12 arc length of between point 1 and point 2 (degrees).
    fn inverse(
        &self,
        lat1: f64,
        lon1: f64,
        lat2: f64,
        lon2: f64,
    ) -> (f64, f64, f64, f64, f64, f64) {
        let capabilities = Caps::DISTANCE | Caps::AZIMUTH | Caps::GEODESICSCALE;
        let (a12, s12, azi1, azi2, _m12, M12, M21, _S12) =
            self._gen_inverse_azi(lat1, lon1, lat2, lon2, capabilities);

        (s12, azi1, azi2, M12, M21, a12)
    }
}

impl InverseGeodesic<(f64, f64, f64, f64, f64, f64, f64)> for Geodesic 
{
    /// See the documentation for the InverseGeodesic trait.
    ///
    /// # Returns
    /// - s12 distance between point 1 and point 2 (meters).
    /// - azi1 azimuth at point 1 (degrees).
    /// - azi2 (forward) azimuth at point 2 (degrees).
    /// - m12 reduced length of geodesic (meters).
    /// - M12 geodesic scale of point 2 relative to point 1 (dimensionless).
    /// - M21 geodesic scale of point 1 relative to point 2 (dimensionless).
    /// - a12 arc length of between point 1 and point 2 (degrees).
    fn inverse(
        &self,
        lat1: f64,
        lon1: f64,
        lat2: f64,
        lon2: f64,
    ) -> (f64, f64, f64, f64, f64, f64, f64) 
    {
        let capabilities =
            Caps::DISTANCE | Caps::AZIMUTH | Caps::REDUCEDLENGTH | Caps::GEODESICSCALE;
        let (a12, s12, azi1, azi2, m12, M12, M21, _S12) =
            self._gen_inverse_azi(lat1, lon1, lat2, lon2, capabilities);

        (s12, azi1, azi2, m12, M12, M21, a12)
    }
}

impl InverseGeodesic<(f64, f64, f64, f64, f64, f64, f64, f64)> for Geodesic 
{
    /// See the documentation for the InverseGeodesic trait.
    ///
    /// # Returns
    /// - s12 distance between point 1 and point 2 (meters).
    /// - azi1 azimuth at point 1 (degrees).
    /// - azi2 (forward) azimuth at point 2 (degrees).
    /// - m12 reduced length of geodesic (meters).
    /// - M12 geodesic scale of point 2 relative to point 1 (dimensionless).
    /// - M21 geodesic scale of point 1 relative to point 2 (dimensionless).
    /// - S12 area under the geodesic (meters<sup>2</sup>).
    /// - a12 arc length of between point 1 and point 2 (degrees).
    fn inverse(
        &self,
        lat1: f64,
        lon1: f64,
        lat2: f64,
        lon2: f64,
    ) -> (f64, f64, f64, f64, f64, f64, f64, f64) 
    {
        let capabilities =
            Caps::DISTANCE | Caps::AZIMUTH | Caps::REDUCEDLENGTH | Caps::GEODESICSCALE | Caps::AREA;
        let (a12, s12, azi1, azi2, m12, M12, M21, S12) =
            self._gen_inverse_azi(lat1, lon1, lat2, lon2, capabilities);

        (s12, azi1, azi2, m12, M12, M21, S12, a12)
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use crate::earth::models::WGS84;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_std_geodesic_geodsolve0() 
    {
        let geod = Geodesic::model::<WGS84>();
        let (s12, azi1, azi2, _a12) = geod.inverse(40.6, -73.8, 49.01666667, 2.55);
        assert_approx_eq!(f64, azi1, 53.47022, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2, 111.59367, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 5853226.0, epsilon = 0.5);
    }

    #[test]
    fn test_std_geodesic_geodsolve2() 
    {
        // Check fix for antipodal prolate bug found 2010-09-04
        let geod = Geodesic::new(6.4e6, -1f64 / 150.0);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.07476, 0.0, -0.07476, 180.0);
        assert_approx_eq!(f64, azi1, 90.00078, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2, 90.00078, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 20106193.0, epsilon = 0.5);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.1, 0.0, -0.1, 180.0);
        assert_approx_eq!(f64, azi1, 90.00105, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2, 90.00105, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 20106193.0, epsilon = 0.5);
    }

    #[test]
    fn test_std_geodesic_geodsolve4() 
    {
        // Check fix for short line bug found 2010-05-21
        let geod = Geodesic::model::<WGS84>();
        let s12: f64 = geod.inverse(36.493349428792, 0.0, 36.49334942879201, 0.0000008);
        assert_approx_eq!(f64, s12, 0.072, epsilon = 0.5e-3);
    }    #[test]
    fn test_std_geodesic_geodsolve6() 
    {
        // Check fix for volatile sbet12a bug found 2011-06-25 (gcc 4.4.4
        // x86 -O3).  Found again on 2012-03-27 with tdm-mingw32 (g++ 4.6.1).
        let geod = Geodesic::model::<WGS84>();
        let s12: f64 = geod.inverse(
            88.202499451857,
            0.0,
            -88.202499451857,
            179.981022032992859592,
        );
        assert_approx_eq!(f64, s12, 20003898.214, epsilon = 0.5e-3);
        let s12: f64 = geod.inverse(
            89.333123580033,
            0.0,
            -89.333123580032997687,
            179.99295812360148422,
        );
        assert_approx_eq!(f64, s12, 20003926.881, epsilon = 0.5e-3);
    }

    #[test]
    fn test_std_geodesic_geodsolve9() 
    {
        // Check fix for volatile x bug found 2011-06-25 (gcc 4.4.4 x86 -O3)
        let geod = Geodesic::model::<WGS84>();
        let s12: f64 = geod.inverse(
            56.320923501171,
            0.0,
            -56.320923501171,
            179.664747671772880215,
        );
        assert_approx_eq!(f64, s12, 19993558.287, epsilon = 0.5e-3);
    }

    #[test]
    fn test_std_geodesic_geodsolve10() 
    {
        // Check fix for adjust tol1_ bug found 2011-06-25 (Visual Studio
        // 10 rel + debug)
        let geod = Geodesic::model::<WGS84>();
        let s12: f64 = geod.inverse(
            52.784459512564,
            0.0,
            -52.784459512563990912,
            179.634407464943777557,
        );
        assert_approx_eq!(f64, s12, 19991596.095, epsilon = 0.5e-3);
    }

    #[test]
    fn test_std_geodesic_geodsolve11() 
    {
        // Check fix for bet2 = -bet1 bug found 2011-06-25 (Visual Studio
        // 10 rel + debug)
        let geod = Geodesic::model::<WGS84>();
        let s12: f64 = geod.inverse(
            48.522876735459,
            0.0,
            -48.52287673545898293,
            179.599720456223079643,
        );
        assert_approx_eq!(f64, s12, 19989144.774, epsilon = 0.5e-3);
    }

    #[test]
    fn test_std_geodesic_geodsolve12() 
    {
        // Check fix for inverse geodesics on extreme prolate/oblate
        // ellipsoids Reported 2012-08-29 Stefan Guenther
        // <stefan.gunther@embl.de>; fixed 2012-10-07
        let geod = Geodesic::new(89.8, -1.83);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, -10.0, 160.0);
        assert_approx_eq!(f64, azi1, 120.27, epsilon = 1e-2);
        assert_approx_eq!(f64, azi2, 105.15, epsilon = 1e-2);
        assert_approx_eq!(f64, s12, 266.7, epsilon = 1e-1);
    }

    #[test]
    fn test_std_geodesic_geodsolve14() 
    {
        // Check fix for inverse ignoring lon12 = nan
        let geod = Geodesic::model::<WGS84>();
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 1.0, f64::NAN);
        assert!(azi1.is_nan());
        assert!(azi2.is_nan());
        assert!(s12.is_nan());
    }

    #[test]
    fn test_std_geodesic_geodsolve33() 
    {
        // Check max(-0.0,+0.0) issues 2015-08-22 (triggered by bugs in Octave --
        // sind(-0.0) = +0.0 -- and in some version of Visual Studio --
        // fmod(-0.0, 360.0) = +0.0.
        let geod = Geodesic::model::<WGS84>();
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 0.0, 179.0);
        assert_approx_eq!(f64, azi1, 90.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2, 90.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 19926189.0, epsilon = 0.5);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 0.0, 179.5);
        assert_approx_eq!(f64, azi1, 55.96650, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2, 124.03350, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 19980862.0, epsilon = 0.5);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 0.0, 180.0);
        assert_approx_eq!(f64, azi1, 0.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2.abs(), 180.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 20003931.0, epsilon = 0.5);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 1.0, 180.0);
        assert_approx_eq!(f64, azi1, 0.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2.abs(), 180.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 19893357.0, epsilon = 0.5);

        let geod = Geodesic::new(6.4e6, 0.0);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 0.0, 179.0);
        assert_approx_eq!(f64, azi1, 90.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2, 90.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 19994492.0, epsilon = 0.5);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 0.0, 180.0);
        assert_approx_eq!(f64, azi1, 0.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2.abs(), 180.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 20106193.0, epsilon = 0.5);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 1.0, 180.0);
        assert_approx_eq!(f64, azi1, 0.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2.abs(), 180.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 19994492.0, epsilon = 0.5);

        let geod = Geodesic::new(6.4e6, -1.0 / 300.0);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 0.0, 179.0);
        assert_approx_eq!(f64, azi1, 90.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2, 90.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 19994492.0, epsilon = 0.5);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 0.0, 180.0);
        assert_approx_eq!(f64, azi1, 90.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2, 90.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 20106193.0, epsilon = 0.5);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 0.5, 180.0);
        assert_approx_eq!(f64, azi1, 33.02493, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2, 146.97364, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 20082617.0, epsilon = 0.5);
        let (s12, azi1, azi2, _a12) = geod.inverse(0.0, 0.0, 1.0, 180.0);
        assert_approx_eq!(f64, azi1, 0.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, azi2.abs(), 180.0, epsilon = 0.5e-5);
        assert_approx_eq!(f64, s12, 20027270.0, epsilon = 0.5);
    }

    #[test]
    fn test_std_geodesic_geodsolve55() 
    {
        // Check fix for nan + point on equator or pole not returning all nans in
        // Geodesic::Inverse, found 2015-09-23.
        let geod = Geodesic::model::<WGS84>();
        let (s12, azi1, azi2, _a12) = geod.inverse(f64::NAN, 0.0, 0.0, 90.0);
        assert!(azi1.is_nan());
        assert!(azi2.is_nan());
        assert!(s12.is_nan());
        let (s12, azi1, azi2, _a12) = geod.inverse(f64::NAN, 0.0, 90.0, 3.0);
        assert!(azi1.is_nan());
        assert!(azi2.is_nan());
        assert!(s12.is_nan());
    }

    #[test]
    fn test_std_geodesic_geodsolve59() 
    {
        // Check for points close with longitudes close to 180 deg apart.
        let geod = Geodesic::model::<WGS84>();
        let (s12, azi1, azi2, _a12) = geod.inverse(5.0, 0.00000000000001, 10.0, 180.0);
        assert_approx_eq!(f64, azi1, 0.000000000000035, epsilon = 1.5e-14);
        assert_approx_eq!(f64, azi2, 179.99999999999996, epsilon = 1.5e-14);
        assert_approx_eq!(f64, s12, 18345191.174332713, epsilon = 5e-9);
    }

    #[test]
    fn test_std_geodesic_geodsolve76() {
        // The distance from Wellington and Salamanca (a classic failure of
        // Vincenty)
        let geod = Geodesic::model::<WGS84>();
        let (s12, azi1, azi2, _a12) = geod.inverse(
            -(41.0 + 19.0 / 60.0),
            174.0 + 49.0 / 60.0,
            40.0 + 58.0 / 60.0,
            -(5.0 + 30.0 / 60.0),
        );
        assert_approx_eq!(f64, azi1, 160.39137649664, epsilon = 0.5e-11);
        assert_approx_eq!(f64, azi2, 19.50042925176, epsilon = 0.5e-11);
        assert_approx_eq!(f64, s12, 19960543.857179, epsilon = 0.5e-6);
    }

    #[test]
    fn test_std_geodesic_geodsolve78() {
        // An example where the NGS calculator fails to converge
        let geod = Geodesic::model::<WGS84>();
        let (s12, azi1, azi2, _a12) = geod.inverse(27.2, 0.0, -27.1, 179.5);
        assert_approx_eq!(f64, azi1, 45.82468716758, epsilon = 0.5e-11);
        assert_approx_eq!(f64, azi2, 134.22776532670, epsilon = 0.5e-11);
        assert_approx_eq!(f64, s12, 19974354.765767, epsilon = 0.5e-6);
    }
}
