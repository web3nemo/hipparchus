use std::ops::{Add, Sub, Mul, Div};
use std::iter::repeat_with;
use num::{FromPrimitive, Zero, One};

#[repr(i32)]
pub enum Sequence<T> where
    T: Add + Sub + Mul + Div<Output=T> + FromPrimitive + Zero + One + Copy + Clone,
{
    /// Arithmetic sequence with initial value and difference
    Arithmetic { init:T, difference:T } = 1,

    /// Geometric sequence with initial value and ratio
    Geometric { init:T, ratio:T } = 2,

    /// Natural sequence (starting from 1 or 0)
    Natural(bool) = 3,

    /// Odd sequence starting from 1
    Odd = 4,

    /// Even sequence starting from 0 or 1
    Even(bool) = 5,

    /// Power sequence with specified radix
    Power(T) = 6,

    /// Triangular sequence
    Triangular = 10,

    /// Square sequence
    Square = 11,

    /// Cubic sequence
    Cubic = 12,

    /// Harmonic sequence with initial value and difference
    Harmonic { init:T, difference:T } = 13,

    /// Fibonacci sequence
    Fibonacci = 20,

    /// Lucas sequence
    Lucas = 21,

    /// Padova sequence
    Padova = 22,

    /// Catalan sequence
    Catalan = 29,

    /// Look-and-say sequence with initial value
    LookAndSay(usize) = 30,
}

