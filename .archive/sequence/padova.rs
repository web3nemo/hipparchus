use crate::value::Fp;

pub struct Padova<T:Fp>
{
    previous: T,
    current: T,
    next: T,
    neeext: T,

}

impl<T:Fp> Iterator for Padova<T>
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        let next = self.next;

        self.previous = self.current;
        self.current = self.next;
        self.next = self.neeext;
        self.neeext = self.previous + self.current;
        
        Some(next)
    }
}

impl<T:Fp> Padova<T>
{
    pub fn new() -> Padova<T>
    {
        Padova
        {
            previous: T::zero(),
            current: T::zero(),
            next: T::one(),
            neeext: T::one(),
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;

    // Test padova
    #[test]
    fn test_padova()
    {
        let mut f = Padova::<f32>::new();
        let expected = vec![1.0, 1.0, 1.0, 2.0, 2.0, 3.0, 4.0, 5.0, 7.0, 9.0, 12.0];
        for i in 0..expected.len()
        {
            assert_approx_eq!(f32, expected[i], f.next().unwrap());
        }
    }
}
