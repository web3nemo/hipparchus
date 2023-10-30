use crate::value::Fp;

pub struct ArithmeticSequence<T:Fp>
{
    next: T,
    difference: T,
}

impl<T:Fp> Iterator for ArithmeticSequence<T>
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        let current = self.next;
        self.next = current + self.difference;
        Some(current)
    }
}

impl<T:Fp> ArithmeticSequence<T>
{
    pub fn new(value:T, difference:T) -> ArithmeticSequence<T>
    {
        ArithmeticSequence
        {
            next: value,
            difference: difference,
        }
    }

    pub fn natural(zero:bool) -> ArithmeticSequence<T>
    {
        ArithmeticSequence
        {
            next: if zero { T::zero() } else { T::one() },
            difference: T::one(),
        }
    }

    pub fn odd() -> ArithmeticSequence<T>
    {
        ArithmeticSequence
        {
            next: T::one(),
            difference: T::from_i32(2).unwrap(),
        }
    }

    pub fn even(zero:bool) -> ArithmeticSequence<T>
    {
        let two = T::from_i32(2).unwrap();
        ArithmeticSequence
        {
            next: if zero { T::zero() } else { two },
            difference: two,
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;
    #[test]
    fn test_arithmetic()
    {
        let mut v = ArithmeticSequence::<f32>::new(0.0, 1.0);
        let expected = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        expected.iter().for_each(|&x|
        {
            assert_approx_eq!(f32, x, v.next().unwrap());
        });
    }

    #[test]
    fn test_arithmetic_natural()
    {
        let mut v = ArithmeticSequence::<f32>::natural(false);
        let expected = vec![1.0, 2.0, 3.0, 4.0];
        expected.iter().for_each(|&x|
        {
            assert_approx_eq!(f32, x, v.next().unwrap());
        });
    }
    
    #[test]
    fn test_arithmetic_natural_zero()
    {
        let mut v = ArithmeticSequence::<f32>::natural(true);
        let expected = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        expected.iter().for_each(|&x|
        {
            assert_approx_eq!(f32, x, v.next().unwrap());
        });
    }

    #[test]
    fn test_arithmetic_odd()
    {
        let mut v = ArithmeticSequence::<f32>::odd();
        let expected = vec![1.0, 3.0, 5.0, 7.0, 9.0];
        expected.iter().for_each(|&x|
        {
            assert_approx_eq!(f32, x, v.next().unwrap());
        });
    }

    #[test]
    fn test_arithmetic_even()
    {
        let mut v = ArithmeticSequence::<f32>::even(false);
        let expected = vec![2.0, 4.0, 6.0, 8.0];
        expected.iter().for_each(|&x|
        {
            assert_approx_eq!(f32, x, v.next().unwrap());
        });
    }

    #[test]
    fn test_arithmetic_even_zero()
    {
        let mut v = ArithmeticSequence::<f32>::even(true);
        let expected = vec![0.0, 2.0, 4.0, 6.0, 8.0];
        expected.iter().for_each(|&x|
        {
            assert_approx_eq!(f32, x, v.next().unwrap());
        });
    }
}
