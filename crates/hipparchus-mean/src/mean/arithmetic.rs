use num::{FromPrimitive, Float};

pub fn arithmetic<'a, T, I>(it: I) -> Option<T>
where
    T: Float + FromPrimitive + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut total:i32 = 0;
    let mut agg:T = T::from_i32(0).unwrap();
    it.for_each(|v|
    {
        total += 1;
        agg = agg + *v;
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
    use float_cmp::assert_approx_eq;

    // Test arithmetic mean 
    #[test]
    fn test_arithmetic()
    {
        assert_approx_eq!
        (
            f32, 3.0,
            arithmetic(vec![1.0, 2.0, 3.0, 4.0, 5.0].iter()).unwrap()
        );
    }

    // Test arithmetic mean 
    #[test]
    fn test_arithmetic_equal()
    {
        assert_approx_eq!
        (
            f32, 1.0,
            arithmetic(vec![1.0, 1.0, 1.0].iter()).unwrap()
        );
    }

    // Test arithmetic mean 
    #[test]
    fn test_arithmetic_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            arithmetic(e.iter())
        );
    }
}
