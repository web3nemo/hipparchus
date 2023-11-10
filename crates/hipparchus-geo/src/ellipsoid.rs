use std::f64::consts::PI;
use hipparchus_mean::LpNorm;

use crate::LatLon;

pub enum EarthRadius
{
    /// Traditional fixed value for earth radius
    Default = 0, 

    /// equatorial radius
    Equatorial,

    /// polar radius
    Polar,

    /// arithmetic mean of length of radius of the earth
    ArithmeticMean,

    /// radius of the sphere of equal volume of the earth
    VolumeMean,

    /// radius of the sphere of equal surface area of the earth
    SurfaceAreaMean,
}

pub enum EarthVolume
{
    /// Traditional fixed value for earth volume
    Default = 0, 

    /// volume of the sphere with default earth radius 
    Sphere = 1,

    /// volume of the earth ellipsoid (more accurate)
    Ellipsoid = 2,
}

pub enum EarthSurfaceArea
{
    Default = 0,

    /// surface area of the sphere with default earth radius   
    Sphere = 1,

    /// surface area of the earth spheroid (with accurate formula for prolate/oblate spheroid)
    Spheriod = 2,

    /// Estimated surface area of the earth ellipsoid (e<1.061%)
    Thomsen = 3,

    /// Estimated surface area of the earth ellipsoid (e<1.178%)
    Cantrell = 4,
}

/// Ellipsoid
pub trait Ellipsoid
{
    /// semi-major axis: equatorial radius
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
    const N:f64 = (Self::A - Self::B) / (Self::A + Self::B);

    /// semi-minor axis: or polar radius
    const B:f64 = Self::A * (1.0 - Self::F);

    /// TODO
    const C:f64 = Self::A * Self::A / Self::B;

    /// E => E1^2, square of the 1st eccentricity
    const E1_SQUARE:f64 = (Self::A * Self::A - Self::B * Self::B) / (Self::A * Self::A);

    /// E' => E2^2, square of the 2nd eccentricity
    const E2_SQUARE:f64 = (Self::A * Self::A - Self::B * Self::B) / (Self::B * Self::B);

    /// E" => E3^2, square of the 3rd eccentricity
    const E3_SQUARE:f64 = (Self::A * Self::A - Self::B * Self::B) / (Self::A * Self::A + Self::B * Self::B);

    /// E1, the 1st eccentricity
    fn e1() ->f64
    {
        f64::sqrt(Self::E1_SQUARE)
    }

    /// E2, the 2nd eccentricity
    fn e2() -> f64
    {
        f64::sqrt(Self::E2_SQUARE)
    }
    
    /// E2, the 3rd eccentricity
    fn e3() -> f64
    {
        f64::sqrt(Self::E3_SQUARE)
    }
    
    /// average radius: radius of the sphere of equal volume
    fn radius(r:EarthRadius) -> f64
    {
        match r
        {
            EarthRadius::Default => 6_371_000.0,
            EarthRadius::Equatorial => Self::A,
            EarthRadius::Polar => Self::B,
            EarthRadius::ArithmeticMean => (Self::A * 2.0 + Self::B) / 3.0,
            EarthRadius::VolumeMean => f64::powf(Self::A * Self::A * Self::B, 1.0/3.0),
            EarthRadius::SurfaceAreaMean => f64::sqrt(Self::surface_area(EarthSurfaceArea::Spheriod) * 0.25 / PI),
        }
    }

    /// volume of the earth ellipsoid
    fn volume(v:EarthVolume) -> f64
    {
        match v
        {
            EarthVolume::Default => 1.08321e21,
            EarthVolume::Sphere => Self::radius(EarthRadius::Default).powi(3) * PI / 0.75,
            EarthVolume::Ellipsoid => Self::A * Self::A * Self::B * PI / 0.75,
        }
    }

    /// surface area of the earth ellipsoid 
    fn surface_area(sa:EarthSurfaceArea) -> f64
    {
        match sa
        {
            EarthSurfaceArea::Default => 5.10072e14,
            EarthSurfaceArea::Sphere => Self::radius(EarthRadius::Default).powi(2) * PI * 4.0,
            EarthSurfaceArea::Spheriod =>
            {
                let e = Self::e1();
                let k = 1.0 + (1.0 - e * e) / e * f64::atanh(e);
                k * Self::A * Self::A * PI * 2.0
            },
            EarthSurfaceArea::Cantrell =>
            {
                [
                    Self::A * Self::B,
                    Self::A * Self::A,
                    Self::B * Self::A
                ].iter()
                .lpnorm(1.6).unwrap()
                / f64::powf(3.0, 1.0/1.6)
                * PI * 4.0 
            },
            EarthSurfaceArea::Thomsen =>
            {
                [
                    Self::A * Self::B,
                    Self::A * Self::A,
                    Self::B * Self::A
                ].iter()
                .lpnorm(1.6075).unwrap()
                / f64::powf(3.0, 1.0/1.6075)
                * PI * 4.0 
            },
        }
    }

