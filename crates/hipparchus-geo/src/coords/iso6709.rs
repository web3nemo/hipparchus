use crate::coords::coord::Coord;
use crate::unit::Unit;
use hipparchus_az::DegreeMinuteSecond;

/// ISO6709 format of latitude/longitude value.
/// See also in https://en.wikipedia.org/wiki/ISO_6709.
pub trait ISO6709
{
    fn iso6709(&self, coord:Coord, unit: Unit) -> String;
}

impl ISO6709 for DegreeMinuteSecond
{
    fn iso6709(&self, coord: Coord, unit: Unit) -> String
    {
        match coord
        {
            Coord::Latitude => match unit
            {
                Unit::Degree => format!("{sn}{v:07.4}", sn=self.sign(), v=self.value().abs()),
                Unit::Minute => format!("{sn}{d:02}{m:06.3}", sn=self.sign(), d=self.degree(), m=self.fraction()),
                Unit::Second => format!("{sn}{d:02}{m:02}{s:05.2}", sn=self.sign(), d=self.degree(), m=self.minute(), s=self.second()),
            },
            Coord::Longitude => match unit
            {
                Unit::Degree => format!("{sn}{v:08.4}", sn=self.sign(), v=self.value().abs()),
                Unit::Minute => format!("{sn}{d:03}{m:06.3}", sn=self.sign(), d=self.degree(), m=self.fraction()),
                Unit::Second => format!("{sn}{d:03}{m:02}{s:05.2}", sn=self.sign(), d=self.degree(), m=self.minute(), s=self.second()),
            },
        }
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(90.0, "+90.0000")]
    #[case(70.51, "+70.5100")]
    #[case(0.0, "+00.0000")]
    #[case(-70.51, "-70.5100")]
    #[case(-90.0, "-90.0000")]
    fn test_dms_iso6709_lat_degree(#[case] value: f64, #[case] expected: &str)
    {
        let dms = DegreeMinuteSecond::with(Coord::Latitude.norm(value));
        let text = dms.iso6709(Coord::Latitude, Unit::Degree);
        assert_eq!(expected, text);
    }

    #[rstest]
    #[case(180.0, "-180.0000")]
    #[case(120.51, "+120.5100")]
    #[case(90.0, "+090.0000")]
    #[case(70.51, "+070.5100")]
    #[case(0.0, "+000.0000")]
    #[case(-70.51, "-070.5100")]
    #[case(-90.0, "-090.0000")]
    #[case(-120.51, "-120.5100")]
    #[case(-180.0, "-180.0000")]
    fn test_dms_iso6709_lon_degree(#[case] value: f64, #[case] expected: &str)
    {
        let dms = DegreeMinuteSecond::with(Coord::Longitude.norm(value));
        let text = dms.iso6709(Coord::Longitude, Unit::Degree);
        assert_eq!(expected, text);
    }

    #[rstest]
    #[case(90.0, "+9000.000")]
    #[case(70.51, "+7030.600")]
    #[case(0.0, "+0000.000")]
    #[case(-70.51, "-7030.600")]
    #[case(-90.0, "-9000.000")]
    fn test_dms_iso6709_lat_minute(#[case] value: f64, #[case] expected: &str)
    {
        let dms = DegreeMinuteSecond::with(Coord::Latitude.norm(value));
        let text = dms.iso6709(Coord::Latitude, Unit::Minute);
        assert_eq!(expected, text);
    }

    #[rstest]
    #[case(180.0, "-18000.000")]
    #[case(120.51, "+12030.600")]
    #[case(90.0, "+09000.000")]
    #[case(70.51, "+07030.600")]
    #[case(0.0, "+00000.000")]
    #[case(-70.51, "-07030.600")]
    #[case(-90.0, "-09000.000")]
    #[case(-120.51, "-12030.600")]
    #[case(-180.0, "-18000.000")]
    fn test_dms_iso6709_lon_minute(#[case] value: f64, #[case] expected: &str)
    {
        let dms = DegreeMinuteSecond::with(Coord::Longitude.norm(value));
        let text = dms.iso6709(Coord::Longitude, Unit::Minute);
        assert_eq!(expected, text);
    }

    #[rstest]
    #[case(90.0, "+900000.00")]
    #[case(70.51, "+703036.00")]
    #[case(0.0, "+000000.00")]
    #[case(-70.51, "-703036.00")]
    #[case(-90.0, "-900000.00")]
    fn test_dms_iso6709_lat_second(#[case] value: f64, #[case] expected: &str)
    {
        let dms = DegreeMinuteSecond::with(Coord::Latitude.norm(value));
        let text = dms.iso6709(Coord::Latitude, Unit::Second);
        assert_eq!(expected, text);
    }

    #[rstest]
    #[case(180.0, "-1800000.00")]
    #[case(120.51, "+1203036.00")]
    #[case(90.0, "+0900000.00")]
    #[case(70.51, "+0703036.00")]
    #[case(0.0, "+0000000.00")]
    #[case(-70.51, "-0703036.00")]
    #[case(-90.0, "-0900000.00")]
    #[case(-120.51, "-1203036.00")]
    #[case(-180.0, "-1800000.00")]
    fn test_dms_iso6709_lon_second(#[case] value: f64, #[case] expected: &str)
    {
        let dms = DegreeMinuteSecond::with(Coord::Longitude.norm(value));
        let text = dms.iso6709(Coord::Longitude, Unit::Second);
        assert_eq!(expected, text);
    }


}