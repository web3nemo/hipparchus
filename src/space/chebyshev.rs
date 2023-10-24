use num::Float;

pub fn chebyshev<T: Float>(x: &[T], y: &[T]) -> T
{
    x.iter()
        .zip(y.iter())
        .map(|(a, &b)| a.sub(b).abs() )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests 
{
    use super::chebyshev;
    use float_cmp::assert_approx_eq;

    // Test chebyshev distance calculation on f32 vectors 
    #[test]
    fn test_chebyshev_f32()
    {
        assert_approx_eq!
        (
            f32,
            2.0,
            chebyshev::<f32>
            (
                &[0.0, 1.0],
                &[1.0, -1.0]
            )
        );
    }

    // Test chebyshev distance calculation on f64 vectors 
    #[test]
    fn test_chebyshev_f64()
    {
        assert_approx_eq!
        (
            f64,
            2.0,
            chebyshev::<f64>
            (
                &[0.0, 1.0],
                &[1.0, -1.0]
            )
        );
   }
}
