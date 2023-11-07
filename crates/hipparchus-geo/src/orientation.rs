use std::fmt::Display;
use std::str::FromStr;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::{Coord, Sign};

/// 4 directions on a 2D plane.
#[repr(i8)]
#[derive(Debug, PartialEq, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum Orientation
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

impl Orientation
{
    /// Create a `D4` direction enum from a coordinate and a sign.
    pub fn with(coord:Coord, sign:Sign) -> Orientation
    {
        // NOTE: Leverage arithmetic on `Coord` & `Sign` to get the correct value of D4 (with bidirectional enum-to-int conversion)
        let c:i8 = coord.into();
        let s:i8 = sign.into();
        Orientation::try_from(c * s).unwrap()
    }

    /// Get coordinate axis definition from the `D4` direction
    pub fn coord(self) -> Coord
    {
        match self
        {
            Orientation::North | Orientation::South => Coord::Latitude,
            Orientation::East | Orientation::West => Coord::Longitude,
        }
    }

    /// Get sign of coordinate value from the `D4` direction
    pub fn sign(self) -> Sign
    {
        match self
        {
            Orientation::North | Orientation::East => Sign::Positive,
            Orientation::South | Orientation::West => Sign::Negative,
        }
    }

    pub fn abbr(self) -> char
    {
        match self
        {
            Orientation::North => 'N',
            Orientation::South => 'S',
            Orientation::East => 'E',
            Orientation::West => 'W',
        }
    }
}

/// Display `D4` direction enum as a single character.
impl Display for Orientation
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.abbr())
    }
}

/// Parse `D4` direction enum from a single character.
impl FromStr for Orientation
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "N" => Ok(Orientation::North),
            "S" => Ok(Orientation::South),
            "E" => Ok(Orientation::East),
            "W" => Ok(Orientation::West),
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
    #[case(Orientation::North, 'N', Coord::Latitude, Sign::Positive)]
    #[case(Orientation::South, 'S', Coord::Latitude, Sign::Negative)]
    #[case(Orientation::East, 'E', Coord::Longitude, Sign::Positive)]
    #[case(Orientation::West, 'W', Coord::Longitude, Sign::Negative)]
    fn test_d4_with
    (
        #[case] direction: Orientation,
        #[case] abbr: char,
        #[case] coord: Coord,
        #[case] sign: Sign,
    )
    {
        let d4 = Orientation::with(coord, sign);
        assert_eq!(direction, d4);
        assert_eq!(abbr, d4.abbr());
        assert_eq!(coord, d4.coord());
        assert_eq!(sign, d4.sign());
    }

    #[rstest]
    #[case(Orientation::North, "N")]
    #[case(Orientation::South, "S")]
    #[case(Orientation::East, "E")]
    #[case(Orientation::West, "W")]
    fn test_d4_str(#[case] d4: Orientation, #[case] text: String)
    {
        assert_eq!(text, d4.to_string());
        assert_eq!(d4, Orientation::from_str(text.as_str()).unwrap());
    }
}
