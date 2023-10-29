use crate::value::Fp;

pub fn wma<T:Fp>(s: &[T]) -> Option<T>
{
    let zero = T::from_i32(0).unwrap();
    let mut total = 0;
    let mut weight = zero;
    let mut agg = zero;
    s.iter().for_each(|v|
    {
        total += 1;
        let w = T::from_i32(total).unwrap();
        weight = weight + w;
        agg = agg + *v * w;
    });
    match total
    {
        0 => None,
        _ => Some(agg / weight)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_wma()
    {
        let n = 5.0 as f32;
        let s = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = (2.0 * n + 1.0) / 3.0;
        assert_approx_eq!
        (
            f32, expected,
            wma(&s).unwrap()
        );
    }

    #[test]
    fn test_wma_eq()
    {
        let s = vec![1.0; 5];
        assert_approx_eq!
        (
            f32, 1.0,
            wma(&s).unwrap()
        );
    }

    #[test]
    fn test_wma_empty()
    {
        let s = vec![] as Vec<f32>;
        assert_eq!(None, wma(&s));
    }
}
