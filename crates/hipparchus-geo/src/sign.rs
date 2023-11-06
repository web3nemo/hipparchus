use std::fmt::Display;
use std::str::FromStr;
use num::Signed;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(i8)]
#[derive(Debug, PartialEq, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum Sign
{
    Positive = 1,
    Negative = -1,
}

impl Sign
{
    pub fn abbr(self) -> &'static str
    {
        match self
        {
            Self::Positive => "+",
            Self::Negative => "-",
        }
    }
}

pub trait WithSign
{
    fn sign(&self) -> Sign;
}

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.abbr())
    }
}

/// Parse `Sign` enum from a single character.
impl FromStr for Sign
{
    type Err = ();

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
}