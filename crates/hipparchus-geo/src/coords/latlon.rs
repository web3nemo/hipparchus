use crate::coords::coord::Coord;
use crate::unit::Unit;
use hipparchus_az::DegreeMinuteSecond;
use crate::coords::iso6709::ISO6709;
use crate::coords::nmea0183::NMEA0183;

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

    /// Get the ISO6709 format of latitude/longitude value.
    pub fn iso6709(&self, unit:Unit) -> String
    {
        let lat = DegreeMinuteSecond::with(self.lat);
        let lon = DegreeMinuteSecond::with(self.lon);
        format!("{lat},{lon}", lat=lat.iso6709(Coord::Latitude, unit), lon=lon.iso6709(Coord::Longitude, unit))
    }

    /// Get the MMEA0183 format of latitude/longitude value.
    pub fn nmea0183(&self) -> String
    {
        let lat = DegreeMinuteSecond::with(self.lat);
        let lon = DegreeMinuteSecond::with(self.lon);
        format!("{lat},{lon}", lat=lat.nmea0183(Coord::Latitude), lon=lon.nmea0183(Coord::Longitude))
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

    #[rstest]
    #[case(0.0, 0.0, Unit::Degree, "+00.0000,+000.0000")]
    #[case(39.908823, 116.397470, Unit::Degree, "+39.9088,+116.3975")]
    fn test_latlon_iso6709(#[case] lat: f64, #[case] lon: f64, #[case] unit: Unit, #[case] expected: &str)
    {
        let latlon = LatLon::new(lat, lon);
        assert_eq!(expected, latlon.iso6709(unit));
    }

    #[rstest]
    #[case(0.0, 0.0, "0000.000,N,00000.000,E")]
    #[case(39.908823, 116.397470, "3954.529,N,11623.848,E")]
    fn test_latlon_nmea0183(#[case] lat: f64, #[case] lon: f64, #[case] expected: &str)
    {
        let latlon = LatLon::new(lat, lon);
        assert_eq!(expected, latlon.nmea0183());
    }
}    
