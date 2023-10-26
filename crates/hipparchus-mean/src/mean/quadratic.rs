use num::{FromPrimitive, Float};

pub fn quadratic<'a, T, I>(it: I) -> Option<T>
where
    T: Float + FromPrimitive + Copy + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut total:i32 = 0;
    let mut agg:T = T::from_i32(0).unwrap();
    it.for_each(|v|
    {
        total += 1;
        agg = agg + v.mul(v.clone());
    });

    match total
    {
        0 => None,
        _ =>
        {
            let t = T::from_i32(total).unwrap();
            let r = agg.div(t).sqrt();
            Some(r)
        },
    }
}

#[cfg(test)]
mod tests 
{
    use super::quadratic;
    use float_cmp::assert_approx_eq;

    // Test quadratic mean 
    #[test]
    fn test_quadratic()
    {
        assert_approx_eq!
        (
            f32, 5.0,
            quadratic(vec![1.0, 7.0].iter()).unwrap()
        );
    }

    // Test quadratic mean 
    #[test]
    fn test_quadratic_equal()
    {
        assert_approx_eq!
        (
            f32, 1.0,
            quadratic(vec![1.0, 1.0, 1.0].iter()).unwrap()
        );
    }

    // Test quadratic mean 
    #[test]
    fn test_quadratic_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            quadratic(e.iter())
        );
    }
}
