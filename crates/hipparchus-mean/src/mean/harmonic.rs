use std::ops::{Add, Div};
use num::traits::Inv;
use num::FromPrimitive;

pub fn harmonic<'a, T, I>(it: I) -> Option<T>
where
    T: FromPrimitive + Copy + Add<Output=T> + Div<T, Output=T> + Inv<Output=T> + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut total:i32 = 0;
    let mut agg:T = T::from_i32(0).unwrap();
    it.for_each(|v|
    {
        total += 1;
        agg = agg + v.clone().inv();
    });

    match total
    {
        0 => None,
        _ =>
        {
            let t = T::from_i32(total).unwrap();
            let r = agg.div(t).inv();
            Some(r)
        },
    }
}

#[cfg(test)]
mod tests 
{
    use super::harmonic;
    use float_cmp::assert_approx_eq;

    // Test harmonic mean 
    #[test]
    fn test_harmonic()
    {
        assert_approx_eq!
        (
            f32, 0.5,
            harmonic(vec![1.0, 1.0, 0.5, 0.25].iter()).unwrap()
        );
    }

    // Test harmonic mean 
    #[test]
    fn test_harmonic_equal()
    {
        assert_approx_eq!
        (
            f32, 1.0,
            harmonic(vec![1.0, 1.0, 1.0].iter()).unwrap()
        );
    }

    // Test harmonic mean 
    #[test]
    fn test_harmonic_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            harmonic(e.iter())
        );
    }
}
