use crate::value::Fp;

pub struct GeometricProgression<T:Fp>
{
    next: T,
    ratio: T,
}

impl<T:Fp> Iterator for GeometricProgression<T>
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        let current = self.next;
        self.next = current * self.ratio;
        Some(current)
    }
}

impl<T:Fp> GeometricProgression<T>
{
    pub fn new(value:T, ratio:T) -> GeometricProgression<T>
    {
        GeometricProgression
        {
            next: value,
            ratio: ratio,
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;
    
    // Test arithmetic sequence
    #[test]
    fn test_arithmetic()
    {
        let mut v = GeometricProgression::<f32>::new(1.0, 2.0);
        let expected = vec![1.0, 2.0, 4.0, 8.0, 16.0];
        for i in 0..expected.len()
        {
            assert_approx_eq!(f32, expected[i], v.next().unwrap());
        }
    }
}
