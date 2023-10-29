use crate::value::Fp;

pub fn lpnorm_inf<'a, T, I>(it: I) -> Option<T>
where
    T: Fp + 'a,
    I: Iterator<Item = &'a T>,
{
    let agg = it.max_by(|a, b| a.partial_cmp(b).unwrap());
    match agg
    {
        None => None,
        _ => Some(*(agg.unwrap()))
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;

    // Test Lp norm (p=inf)
    #[test]
    fn test_lpnorm_inf()
    {
        assert_approx_eq!
        (
            f32, 5.0,
            lpnorm_inf(vec![1.0, 2.0, 3.0, 4.0, 5.0].iter()).unwrap()
        );
    }

    // Test Lp norm (p=inf)
    #[test]
    fn test_lpnorm_inf_32_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            lpnorm_inf(e.iter())
        );
    }
}
