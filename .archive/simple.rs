use crate::value::Fp;
use crate::mean::Mean;

pub fn sma<T:Fp>(s: &[T]) -> Option<T>
{
    s.iter().arithmetic_mean()
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_sma()
    {
        let s = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_approx_eq!
        (
            f32, 3.0,
            sma(&s).unwrap()
        );
    }

    #[test]
    fn test_sma_eq()
    {
        let s = vec![1.0; 5];
        assert_approx_eq!
        (
            f32, 1.0,
            sma(&s).unwrap()
        );
    }

    #[test]
    fn test_sma_empty()
    {
        let s = vec![] as Vec<f32>;
        assert_eq!(None, sma(&s));
    }
}
