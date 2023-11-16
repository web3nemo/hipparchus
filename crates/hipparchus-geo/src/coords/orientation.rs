use std::fmt::Display;
use std::str::FromStr;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::coords::coord::Coord;
use hipparchus_az::Sign;

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

    /// Get coordinate axis definition from the orientation
    pub fn coord(self) -> Coord
    {
        match self
        {
            Orientation::North | Orientation::South => Coord::Latitude,
            Orientation::East | Orientation::West => Coord::Longitude,
        }
    }

    /// Get sign of coordinate value from the orientation
    pub fn sign(self) -> Sign
    {
        match self
        {
            Orientation::North | Orientation::East => Sign::Positive,
            Orientation::South | Orientation::West => Sign::Negative,
        }
    }

    /// Get the abbreviation of the orientation.
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

/// Display orientation enum as a single character.
impl Display for Orientation
{
    /// Display orientation enum as a single character.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.abbr())
    }
}

/// Parse orientation enum from a single character.
impl FromStr for Orientation
{
    /// Error type
    type Err = ();

    /// Parse orientation enum from a single character.
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
    fn test_orientation_with
    (
        #[case] direction: Orientation,
        #[case] abbr: char,
        #[case] coord: Coord,
        #[case] sign: Sign,
    )
    {
        let orientation = Orientation::with(coord, sign);
        assert_eq!(direction, orientation);
        assert_eq!(abbr, orientation.abbr());
        assert_eq!(coord, orientation.coord());
        assert_eq!(sign, orientation.sign());
    }

    #[rstest]
    #[case(Orientation::North, "N")]
    #[case(Orientation::South, "S")]
    #[case(Orientation::East, "E")]
    #[case(Orientation::West, "W")]
    fn test_orientation_str(#[case] orientation: Orientation, #[case] text: String)
    {
        assert_eq!(text, orientation.to_string());
        assert_eq!(orientation, Orientation::from_str(text.as_str()).unwrap());
    }

    #[rstest]
    #[case("")]
    #[case("ABCD")]
    fn test_orientation_str_error(#[case] text: String)
    {
        let unit = Orientation::from_str(text.as_str());
        assert!(unit.is_err());
    }
}
