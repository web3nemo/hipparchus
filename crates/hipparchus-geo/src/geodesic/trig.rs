// Compute atan2(y, x) with result in degrees
pub fn atan2d(y: f64, x: f64) -> f64 
{
    let mut x = x;
    let mut y = y;
    let mut q = if y.abs() > x.abs() 
    {
        std::mem::swap(&mut x, &mut y);
        2.0
    }
    else
    {
        0.0
    };
    if x < 0.0 
    {
        q += 1.0;
        x = -x;
    }
    let mut ang = y.atan2(x).to_degrees();
    if q == 1.0 
    {
        ang = if y >= 0.0 { 180.0 - ang } else { -180.0 - ang };
    }
    else if q == 2.0
    {
        ang = 90.0 - ang;
    }
    else if q == 3.0
    {
        ang += -90.0;
    }
    ang
}

pub fn eatanhe(x: f64, es: f64) -> f64 
{
    if es > 0.0 
    {
        es * (es * x).atanh()
    } 
    else 
    {
        -es * (es * x).atan()
    }
}

// Round an angle so taht small values underflow to 0
pub fn ang_round(x: f64) -> f64 
{
    // The makes the smallest gap in x = 1/16 - nextafter(1/16, 0) = 1/2^57
    // for reals = 0.7 pm on the earth if x is an angle in degrees.  (This
    // is about 1000 times more resolution than we get with angles around 90
    // degrees.)  We use this to avoid having to deal with near singular
    // cases when x is non-zero but tiny (e.g., 1.0e-200).
    let z = 1.0 / 16.0;
    let mut y = x.abs();
    // The compiler mustn't "simplify" z - (z - y) to y
    if y < z 
    {
        y = z - (z - y);
    };
    if x == 0.0 
    {
        0.0
    } 
    else if x < 0.0 
    {
        -y
    } 
    else 
    {
        y
    }
}

/// remainder of x/y in the range [-y/2, y/2]
fn remainder(x: f64, y: f64) -> f64 
{
    // z = math.fmod(x, y) if Math.isfinite(x) else Math.nan
    let z = if x.is_finite() { x % y } else { std::f64::NAN };

    // # On Windows 32-bit with python 2.7, math.fmod(-0.0, 360) = +0.0
    // # This fixes this bug.  See also Math::AngNormalize in the C++ library.
    // # sincosd has a similar fix.
    // z = x if x == 0 else z
    let z = if x == 0.0 { x } else { z };

    // return (z + y if z < -y/2 else
    // (z if z < y/2 else z -y))
    if z < -y / 2.0 
    {
        z + y
    } 
    else if z < y / 2.0 
    {
        z
    } else 
    {
        z - y
    }
}

/// reduce angle to (-180,180]
pub fn ang_normalize(x: f64) -> f64 
{
    // y = Math.remainder(x, 360)
    // return 180 if y == -180 else y
    let y = remainder(x, 360.0);
    if y == -180.0 
    {
        180.0
    } 
    else 
    {
        y
    }
}

// compute y - x and reduce to [-180,180] accurately
pub fn ang_diff(x: f64, y: f64) -> (f64, f64)
{
    let (d, t) = sum(ang_normalize(-x), ang_normalize(y));
    let d = ang_normalize(d);
    if d == 180.0 && t > 0.0
    {

        sum(-180.0, t)
    }
    else
    {
        sum(d, t)
    }
}

pub fn fmod(x: f64, y: f64) -> f64 
{
    x % y
}

/// Compute sine and cosine of x in degrees
pub fn sincosd(x: f64) -> (f64, f64) 
{
    // r = math.fmod(x, 360) if Math.isfinite(x) else Math.nan
    let mut r = if x.is_finite() 
    {
        fmod(x, 360.0)
    } 
    else 
    {
        std::f64::NAN
    };

    // q = 0 if Math.isnan(r) else int(round(r / 90))
    let mut q = if r.is_nan() 
    {
        0
    } 
    else 
    {
        (r / 90.0).round() as i32
    };

    // r -= 90 * q; r = math.radians(r)
    r -= 90.0 * q as f64;
    r = r.to_radians();

    // s = math.sin(r); c = math.cos(r)
    let s = r.sin();
    let c = r.cos();

    // q = q % 4
    q %= 4;

    // if q == 1:
    //     s, c =  c, -s
    // elif q == 2:
    //     s, c = -s, -c
    // elif q == 3:
    //     s, c = -c,  s
    let q = if q < 0 { q + 4 } else { q };
    let (s, c) = if q == 1 
    {
        (c, -s)
    }
    else if q == 2 
    {
        (-s, -c)
    } 
    else if q == 3 
    {
        (-c, s)
    } 
    else 
    {
        debug_assert_eq!(q, 0);
        (s, c)
    };

    // # Remove the minus sign on -0.0 except for sin(-0.0).
    // # On Windows 32-bit with python 2.7, math.fmod(-0.0, 360) = +0.0
    // # (x, c) here fixes this bug.  See also Math::sincosd in the C++ library.
    // # AngNormalize has a similar fix.
    //     s, c = (x, c) if x == 0 else (0.0+s, 0.0+c)
    // return s, c
    let (s, c) = if x == 0.0 { (x, c) } else { (0.0 + s, 0.0 + c) };

    (s, c)
}

