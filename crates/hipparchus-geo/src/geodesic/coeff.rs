// TODO: Add unit tests

use hipparchus_mean::value::Power;
use crate::geodesic::constants::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct A3X
{
    pub data:[f64;Self::COEFF_SIZE],
}

impl A3X
{
    pub const COEFF_SIZE: usize = GEODESIC_ORDER;

    pub fn new(n:f64) -> Self
    {
        const COEFF: [f64; 18] =
        [
            -3.0,   128.0,  -2.0,   -3.0,   64.0,
            -1.0,    -3.0,  -1.0,   16.0,   3.0,
            -1.0,    -2.0,   8.0,    1.0,   -1.0,
            2.0,     1.0,   1.0,
        ];
        let mut a3x = [0.0f64;Self::COEFF_SIZE];
        let mut o = 0;
        for (k, j) in (0..GEODESIC_ORDER).rev().enumerate()
        {
            let m = j.min(GEODESIC_ORDER - j - 1);
            a3x[k] = polyval(m, &COEFF[o..], n) / COEFF[o + m + 1];
            o += m + 2;
        }
        Self{ data: a3x }
    }

    pub fn a3f(&self, eps: f64) -> f64
    {
        polyval(GEODESIC_ORDER - 1, &self.data, eps)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct C3X
{
    pub data:[f64;Self::COEFF_SIZE],
}

impl C3X
{
    pub const COEFF_SIZE: usize = 15;

    pub fn new(n:f64) -> Self
    {
        const COEFF: [f64; 45] =
        [
              3.0,      128.0,        2.0,        5.0,       128.0,
            -1.0,        3.0,        3.0,       64.0,        -1.0,
              0.0,        1.0,        8.0,       -1.0,         1.0,
              4.0,        5.0,      256.0,        1.0,         3.0,
            128.0,       -3.0,       -2.0,        3.0,        64.0,
              1.0,       -3.0,        2.0,       32.0,         7.0, 
            512.0,      -10.0,        9.0,      384.0,         5.0, 
            -9.0,        5.0,      192.0,        7.0,       512.0, 
            -14.0,        7.0,      512.0,       21.0,      2560.0,
        ];
        let mut c3x = [0.0f64;Self::COEFF_SIZE];
        let mut o = 0;
        let mut k = 0;
        for l in 1..GEODESIC_ORDER
        {
            for j in (l..GEODESIC_ORDER).rev()
            {
                let m = j.min(GEODESIC_ORDER - j - 1);
                c3x[k] = polyval(m, &COEFF[o..], n) / COEFF[o + m + 1];
                k += 1;
                o += m + 2;
            }
        }
        Self{ data: c3x }
    }

    pub fn c3f(&self, eps: f64, c: &mut [f64])
    {
        let mut mult = 1.0;
        let mut o = 0;
        for (l, c_item) in c
            .iter_mut()
            .enumerate()
            .take(GEODESIC_ORDER)
            .skip(1)
        {
            let m = GEODESIC_ORDER - l - 1;
            mult *= eps;
            *c_item = mult * polyval(m, &self.data[o..], eps);
            o += m + 1;
        }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct C4X
{
    pub data:[f64;Self::COEFF_SIZE],
}

impl C4X
{
    pub const COEFF_SIZE: usize = 21;

    pub fn new(n:f64) -> Self
    {
        const COEFF: [f64; 77] =
        [
                97.0,    15015.0,     1088.0,     156.0,     45045.0,
              -224.0,    -4784.0,     1573.0,   45045.0,    -10656.0,
            14144.0,    -4576.0,     -858.0,   45045.0,        64.0, 
              624.0,    -4576.0,     6864.0,   -3003.0,     15015.0, 
              100.0,      208.0,      572.0,    3432.0,    -12012.0, 
            30030.0,    45045.0,        1.0,    9009.0,     -2944.0, 
              468.0,   135135.0,     5792.0,    1040.0,     -1287.0,
            135135.0,     5952.0,   -11648.0,    9152.0,     -2574.0,
            135135.0,      -64.0,     -624.0,    4576.0,     -6864.0,
              3003.0,   135135.0,        8.0,   10725.0,      1856.0,
              -936.0,   225225.0,    -8448.0,    4992.0,     -1144.0, 
            225225.0,    -1440.0,     4160.0,   -4576.0,      1716.0,
            225225.0,     -136.0,    63063.0,    1024.0,      -208.0,
            105105.0,     3584.0,    -3328.0,    1144.0,    315315.0,
              -128.0,   135135.0,    -2560.0,     832.0,    405405.0,
              128.0,    99099.0,
        ];
          let mut c4x = [0.0f64;Self::COEFF_SIZE];
        let mut o = 0;
        let mut k = 0;
        for l in 0..GEODESIC_ORDER
        {
            for j in (l..GEODESIC_ORDER).rev()
            {
                let m = GEODESIC_ORDER - j - 1;
                c4x[k] = polyval(m, &COEFF[o..], n) / COEFF[o + m + 1];
                k += 1;
                o += m + 2;
            }
        }
        Self{ data: c4x }
    }

    pub fn c4f(&self, eps: f64, c: &mut [f64])
    {
        let mut mult = 1.0;
        let mut o = 0;
        for (l, c_item) in c.iter_mut().enumerate().take(GEODESIC_ORDER)
        {
            let m = GEODESIC_ORDER - l - 1;
            *c_item = mult * polyval(m, &self.data[o..], eps);
            o += m + 1;
            mult *= eps;
        }
    }
}

pub fn coeff_a1m1f(eps: f64, geodesic_order: usize) -> f64 
{
    const COEFF: [f64; 5] = [1.0, 4.0, 64.0, 0.0, 256.0];
    let m = geodesic_order / 2;
    let t = polyval(m, &COEFF, eps.sq()) / COEFF[m+1];
    (t + eps) / (1.0 - eps)
}

pub fn coeff_c1f(eps: f64, c: &mut [f64], geodesic_order: usize) 
{
    const COEFF: [f64; 18] = 
    [
          -1.0,      6.0,    -16.0,    32.0,     -9.0, 
          64.0,   -128.0,   2048.0,     9.0,    -16.0,
         768.0,      3.0,     -5.0,   512.0,     -7.0,
        1280.0,     -7.0,   2048.0,
    ];
    let eps2 = eps.sq();
    let mut d = eps;
    let mut o = 0;
    for l in 1..=geodesic_order 
    {
        let m = (geodesic_order - l) / 2;
        c[l] =
            d * polyval(m, &COEFF[o..], eps2) / COEFF[o + m + 1];
        o += m + 2;
        d *= eps;
    }
}

pub fn coeff_c1pf(eps: f64, c: &mut [f64], geodesic_order: usize) 
{
    const COEFF: [f64; 18] =
    [
          205.0,     -432.0,      768.0,    1536.0,     4005.0, 
        -4736.0,     3840.0,    12288.0,    -225.0,      116.0, 
          384.0,    -7173.0,     2695.0,    7680.0,     3467.0, 
         7680.0,    38081.0,    61440.0,
    ];
    let eps2 = eps.sq();
    let mut d = eps;
    let mut o = 0;
    for l in 1..=geodesic_order 
    {
        let m = (geodesic_order - l) / 2;
        c[l] = d * polyval(m, &COEFF[o..], eps2) / COEFF[o + m + 1];
        o += m + 2;
        d *= eps;
    }
}

pub fn coeff_a2m1f(eps: f64, geodesic_order: usize) -> f64 
{
    const COEFF: [f64; 5] = [-11.0, -28.0, -192.0, 0.0, 256.0];
    let m = geodesic_order / 2;
    let t = polyval(m, &COEFF, eps.sq()) / COEFF[m + 1];
    (t - eps) / (1.0 + eps)
}

pub fn coeff_c2f(eps: f64, c: &mut [f64], geodesic_order: usize) 
{
    const COEFF: [f64; 18] = 
    [
           1.0,       2.0,      16.0,      32.0,    35.0, 
          64.0,     384.0,    2048.0,      15.0,    80.0, 
         768.0,       7.0,      35.0,     512.0,    63.0,
        1280.0,      77.0,    2048.0,
    ];
    let eps2 = eps.sq();
    let mut d = eps;
    let mut o = 0;
    for l in 1..=geodesic_order
    {
        let m = (geodesic_order - l) / 2;
        c[l] =
            d * polyval(m, &COEFF[o..], eps2) / COEFF[o+m+1];
        o += m + 2;
        d *= eps;
    }
}

// Evaluate a polynomial
fn polyval(n: usize, p: &[f64], x: f64) -> f64 
{
    let mut y = p[0];
    for val in &p[1..=n] 
    {
        y = y * x + val;
    }
    y
}

#[cfg(test)]
mod tests 
{
    use super::*;

    // Results for the assertions are taken by running the python implementation
    #[test]
    fn test_c2f()
    {
        let mut c = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        coeff_c2f(0.12, &mut c, 6);
        assert_eq!(
            c,
            vec![
                1.0,
                0.0601087776,
                0.00270653103,
                0.000180486,
                1.4215824e-05,
                1.22472e-06,
                1.12266e-07
            ]
        )
    }

    #[test]
    fn test_a2m1f() 
    {
        assert_eq!(coeff_a2m1f(0.12, 6), -0.11680607884285714);
    }

    #[test]
    fn test_c1pf() 
    {
        let mut c = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        coeff_c1pf(0.12, &mut c, 6);
        assert_eq!(
            c,
            vec![
                1.0,
                0.059517321000000005,
                0.004421053215,
                0.0005074200000000001,
                6.997613759999999e-05,
                1.1233080000000001e-05,
                1.8507366e-06
            ]
        )
    }

    #[test]
    fn test_c1f() 
    {
        let mut c = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        coeff_c1f(0.12, &mut c, 6);
        assert_eq!(
            c,
            vec![
                1.0,
                -0.059676777599999994,
                -0.000893533122,
                -3.57084e-05,
                -2.007504e-06,
                -1.3607999999999999e-07,
                -1.0205999999999999e-08
            ]
        )
    }

    #[test]
    fn test_a1m1f() 
    {
        assert_eq!(coeff_a1m1f(0.12, 6), 0.1404582405272727);
    }
}