    /// Haversine distance of 2 geodetic points
    fn haversine(l1: &LatLon, l2: &LatLon) -> f64
    {
        let lat1 = l1.latitude().to_radians();
        let lon1 = l1.longitude().to_radians();
        let lat2 = l2.latitude().to_radians();
        let lon2 = l2.longitude().to_radians();

        let hdx = (lon2 - lon1) / 2.0;
        let hdy = (lat2 - lat1) / 2.0;
        let ratio = (hdy.sin().powi(2) + hdx.sin().powi(2)*lat2.cos()*lat1.cos()).sqrt().asin() * 2.0;
        ratio * Self::radius(EarthRadius::Default)
    }

    /// Vincenty distance of 2 geodetic points with a specific precision
    fn vincenty(l1: &LatLon, l2: &LatLon, p: f64) -> f64
    {
        let lat1 = l1.latitude().to_radians();
        let lon1 = l1.longitude().to_radians();
        let lat2 = l2.latitude().to_radians();
        let lon2 = l2.longitude().to_radians();

        let u1 = ((1.0 - Self::F) * lat1.tan()).atan();
        let u2 = ((1.0 - Self::F) * lat2.tan()).atan();
        let sin_u1 = u1.sin();
        let cos_u1 = u1.cos();
        let sin_u2 = u2.sin();
        let cos_u2 = u2.cos();
    
        let l = lon1 - lon2;
        let mut lambda = l;
        loop
        {
            let sin_lambda = lambda.sin();
            let cos_lambda = lambda.cos();
            let sin_sigma = ((cos_u2*sin_lambda).powi(2) + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda).powi(2)).sqrt();
            if sin_sigma == 0.0
            {
                return 0.0;
            }
            let cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
            let sigma = f64::atan2(sin_sigma, cos_sigma);
    
            let alpha = (cos_u1 * cos_u2 * sin_lambda / sin_sigma).asin();
            let cos_square_alpha = alpha.cos() * alpha.cos();
            let cos_sigma_m = cos_sigma - 2.0 * sin_u1 * sin_u2 / cos_square_alpha;
            let c = Self::F / 16.0 * cos_square_alpha * (4.0 + Self::F * (4.0 - 3.0 * cos_square_alpha));
    
            let last_lambda = lambda;
            lambda = l + ( 1.0 - c ) * Self::F * alpha.sin() * (sigma + c * (cos_sigma_m + c * cos_sigma * (-1.0 + 2.0 * cos_sigma_m * cos_sigma_m)));
            if (lambda - last_lambda).abs() <= p
            {
                let usq = cos_square_alpha * (Self::A * Self::A - Self::B * Self::B) / (Self::B * Self::B);
                let a = 1.0 + usq / 16384.0 * (4096.0 + usq * (-768.0 + usq * (320.0 - 175.0 * usq)));
                let b = usq / 1024.0 * (256.0 + usq * (-128.0 + usq * (74.0 - 47.0 * usq)));
                let delta_sigma = b * sin_sigma * (cos_sigma_m + b / 4.0 * (cos_sigma * (-1.0 + 2.0 * cos_sigma_m * cos_sigma_m) - b / 6.0 * cos_sigma_m * (-3.0 + 4.0 * sin_sigma * sin_sigma) * (-3.0 + 4.0 * cos_sigma_m * cos_sigma_m)));
                return Self::B * a * (sigma - delta_sigma);
            }
        }
    }
}

pub struct WGS84 { }

impl Ellipsoid for WGS84
{
    const A:f64 = 6_378_137.0;
    const F_INV:f64 = 298.257_223_563;
}

