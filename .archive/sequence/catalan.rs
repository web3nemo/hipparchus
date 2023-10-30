use crate::value::Fp;

pub struct Catalan<T:Fp>
{
    next: T,
    n: i32,
}

impl<T:Fp> Iterator for Catalan<T>
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        let current = self.next;
        self.next = current * T::from_i32(4 * self.n + 2).unwrap() / T::from_i32(self.n + 2).unwrap();
        self.n += 1;
        Some(current)
    }
}

impl<T:Fp> Catalan<T>
{
    pub fn new() -> Catalan<T>
    {
        Catalan
        {
            next: T::one(),
            n: 0,
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;

    // Test catalan
    #[test]
    fn test_catalan()
    {
        let mut v = Catalan::<f32>::new();
        let expected = vec!
        [
            1.0, 1.0, 2.0, 5.0, 14.0, 
            42.0, 132.0, 429.0, 1430.0, 4862.0,
            16796.0, 58786.0, 208012.0, 742900.0, 2674440.0,
            9694845.0, 35357670.0, 129644790.0, 477638700.0, 1767263190.0
        ];
        for i in 0..expected.len()
        {
            assert_approx_eq!(f32, expected[i], v.next().unwrap());
        }
    }
}

