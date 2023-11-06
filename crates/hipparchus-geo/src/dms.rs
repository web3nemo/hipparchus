use num::Zero;
use crate::sign::{Sign, WithSign};
use crate::coord::Coord;
use crate::unit::Unit;
use crate::direction::D4;

pub struct DegreeMinuteSecond
{
    sign: Sign,
    degree: u16,
    minute: u8,
    second: f32,
}

impl DegreeMinuteSecond
{
    pub fn new(sign:Sign, degree:u16, minute:u8, second:f32) -> Self
    {
        assert!(minute < 60 && second >= 0.0 && second < 60.0);
        Self
        {
            sign: if degree.is_zero() && minute.is_zero() && second.is_zero() { Sign::Positive } else { sign},
            degree, minute, second
        }
    }

    pub fn sign(&self) -> Sign { self.sign }
    pub fn degree(&self) -> u16 { self.degree }
    pub fn minute(&self) -> u8 { self.minute }
    pub fn second(&self) -> f32 { self.second }

    pub fn with(value:f64) -> Self
    {
        let sign = value.sign();
        let value = value.abs();
        let degree = value as u16;
        let minute = ((value - degree as f64) * 60.0) as u8;
        let second = (value - degree as f64) * 3600.0 - (minute as f64) * 60.0;
        Self{ sign, degree, minute, second: second as f32}
    }

    /// Get the whole value in degrees.
    pub fn value(&self) -> f64
    {
        let v = self.degree as f64 + self.minute as f64 / 60.0 + self.second as f64 / 3600.0;
        match self.sign
        {
            Sign::Positive => v,
            Sign::Negative => -v,
        }
    }

    /// Get the fraction value in minutes (less than 1 degree): minutes + seconds.
    pub fn fraction(&self) -> f64
    {
        self.minute as f64 + self.second as f64 / 60.0
    }

    /// Create DMS instance with value of 0.0 degrees.
    pub fn zero() -> Self
    {
        Self::with(0.0)
    }

    /// Detect if DMS value is zero.
    pub fn is_zero(&self) -> bool
    {
        self.degree.is_zero() && self.minute.is_zero() && self.second.is_zero()
    }

    /// Get the ISO6709 format of latitude/longitude value.
    pub fn iso6709(&self, coord: Coord, unit: Unit) -> String
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

    /// Get the NMEA0183 format of latitude/longitude value.
    pub fn nmea0183(self, coord: Coord) -> String
    {
        match coord
        {
            Coord::Latitude => format!("{d:02}{m:06.3},{ns}", d=self.degree(), m=self.fraction(), ns=D4::with(coord, self.sign())),
            Coord::Longitude => format!("{d:03}{m:06.3},{ew}", d=self.degree(), m=self.fraction(), ew=D4::with(coord, self.sign())),
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_dms_size()
    {
        assert_eq!(8, std::mem::size_of::<DegreeMinuteSecond>());
    }

    #[rstest]
    #[case(60.51, 30.6, false, Sign::Positive, 60, 30, 36.0)]
    #[case(-60.51, 30.6, false, Sign::Negative, 60, 30, 36.0)]
    #[case(0.0, 0.0, true, Sign::Positive, 0, 0, 0.0)]
    fn test_dms_new
    (
        #[case] value: f64, #[case] fraction: f64, #[case] zero: bool,
        #[case] sign: Sign, #[case] degree: u16, #[case] minute: u8, #[case] second: f32
    )
    {
        let dms = DegreeMinuteSecond::new(sign, degree, minute, second);
        assert_eq!(sign, dms.sign());
        assert_eq!(degree, dms.degree());
        assert_eq!(minute, dms.minute());
        assert_approx_eq!(f32, second, dms.second());
        assert_approx_eq!(f64, value, dms.value());
        assert_approx_eq!(f64, fraction, dms.fraction());
        assert_eq!(zero, dms.is_zero());
    }

    #[rstest]
    #[case(60.51, 30.6, false, Sign::Positive, 60, 30, 36.0)]
    #[case(-60.51, 30.6, false, Sign::Negative, 60, 30, 36.0)]
    #[case(0.0, 0.0, true, Sign::Positive, 0, 0, 0.0)]
    fn test_dms_with
    (
        #[case] value: f64, #[case] fraction: f64, #[case] zero: bool,
        #[case] sign: Sign, #[case] degree: u16, #[case] minute: u8, #[case] second: f32
    )
    {
        let dms = DegreeMinuteSecond::with(value);
        assert_eq!(sign, dms.sign());
        assert_eq!(degree, dms.degree());
        assert_eq!(minute, dms.minute());
        assert_approx_eq!(f32, second, dms.second());
        assert_approx_eq!(f64, value, dms.value());
        assert_approx_eq!(f64, fraction, dms.fraction());
        assert_eq!(zero, dms.is_zero());
    }

    #[test]
    fn test_zero()
    {
        let dms = DegreeMinuteSecond::zero();
        assert_eq!(Sign::Positive, dms.sign());
        assert_eq!(0, dms.degree());
        assert_eq!(0, dms.minute());
        assert_approx_eq!(f32, 0.0, dms.second());
        assert_approx_eq!(f64, 0.0, dms.value());
        assert_approx_eq!(f64, 0.0, dms.fraction());
        assert_eq!(true, dms.is_zero());
    }

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