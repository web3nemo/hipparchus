use crate::value::Fp;

pub fn harmonic<'a, T, I>(it: I) -> Option<T>
where
    T: Fp + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut total:i32 = 0;
    let one = T::from_i32(1).unwrap();
    let mut agg:T = T::from_i32(0).unwrap();
    it.for_each(|v|
    {
        total += 1;
        agg = agg + one / (*v);
    });

    match total
    {
        0 => None,
        _ =>
        {
            let t = T::from_i32(total).unwrap();
            let r = t / agg;
            Some(r)
        },
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
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
