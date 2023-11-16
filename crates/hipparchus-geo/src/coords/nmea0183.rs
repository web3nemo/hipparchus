use crate::Orientation;
use crate::coords::coord::Coord;
use hipparchus_az::DegreeMinuteSecond;

/// The NMEA0183 format of latitude/longitude value
pub trait NMEA0183
{
    /// Get the NMEA0183 format of latitude/longitude value.
    fn nmea0183(&self, coord: Coord) -> String;
}

impl NMEA0183 for DegreeMinuteSecond
{
    fn nmea0183(&self, coord: Coord) -> String
    {
        match coord
        {
            Coord::Latitude => format!("{d:02}{m:06.3},{ns}", d=self.degree(), m=self.fraction(), ns=Orientation::with(coord, self.sign())),
            Coord::Longitude => format!("{d:03}{m:06.3},{ew}", d=self.degree(), m=self.fraction(), ew=Orientation::with(coord, self.sign())),
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(90.0, "9000.000,N")]
    #[case(70.51, "7030.600,N")]
    #[case(0.0, "0000.000,N")]
    #[case(-70.51, "7030.600,S")]
    #[case(-90.0, "9000.000,S")]
    fn test_dms_nmea0183_lat(#[case] value: f64, #[case] expected: &str)
    {
        let dms = DegreeMinuteSecond::with(Coord::Latitude.norm(value));
        let text = dms.nmea0183(Coord::Latitude);
        assert_eq!(expected, text);
    }

    #[rstest]
    #[case(180.0, "18000.000,W")]
    #[case(120.51, "12030.600,E")]
    #[case(90.0, "09000.000,E")]
    #[case(70.51, "07030.600,E")]
    #[case(0.0, "00000.000,E")]
    #[case(-70.51, "07030.600,W")]
    #[case(-90.0, "09000.000,W")]
    #[case(-120.51, "12030.600,W")]
    #[case(-180.0, "18000.000,W")]
    fn test_dms_nmea0183_lon(#[case] value: f64, #[case] expected: &str)
    {
        let dms = DegreeMinuteSecond::with(Coord::Longitude.norm(value));
        let text = dms.nmea0183(Coord::Longitude);
        assert_eq!(expected, text);
    }
}
