use crate::value::Fp;
use float_cmp::approx_eq;

use crate::norm::l0norm::l0norm;
use crate::norm::l1norm::l1norm;
use crate::norm::l2norm::l2norm;
use crate::norm::lpnorm_inf::lpnorm_inf;

pub fn lpnorm<'a, T, I>(it: I, p:f32) -> Option<T>
where
    T: Fp + 'a,
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
            let k = T::from_f32(p).unwrap();
            let mut empty = true;
            let sum = it.fold(T::zero(), |s,&x|
            {
                empty = false;
                s + x.powf(k)
            });
            match empty
            {
                true => None,
                false => Some(sum.powf(T::one()/k)),
            }
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
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