#[cfg(test)]
mod tests
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_wgs84_eccentricity()
    {
        let e1 = WGS84::e1();
        let e2 = WGS84::e2();
        assert_approx_eq!(f64, (WGS84::A + WGS84::B) * (WGS84::A - WGS84::B), e1 * e2 * WGS84::A * WGS84::B, epsilon=1e-2);
    }

    #[rstest]
    #[case(EarthRadius::Default, 6371000.0)]
    #[case(EarthRadius::Equatorial, WGS84::A)]
    #[case(EarthRadius::Polar, WGS84::B)]
    fn test_wgs84_radius(#[case] earth: EarthRadius, #[case] v: f64)
    {
        assert_approx_eq!(f64, v, WGS84::radius(earth));
    }

    #[rstest]
    #[case(EarthRadius::ArithmeticMean, EarthRadius::Default, 10.0)]
    #[case(EarthRadius::VolumeMean, EarthRadius::Default, 10.0)]
    #[case(EarthRadius::SurfaceAreaMean, EarthRadius::Default, 10.0)]
    fn test_wgs84_radius_range(#[case] earth: EarthRadius, #[case] base: EarthRadius, #[case] epsilon: f64)
    {
        let r = WGS84::radius(earth);
        let d = WGS84::radius(base);
        assert!(f64::abs(r-d) < epsilon);

        let a = WGS84::A;
        let b = WGS84::B;
        assert!(r > b && r < a);
    }

    #[rstest]
    #[case(EarthVolume::Default, 1.08321e21)]
    fn test_wgs84_volume(#[case] earth: EarthVolume, #[case]d: f64)
    {
        let v = WGS84::volume(earth);
        assert_eq!(d, v);
    }

    #[rstest]
    #[case(EarthVolume::Sphere, EarthVolume::Default, 1e-5)]
    #[case(EarthVolume::Ellipsoid, EarthVolume::Default,1e-5)]
    fn test_wgs84_volume_range(#[case] earth: EarthVolume, #[case] base: EarthVolume, #[case] epsilon: f64)
    {
        let d = WGS84::volume(base);
        let v = WGS84::volume(earth);
        let e = f64::abs(v-d) / d;
        assert!(e < epsilon, "with (e={}, v={})", e, v);
    }

    #[rstest]
    #[case(EarthSurfaceArea::Default, 5.10072e14)]
    fn test_wgs84_surface_area(#[case] earth: EarthSurfaceArea, #[case]d: f64)
    {
        let v = WGS84::surface_area(earth);
        assert_eq!(d, v);
    }

    #[rstest]
    #[case(EarthSurfaceArea::Spheriod, EarthSurfaceArea::Default, 1.5e-5)]
    #[case(EarthSurfaceArea::Sphere, EarthSurfaceArea::Spheriod, 2.5e-6)]
    #[case(EarthSurfaceArea::Cantrell, EarthSurfaceArea::Spheriod, 5.0e-8)]
    #[case(EarthSurfaceArea::Thomsen, EarthSurfaceArea::Spheriod, 5.0e-8)]
    fn test_wgs84_surface_area_range(#[case] earth: EarthSurfaceArea, #[case] base: EarthSurfaceArea, #[case] epsilon: f64)
    {
        let d = WGS84::surface_area(base);
        let sa = WGS84::surface_area(earth);
        assert_approx_eq!(f64, d, sa, epsilon=epsilon * d);
    }

    #[rstest]
    #[case(40.7127, -74.0059, 34.0500, -118.2500, 3936385.0963892923, 1e-6)]
    #[case(38.898556, -77.037852, 38.897147, -77.043934, 549.1557912048178, 1e-6)]
    #[case(38.897448, -77.036585, 38.889825, -77.009080, 2526.8200141136494, 1e-6)]
    fn test_haversine
    (
        #[case] lat1: f64, #[case] lon1: f64, 
        #[case] lat2: f64, #[case] lon2: f64, 
        #[case] distance: f64, #[case] epsilon: f64
    )
    {
        let p1 = LatLon::new(lat1, lon1);
        let p2 = LatLon::new(lat2, lon2);
        let d1to2 = WGS84::haversine(&p1, &p2);
        let d2to1 = WGS84::haversine(&p2, &p1);
        assert_approx_eq!(f64, distance, d1to2, epsilon=distance * epsilon);
        assert_approx_eq!(f64, distance, d2to1, epsilon=distance * epsilon);
    }

    #[rstest]
    #[case(40.7791472, -73.9680804, 42.3541165, -71.0693514, 298396.057, 1e-5)]
    fn test_vincenty
    (
        #[case] lat1: f64, #[case] lon1: f64,
        #[case] lat2: f64, #[case] lon2: f64,
        #[case] distance: f64, #[case] epsilon: f64
    )
    {
        let p1 = LatLon::new(lat1, lon1);
        let p2 = LatLon::new(lat2, lon2);
        let d1to2 = WGS84::vincenty(&p1, &p2, 1e-9);
        let d2to1 = WGS84::vincenty(&p2, &p1, 1e-9);
        assert_approx_eq!(f64, distance, d1to2, epsilon=distance * epsilon);
        assert_approx_eq!(f64, distance, d2to1, epsilon=distance * epsilon);
    }
}