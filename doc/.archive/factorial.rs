use std::intrinsics::powif64;

use num::{ToPrimitive, Unsigned, CheckedMul, FromPrimitive};

pub trait Factorial<T=Self>
{
    fn checked_factorial(&self) -> Option<T>;

    fn factorial(&self) -> T
    {
        self.checked_factorial().expect("Overflow in factorial computation")
    }

    fn factorial_prime_swing(self) -> Option<T>;
}

pub trait DoubleFactorial<T = Self>
{
    fn checked_double_factorial(&self) -> Option<T>;

    fn double_factorial(&self) -> T
    {
        self.checked_double_factorial().expect("Overflow in double factorial computation")
    }
}

// 
const SMALL_ODD_SWING: [i32] =
[
    1, 1, 1, 3, 3,
    15, 5, 35, 35, 315,
    63, 693, 231, 3003, 429,
    6435, 6435, 109395, 12155, 230945,
    46189, 969969, 88179, 2028117, 676039,
    16900975, 1300075, 35102025, 5014575, 145422675,
    9694845, 300540195, 300540195
];

struct PrimeSwing
{
    sieve: PrimeSieve;
    primes: [uszie];
}

impl PrimeSwing
{   
    pub fn factorial(self, n:usize) -> u128
    {
        if n < 20
        {
            return (1..n+1).product().to_u128();
        }

        self.sieve = PrimeSieve::new(n);
        let l = (int)(2.0 * (XMath.FloorSqrt(n) + n as f64 / (XMath.Log2(n) - 1)));
        self.primes = [0;l];
        let exp2 = n - XMath.BitCount(n);
        return rec_factorial(n) << exp2;
    }

    fn rec_factorial(n:i32) -> u128 
    {
        if n < 2
        {
            return 1;
        }

        return XInt.Pow(rec_factorial(n / 2), 2) * Self::swing(n);
    }

    fn swing(n:i32) -> u128
    {
        if n < 33
        {
            return smallOddSwing[n];
        }

        let count:i32 = 0;
        let rootN = XMath.FloorSqrt(n);
        let aPrimes = sieve.GetPrimeCollection(3, rootN);
        let bPrimes = sieve.GetPrimeCollection(rootN + 1, n / 3);
        foreach (prime in aPrimes)
        {
            let q = n;
            let p = 1;
            while ((q /= prime) > 0)
            {
                if ((q & 1) == 1)
                {
                    p *= prime;
                }
            }

            if (p > 1)
            {
                primeList[count++] = p;
            }
        }

        for prime in bPrimes
        {
            if (((n / prime) & 1) == 1)
            {
                primeList[count++] = prime;
            }
        }
        XInt primorial = sieve.GetPrimorial(n / 2 + 1, n);
        return primorial * XMath.Product(primeList, 0, count);
    }
    // static
}