// Functions that used to be inside Geodesic
pub fn sin_cos_series(sinp: bool, sinx: f64, cosx: f64, c: &[f64]) -> f64 
{
    let mut k = c.len();
    let mut n: i64 = k as i64 - if sinp { 1 } else { 0 };
    let ar: f64 = 2.0 * (cosx - sinx) * (cosx + sinx);
    let mut y1 = 0.0;
    let mut y0: f64 = if n & 1 != 0 
    {
        k -= 1;
        c[k]
    } 
    else 
    {
        0.0
    };
    n /= 2;
    while n > 0 
    {
        n -= 1;
        k -= 1;
        y1 = ar * y0 - y1 + c[k];
        k -= 1;
        y0 = ar * y1 - y0 + c[k];
    }
    if sinp 
    {
        2.0 * sinx * cosx * y0
    } else 
    {
        cosx * (y0 - y1)
    }
}

// Error free transformation of a sum
pub fn sum(u: f64, v: f64) -> (f64, f64) 
{
    let s = u + v;
    let up = s - v;
    let vpp = s - up;
    let up = up - u;
    let vpp = vpp - v;
    let t = -(up + vpp);
    (s, t)
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_sincosd() {
        let res = sincosd(-77.03196);
        assert_approx_eq!(f64, res.0, -0.9744953925159129);
        assert_approx_eq!(f64, res.1, 0.22440750870961693);

        let res = sincosd(69.48894);
        assert_approx_eq!(f64, res.0, 0.9366045700708676);
        assert_approx_eq!(f64, res.1, 0.3503881837653281);
        let res = sincosd(-1.0);
        assert_approx_eq!(f64, res.0, -0.01745240643728351);
        assert_approx_eq!(f64, res.1, 0.9998476951563913);
    }

    #[test]
    fn test_sin_cos_series() {
        assert_eq!(
            sin_cos_series(
                false,
                -0.8928657853278468,
                0.45032287238256896,
                &[
                    0.6660771734724675,
                    1.5757752625233906e-05,
                    3.8461688963148916e-09,
                    1.3040960748120204e-12,
                    5.252912023008548e-16,
                    2.367770858285795e-19
                ],
            ),
            0.29993425660538664
        );

        assert_eq!(
            sin_cos_series(
                false,
                -0.8928657853278468,
                0.45032287238256896,
                &[0., 1., 2., 3., 4., 5.],
            ),
            1.8998562852254026
        );
        assert_eq!(
            sin_cos_series(
                true,
                0.2969032234925426,
                0.9549075745221299,
                &[
                    0.0,
                    -0.0003561309485314716,
                    -3.170731714689771e-08,
                    -7.527972480734327e-12,
                    -2.5133854116682488e-15,
                    -1.0025061462383107e-18,
                    -4.462794158625518e-22
                ],
            ),
            -0.00020196665516199853
        );
        assert_eq!(
            sin_cos_series(
                true,
                -0.8928657853278468,
                0.45032287238256896,
                &[
                    0.0,
                    -0.0003561309485314716,
                    -3.170731714689771e-08,
                    -7.527972480734327e-12,
                    -2.5133854116682488e-15,
                    -1.0025061462383107e-18,
                    -4.462794158625518e-22
                ],
            ),
            0.00028635444718997857
        );

        assert_eq!(
            sin_cos_series(true, 0.12, 0.21, &[1.0, 2.0]),
            0.10079999999999999
        );
        assert_eq!(
            sin_cos_series(
                true,
                -0.024679833885152578,
                0.9996954065111039,
                &[
                    0.0,
                    -0.0008355098973052918,
                    -1.7444619952659748e-07,
                    -7.286557795511902e-11,
                    -3.80472772706481e-14,
                    -2.2251271876594078e-17,
                    1.2789961247944744e-20
                ],
            ),
            4.124513511893872e-05
        );
    }
}



