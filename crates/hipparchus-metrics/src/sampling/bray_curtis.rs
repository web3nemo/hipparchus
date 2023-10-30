use std::iter::Sum;
use num::Float;

pub fn bray_curtis<T>(x: &[T], y: &[T]) -> T
    where T: Float + Sum
{
    let sx:T = x.iter().map(|&v| v).sum();
    let sy:T = y.iter().map(|&v| v).sum();
    let sxy = x.iter().zip(y.iter()).map(|(a, &b)| a.min(b) ).sum();
    T::from(1).unwrap() - T::from(2).unwrap() * sxy / (sx + sy)
}

#[cfg(test)]
mod tests 
{
    use super::bray_curtis;
    use float_cmp::assert_approx_eq;

    // Test bray-curtis distance
    #[test]
    fn test_bray_curtis()
    {
        assert_approx_eq!
        (
            f32,
            0.39393938,
            bray_curtis::<f32>
            (
                &[6.0, 7.0, 4.0],
                &[10.0, 0.0, 6.0]
            )
        );
    }

    // Test bray-curtis distance
    #[test]
    fn test_bray_curtis_zero()
    {
        assert_approx_eq!
        (
            f32,
            0.0,
            bray_curtis::<f32>
            (
                &[1.0, 2.0, 3.0],
                &[1.0, 2.0, 3.0]
            )
        );
    }
}