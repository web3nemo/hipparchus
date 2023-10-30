use std::{ops::{Add, Sub, Mul, Div}, iter::repeat_with};
use num::{FromPrimitive, Zero, One};

#[repr(i32)]
pub enum Sequence<T> where
    T: Add + Sub + Mul + Div<Output=T> + FromPrimitive + Zero + One + Copy + Clone,
{
    Arithmetic { init:T, difference:T } = 0,
    Geometric { init:T, ratio:T } = 1,
    Natural(bool) = 10,
    Odd = 11,
    Even(bool) = 12,
    Fibonacci = 20,
    Padova = 21,
    Catalan = 30,
    LookAndSay(usize) = 31,
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
            Sequence::Natural(zero) => repeat_with(Self::arithmetic
                (
                    if zero { T::zero() } else {T::one() },
                    T::one()
                )).take(n).collect(),
            Sequence::Odd => repeat_with(Self::arithmetic
                (
                    T::one(), 
                    T::from_i32(2).unwrap()
                )).take(n).collect(),
            Sequence::Even(zero) => repeat_with(Self::arithmetic
                (
                    if zero { T::zero() } else { T::from_i32(2).unwrap() },
                    T::from_i32(2).unwrap()
                )).take(n).collect(),
            Sequence::Fibonacci => repeat_with(Self::fibonacci()).take(n).collect(),
            Sequence::Padova => repeat_with(Self::padova()).take(n).collect(),
            Sequence::Catalan => repeat_with(Self::catalan()).take(n).collect(),
            Sequence::LookAndSay ( with ) => repeat_with(Self::lookandsay(with)).take(n).collect(),
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

    // NOTE: initial value of variable "previous" is not used (by design)
    #[allow(unused_assignments)]
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

    fn lookandsay(with:usize) -> impl FnMut() -> T
    {
        let mut data:Vec<i8> = with.to_string().chars().map(|c|c.to_digit(10).unwrap() as i8).collect();
        move ||
        {
            // fetch current value
            let current = T::from_u64(data.iter().fold(0u64, |s,&x| s * 10 + x as u64 )).unwrap();

            // look-and-say for next vector
            let mut v = Vec::new();
            let mut digit = data[0];
            let mut total = 0;
            data.iter().for_each(|&i|
            {
                if i == digit
                {
                    total += 1;
                }
                else
                {
                    v.push(total);
                    v.push(digit);
                    total = 1;
                    digit = i;
                }
            });
            v.push(total);
            v.push(digit);
            data = v;

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
    fn test_sequence_natural_i32()
    {
        let expected = vec![1, 2, 3, 4, 5];
        let n = expected.len();
        let actual = Sequence::Natural(false).vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_natural_zero_i32()
    {
        let expected = vec![0, 1, 2, 3, 4, 5];
        let n = expected.len();
        let actual = Sequence::Natural(true).vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_odd_i32()
    {
        let expected = vec![1, 3, 5, 7, 9];
        let n = expected.len();
        let actual = Sequence::Odd.vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_even_i32()
    {
        let expected = vec![2, 4, 6, 8, 10];
        let n = expected.len();
        let actual = Sequence::Even(false).vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_even_zero_i32()
    {
        let expected = vec![0, 2, 4, 6, 8, 10];
        let n = expected.len();
        let actual = Sequence::Even(true).vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_fibonacci_i32()
    {
        let expected = vec!
        [
            0,      1,      1,      2,      3, 
            5,      8,      13,     21,     34, 
            55,     89,     144,    233,    377,
            610,    987,    1597,   2584,   4181,
        ];
        let n = expected.len();
        let actual = Sequence::Fibonacci.vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_padova_i32()
    {
        let expected = vec!
        [
            1,      1,      1,      2,      2,
            3,      4,      5,      7,      9,
            12,     16,     21,     28,     37,
            49,     65,     86,     114,    151,
        ];
        let n = expected.len();
        let actual = Sequence::Padova.vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_catalan_i64()
    {
        let expected = vec!
        [
            1i64,       1,          2,          5,          14, 
            42,         132,        429,        1430,       4862,
            16796,      58786,      208012,     742900,     2674440,
            9694845,    35357670,   129644790,  477638700,  1767263190,
        ];
        let n = expected.len();
        let actual = Sequence::Catalan.vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_lookandsay_u64()
    {
        let expected = vec!
        [
            1u64,
            11,
            21,
            1211,
            111221,
            312211,
            13112221,
            1113213211,
            31131211131221,
            13211311123113112211,
        ];
        let n = expected.len();
        let actual = Sequence::LookAndSay(1).vec(n);
        assert_eq!(expected, actual);
    }
}
