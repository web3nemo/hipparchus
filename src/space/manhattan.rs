use std::iter::Sum;
use num::Float;

pub fn manhattan<T: Float+Sum>(x: &[T], y: &[T]) -> T
{
    x.iter()
        .zip(y.iter())
        .map(|(a, &b)| a.sub(b).abs())
        .sum()
}

#[cfg(test)]
mod tests 
{
    use super::manhattan;
    use float_cmp::assert_approx_eq;

    // Test manhattan distance calculation on f32 vectors 
    #[test]
    fn test_manhattan_f32()
    {
        assert_approx_eq!
        (
            f32,
            3.0,
            manhattan::<f32>
            (
                &[0.0, 1.0],
                &[1.0, -1.0]
            )
        );
    }

    // Test manhattan distance calculation on f64 vectors 
    #[test]
    fn test_manhattan_f64()
    {
        assert_approx_eq!
        (
            f64,
            3.0,
            manhattan::<f64>
            (
                &[0.0, 1.0],
                &[1.0, -1.0]
            )
        );
   }
}
