use std::{fmt::Display, str::FromStr};

#[repr(i8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Unit
{
    Degree = 0,
    Minute = 1,
    Second = 2, 
    Radian = -1,
}

impl Unit
{
    pub fn abbr(self) -> &'static str
    {
        match self
        {
            Self::Degree => "°",
            Self::Minute => "'",
            Self::Second => "\"",
            Self::Radian => "rad",
        }
    }

    pub fn coefficient(self) -> f64
    {
        match self
        {
            Self::Degree => 1.0,
            Self::Minute => 60.0,
            Self::Second => 3600.0,
            Self::Radian => std::f64::consts::PI / 180.0,
        }
    }

    pub fn convert(value: f64, from: Unit, to: Unit) -> f64
    {
        value / from.coefficient() * to.coefficient()
    }
}

/// Display `Unit` enum as a single character.
impl Display for Unit
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.abbr())
    }
}

/// Parse `Unit` enum from a single character.
impl FromStr for Unit
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "°" => Ok(Unit::Degree),
            "'" => Ok(Unit::Minute),
            "\"" => Ok(Unit::Second),
            "rad" => Ok(Unit::Radian),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(100.0, Unit::Degree)]
    #[case(100.0, Unit::Minute)]
    #[case(100.0, Unit::Second)]
    #[case(100.0, Unit::Radian)]
    fn test_unit_convert_same(#[case] value: f64, #[case] unit: Unit)
    {
        let actual = Unit::convert(value, unit, unit);
        assert_approx_eq!(f64, value, actual);
    }

    #[rstest]
    #[case(100.0, Unit::Degree, Unit::Minute, 6000.0)]
    #[case(100.0, Unit::Degree, Unit::Second, 360000.0)]
    #[case(100.0, Unit::Minute, Unit::Second, 6000.0)]
    fn test_unit_convert_dms(#[case] value: f64, #[case] from: Unit, #[case] to: Unit, #[case] expected: f64)
    {
        let actual = Unit::convert(value, from, to);
        assert_approx_eq!(f64, expected, actual);

        let actual = Unit::convert(actual, to, from);
        assert_approx_eq!(f64, value, actual);
    }

    #[rstest]
    #[case(100.0, Unit::Degree, 1.7453292519943295)]
    #[case(100.0, Unit::Minute, 0.02908882086657216)]
    #[case(100.0, Unit::Second, 0.000484813681109536)]
    fn test_unit_convert_radian(#[case] value: f64, #[case] from: Unit, #[case] expected: f64)
    {
        let actual = Unit::convert(value, from, Unit::Radian);
        assert_approx_eq!(f64, expected, actual);

        let actual = Unit::convert(actual, Unit::Radian, from);
        assert_approx_eq!(f64, value, actual);
    }

    #[rstest]
    #[case(Unit::Degree, "°")]
    #[case(Unit::Minute, "'")]
    #[case(Unit::Second, "\"")]
    #[case(Unit::Radian, "rad")]
    fn test_unit_str(#[case] unit: Unit, #[case] text: String)
    {
        assert_eq!(text, unit.to_string());
        assert_eq!(unit, Unit::from_str(text.as_str()).unwrap());
    }

}