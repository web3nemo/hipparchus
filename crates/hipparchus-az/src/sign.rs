use std::fmt::Display;
use std::str::FromStr;
use num::Signed;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Sign of a number
#[repr(i8)]
#[derive(Debug, PartialEq, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum Sign
{
    /// Positive sign
    Positive = 1,

    /// Negative sign
    Negative = -1,
}

impl Sign
{
    /// Get the sign abbreviation
    pub fn abbr(self) -> &'static str
    {
        match self
        {
            Self::Positive => "+",
            Self::Negative => "-",
        }
    }
}

/// Trait for types that have a sign
pub trait WithSign
{
    /// Get the sign of the value
    fn sign(&self) -> Sign;
}

/// Implement `WithSign` trait for `Signed` types
impl<T:Signed> WithSign for T
{
    fn sign(&self) -> Sign
    {
        if self.is_negative() { Sign::Negative } else { Sign::Positive }
    }
}

/// Display `Sign` enum as a single character.
impl Display for Sign
{
    /// Display `Sign` enum as a single character.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.abbr())
    }
}

/// Parse `Sign` enum from a single character.
impl FromStr for Sign
{
    /// Error type
    type Err = ();

    /// Parse `Sign` enum from a single character.
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "+" => Ok(Sign::Positive),
            "-" => Ok(Sign::Negative),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1.0f32, Sign::Positive)]
    #[case(-1.0f32, Sign::Negative)]
    #[case(0.0f32, Sign::Positive)]
    fn test_sign_pos(#[case] v: f32, #[case] e: Sign)
    {
        assert_eq!(e, v.sign());
    }

    #[rstest]
    #[case(Sign::Positive, "+")]
    #[case(Sign::Negative, "-")]
    fn test_sign_str(#[case] s: Sign, #[case] text: String)
    {
        assert_eq!(text, s.to_string());
        assert_eq!(s, Sign::from_str(text.as_str()).unwrap());
    }

    #[rstest]
    #[case("")]
    #[case("ABCD")]
    fn test_sign_str_error(#[case] text: String)
    {
        let unit = Sign::from_str(text.as_str());
        assert!(unit.is_err());
    }
}