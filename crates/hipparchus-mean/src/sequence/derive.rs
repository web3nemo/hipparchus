use crate::value::Fp;

pub struct DerivedIterator<'a,T,I> where
    T: Fp,
    I: Iterator<Item=T> + 'a,
{
    from: &'a mut I,
    f: fn(T)->T,
}

impl<'a,T,I> Iterator for DerivedIterator<'a,T,I> where
    T: Fp,
    I: Iterator<Item=T> + 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        let from = self.from.next().unwrap();
        let f = self.f;
        let result = f(from);
        Some(result)
    }
}

impl<'a,T,I> DerivedIterator<'a,T,I> where
    T: Fp,
    I: Iterator<Item=T> +'a,
{
    pub fn new(from:&'a mut I, f:fn(T)->T) -> DerivedIterator<'a, T,I>
    {
        DerivedIterator{ from, f }
    }
}

pub struct DerivedWithIterator<'a,T,I> where
    T: Fp,
    I: Iterator<Item=T> + 'a,
{
    from: &'a mut I,
    with: &'a mut I,
    f: fn(T, T)->T,
}

impl<'a,T,I> Iterator for DerivedWithIterator<'a,T,I> where
    T: Fp,
    I: Iterator<Item=T> + 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        let from = self.from.next().unwrap();
        let to = self.with.next().unwrap();
        let f = self.f;
        let result = f(from, to);
        Some(result)
    }
}

impl<'a,T,I> DerivedWithIterator<'a,T,I> where
    T: Fp,
    I: Iterator<Item=T> +'a,
{
    pub fn new(from:&'a mut I, with:&'a mut I, f:fn(T,T)->T) -> DerivedWithIterator<'a, T,I>
    {
        DerivedWithIterator{ from, with, f }
    }
}

pub trait Derive<'a,T,I> where
    T: Fp,
    I: Iterator<Item=T> + 'a,
{
    fn derive(&'a mut self, f:fn(T)->T) -> DerivedIterator<'a,T,I>;
    
    fn derivewith(&'a mut self, with:&'a mut I, f:fn(T,T)->T)-> DerivedWithIterator<'a,T,I>;
}

impl<'a,T,I> Derive<'a,T,I> for I where
    T: Fp,
    I: Iterator<Item=T> + 'a,
{
    fn derive(&'a mut self, f:fn(T)->T) -> DerivedIterator<'a,T,I>
    {
        DerivedIterator::new(self, f)
    }

    fn derivewith(&'a mut self, with:&'a mut I, f:fn(T,T)->T)-> DerivedWithIterator<'a,T,I>
    {
        DerivedWithIterator::new(self, with, f)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;
    
    use crate::sequence::arithmetic::ArithmeticSequence;

    // Test derive
    #[test]
    fn test_derive()
    {
        fn f(v:f32) -> f32 { v + 1.0 }
        let mut from = ArithmeticSequence::natural(false);
        let mut actual = from.derive(f);

        let expected = vec![2.0, 3.0, 4.0, 5.0];
        for i in 0..expected.len()
        {
            assert_approx_eq!(f32, expected[i], actual.next().unwrap());
        }
    }

    // Test derive with sequence
    #[test]
    fn test_deriveseq()
    {
        fn f(v:f32, s:f32) -> f32 { v + s }
        let mut from = ArithmeticSequence::<f32>::natural(false);
        let mut to = ArithmeticSequence::<f32>::natural(false);
        let mut actual = from.derivewith(&mut to, f);

        let expected = vec![2.0, 4.0, 6.0, 8.0];
        for i in 0..expected.len()
        {
            assert_approx_eq!(f32, expected[i], actual.next().unwrap());
        }
    }
}
