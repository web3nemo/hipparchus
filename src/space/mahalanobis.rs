use std::{iter::Sum, ops::AddAssign, fmt::Debug};
use num::Float;
use ndarray::{prelude::*, ScalarOperand};
use ndarray_inverse::Inverse;

pub fn mahalanobis<T:Float+Sum+AddAssign+Debug+ScalarOperand>(x: &Array1<T>, y: &Array1<T>, cov: &Array2<T>) -> T
{
    let delta = y - x;
    let delta_t = delta.t();
    let cov_inv = cov.inv().unwrap();
    delta_t.dot(&cov_inv).dot(&delta).sqrt()
}

#[cfg(test)]
mod tests 
{
    use super::mahalanobis;
    use float_cmp::assert_approx_eq;
    use ndarray::{Array2, array};

    // Test mahalanobis distance calculation on f64 vectors 
    #[test]
    fn test_mahalanobis_f64()
    {
        assert_approx_eq!
        (
            f64,
            2.4482359438019334,
            mahalanobis::<f64>
            (
                &array!(11.0,  9.0, 35.0),
                &array!(15.0, 46.0, 13.0),
                &Array2::from_shape_vec( (3,3), vec![34.92, 26.58, -6.08, 26.58, 264.25, -152.08, -6.08, -152.08, 90.92]).unwrap()
            )
        );
   }
}