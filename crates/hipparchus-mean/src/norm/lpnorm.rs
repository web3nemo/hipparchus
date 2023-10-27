use num::{Float, FromPrimitive};
use float_cmp::approx_eq;

use crate::norm::l0norm::l0norm;
use crate::norm::l1norm::l1norm;
use crate::norm::l2norm::l2norm;
use crate::norm::lpnorm_inf::lpnorm_inf;

pub fn lpnorm<'a, T, I>(it: I, p:f32) -> Option<T>
where
    T: Float + FromPrimitive + 'a,
    I: Iterator<Item = &'a T>,
{
    match p
    {
        p if approx_eq!(f32, p, 0.0) => l0norm(it),
        p if approx_eq!(f32, p, 1.0) => l1norm(it),
        p if approx_eq!(f32, p, 2.0) => l2norm(it),
        p if p.is_infinite() => lpnorm_inf(it),
        _ =>
        {
            let mut total:i32 = 0;
            let mut agg = T::from_i32(0).unwrap();
            it.for_each(|v|
            {
                total += 1;
                agg = agg + (*v).powf(T::from_f32(p).unwrap());
            });
            match total
            {
                0 => None,
                _ => Some(agg.powf(T::from_f32(1.0 / p).unwrap())),
            }
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::lpnorm;
    use float_cmp::assert_approx_eq;

    // Test Lp norm 
    #[test]
    fn test_lpnorm()
    {
        assert_approx_eq!
        (
            f32, 3.0,
            lpnorm(vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0].iter(), 3.0).unwrap()
        );
    }

    // Test Lp norm 
    #[test]
    fn test_lpnorm_l2()
    {
        assert_approx_eq!
        (
            f32, 5.0,
            lpnorm(vec![3.0, 4.0].iter(), 2.0).unwrap()
        );
    }

    // Test Lp norm 
    #[test]
    fn test_lpnorm_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            lpnorm(e.iter(), 3.0)
        );
    }
}
