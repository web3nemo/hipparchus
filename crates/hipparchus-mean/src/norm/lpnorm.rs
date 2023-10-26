use std::ops::AddAssign;
use num::{Float, FromPrimitive, traits::Inv};

pub fn lpnorm<'a, T, I>(it: I, p:T) -> Option<T>
where
    T: Float + FromPrimitive + Copy + AddAssign + Inv<Output=T> + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut total:i32 = 0;
    let mut agg = T::from_i32(0).unwrap();
    it.for_each(|v|
    {
        total += 1;
        agg += (*v).powf(p);
    });
    match total
    {
        0 => None,
        _ => Some(agg.powf(p.inv())),
    }
}

#[cfg(test)]
mod tests 
{
    use super::lpnorm;
    use float_cmp::assert_approx_eq;

    // Test Lp norm 
    #[test]
    fn test_lpnorm()
    {
        assert_approx_eq!
        (
            f32, 5.0,
            lpnorm(vec![3.0, 4.0].iter(), 2.0).unwrap()
        );
    }

    // Test Lp norm 
    #[test]
    fn test_lpnorm_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            lpnorm(e.iter(), 2.0)
        );
    }
}
