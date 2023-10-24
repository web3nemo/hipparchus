use num::Float;
use std::iter::Sum;

pub fn dotproduct<T: Float+Sum>(x: &[T], y: &[T]) -> T
{
    x.iter()
        .zip(y.iter())
        .map(|(a, &b)| a.mul(b) )
        .sum::<T>()
}

#[cfg(test)]
mod tests 
{
    use super::dotproduct;
    use float_cmp::assert_approx_eq;

    // Test cosine distance calculation on f32 vectors 
    #[test]
    fn test_cosine_f32()
    {
        assert_approx_eq!
        (
            f32,
            0.0,
            dotproduct::<f32>
            (
                &[1.0, 1.0],
                &[1.0, -1.0]
            )
        );
    }

    // Test cosine distance calculation on f64 vectors 
    #[test]
    fn test_cosine_f64()
    {
        assert_approx_eq!
        (
            f64,
            0.0,
            dotproduct::<f64>
            (
                &[1.0, 1.0],
                &[1.0, -1.0]
            )
        );
   }
}