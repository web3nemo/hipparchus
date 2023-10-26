use std::ops::{Add, Div};
use num::FromPrimitive;

pub fn sma<'a, T, I>(it: I) -> Option<T>
where
    T: FromPrimitive + Copy + Add<Output=T> + Div<T, Output=T> + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut l:i32 = 0;
    let mut s:T = T::from_i32(0).unwrap();
    it.for_each(|t|
    {
        l += 1;
        s = s + t.clone();
    });

    match l
    {
        0 => None,
        _ =>
        {
            let l = T::from_i32(l).unwrap();
            let r = s.div(l);
            Some(r)
        },
    }
}

#[cfg(test)]
mod tests 
{
    use super::sma;

    // Test simple moving average
    #[test]
    fn test_sma()
    {
        assert_eq!
        (
            3,
            sma(vec![1, 2, 3, 4, 5].iter()).unwrap()
        );
    }

    // Test simple moving average 
    #[test]
    fn test_sma_empty()
    {
        assert_eq!
        (
            Option::<i32>::None,
            sma(vec![].iter())
        );
    }
}
