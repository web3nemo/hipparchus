use crate::coord::Coord;

/// Latitude and longitude in degrees.
pub struct LatLon
{
    lat: f64,
    lon: f64,
}

impl LatLon
{
    pub fn new(lat:f64, lon:f64) -> Self
    {
        Self
        { 
            lat: Coord::Latitude.norm(lat),
            lon: Coord::Longitude.norm(lon),
        }
    }

    /// Get the latitude.
    pub fn latitude(&self) -> f64
    {
        self.lat
    }

    /// Get the longitude.
    pub fn longitude(&self) -> f64
    {
        self.lon
    }

    pub fn iso6709(&self) -> String
    {
        todo!()
    }

    pub fn nmea0183(&self) -> String
    {
        todo!()
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(39.908823, 116.397470)]
    fn test_latlon(#[case] lat: f64, #[case] lon: f64)
    {
        let latlon = LatLon::new(lat, lon);
        assert_approx_eq!(f64, lat, latlon.latitude());
        assert_approx_eq!(f64, lon, latlon.longitude());
    }
}    
