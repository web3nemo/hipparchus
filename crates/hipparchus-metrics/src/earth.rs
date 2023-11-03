use hipparchus_geo::LatLon;
use crate::metrics::Metrics;

#[repr(i32)]
#[derive(Clone,PartialEq,Debug)]
pub enum EarthDistance
{
    /// Haversine distance
    Haversine = 1,

    /// Vincenty distance
    Vincenty(f64) = 2,
}

impl EarthDistance
{
    // sphere constants of earth
    pub const R:f64 = 6371393f64;

    // WGS84, ellipsoids constants of earth
    pub const A:f64 = 6378137.0f64;
    pub const B:f64 = 6356752.314245f64;
    pub const F:f64 = 1.0f64 / 298.257223563f64;

    fn haversine(lat1:f64, lon1:f64, lat2:f64, lon2:f64) -> f64
    {
        let hdx = (lon2 - lon1) / 2.0;
        let hdy = (lat2 - lat1) / 2.0;
        (hdy.sin().powi(2) + hdx.sin().powi(2)*lat2.cos()*lat1.cos()).sqrt().asin() * 2.0
    }

    fn vincenty(lat1:f64, lon1:f64, lat2:f64, lon2:f64, p:f64) -> f64
    {
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

impl Metrics<&LatLon, f64> for EarthDistance
{
    fn measure(self, c1:&LatLon, c2:&LatLon) -> f64
    {
        match self
        {
            EarthDistance::Haversine => Self::haversine
            (
                c1.latitude().to_radians(), c1.longitude().to_radians(),
                c2.latitude().to_radians(), c2.longitude().to_radians()
            ) * Self::R,
            EarthDistance::Vincenty(p) => Self::vincenty
            (
                c1.latitude().to_radians(), c1.longitude().to_radians(),
                c2.latitude().to_radians(), c2.longitude().to_radians(),
                p
            ),
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