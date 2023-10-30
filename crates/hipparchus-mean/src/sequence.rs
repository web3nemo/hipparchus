use std::{ops::{Add, Sub, Mul, Div}, iter::repeat_with};
use num::{FromPrimitive, Zero, One};

#[repr(i32)]
pub enum Sequence<T> where
    T: Add + Sub + Mul + Div<Output=T> + FromPrimitive + Zero + One + Copy + Clone,
{
    Arithmetic { init:T, difference:T } = 0,
    Geometric { init:T, ratio:T } = 1,
    Fibonacci = 20,
    Padova = 21,
    Catalan = 30,
    LookAndSay { with:usize } = 31,
}

impl<'a, T> Sequence<T> where
    T: Add + Sub + Mul + Div<Output=T> + FromPrimitive + Zero + One + Copy + Clone,
{
    pub fn vec(self, n:usize) -> Vec<T>
    {
        match self
        {
            Sequence::Arithmetic { init, difference } => repeat_with(Self::arithmetic(init, difference)).take(n).collect(),
            Sequence::Geometric { init, ratio } => repeat_with(Self::geometric(init, ratio)).take(n).collect(),
            Sequence::Fibonacci => repeat_with(Self::fibonacci()).take(n).collect(),
            Sequence::Padova => repeat_with(Self::padova()).take(n).collect(),
            Sequence::Catalan => repeat_with(Self::catalan()).take(n).collect(),
            Sequence::LookAndSay { with } => todo!(),
        }
    }

    fn arithmetic(init:T, difference:T) -> impl FnMut() -> T
    {
        let mut next = init;
        move ||
        {
            let current = next;
            next = next + difference;
            current
        }
    }

    fn geometric(init:T, ratio:T) -> impl FnMut() -> T
    {
        let mut next = init;
        move ||
        {
            let current = next;
            next = next * ratio;
            current
        }
    }

    fn fibonacci() -> impl FnMut() -> T
    {
        let mut current = T::zero();
        let mut next = T::one();
        move ||
        {
            let value = current;
            let sum = current + next;
            current = next;
            next = sum;
            value
        }
    }

    fn padova() -> impl FnMut() -> T
    {
        let mut previous = T::zero();
        let mut current = T::zero();
        let mut next = T::one();
        let mut neeext = T::one();

        move ||
        {
            let value = next;

            previous = current;
            current = next;
            next = neeext;
            neeext = previous + current;
            
            value
        }
    }

    fn catalan() -> impl FnMut() -> T
    {
        let mut next = T::one();
        let mut n = 0;
        move ||
        {
            let current = next;
            next = current * T::from_usize(4 * n + 2).unwrap() / T::from_usize(n + 2).unwrap();
            n += 1;
            current
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_sequence_arithmetic_i32()
    {
        let expected = vec![1, 2, 3, 4, 5];
        let n = expected.len();
        let actual = Sequence::Arithmetic { init: 1, difference: 1 }.vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_geometric_i32()
    {
        let expected = vec![1, 2, 4, 8, 16];
        let n = expected.len();
        let actual = Sequence::Geometric { init: 1, ratio: 2 }.vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_fibonacci_i32()
    {
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
        let n = expected.len();
        let actual = Sequence::Fibonacci.vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_padova_i32()
    {
        let expected = vec!
        [
            1,   1,   1,   2,   2,
            3,   4,   5,   7,   9,
            12,  16,  21,  28,  37,
            49,  65,  86,  114, 151
        ];
        let n = expected.len();
        let actual = Sequence::Padova.vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_catalan_i32()
    {
        let expected = vec!
        [
            1,          1,          2,          5,          14, 
            42,         132,        429,        1430,       4862,
            16796,      58786,      208012,     742900,     2674440,
            9694845,    35357670,   129644790,  477638700,  1767263190
        ];
        let n = expected.len();
        let actual = Sequence::Catalan.vec(n);
        assert_eq!(expected, actual);
    }
}
