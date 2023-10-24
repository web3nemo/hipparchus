use num;
use num::Float;
use std::iter::Sum;

pub fn euclidean<T: Float+Sum>(x: &[T], y: &[T]) -> T
{
    x.iter()
        .zip(y.iter())
        .map(|(a, &b)| a.sub(b) )
        .map(|v| v * v)
        .sum::<T>()
        .sqrt()
}

#[cfg(test)]
mod tests 
{
    use super::euclidean;
    use float_cmp::assert_approx_eq;

    // Test euclidean distance calculation on f32 vectors 
    #[test]
    fn test_euclidean_f32()
    {
        assert_approx_eq!
        (
            f32,
            2.0,
            euclidean::<f32>
            (
                &[1.0, 1.0],
                &[1.0, -1.0]
            )
        );
    }

    // Test euclidean distance calculation on f64 vectors 
    #[test]
    fn test_euclidean_f64()
    {
        assert_approx_eq!
        (
            f64,
            2.0,
            euclidean::<f64>
            (
                &[1.0, 1.0],
                &[1.0, -1.0]
            )
        );
   }
}