use num::Float;
use std::iter::Sum;

pub fn kullback_leibler<T: Float+Sum>(x: &[T], y: &[T]) -> T
{
    x.iter()
        .zip(y.iter())
        .map(|(p, &q)| p.mul(p.ln()-q.ln()) )
        .sum::<T>()
}

#[cfg(test)]
mod tests 
{
    use super::kullback_leibler;
    use float_cmp::assert_approx_eq;

    // Test cosine distance calculation on f32 vectors 
    #[test]
    fn test_kl_f32()
    {
        assert_approx_eq!
        (
            f32,
            0.0,
            kullback_leibler::<f32>
            (
                &[0.5, 0.5],
                &[0.5, 0.5]
            )
        );

        assert_approx_eq!
        (
            f32,
            11.512684,
            kullback_leibler::<f32>
            (
                &[0.00001, 0.99999],
                &[0.99999, 0.00001]
            )
        );
    }

    // Test cosine distance calculation on f64 vectors 
    #[test]
    fn test_kl_f64()
    {
        assert_approx_eq!
        (
            f64,
            0.0,
            kullback_leibler::<f64>
            (
                &[0.5, 0.5],
                &[0.5, 0.5]
            )
        );

        assert_approx_eq!
        (
            f64,
            11.51268520661093,
            kullback_leibler::<f64>
            (
                &[0.00001, 0.99999],
                &[0.99999, 0.00001]
            )
        );
   }
}