impl<T> Sequence<T> where
    T: Add + Sub + Mul + Div<Output=T> + FromPrimitive + Zero + One + Copy + Clone,
{
    /// Create a recursive sequence with specified size, initial value and customized function to compute next value
    pub fn recursive<F>(n:usize, init:T, f:F) -> Vec<T>
        where F: Fn(T) -> T
    {
        let mut next = init;
        repeat_with(||
        {
            let current = next;
            next = f(next);
            current
        }).take(n).collect()
    }

    /// Create a sequence with specified size
    pub fn vec(self, n:usize) -> Vec<T>
    {
        match self
        {
            Sequence::Arithmetic { init, difference } => Self::recursive(n, init, |x| x + difference),
            Sequence::Geometric { init, ratio } => Self::recursive(n, init, |x| x * ratio),
            Sequence::Natural(zero) => Sequence::Arithmetic
            {
                init: if zero { T::zero() } else { T::one() },
                difference: T::one(),
            }.vec(n),
            Sequence::Odd => Sequence::Arithmetic
            {
                init: T::one(),
                difference: T::from_i32(2).unwrap(),
            }.vec(n),
            Sequence::Even(zero) => Sequence::Arithmetic
            {
                init: if zero { T::zero() } else { T::from_i32(2).unwrap() },
                difference: T::from_i32(2).unwrap(),
            }.vec(n),
            Sequence::Power(radix) => Sequence::Geometric
            {
                init: T::one(),
                ratio: radix,
            }.vec(n),
            Sequence::Triangular => Sequence::Natural(false).map
            (
                n, |x|
                (T::one() + x) * x / T::from_i32(2).unwrap()
            ),
            Sequence::Square => Sequence::Natural(false).map
            (
                n, |x|
                x * x
            ),
            Sequence::Cubic => Sequence::Natural(false).map
            (
                n, |x|
                x * x * x
            ),
            Sequence::Harmonic { init, difference } => Sequence::Arithmetic { init, difference }.map
            (
                n, |x|
                T::one() / x
            ),
            Sequence::Fibonacci => repeat_with(Self::fibonacci_lucas
                (
                    T::zero(),
                    T::one()
                )).take(n).collect(),
            Sequence::Lucas => repeat_with(Self::fibonacci_lucas
                (
                    T::from_i32(2).unwrap(),
                    T::one()
                )).take(n).collect(),
            Sequence::Padova => repeat_with(Self::padova()).take(n).collect(),
            Sequence::Catalan => repeat_with(Self::catalan()).take(n).collect(),
            Sequence::LookAndSay ( with ) => repeat_with(Self::lookandsay(with)).take(n).collect(),
        }
    }

    /// Derive a new sequence based on specified mapping function with specified size
    pub fn map<F>(self, n:usize, f:F) -> Vec<T>
        where F: Fn(T) -> T
    {
        let me = self.vec(n);
        let mut index = 0;

        repeat_with(move ||
        {
            let mapped = f(me[index]);
            index += 1;
            mapped
        }).take(n).collect()
    }

    /// Derive a new sequence based on specified folding function with specified size. 
    /// The folding function accept a slice of optional values, where None indicates out-of-range.
    pub fn fold<F>(self, n:usize, start:i32, end:i32, f:F) -> Vec<T>
        where F: Fn(&[Option<T>]) -> T
    {
        let me = self.vec(n);
        let mut index = 0;

        repeat_with(move ||
        {
            let mut data = Vec::<Option<T>>::new();
            let a = index + start;
            let b = index + end;
            for k in a..b
            {
                data.push( if k < 0 || k >= n as i32 { None } else { Some(me[k as usize]) } )
            }

            let mapped = f(&data);
            index += 1;
            mapped
        }).take(n).collect()
    }

    fn fibonacci_lucas(first:T, second:T) -> impl FnMut() -> T
    {
        let mut current = first;
        let mut next = second;
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
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_sequence_recursive_i32()
    {
        let expected = vec![1, -1, 1, -1, 1];
        let n = expected.len();
        let actual = Sequence::recursive(n, 1, |x| -x );
        assert_eq!(expected, actual);
    }

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
    fn test_sequence_power_i32()
    {
        let expected = vec![1, 3, 9, 27, 81];
        let n = expected.len();
        let actual = Sequence::Power(3).vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_triangular_i32()
    {
        let expected = vec!
        [
            1,      3,      6,      10,     15, 
            21,     28,     36,     45,     55, 
            66,     78,     91,     105,    120, 
            136,    153,    171,    190,    210, 
        ];
        let n = expected.len();
        let actual = Sequence::Triangular.vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_square_i32()
    {
        let expected = vec![1, 4, 9, 16, 25];
        let n = expected.len();
        let actual = Sequence::Square.vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_cubic_i32()
    {
        let expected = vec![1, 8, 27, 64, 125];
        let n = expected.len();
        let actual = Sequence::Cubic.vec(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_harmonic_f32()
    {
        let expected = vec!
        [
            1.0/1.0,    1.0/2.0,    1.0/3.0,    1.0/4.0,    1.0/5.0, 
            1.0/6.0,    1.0/7.0,    1.0/8.0,    1.0/9.0,    1.0/10.0, 
        ];
        let n = expected.len();
        let actual = Sequence::Harmonic::<f32> { init: 1.0, difference: 1.0 }.vec(n);
        assert_approx_eq!(&[f32], &expected, &actual);
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
    fn test_sequence_lucas_i32()
    {
        let expected = vec!
        [
            2,      1,      3,      4,      7, 
            11,     18,     29,     47,     76, 
            123,    199,    322,    521,    843,
            1364,   2207,   3571,   5778,   9349,
        ];
        let n = expected.len();
        let actual = Sequence::Lucas.vec(n);
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

    #[test]
    fn test_sequence_map_i32()
    {
        // Natural * 2 = Even
        let expected = Sequence::<i32>::Even(true).vec(4);
        let actual = Sequence::<i32>::Natural(true).map(4, |x| x * 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sequence_fold_i32()
    {
        // Triangular.Fold(2, sum) = Square
        let expected = Sequence::<i32>::Square.vec(4);
        let actual = Sequence::<i32>::Triangular.fold(4, -1, 1, |v|
        {
            v.iter().map(|x| x.unwrap_or(0)).sum()
        });
        assert_eq!(expected, actual);
    }
}
