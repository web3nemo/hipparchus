use std::ops::AddAssign;
use num::{Float, FromPrimitive};

pub fn l1norm<'a, T, I>(it: I) -> Option<T>
where
    T: Float + FromPrimitive + Copy + AddAssign + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut total:i32 = 0;
    let mut agg = T::from_i32(0).unwrap();
    it.for_each(|v|
    {
        total += 1;
        agg += *v;
    });
    match total
    {
        0 => None,
        _ => Some(agg),
    }
}

#[cfg(test)]
mod tests 
{
    use super::l1norm;
    use float_cmp::assert_approx_eq;

    // Test L1 norm 
    #[test]
    fn test_l1norm()
    {
        assert_approx_eq!
        (
            f32, 15.0,
            l1norm(vec![1.0, 2.0, 3.0, 4.0, 5.0].iter()).unwrap()
        );
    }

    // Test L1 norm 
    #[test]
    fn test_l1norm_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            l1norm(e.iter())
        );
    }
}
