#![allow(non_snake_case)]
#![allow(clippy::excessive_precision)]
use hipparchus_mean::value::Power;

// Normalize a two-vector
pub fn norm(x: &mut f64, y: &mut f64)
{
    let r = x.hypot(*y);
    *x /= r;
    *y /= r;
}

// Evaluate a polynomial
pub fn polyval(n: usize, p: &[f64], x: f64) -> f64 
{
    let mut y = p[0];
    for val in &p[1..=n] 
    {
        y = y * x + val;
    }
    y
}

// Solve astroid equation
pub fn astroid(x: f64, y: f64) -> f64 
{
    let p = x.sq();
    let q = y.sq();
    let r = (p + q - 1.0) / 6.0;
    if !(q == 0.0 && r <= 0.0) 
    {
        let s = p * q / 4.0;
        let r2 = r.sq();
        let r3 = r * r2;
        let disc = s * (s + 2.0 * r3);
        let mut u = r;
        if disc >= 0.0 
        {
            let mut t3 = s + r3;
            t3 += if t3 < 0.0 { -disc.sqrt() } else { disc.sqrt() };
            let t = t3.cbrt();
            u += t + if t != 0.0 { r2 / t } else { 0.0 };
        } 
        else 
        {
            let ang = (-disc).sqrt().atan2(-(s + r3));
            u += 2.0 * r * (ang / 3.0).cos();
        }
        let v = (u.sq() + q).sqrt();
        let uv = if u < 0.0 { q / (v - u) } else { u + v };
        let w = (uv - q) / (2.0 * v);
        uv / ((uv + w.sq()).sqrt() + w)
    }
    else
    {
        0.0
    }
}

pub fn _A1m1f(eps: f64, geodesic_order: usize) -> f64 
{
    const COEFF: [f64; 5] = [1.0, 4.0, 64.0, 0.0, 256.0];
    let m = geodesic_order / 2;
    let t = polyval(m, &COEFF, eps.sq()) / COEFF[m+1];
    (t + eps) / (1.0 - eps)
}

pub fn _C1f(eps: f64, c: &mut [f64], geodesic_order: usize) 
{
    const COEFF: [f64; 18] = 
    [
        -1.0, 6.0, -16.0, 32.0, -9.0, 64.0, -128.0, 2048.0, 9.0, -16.0, 768.0, 3.0, -5.0, 512.0,
        -7.0, 1280.0, -7.0, 2048.0,
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

pub fn _C1pf(eps: f64, c: &mut [f64], geodesic_order: usize) 
{
    const COEFF: [f64; 18] = [
        205.0, -432.0, 768.0, 1536.0, 4005.0, -4736.0, 3840.0, 12288.0, -225.0, 116.0, 384.0,
        -7173.0, 2695.0, 7680.0, 3467.0, 7680.0, 38081.0, 61440.0,
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

pub fn _A2m1f(eps: f64, geodesic_order: usize) -> f64 
{
    const COEFF: [f64; 5] = [-11.0, -28.0, -192.0, 0.0, 256.0];
    let m = geodesic_order / 2;
    let t = polyval(m, &COEFF, eps.sq()) / COEFF[m + 1];
    (t - eps) / (1.0 + eps)
}

pub fn _C2f(eps: f64, c: &mut [f64], geodesic_order: usize) 
{
    const COEFF: [f64; 18] = [
        1.0, 2.0, 16.0, 32.0, 35.0, 64.0, 384.0, 2048.0, 15.0, 80.0, 768.0, 7.0, 35.0, 512.0, 63.0,
        1280.0, 77.0, 2048.0,
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

#[cfg(test)]
mod tests 
{
    use super::*;

    // Results for the assertions are taken by running the python implementation
    #[test]
    fn test__C2f() {
        let mut c = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        _C2f(0.12, &mut c, 6);
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
    fn test__A2m1f() {
        assert_eq!(_A2m1f(0.12, 6), -0.11680607884285714);
    }

    #[test]
    fn test__C1pf() {
        let mut c = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        _C1pf(0.12, &mut c, 6);
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
    fn test__C1f() 
    {
        let mut c = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        _C1f(0.12, &mut c, 6);
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
    fn test__A1m1f() {
        assert_eq!(_A1m1f(0.12, 6), 0.1404582405272727);
    }

    #[test]
    fn test_astroid() {
        assert_eq!(astroid(21.0, 12.0), 23.44475767500982);
    }
}
