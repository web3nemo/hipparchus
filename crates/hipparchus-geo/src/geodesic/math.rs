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

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_astroid() 
    {
        assert_eq!(astroid(21.0, 12.0), 23.44475767500982);
    }
}
