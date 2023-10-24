use std::iter::Sum;
use num::Float;

pub fn canberra<T: Float+Sum>(x: &[T], y: &[T]) -> T
{
    x.iter()
        .zip(y.iter())
        .map( | (a, &b) | a.sub(b).abs() / a.abs().add(b.abs()) )
        .sum()
}

#[cfg(test)]
mod tests 
{
    use super::canberra;
    use float_cmp::assert_approx_eq;

    // Test canberra distance calculation on f32 vectors 
    #[test]
    fn test_canberra_f32()
    {
        assert_approx_eq!
        (
            f32,
            2.0,
            canberra::<f32>
            (
                &[0.0, 1.0],
                &[1.0, -1.0]
            )
        );
    }

    // Test canberra distance calculation on f64 vectors 
    #[test]
    fn test_manhattan_f64()
    {
        assert_approx_eq!
        (
            f64,
            2.0,
            canberra::<f64>
            (
                &[0.0, 1.0],
                &[1.0, -1.0]
            )
        );
   }
}
