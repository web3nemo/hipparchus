use num::{Float, FromPrimitive};

pub fn l0norm<'a, T, I>(it: I) -> Option<T>
where
    T: Float + FromPrimitive + 'a,
    I: Iterator<Item = &'a T>,
{
    let zero = T::from_i32(0).unwrap();
    let mut agg = 0;
    it.for_each(|v| match v
    {
        z if z.eq(&zero) => {},
        _ => agg += 1,
    });
    match agg
    {
        0 => None,
        _ => Some(T::from_usize(agg).unwrap()),
    }
}

#[cfg(test)]
mod tests 
{
    use super::l0norm;
    use float_cmp::assert_approx_eq;

    // Test L0 norm 
    #[test]
    fn test_l0norm()
    {
        assert_approx_eq!
        (
            f32, 5.0,
            l0norm(vec![1.0, 2.0, 3.0, 4.0, 5.0].iter()).unwrap()
        );
    }

    // Test L0 norm 
    #[test]
    fn test_l0norm_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            l0norm(e.iter())
        );
    }
}
