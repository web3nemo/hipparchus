use num::Signed;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(i8)]
#[derive(Debug, PartialEq, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum Sign
{
    Positive = 1,
    Negative = -1,
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
}
