use std::f64::consts::PI;
use hipparchus_mean::LpNorm;
use crate::earth::models::Model;
use crate::LatLon;

pub enum Radius
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

pub enum Volume
{
    /// Traditional fixed value for earth volume
    Default = 0, 

    /// volume of the sphere with default earth radius 
    Sphere = 1,

    /// volume of the earth ellipsoid (more accurate)
    Ellipsoid = 2,
}

pub enum SurfaceArea
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

pub trait Geometry
{
    fn radius(r:Radius) -> f64;
    fn volume(v:Volume) -> f64;
    fn surface_area(sa:SurfaceArea) -> f64;

    fn haversine(l1: &LatLon, l2: &LatLon) -> f64;
    fn vincenty(l1: &LatLon, l2: &LatLon, p: f64) -> f64;

}

impl<T> Geometry for T where T: Model
{
    fn radius(r:Radius) -> f64
    {
        match r
        {
            Radius::Default => 6_371_000.0,
            Radius::Equatorial => Self::A,
            Radius::Polar => Self::B,
            Radius::ArithmeticMean => (Self::A * 2.0 + Self::B) / 3.0,
            Radius::VolumeMean => f64::powf(Self::A * Self::A * Self::B, 1.0/3.0),
            Radius::SurfaceAreaMean => f64::sqrt(Self::surface_area(SurfaceArea::Spheriod) * 0.25 / PI),
        }
    }
    
    fn volume(v:Volume) -> f64
    {
        match v
        {
            Volume::Default => 1.08321e21,
            Volume::Sphere => Self::radius(Radius::Default).powi(3) * PI / 0.75,
            Volume::Ellipsoid => Self::A * Self::A * Self::B * PI / 0.75,
        }
    }

    fn surface_area(sa:SurfaceArea) -> f64
    {
        match sa
        {
            SurfaceArea::Default => 5.10072e14,
            SurfaceArea::Sphere => Self::radius(Radius::Default).powi(2) * PI * 4.0,
            SurfaceArea::Spheriod =>
            {
                let e = Self::e1();
                let k = 1.0 + (1.0 - e * e) / e * f64::atanh(e);
                k * Self::A * Self::A * PI * 2.0
            },
            SurfaceArea::Cantrell =>
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
            SurfaceArea::Thomsen =>
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

    fn haversine(l1: &LatLon, l2: &LatLon) -> f64
    {
        let lat1 = l1.latitude().to_radians();
        let lon1 = l1.longitude().to_radians();
        let lat2 = l2.latitude().to_radians();
        let lon2 = l2.longitude().to_radians();

        let hdx = (lon2 - lon1) / 2.0;
        let hdy = (lat2 - lat1) / 2.0;
        let ratio = (hdy.sin().powi(2) + hdx.sin().powi(2)*lat2.cos()*lat1.cos()).sqrt().asin() * 2.0;
        ratio * Self::radius(Radius::Default)
    }

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

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::earth::models::WGS84;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(Radius::Default, 6371000.0)]
    #[case(Radius::Equatorial, WGS84::A)]
    #[case(Radius::Polar, WGS84::B)]
    fn test_wgs84_radius(#[case] earth: Radius, #[case] v: f64)
    {
        assert_approx_eq!(f64, v, WGS84::radius(earth));
    }

    #[rstest]
    #[case(Radius::ArithmeticMean, Radius::Default, 10.0)]
    #[case(Radius::VolumeMean, Radius::Default, 10.0)]
    #[case(Radius::SurfaceAreaMean, Radius::Default, 10.0)]
    fn test_wgs84_radius_range(#[case] earth: Radius, #[case] base: Radius, #[case] epsilon: f64)
    {
        let r = WGS84::radius(earth);
        let d = WGS84::radius(base);
        assert!(f64::abs(r-d) < epsilon);

        let a = WGS84::A;
        let b = WGS84::B;
        assert!(r > b && r < a);
    }

    #[rstest]
    #[case(Volume::Default, 1.08321e21)]
    fn test_wgs84_volume(#[case] earth: Volume, #[case]d: f64)
    {
        let v = WGS84::volume(earth);
        assert_eq!(d, v);
    }

    #[rstest]
    #[case(Volume::Sphere, Volume::Default, 1e-5)]
    #[case(Volume::Ellipsoid, Volume::Default,1e-5)]
    fn test_wgs84_volume_range(#[case] earth: Volume, #[case] base: Volume, #[case] epsilon: f64)
    {
        let d = WGS84::volume(base);
        let v = WGS84::volume(earth);
        let e = f64::abs(v-d) / d;
        assert!(e < epsilon, "with (e={}, v={})", e, v);
    }

    #[rstest]
    #[case(SurfaceArea::Default, 5.10072e14)]
    fn test_wgs84_surface_area(#[case] earth: SurfaceArea, #[case]d: f64)
    {
        let v = WGS84::surface_area(earth);
        assert_eq!(d, v);
    }

    #[rstest]
    #[case(SurfaceArea::Spheriod, SurfaceArea::Default, 1.5e-5)]
    #[case(SurfaceArea::Sphere, SurfaceArea::Spheriod, 2.5e-6)]
    #[case(SurfaceArea::Cantrell, SurfaceArea::Spheriod, 5.0e-8)]
    #[case(SurfaceArea::Thomsen, SurfaceArea::Spheriod, 5.0e-8)]
    fn test_wgs84_surface_area_range(#[case] earth: SurfaceArea, #[case] base: SurfaceArea, #[case] epsilon: f64)
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