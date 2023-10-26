use std::iter::Sum;
use num::{Float, FromPrimitive};

// 2∗∑(Pi∗Qi)/(Pi+Qi)
pub fn harmonicmean<T>(x: &[T], y: &[T]) -> T
    where T:Float + Sum + FromPrimitive
{
    x.iter()
        .zip(y.iter())
        .map(|(a, &b)| a.mul(b) / a.add(b) )
        .sum::<T>()
        .mul(T::from(2).unwrap())
}

#[cfg(test)]
mod tests 
{
    use super::harmonicmean;
    use float_cmp::assert_approx_eq;

    // Test harmonic mean distance calculation on f32 vectors 
    #[test]
    fn test_harmonicmean()
    {
        assert_approx_eq!
        (
            f32,
            0.5,
            harmonicmean::<f32>
            (
                &[0.5, 0.5],
                &[0.5, 0.5]
            )
        );
    }
}