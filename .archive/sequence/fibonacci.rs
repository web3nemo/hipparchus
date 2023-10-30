use crate::value::Fp;

pub struct Fibonacci<T:Fp>
{
    current: T,
    next: T,
}

impl<T:Fp> Iterator for Fibonacci<T>
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        let sum = self.current + self.next;
        self.current = self.next;
        self.next = sum;
        Some(self.current)
    }
}

impl<T:Fp> Fibonacci<T>
{
    pub fn new() -> Fibonacci<T>
    {
        Fibonacci
        {
            current: T::zero(),
            next: T::one(),
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;
    
    // Test fibonacci
    #[test]
    fn test_fibonacci()
    {
        let mut f = Fibonacci::<f32>::new();
        let expected = vec![1.0, 1.0, 2.0, 3.0, 5.0];
        for i in 0..expected.len()
        {
            assert_approx_eq!(f32, expected[i], f.next().unwrap());
        }
    }
}
