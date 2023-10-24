use num::Float;
use std::iter::Sum;

pub fn crossentropy<T: Float+Sum>(x: &[T], y: &[T]) -> T
{
    x.iter()
        .zip(y.iter())
        .map(|(p, &q)| -p.mul(q.ln()) )
        .sum::<T>()
}

#[cfg(test)]
mod tests 
{
    use super::crossentropy;
    use float_cmp::assert_approx_eq;

    // Test cosine distance calculation on f32 vectors 
    #[test]
    fn test_crossentropy_f32()
    {
        assert_approx_eq!
        (
            f32,
            0.693147,
            crossentropy::<f32>
            (
                &[0.5, 0.5],
                &[0.5, 0.5]
            )
        );
    }

    // Test cosine distance calculation on f64 vectors 
    #[test]
    fn test_crossentropy_f64()
    {
        assert_approx_eq!
        (
            f64,
            0.6931471805599453,
            crossentropy::<f64>
            (
                &[0.5, 0.5],
                &[0.5, 0.5]
            )
        );
   }
}