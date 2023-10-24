use num::{Float, FromPrimitive};
use std::iter::Sum;
use super::kld::dkl;

pub fn djs<T: Float+Sum+FromPrimitive>(x: &[T], y: &[T]) -> T
{
    let half = T::from(0.5f64).unwrap();
    let v = x.iter()
        .zip(y.iter())
        .map(|(p, &q)| p.add(q).mul(half) )
        .collect::<Vec<T>>();
    let m = v.as_slice().try_into().unwrap();
    ( dkl(x, m) + dkl(y, m) ) * half
}

#[cfg(test)]
mod tests 
{
    use super::djs;
    use float_cmp::assert_approx_eq;

    // Test Jensen–Shannon distance calculation on f32 distribution
    #[test]
    fn test_js_f32()
    {
        assert_approx_eq!
        (
            f32,
            0.0,
            djs::<f32>
            (
                &[0.5, 0.5],
                &[0.5, 0.5]
            )
        );

        assert_approx_eq!
        (
            f32,
            0.6930221,
            djs::<f32>
            (
                &[0.00001, 0.99999],
                &[0.99999, 0.00001]
            )
        );
    }

    // Test Jensen–Shannon distance calculation on f64 distribution 
    #[test]
    fn test_js_f64()
    {
        assert_approx_eq!
        (
            f64,
            0.0,
            djs::<f64>
            (
                &[0.5, 0.5],
                &[0.5, 0.5]
            )
        );

        assert_approx_eq!
        (
            f64,
            0.6930220513552958,
            djs::<f64>
            (
                &[0.00001, 0.99999],
                &[0.99999, 0.00001]
            )
        );
   }
}