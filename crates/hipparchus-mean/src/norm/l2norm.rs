use std::ops::AddAssign;
use num::{Float, FromPrimitive};

pub fn l2norm<'a, T, I>(it: I) -> Option<T>
where
    T: Float + FromPrimitive + Copy + AddAssign + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut total:i32 = 0;
    let mut agg = T::from_i32(0).unwrap();
    it.for_each(|v|
    {
        total += 1;
        agg += *v * *v;
    });
    match total
    {
        0 => None,
        _ => Some(agg.sqrt()),
    }
}

#[cfg(test)]
mod tests 
{
    use super::l2norm;
    use float_cmp::assert_approx_eq;

    // Test L2 norm 
    #[test]
    fn test_l2norm()
    {
        assert_approx_eq!
        (
            f32, 5.0,
            l2norm(vec![3.0, 4.0].iter()).unwrap()
        );
    }

    // Test L2 norm 
    #[test]
    fn test_l2norm_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            l2norm(e.iter())
        );
    }
}
