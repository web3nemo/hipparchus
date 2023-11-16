#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Ellipsoid
{
    pub a: f64,
    pub finv: f64,

    pub f: f64,
    pub m: f64,
    pub n: f64,
    pub b: f64,
    pub c: f64,

    pub p: f64,
    pub q: f64,
    pub e1sq: f64,
    pub e2sq: f64,
    pub e3sq: f64,

    pub e0: f64,
    pub e1: f64,
    pub e2: f64,
    pub e3: f64,
    pub e4: f64,
}

impl Ellipsoid
{
    pub fn new(a:f64, finv:f64) -> Self
    {
        let f = 1.0 / finv;
        let m = f / (1.0 - f);
        let n = f / (2.0 - f);
        let b = a * (1.0 - f);
        let c = a * a / b;

        let p = a * a - b * b;
        let q = 1.0 - f;
        let e1sq = f * (2.0 - f);
        let e2sq = (a * a - b * b) / (b * b);
        let e3sq = (a * a - b * b) / (a * a + b * b);
        let e0 = f64::sqrt(p);
        let e1 = f64::sqrt(e1sq);
        let e2 = f64::sqrt(e2sq);
        let e3 = f64::sqrt(e3sq);
        let e4 = f64::acos(q);

        Self
        {
            a, finv,
            f, m, n, b, c,
            p, q, e1sq, e2sq, e3sq, 
            e0, e1, e2, e3, e4
        }
    }

    pub fn flattening(&self, index: usize) -> f64
    {
        match index
        {
            1 => self.f,
            2 => self.m,
            3 => self.n,
            _ => panic!("flattening index must be 1, 2 or 3"),
        }
    }

    pub fn eccentricity(&self, index: usize) -> f64
    {
        match index
        {
            0 => self.e0,
            1 => self.e1,
            2 => self.e2,
            3 => self.e3,
            4 => self.e4,
            _ => panic!("eccentricity index must be 0, 1, 2, 3, 4"),
        }
    }

    pub fn eccentricity_square(&self, index: usize) -> f64
    {
        match index
        {
            0 => self.p,
            1 => self.e1sq,
            2 => self.e2sq,
            3 => self.e3sq,
            _ => panic!("eccentricity index must be 0, 1, 2 or 3"),
        }
    }
}

#[cfg(test)]
mod tests
{
    use rstest::*;
    use float_cmp::assert_approx_eq;
    use crate::earth::models::*;

    #[rstest]
    #[case(WGS84{})]
    fn test_elps<T>(#[case] _elps:T) where T: Model
    {
        let elps = T::elps();
        assert_approx_eq!(f64, elps.a, T::A);
        assert_approx_eq!(f64, elps.b, T::B);
        assert_approx_eq!(f64, elps.p, T::P);
        assert_approx_eq!(f64, elps.q, T::Q);
        assert_approx_eq!(f64, elps.flattening(1), T::flattening(1));
        assert_approx_eq!(f64, elps.flattening(2), T::flattening(2));
        assert_approx_eq!(f64, elps.flattening(3), T::flattening(3));
        assert_approx_eq!(f64, elps.eccentricity(0), T::eccentricity(0));
        assert_approx_eq!(f64, elps.eccentricity(1), T::eccentricity(1));
        assert_approx_eq!(f64, elps.eccentricity(2), T::eccentricity(2));
        assert_approx_eq!(f64, elps.eccentricity(3), T::eccentricity(3));
        assert_approx_eq!(f64, elps.eccentricity(4), T::eccentricity(4));
        assert_approx_eq!(f64, elps.eccentricity_square(0), T::eccentricity_square(0));
        assert_approx_eq!(f64, elps.eccentricity_square(1), T::eccentricity_square(1));
        assert_approx_eq!(f64, elps.eccentricity_square(2), T::eccentricity_square(2));
        assert_approx_eq!(f64, elps.eccentricity_square(3), T::eccentricity_square(3));
    }

    #[rstest]
    #[case(WGS84{}, 4)]
    #[case(WGS84{}, 0)]
    #[should_panic]
    fn test_elps_flattening_panic<T>(#[case] _elps:T, #[case] i:usize) where T: Model
    {
        let elps = T::elps();
        elps.flattening(i);
    }

    #[rstest]
    #[case(WGS84{}, 5)]
    #[should_panic]
    fn test_elps_eccentricity_panic<T>(#[case] _elps:T, #[case] i:usize) where T: Model
    {
        let elps = T::elps();
        elps.flattening(i);
    }

    #[rstest]
    #[case(WGS84{}, 4)]
    #[should_panic]
    fn test_elps_eccentricity_square_panic<T>(#[case] _elps:T, #[case] i:usize) where T: Model
    {
        let elps = T::elps();
        elps.eccentricity_square(i);
    }
}
