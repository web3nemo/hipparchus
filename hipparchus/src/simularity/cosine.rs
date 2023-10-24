use num::Float;
use std::iter::Sum;

pub fn cosine<T: Float+Sum>(x: &[T], y: &[T]) -> T
{
    let xn = x.iter().map(|&v| v*v ).sum::<T>().sqrt();
    let yn = y.iter().map(|&v| v*v ).sum::<T>().sqrt();
    let xy = x.iter().zip(y.iter()).map(|(a, &b)| a.mul(b) ).sum::<T>();
    xy / (xn * yn)
}

#[cfg(test)]
mod tests 
{
    use super::cosine;
    use float_cmp::assert_approx_eq;

    // Test cosine distance calculation on f32 vectors 
    #[test]
    fn test_cosine_f32()
    {
        assert_approx_eq!
        (
            f32,
            0.0,
            cosine::<f32>
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
            cosine::<f64>
            (
                &[1.0, 1.0],
                &[1.0, -1.0]
            )
        );
   }
}