use num::FromPrimitive;

pub fn lpnorm_inf<'a, T, I>(it: I) -> Option<T>
where
    T: FromPrimitive + PartialOrd + Copy + 'a,
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
    use super::lpnorm_inf;
    use float_cmp::assert_approx_eq;

    // Test Lp norm (p=inf)
    #[test]
    fn test_lpnorm_inf_i32()
    {
        let v = vec![1, 2, 3, 4, 5];
        let n = lpnorm_inf(v.iter()).unwrap();
        assert_eq!(5, n);
    }

    // Test Lp norm (p=inf)
    #[test]
    fn test_lpnorm_inf_i32_empty()
    {
        let e = vec![] as Vec<i32>;
        let n = lpnorm_inf(e.iter());
        assert_eq!(Option::<i32>::None, n);
    }

    // Test Lp norm (p=inf)
    #[test]
    fn test_lpnorm_inf_f32()
    {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let n = lpnorm_inf(v.iter()).unwrap();
        assert_approx_eq!(f32, 5.0, n);
    }

    // Test Lp norm (p=inf)
    #[test]
    fn test_lpnorm_inf_32_empty()
    {
        let e = vec![] as Vec<f32>;
        let n = lpnorm_inf(e.iter());
        assert_eq!(Option::<f32>::None, n);
    }
}
