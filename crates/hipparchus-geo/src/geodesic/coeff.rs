// TODO: Remove dependency to math
// TODO: Avoid data copy when retuan value is an array
// TODO: Understand algorithm and consider if make it lazy
// TODO: Add unit tests

use crate::geodesic::math;
use crate::geodesic::constants::*;

pub const COEFF_A3X_SIZE: usize = GEODESIC_ORDER;
const A3: [f64; 18] =
[
    -3.0,   128.0,  -2.0,   -3.0,   64.0,
    -1.0,    -3.0,  -1.0,   16.0,   3.0,
    -1.0,    -2.0,   8.0,    1.0,   -1.0,
    2.0,     1.0,   1.0,
];
pub fn coeff_a3(n:f64) -> [f64;COEFF_A3X_SIZE]
{
    let mut a3x = [0.0f64;COEFF_A3X_SIZE];
    let mut o = 0;
    for (k, j) in (0..GEODESIC_ORDER).rev().enumerate()
    {
        let m = j.min(GEODESIC_ORDER - j - 1);
        a3x[k] = math::polyval(m, &A3[o..], n) / A3[o + m + 1];
        o += m + 2;
    }
    a3x
}

pub const COEFF_C3X_SIZE: usize = 15;
const C3: [f64; 45] =
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
pub fn coeff_c3(n:f64) -> [f64;COEFF_C3X_SIZE]
{
    let mut c3x = [0.0f64;COEFF_C3X_SIZE];
    let mut o = 0;
    let mut k = 0;
    for l in 1..GEODESIC_ORDER
    {
        for j in (l..GEODESIC_ORDER).rev()
        {
            let m = j.min(GEODESIC_ORDER - j - 1);
            c3x[k] = math::polyval(m, &C3[o..], n) / C3[o + m + 1];
            k += 1;
            o += m + 2;
        }
    }
    c3x
}

pub const COEFF_C4X_SIZE: usize = 21;
const C4: [f64; 77] =
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
pub fn coeff_c4(n:f64) -> [f64;COEFF_C4X_SIZE]
{
    let mut c4x = [0.0f64;COEFF_C4X_SIZE];
    let mut o = 0;
    let mut k = 0;
    for l in 0..GEODESIC_ORDER
    {
        for j in (l..GEODESIC_ORDER).rev()
        {
            let m = GEODESIC_ORDER - j - 1;
            c4x[k] = math::polyval(m, &C4[o..], n) / C4[o + m + 1];
            k += 1;
            o += m + 2;
        }
    }
    c4x
}
