use std::{fmt::Display, str::FromStr};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::{Coord, Sign};

/// 4 directions on a 2D plane.
#[repr(i8)]
#[derive(Debug, PartialEq, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum D4
{
    /// North (1) with latitude in range [0, 90]
    North = Coord::Latitude as i8 * Sign::Positive as i8,

    /// South (-1) with latitude in range [-90, 0)
    South = Coord::Latitude as i8 * Sign::Negative as i8,

    /// East (2) with longitude in range [0, 180)
    East = Coord::Longitude as i8 * Sign::Positive as i8,

    /// West (-2) with longitude in range [-180, 0)
    West = Coord::Longitude as i8 * Sign::Negative as i8,
}

impl D4
{
    /// Create a `D4` direction enum from a coordinate and a sign.
    pub fn with(coord:Coord, sign:Sign) -> D4
    {
        // NOTE: Leverage arithmetic on `Coord` & `Sign` to get the correct value of D4 (with bidirectional enum-to-int conversion)
        let c:i8 = coord.into();
        let s:i8 = sign.into();
        D4::try_from(c * s).unwrap()
    }

    /// Get coordinate axis definition from the `D4` direction
    pub fn coord(self) -> Coord
    {
        match self
        {
            D4::North | D4::South => Coord::Latitude,
            D4::East | D4::West => Coord::Longitude,
        }
    }

    /// Get sign of coordinate value from the `D4` direction
    pub fn sign(self) -> Sign
    {
        match self
        {
            D4::North | D4::East => Sign::Positive,
            D4::South | D4::West => Sign::Negative,
        }
    }

    pub fn abbr(self) -> char
    {
        match self
        {
            D4::North => 'N',
            D4::South => 'S',
            D4::East => 'E',
            D4::West => 'W',
        }
    }
}

/// Display `D4` direction enum as a single character.
impl Display for D4
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.abbr())
    }
}

/// Parse `D4` direction enum from a single character.
impl FromStr for D4
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "N" => Ok(D4::North),
            "S" => Ok(D4::South),
            "E" => Ok(D4::East),
            "W" => Ok(D4::West),
            _ => Err(()),
        }
    }
}

/// 8 directions on a 2D plane.
#[repr(i8)]
#[derive(Debug, PartialEq, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum D8
{
    /// 0: origin with latitude = 0 and longitude = 0
    None = 0,

    /// +1: y-axis (x=0, y>0) with same difinition of D4::North
    North = D4::North as i8,

    /// -1: y-axis (x=0, y<0) with same difinition of D4::South
    South = D4::South as i8,

    /// +2: x-axis (x>0, y=0) with same difinition of D4::East
    East = D4::East as i8,

    /// -2: x-axis (x<0, y=0) with same difinition of D4::West
    West = D4::West as i8,

    /// +3: quadrant=1 (x>0, y>0)
    NorthEast = 3,

    /// -4: quadrant=2 (x<0, y>0)
    NorthWest = -4,

    /// -3: quadrant=3 (x<0, y<0)
    SouthWest = -3,

    /// +4: quadrant=4 (x>0, y<0)
    SouthEast = 4,
}

impl D8
{
    pub fn with(lat:D4, lon:D4) -> D8
    {
        // NOTE: Leverage arithmetic on `D4` of latitude and longitude to get the correct value of D8 (with bidirectional enum-to-int conversion)
        let y:i8 = lat.into();
        let x:i8 = lon.into();
        let d8 = if lat.sign() == lon.sign() { y + x } else { (y + x) * 4 };
        D8::try_from(d8).unwrap()
    }

    pub fn quadrant(self) -> i8
    {
        match self
        {
            Self::None | Self::North | Self::South | Self::East | Self::West => 0,
            Self::NorthEast => 1,
            Self::NorthWest => 2,
            Self::SouthWest => 3,
            Self::SouthEast => 4,
        }
    }

    pub fn is_origin(self) -> bool
    {
        self == Self::None
    }

    pub fn is_axis(self) -> bool
    {
        self.quadrant() == 0
    }

    pub fn abbr(self) -> &'static str
    {
        match self
        {
            Self::None => "O",
            Self::North => "N",
            Self::South => "S",
            Self::East => "E",
            Self::West => "W",
            Self::NorthEast => "NE",
            Self::NorthWest => "NW",
            Self::SouthWest => "SW",
            Self::SouthEast => "SE",
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(D4::North, 'N', Coord::Latitude, Sign::Positive)]
    #[case(D4::South, 'S', Coord::Latitude, Sign::Negative)]
    #[case(D4::East, 'E', Coord::Longitude, Sign::Positive)]
    #[case(D4::West, 'W', Coord::Longitude, Sign::Negative)]
    fn test_d4_with
    (
        #[case] direction: D4,
        #[case] abbr: char,
        #[case] coord: Coord,
        #[case] sign: Sign,
    )
    {
        let d4 = D4::with(coord, sign);

        assert_eq!(direction, d4);
        assert_eq!(abbr, d4.abbr());
        assert_eq!(coord, d4.coord());
        assert_eq!(sign, d4.sign());
    }

    #[rstest]
    #[case(D4::North, D8::North)]
    #[case(D4::South, D8::South)]
    #[case(D4::East, D8::East)]
    #[case(D4::West, D8::West)]
    fn test_direction_eq(#[case] d4: D4, #[case] d8: D8)
    {
        assert_eq!(d4 as i8, d8 as i8);
    }

    #[rstest]
    #[case(D8::NorthEast, "NE", 1, 30.0, 120.0)]
    #[case(D8::NorthWest, "NW", 2, 30.0, -120.0)]
    #[case(D8::SouthWest, "SW", 3, -30.0, -120.0)]
    #[case(D8::SouthEast, "SE", 4, -30.0, 120.0)]
    fn test_d8_with
    (
        #[case] direction: D8, #[case] abbr: &str,
        #[case] quadrant: i8,
        #[case] lat: f64, #[case] lon: f64,
    )
    {
        let y = Coord::Latitude.direction(lat);
        let x = Coord::Longitude.direction(lon);
        let d8 = D8::with(y, x);
        assert_eq!(direction, d8);
        assert_eq!(abbr, d8.abbr());
        assert_eq!(quadrant, d8.quadrant());
        assert_eq!(false, d8.is_axis());
        assert_eq!(false, d8.is_origin());
    }
}
 