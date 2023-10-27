use num::{Float, FromPrimitive};
use crate::norm::l0norm::l0norm;
use crate::norm::l1norm::l1norm;
use crate::norm::l2norm::l2norm;
use crate::norm::lpnorm::lpnorm;
use crate::norm::lpnorm_inf::lpnorm_inf;

#[repr(i32)]
pub enum NormAlgorithm
{
    L0 = 0,
    L1 = 1,
    L2 = 2,
    Lp(f32),
    Linf = -1,
}

pub trait Norm<'a, T>
where
    T: Float + FromPrimitive + 'a,
    Self: Iterator<Item = &'a T>
{
    fn norm(self:Self, algo:NormAlgorithm) -> Option<T>;
}

impl<'a, T, I> Norm<'a, T> for I
where
    T: Float + FromPrimitive + 'a,
    I: Iterator<Item = &'a T>,
{
    fn norm(self:Self, algo:NormAlgorithm) -> Option<T>
    {
        match algo
        {
            NormAlgorithm::L0 => l0norm(self),
            NormAlgorithm::L1 => l1norm(self),
            NormAlgorithm::L2 => l2norm(self),
            NormAlgorithm::Lp(p) => lpnorm(self, p),
            NormAlgorithm::Linf => lpnorm_inf(self),
        }
    }
}

#[cfg(test)]
mod tests 
{
    use crate::norm::traits::{NormAlgorithm, Norm};
    use float_cmp::assert_approx_eq;

    // Test L0 norm
    #[test]
    fn test_l0norm()
    {
        let v = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        assert_approx_eq!
        (
            f32, 4.0,
            v.iter().norm(NormAlgorithm::L0).unwrap()
        );
    }

    // Test L1 norm 
    #[test]
    fn test_l1norm()
    {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_approx_eq!
        (
            f32, 15.0,
            v.iter().norm(NormAlgorithm::L1).unwrap()
        );
    }

    // Test L2 norm 
    #[test]
    fn test_l2norm()
    {
        let v = vec![3.0, 4.0];
        assert_approx_eq!
        (
            f32, 5.0,
            v.iter().norm(NormAlgorithm::L2).unwrap()
        );
    }

    // Test Lp norm 
    #[test]
    fn test_lpnorm()
    {
        let v = vec![3.0, 4.0];
        assert_approx_eq!
        (
            f32, 5.0,
            v.iter().norm(NormAlgorithm::Lp(2.0)).unwrap()
        );
    }

    // Test Lp norm (p=inf)
    #[test]
    fn test_lpnorm_inf()
    {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_approx_eq!
        (
            f32, 5.0,
            v.iter().norm(NormAlgorithm::Linf).unwrap()
        );
    }
}
