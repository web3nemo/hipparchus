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
        if self.is_positive() { Sign::Positive } else { Sign::Negative }
    }
}

#[repr(i8)]
#[derive(Debug, PartialEq, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum Coord
{
    Latitude = 1,
    Longitude = 2,
}

impl Coord
{
    pub fn max(self) -> u8
    {
        match self
        {
            Self::Latitude => 90,
            Self::Longitude => 180,
        }
    }

    pub fn min(self) -> u8
    {
        0
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_sign_pos()
    {
        let pos = 1.0f32;
        assert_eq!(Sign::Positive, pos.sign());
    }

    #[test]
    fn test_sign_neg()
    {
        let neg = -1.0f32;
        assert_eq!(Sign::Negative, neg.sign());
    }

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
}    
