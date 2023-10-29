use crate::value::Fp;
use crate::mean::arithmetic::arithmetic;
use crate::mean::geometric::geometric;
use crate::mean::quadratic::quadratic;
use crate::mean::harmonic::harmonic;

pub enum MeanAlgorithm
{
    Arithmetic = 0,
    Geometric = 1,
    Quadratic = 2,
    Harmonic = 3,
}

pub trait Mean<'a, T>
where
    T: Fp + 'a,
    Self: Iterator<Item = &'a T> + 'a
{
    fn mean(self:Self, algo:MeanAlgorithm) -> Option<T>;
}

impl<'a, T, I> Mean<'a, T> for I
where
    T: Fp + 'a,
    I: Iterator<Item = &'a T> + 'a,
{
    fn mean(self:Self, algo:MeanAlgorithm) -> Option<T>
    {
        match algo
        {
            MeanAlgorithm::Arithmetic => arithmetic(self),
            MeanAlgorithm::Geometric => geometric(self),
            MeanAlgorithm::Quadratic => quadratic(self),
            MeanAlgorithm::Harmonic => harmonic(self),
        }
    }
}

#[cfg(test)]
mod tests 
{
    use crate::mean::traits::{MeanAlgorithm, Mean};
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_arithmetic()
    {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_approx_eq!
        (
            f32, 3.0,
            v.iter().mean(MeanAlgorithm::Arithmetic).unwrap()
        );
    }

    #[test]
    fn test_geometric()
    {
        let v = vec![1.0, 1.0, 1.0, 2.0, 4.0, 8.0];
        assert_approx_eq!
        (
            f32, 2.0,
            v.iter().mean(MeanAlgorithm::Geometric).unwrap()
        );
    }

    #[test]
    fn test_quadratic()
    {
        let v = vec![1.0, 7.0];
        assert_approx_eq!
        (
            f32, 5.0,
            v.iter().mean(MeanAlgorithm::Quadratic).unwrap()
        );
    }

    #[test]
    fn test_harmonic()
    {
        let v = vec![1.0, 1.0, 0.5, 0.25];
        assert_approx_eq!
        (
            f32, 0.5,
            v.iter().mean(MeanAlgorithm::Harmonic).unwrap()
        );
    }
}

