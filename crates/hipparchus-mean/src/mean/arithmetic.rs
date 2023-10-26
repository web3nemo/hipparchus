use std::ops::{Add, Div};
use num::FromPrimitive;

pub fn arithmetic<'a, T, I>(it: I) -> Option<T>
where
    T: FromPrimitive + Copy + Add<Output=T> + Div<T, Output=T> + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut total:i32 = 0;
    let mut agg:T = T::from_i32(0).unwrap();
    it.for_each(|v|
    {
        total += 1;
        agg = agg + v.clone();
    });

    match total
    {
        0 => None,
        _ =>
        {
            let t = T::from_i32(total).unwrap();
            let r = agg.div(t);
            Some(r)
        },
    }
}

#[cfg(test)]
mod tests 
{
    use super::arithmetic;

    // Test arithmetic mean 
    #[test]
    fn test_arithmetic()
    {
        assert_eq!
        (
            3,
            arithmetic(vec![1, 2, 3, 4, 5].iter()).unwrap()
        );
    }

    // Test arithmetic mean 
    #[test]
    fn test_arithmetic_equal()
    {
        assert_eq!
        (
            1,
            arithmetic(vec![1, 1, 1].iter()).unwrap()
        );
    }

    // Test arithmetic mean 
    #[test]
    fn test_arithmetic_empty()
    {
        let e = vec![] as Vec<i32>;
        assert_eq!
        (
            Option::<i32>::None,
            arithmetic(e.iter())
        );
    }
}
