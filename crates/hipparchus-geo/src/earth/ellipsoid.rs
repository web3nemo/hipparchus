#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Ellipsoid
{
    pub a: f64,
    pub finv: f64,

    pub f: f64,
    pub m: f64,
    pub n: f64,
    pub q: f64,
    pub b: f64,
    pub c: f64,
    pub e1sq: f64,
    pub e2sq: f64,
    pub e3sq: f64,

    pub e1: f64,
    pub e2: f64,
    pub e3: f64,
}

impl Ellipsoid
{
    pub fn new(a:f64, finv:f64) -> Self
    {
        let f = 1.0 / finv;
        let m = f / (1.0 - f);
        let n = f / (2.0 - f);
        let q = 1.0 - f;
        let b = a * (1.0 - f);
        let c = a * a / b;
        let e1sq = f * (2.0 - f);
        let e2sq = (a * a - b * b) / (b * b);
        let e3sq = (a * a - b * b) / (a * a + b * b);
        let e1 = e1sq.sqrt();
        let e2 = e2sq.sqrt();
        let e3 = e3sq.sqrt();
        Self{ a, finv, f, m, n, q, b, c, e1sq, e2sq, e3sq, e1, e2, e3 }
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
            1 => self.e1,
            2 => self.e2,
            3 => self.e3,
            _ => panic!("eccentricity index must be 1, 2 or 3"),
        }
    }

    pub fn eccentricity_square(&self, index: usize) -> f64
    {
        match index
        {
            1 => self.e1sq,
            2 => self.e2sq,
            3 => self.e3sq,
            _ => panic!("eccentricity index must be 1, 2 or 3"),
        }
    }
}
