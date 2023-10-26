use num::{FromPrimitive, Float};

pub fn geometric<'a, T, I>(it: I) -> Option<T>
where
    T: Float + FromPrimitive + Copy + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut total:i32 = 0;
    let mut agg:T = T::from_i32(1).unwrap();
    it.for_each(|v|
    {
        total += 1;
        agg = agg.mul(v.clone());
    });

    match total
    {
        0 => None,
        _ =>
        {
            let t = T::from_i32(1).unwrap().div(T::from_i32(total).unwrap());
            let r = agg.powf(t);
            Some(r)
        },
    }
}

#[cfg(test)]
mod tests 
{
    use super::geometric;
    use float_cmp::assert_approx_eq;

    // Test geometric mean 
    #[test]
    fn test_geometric()
    {
        assert_approx_eq!
        (
            f32, 2.0,
            geometric(vec![1.0, 1.0, 1.0, 2.0, 4.0, 8.0].iter()).unwrap()
        );
    }

    // Test geometric mean 
    #[test]
    fn test_geometric_equal()
    {
        assert_approx_eq!
        (
            f32, 1.0,
            geometric(vec![1.0, 1.0, 1.0].iter()).unwrap()
        );
    }

    // Test geometric mean 
    #[test]
    fn test_geometric_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            geometric(e.iter())
        );
    }
}
