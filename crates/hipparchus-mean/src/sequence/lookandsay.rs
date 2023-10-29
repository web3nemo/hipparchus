use crate::value::Fp;
use std::marker::PhantomData;

pub struct LookAndSay<'a,T:Fp>
{
    v: Vec<i8>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T:Fp> Iterator for LookAndSay<'a,T>
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        let current = self.current();

        // look-and-say for next vector
        let mut v = Vec::new();
        let mut digit = self.v[0];
        let mut total = 0;
        self.v.iter().for_each(|&i|
        {
            if i == digit
            {
                total += 1;
            }
            else
            {
                v.push(total);
                v.push(digit);
                total = 1;
                digit = i;
            }
        });
        v.push(total);
        v.push(digit);
        self.v = v;

        Some(current)
    }
}

impl<'a,T:Fp> LookAndSay<'a,T>
{
    pub fn new() -> LookAndSay<'a,T>
    {
        Self::with(1)
    }

    pub fn with(data:i32) -> LookAndSay<'a,T>
    {
        Self::from
        (
            data.to_string()
                .chars()
                .map(|c|c.to_digit(10).unwrap() as i8)
                .collect()
        )
    }

    pub fn from(v: Vec<i8>) -> LookAndSay<'a,T> where
    {
        LookAndSay
        {
            v,
            phantom: PhantomData
        }
    }

    fn current(&self) -> T
    {
        T::from_u64
        (
            self.v
                .iter()
                .fold(0u64, |s,&x|s * 10 + x as u64)
        ).unwrap()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_lookandsay_new()
    {
        let expected = vec![1i8];
        let s = LookAndSay::<f64>::new();
        assert_eq!(expected, s.v);
    }

    #[test]
    fn test_lookandsay_with()
    {
        let expected = vec![1i8, 2, 2, 1];
        let s = LookAndSay::<f64>::with(1221);
        assert_eq!(expected, s.v);
    }

    #[test]
    fn test_lookandsay_from()
    {
        let expected = vec![1i8, 2, 2, 1];
        let s = LookAndSay::<f64>::from(expected.clone());
        assert_eq!(expected, s.v);
    }

    #[test]
    fn test_lookandsay_current()
    {
        let expected = 1221;
        let s = LookAndSay::<f64>::with(expected);
        let actual = s.current();
        assert_approx_eq!(f64, expected as f64, actual);
    }

    #[test]
    fn test_lookandsay()
    {
        let expected = vec!
        [
            1u64,
            11,
            21,
            1211,
            111221,
            312211,
            13112221,
            1113213211,
            31131211131221,
            13211311123113112211,
        ];
        let mut s = LookAndSay::<f64>::new();
        for i in 0..10
        {
            let expected = expected[i] as f64;
            let actual = s.next().unwrap();
            assert_approx_eq!(f64, expected, actual);
        }
    }
}