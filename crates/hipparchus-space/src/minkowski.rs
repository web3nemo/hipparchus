use std::iter::Sum;
use num::{Float, traits::Inv};

pub fn minkowski<T: Float + Sum + Inv<Output=T>>(x: &[T], y: &[T], p:T) -> T
{
    x.iter()
        .zip(y.iter())
        .map(|(a, &b)| a.sub(b).abs() )
        .map(|v| v.powf(p))
        .sum::<T>()
        .powf(p.inv())
}

#[cfg(test)]
mod tests 
{
    use super::minkowski;
    use float_cmp::assert_approx_eq;

    // Test minkowski distance calculation on f32 vectors 
    #[test]
    fn test_minkowski_f32()
    {
        assert_approx_eq!
        (
            f32,
            3.0,
            minkowski::<f32>
            (
                &[0.0, 1.0],
                &[1.0, -1.0],
                1.0
            )
        );

        assert_approx_eq!
        (
            f32,
            5.0f32.sqrt(),
            minkowski::<f32>
            (
                &[0.0, 1.0],
                &[1.0, -1.0],
                2.0
            )
        );
    }

    // Test minkowski distance calculation on f64 vectors 
    #[test]
    fn test_minkowski_f64()
    {
        assert_approx_eq!
        (
            f64,
            3.0,
            minkowski::<f64>
            (
                &[0.0, 1.0],
                &[1.0, -1.0],
                1.0
            )
        );

        assert_approx_eq!
        (
            f64,
            5.0f64.sqrt(),
            minkowski::<f64>
            (
                &[0.0, 1.0],
                &[1.0, -1.0],
                2.0
            )
        );
   }
}
