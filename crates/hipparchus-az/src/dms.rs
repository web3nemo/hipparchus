use num::Zero;
use crate::sign::{Sign, WithSign};

/// Struct for angle via combination of degrees, minutes and seconds (DMS).
pub struct DegreeMinuteSecond
{
    /// Sign of the angle
    sign: Sign,

    /// Degree part of the angle
    degree: u16,

    /// Minute part of the angle
    minute: u8,

    /// Second part of the angle
    second: f32,
}

impl DegreeMinuteSecond
{
    /// Create a new DMS instance.
    pub fn new(sign:Sign, degree:u16, minute:u8, second:f32) -> Self
    {
        assert!(minute < 60 && second >= 0.0 && second < 60.0);
        Self
        {
            sign: if degree.is_zero() && minute.is_zero() && second.is_zero() { Sign::Positive } else { sign},
            degree, minute, second
        }
    }

    /// Get the sign of the angle.
    pub fn sign(&self) -> Sign { self.sign }

    /// Get the degree part of the angle.
    pub fn degree(&self) -> u16 { self.degree }

    /// Get the minute part of the angle.
    pub fn minute(&self) -> u8 { self.minute }

    /// Get the second part of the angle.
    pub fn second(&self) -> f32 { self.second }

    /// Create DMS instance with degrees value of angle.
    pub fn with(value:f64) -> Self
    {
        let sign = value.sign();
        let value = value.abs();
        let degree = value as u16;
        let minute = ((value - degree as f64) * 60.0) as u8;
        let second = (value - degree as f64) * 3600.0 - (minute as f64) * 60.0;
        Self{ sign, degree, minute, second: second as f32}
    }

    /// Get the whole angle value in degrees.
    pub fn value(&self) -> f64
    {
        let v = self.degree as f64 + self.minute as f64 / 60.0 + self.second as f64 / 3600.0;
        match self.sign
        {
            Sign::Positive => v,
            Sign::Negative => -v,
        }
    }

    /// Get the fraction value in minutes (less than 1 degree): minutes + seconds.
    pub fn fraction(&self) -> f64
    {
        self.minute as f64 + self.second as f64 / 60.0
    }

    /// Create DMS instance with value of 0.0 degrees.
    pub fn zero() -> Self
    {
        Self::with(0.0)
    }

    /// Detect if DMS value is zero.
    pub fn is_zero(&self) -> bool
    {
        self.degree.is_zero() && self.minute.is_zero() && self.second.is_zero()
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_dms_size()
    {
        assert_eq!(8, std::mem::size_of::<DegreeMinuteSecond>());
    }

    #[rstest]
    #[case(60.51, 30.6, false, Sign::Positive, 60, 30, 36.0)]
    #[case(-60.51, 30.6, false, Sign::Negative, 60, 30, 36.0)]
    #[case(0.0, 0.0, true, Sign::Positive, 0, 0, 0.0)]
    fn test_dms_new
    (
        #[case] value: f64, #[case] fraction: f64, #[case] zero: bool,
        #[case] sign: Sign, #[case] degree: u16, #[case] minute: u8, #[case] second: f32
    )
    {
        let dms = DegreeMinuteSecond::new(sign, degree, minute, second);
        assert_eq!(sign, dms.sign());
        assert_eq!(degree, dms.degree());
        assert_eq!(minute, dms.minute());
        assert_approx_eq!(f32, second, dms.second());
        assert_approx_eq!(f64, value, dms.value());
        assert_approx_eq!(f64, fraction, dms.fraction());
        assert_eq!(zero, dms.is_zero());
    }

    #[rstest]
    #[case(60.51, 30.6, false, Sign::Positive, 60, 30, 36.0)]
    #[case(-60.51, 30.6, false, Sign::Negative, 60, 30, 36.0)]
    #[case(0.0, 0.0, true, Sign::Positive, 0, 0, 0.0)]
    fn test_dms_with
    (
        #[case] value: f64, #[case] fraction: f64, #[case] zero: bool,
        #[case] sign: Sign, #[case] degree: u16, #[case] minute: u8, #[case] second: f32
    )
    {
        let dms = DegreeMinuteSecond::with(value);
        assert_eq!(sign, dms.sign());
        assert_eq!(degree, dms.degree());
        assert_eq!(minute, dms.minute());
        assert_approx_eq!(f32, second, dms.second());
        assert_approx_eq!(f64, value, dms.value());
        assert_approx_eq!(f64, fraction, dms.fraction());
        assert_eq!(zero, dms.is_zero());
    }

    #[test]
    fn test_zero()
    {
        let dms = DegreeMinuteSecond::zero();
        assert_eq!(Sign::Positive, dms.sign());
        assert_eq!(0, dms.degree());
        assert_eq!(0, dms.minute());
        assert_approx_eq!(f32, 0.0, dms.second());
        assert_approx_eq!(f64, 0.0, dms.value());
        assert_approx_eq!(f64, 0.0, dms.fraction());
        assert_eq!(true, dms.is_zero());
    }
}