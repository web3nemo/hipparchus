use num::Zero;
use crate::coord::{Coord, Sign, WithSign};
use crate::direction::D4;

pub struct DegreeMinuteSecond
{
    coord: Coord,
    sign: Sign,
    degree: u8,
    minute: u8,
    second: f32,
}

impl DegreeMinuteSecond
{
    pub fn latitude(v:f64) -> Self
    {
        // TODO: fmod for out-of-range input value
        assert!(v >= -90.0 && v <= 90.0);
        Self::coord(Coord::Latitude, v)
    }

    pub fn longitude(v:f64) -> Self
    {
        // TODO: fmod for out-of-range input value
        assert!(v >= -180.0 && v <= 180.0);
        Self::coord(Coord::Longitude, v)
    }

    pub fn coord(coord:Coord, value:f64) -> Self
    {
        let sign = value.sign();
        let value = value.abs();
        let degree = value as u8;
        let minute = ((value - degree as f64) * 60.0) as u8;
        let second = (value - degree as f64) * 3600.0 - (minute as f64) * 60.0;
        Self{ coord, sign, degree, minute, second: second as f32}
    }

    pub fn with(coord:Coord, sign:Sign, degree:u8, minute:u8, second:f32) -> Self
    {
        assert!(minute < 60 && second >= 0.0 && second < 60.0);
        if minute.is_zero() && second.is_zero() { assert!(degree <= coord.max()) } else { assert!(degree < coord.max()) }
        Self{coord, sign, degree, minute, second }
    }

    pub fn value(&self) -> f64
    {
        let v = self.degree as f64 + self.minute as f64 / 60.0 + self.second as f64 / 3600.0;
        match self.sign
        {
            Sign::Positive => v,
            Sign::Negative => -v
        }
    }

    pub fn direction(&self) -> D4
    {
        if self.is_zero() || self.is_meridian() { D4::None } else { D4::with(self.coord, self.sign) }
    }

    pub fn is_zero(&self) -> bool
    {
        self.degree.is_zero() && self.minute.is_zero() && self.second.is_zero()
    }

    pub fn is_max(&self) -> bool
    {
        self.degree == self.coord.max() && self.minute.is_zero() && self.second.is_zero()
    }

    pub fn is_meridian(&self) -> bool
    {
        self.coord == Coord::Longitude && (self.is_zero() || self.is_max())
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;
    use crate::coord::Coord;
 
    #[rstest]
    #[case(60.51, D4::North, Sign::Positive, 60, 30, 36.0)]
    #[case(-60.51, D4::South, Sign::Negative, 60, 30, 36.0)]
    #[case(0.0, D4::None, Sign::Positive, 0, 0, 0.0)]
    #[case(90.0, D4::North, Sign::Positive, 90, 0, 0.0)]
    #[case(-90.0, D4::South, Sign::Negative, 90, 0, 0.0)]
    fn test_dms_lat
    (
        #[case] value: f64, #[case] direction: D4,
        #[case] sign: Sign, #[case] degree: u8, #[case] minute: u8, #[case] second: f32,
    )
    {
        let dms = DegreeMinuteSecond::with(Coord::Latitude, sign, degree, minute, second);
        assert_eq!(Coord::Latitude, dms.coord);
        assert_eq!(sign, dms.sign);
        assert_eq!(degree, dms.degree);
        assert_eq!(minute, dms.minute);
        assert_approx_eq!(f32, second, dms.second);
        assert_eq!(direction, dms.direction());
        assert_approx_eq!(f64, value, dms.value());

        let dms = DegreeMinuteSecond::coord(Coord::Latitude, value);
        assert_eq!(Coord::Latitude, dms.coord);
        assert_eq!(sign, dms.sign);
        assert_eq!(degree, dms.degree);
        assert_eq!(minute, dms.minute);
        assert_approx_eq!(f32, second, dms.second);
        assert_eq!(direction, dms.direction());
        assert_approx_eq!(f64, value, dms.value());

        let dms = DegreeMinuteSecond::latitude(value);
        assert_eq!(Coord::Latitude, dms.coord);
        assert_eq!(sign, dms.sign);
        assert_eq!(degree, dms.degree);
        assert_eq!(minute, dms.minute);
        assert_approx_eq!(f32, second, dms.second);
        assert_eq!(direction, dms.direction());
        assert_approx_eq!(f64, value, dms.value());
    }

    #[rstest]
    #[case(120.51, D4::East, Sign::Positive, 120, 30, 36.0)]
    #[case(-120.51, D4::West, Sign::Negative, 120, 30, 36.0)]
    #[case(0.0, D4::None, Sign::Positive, 0, 0, 0.0)]
    #[case(180.0, D4::None, Sign::Positive, 180, 0, 0.0)]
    #[case(-180.0, D4::None, Sign::Negative, 180, 0, 0.0)]
    fn test_dms_lon
    (
        #[case] value: f64, #[case] direction: D4,
        #[case] sign: Sign, #[case] degree: u8, #[case] minute: u8, #[case] second: f32,
    )
    {
        let dms = DegreeMinuteSecond::with(Coord::Longitude, sign, degree, minute, second);
        assert_eq!(Coord::Longitude, dms.coord);
        assert_eq!(sign, dms.sign);
        assert_eq!(degree, dms.degree);
        assert_eq!(minute, dms.minute);
        assert_approx_eq!(f32, second, dms.second);
        assert_eq!(direction, dms.direction());
        assert_approx_eq!(f64, value, dms.value());

        let dms = DegreeMinuteSecond::coord(Coord::Longitude, value);
        assert_eq!(Coord::Longitude, dms.coord);
        assert_eq!(sign, dms.sign);
        assert_eq!(degree, dms.degree);
        assert_eq!(minute, dms.minute);
        assert_approx_eq!(f32, second, dms.second);
        assert_eq!(direction, dms.direction());
        assert_approx_eq!(f64, value, dms.value());

        let dms = DegreeMinuteSecond::longitude(value);
        assert_eq!(Coord::Longitude, dms.coord);
        assert_eq!(sign, dms.sign);
        assert_eq!(degree, dms.degree);
        assert_eq!(minute, dms.minute);
        assert_approx_eq!(f32, second, dms.second);
        assert_eq!(direction, dms.direction());
        assert_approx_eq!(f64, value, dms.value());
    }
}