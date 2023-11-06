use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::direction::D4;
use crate::sign::WithSign;

#[repr(i8)]
#[derive(Debug, PartialEq, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum Coord
{
    Latitude = 1,
    Longitude = 2,
}

impl Coord
{
    /// Get the maximum value of latitude or longitude.
    pub fn max(self) -> u8
    {
        match self
        {
            Self::Latitude => 90,
            Self::Longitude => 180,
        }
    }

    /// Get the minimum value of latitude or longitude.
    pub fn min(self) -> u8
    {
        0
    }

    /// Normalize the value of latitude or longitude.
    /// - Latitude: -90.0 <= value <= 90.0
    /// - Longitude: -180.0 <= value < 180.0
    pub fn norm(self, value:f64) -> f64
    {
        match self
        {
            Self::Latitude =>
            {
                let v = match value % 360.0
                {
                    v if v < -180.0 => v + 360.0,
                    v if v > 180.0 => v - 360.0,
                    v => v,   
                };
                match v
                {
                    v if v < -90.0 => -180.0 - v,
                    v if v > 90.0 => 180.0 - v,
                    v => v,
                }
            },
            Self::Longitude =>
            {
                match value % 360.0
                {
                    v if v < -180.0 => v + 360.0,
                    v if v >= 180.0 => v - 360.0,
                    v => v,
                }
            },
        }
    }

    /// Get the direction from latitude/longitude value.
    pub fn direction(self, value:f64) -> D4 where
    {
        D4::with(self, self.norm(value).sign())
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_coord_min()
    {
        assert_eq!(0, Coord::Latitude.min());
        assert_eq!(0, Coord::Longitude.min());
    }

    #[test]
    fn test_coord_max()
    {
        assert_eq!(90, Coord::Latitude.max());
        assert_eq!(180, Coord::Longitude.max());
    }

    #[rstest]
    #[case(Coord::Longitude, 120.0)]
    #[case(Coord::Longitude, -120.0)]
    #[case(Coord::Latitude, 70.0)]
    #[case(Coord::Latitude, -70.0)]
    fn test_coord_norm(#[case] coord: Coord, #[case] value: f64)
    {
        assert_approx_eq!(f64, value, coord.norm(value));
        assert_approx_eq!(f64, value, coord.norm(value + 360.0));
        assert_approx_eq!(f64, value, coord.norm(value - 360.0));
    }

    #[rstest]
    #[case(Coord::Longitude, 200.0, -160.0)]
    #[case(Coord::Longitude, -200.0, 160.0)]
    #[case(Coord::Latitude, 110.0, 70.0)]
    #[case(Coord::Latitude, -110.0, -70.0)]
    fn test_coord_norm_overflow(#[case] coord: Coord, #[case] value: f64, #[case] expected: f64)
    {
        assert_approx_eq!(f64, expected, coord.norm(value));
        assert_approx_eq!(f64, expected, coord.norm(value + 360.0));
        assert_approx_eq!(f64, expected, coord.norm(value - 360.0));
    }

    #[rstest]
    #[case(Coord::Longitude, 0.0, 0.0)]
    #[case(Coord::Longitude, 180.0, -180.0)]
    #[case(Coord::Longitude, -180.0, -180.0)]
    fn test_coord_norm_special(#[case] coord: Coord, #[case] value: f64, #[case] expected: f64)
    {
        assert_eq!(expected, coord.norm(value));
        assert_approx_eq!(f64, expected, coord.norm(value + 360.0));
        assert_approx_eq!(f64, expected, coord.norm(value - 360.0));
    }
}    
