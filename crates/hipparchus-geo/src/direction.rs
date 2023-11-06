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
    #[case(D4::North, "N")]
    #[case(D4::South, "S")]
    #[case(D4::East, "E")]
    #[case(D4::West, "W")]
    fn test_d4_str(#[case] d4: D4, #[case] text: String)
    {
        assert_eq!(text, d4.to_string());
        assert_eq!(d4, D4::from_str(text.as_str()).unwrap());
    }
}
