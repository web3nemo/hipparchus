use num::Float;
use std::iter::Sum;

pub fn hellinger<T: Float+Sum>(x: &[T], y: &[T]) -> T
{
    x.iter()
        .zip(y.iter())
        .map(|(p, &q)| (p.sqrt() - q.sqrt()).powi(2) )
        .sum::<T>()
        .div(T::from(2).unwrap())
        .sqrt()
}

#[cfg(test)]
mod tests 
{
    use super::hellinger;
    use float_cmp::assert_approx_eq;

    // Test hellinger distance calculation on f64 vectors 
    #[test]
    fn test_hellinger()
    {
        assert_approx_eq!
        (
            f32,
            1.0,
            hellinger::<f32>
            (
                &[0.0, 0.0],
                &[1.0, 1.0]
            )
        );
   }

    // Test hellinger distance calculation on f32 vectors 
    #[test]
    fn test_hellingery_zero()
    {
        assert_approx_eq!
        (
            f32,
            0.0,
            hellinger::<f32>
            (
                &[0.5, 0.5],
                &[0.5, 0.5]
            )
        );
    }
}