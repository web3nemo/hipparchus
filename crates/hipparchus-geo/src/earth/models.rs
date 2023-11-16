use crate::earth::ellipsoid::Ellipsoid;

/// Radius of the earth
pub enum Radius
{
    /// equatorial radius: a
    Equatorial,

    /// polar radius: b
    Polar,

    /// average radius of equatorial & polar radius: (a + b) / 2
    Mixed,

    /// arithmetic mean of length of radius of the earth: (2a + b) / 3
    ArithmeticMean,

    /// radius of the sphere of equal surface area of the earth
    SurfaceAreaMean,

    /// radius of the sphere of equal volume of the earth
    VolumeMean,
}

/// Ellipsoid Model
pub trait Model
{
    /// The datum name of the ellipsoid model
    const NAME: &'static str;

    /// The equatorial radius (semi-major axis)
    const A:f64;

    /// flattening
    const F_INV:f64;

    /// The 1st flattening of the ellipsoid.
    /// - zero: sphere
    /// - positive: oblate ellipsoid
    /// - negative: prolate ellipsoid
    const F:f64 = 1.0 / Self::F_INV;

    /// The 2nd flattening
    const M:f64 = Self::F / (1.0 - Self::F);

    /// The 3rd flattening
    const N:f64 = Self::F / (2.0 - Self::F);

    /// The polar radius (semi-minor axis)
    const B:f64 = Self::A * (1.0 - Self::F);

    /// E => E1^2, square of the 1st eccentricity
    const E1SQ:f64 = Self::F * (2.0 - Self::F);

    /// E' => E2^2, square of the 2nd eccentricity
    const E2SQ:f64 = (Self::A * Self::A - Self::B * Self::B) / (Self::B * Self::B);

    /// E" => E3^2, square of the 3rd eccentricity
    const E3SQ:f64 = (Self::A * Self::A - Self::B * Self::B) / (Self::A * Self::A + Self::B * Self::B);

    /// E1, the 1st eccentricity
    fn e1() ->f64
    {
        f64::sqrt(Self::E1SQ)
    }

    /// E2, the 2nd eccentricity
    fn e2() -> f64
    {
        f64::sqrt(Self::E2SQ)
    }
    
    /// E2, the 3rd eccentricity
    fn e3() -> f64
    {
        f64::sqrt(Self::E3SQ)
    }

    /// E(L)^2 => A^2 - B^2, square of linear eccentricity
    const P:f64 = Self::A * Self::A - Self::B * Self::B;

    /// E(L) = sqrt(a^2-b^2), linear eccentricity, also known as focal distance
    fn linear_eccentricity() -> f64
    {
        f64::sqrt(Self::P)
    }

    /// The radius ratio: Q = B / A
    const Q:f64 = 1.0 - Self::F;

    /// E(A) = acos(b/a), angular eccentricity
    fn angular_eccentricity() -> f64
    {
        f64::acos(Self::Q)
    }

    fn elps() -> Ellipsoid
    {
        Ellipsoid::new(Self::A, Self::F_INV)
    }

    /// Flattening is a measure of the compression of a circle or sphere along a diameter to form an ellipse or an ellipsoid of revolution (spheroid) respectively. 
    /// See also in https://en.wikipedia.org/wiki/Flattening
    fn flattening(index: usize) -> f64
    {
        match index
        {
            1 => Self::F,
            2 => Self::M,
            3 => Self::N,
            _ => panic!("flattening index must be 1, 2 or 3"),
        }
    }

    fn eccentricity(index: usize) -> f64
    {
        match index
        {
            0 => Self::linear_eccentricity(),
            1 => Self::e1(),
            2 => Self::e2(),
            3 => Self::e3(),
            4 => Self::angular_eccentricity(),
            _ => panic!("eccentricity index must be 0, 1, 2, 3 or 4"),
        }
    }

    fn eccentricity_square(index: usize) -> f64
    {
        match index
        {
            0 => Self::P,
            1 => Self::E1SQ,
            2 => Self::E2SQ,
            3 => Self::E3SQ,
            _ => panic!("eccentricity index must be 0, 1, 2 or 3"),
        }
    }

    fn radius(r:Radius) -> f64
    {
        match r
        {
            Radius::Equatorial => Self::A,
            Radius::Polar => Self::B,
            Radius::Mixed => (Self::A + Self::B) / 2.0,
            Radius::ArithmeticMean => (Self::A * 2.0 + Self::B) / 3.0,
            Radius::SurfaceAreaMean => f64::sqrt(Self::surface_area() / (4.0 * std::f64::consts::PI)),
            Radius::VolumeMean => f64::powf(Self::A * Self::A * Self::B, 1.0/3.0),
        }
    }

    fn surface_area() -> f64
    {
        let a = Self::A;
        let b = Self::B;
        let e = Self::angular_eccentricity();
        let esin = f64::sin(e);
        2.0 * std::f64::consts::PI * ( a * a + b * b * f64::atanh(esin) / esin )
    }

    fn volume() -> f64
    {
        Self::A * Self::A * Self::B * std::f64::consts::PI / 0.75
    }
}

#[macro_export]
macro_rules! ellipsoid_model
{
    ($tt:tt, $n:expr, $a:expr, $finv:expr) =>
    {
        #[derive(Debug, Copy, Clone, PartialEq)]        
        pub struct $tt { }
        impl Model for $tt
        {
            const NAME: &'static str = $n;
            const A:f64 = $a;
            const F_INV:f64 = $finv;
        }
    }
}

// World-wide Ellipsoid Models
// - https://en.wikipedia.org/wiki/Earth_ellipsoid
ellipsoid_model!( GRS67,            "GRS-67 (1967)",                6_378_160.0,        298.247_167_427 );
ellipsoid_model!( GRS80,            "GRS-80 (1979)",                6_378_137.0,        298.257_222_101 );
ellipsoid_model!( IERS1989,         "IERS (1989)",                  6_378_136.0,        298.257 );
ellipsoid_model!( IERS1992,         "IERS (1992)",                  6_378_136.6,        298.256_42 );
ellipsoid_model!( Intl1924,         "International (1924)",         6_378_388.0,        297.0 );
ellipsoid_model!( Intl1967,         "New International (1967)",     6_378_157.5,        298.249_615_39 );	
ellipsoid_model!( WGS66,            "WGS66 (1966)",                 6_378_145.0,        298.25 );
ellipsoid_model!( WGS72,            "WGS-72 (1972)", 	            6_378_135.0,        298.26 );
ellipsoid_model!( WGS84,            "WGS-84 (1984)",                6_378_137.0,        298.257_223_563 );

// Regional Ellipsoid Models
// - https://en.wikipedia.org/wiki/Earth_ellipsoid
ellipsoid_model!( Airy1830,         "Airy (1830)",                  6_377_563.396,      299.324_964_6 );
ellipsoid_model!( ANS66,            "Australian National (1966)",   6_378_160.0,        298.25 );
ellipsoid_model!( Bessel1841,       "Bessel (1841)",                6_377_397.155,      299.152_812_8 );
ellipsoid_model!( CGCS2000,         "CGCS 2000",                    6_378_137.0,        298.257_222_1 );
ellipsoid_model!( Clarke1866,       "Clarke (1866)",                6_378_206.4,        294.978_698_2 );
ellipsoid_model!( Clarke1878,       "Clarke (1878)",                6_378_190.0,        293.465_998_0 );
ellipsoid_model!( Clarke1880,       "Clarke (1880)",                6_378_249.145,      293.465 );
ellipsoid_model!( Helmert1906,      "Helmert (1906)",               6_378_200.0,        298.3 );
ellipsoid_model!( Hayford,          "Hayford (1910)",               6_378_388.0,        297.0 );
ellipsoid_model!( Krasov40,         "Krassovsky (1940)",            6_378_245.0,        298.3 );
ellipsoid_model!( Maupertuis1738,   "Maupertuis (1738)",            6_397_300.0,        191.0 );
ellipsoid_model!( Plessis1817,      "Plessis (1817)",               6_376_523.0,        308.64 );
ellipsoid_model!( SA1969,           "South American (1969)",        6_378_160.0,        298.25 );

// Sphere Models
ellipsoid_model!( Sphere,           "Sphere (Mean)",                6_371_008.771_415,  f64::INFINITY );
ellipsoid_model!( SphereAuthalic,   "Sphere, Authalic",             6_371_000.0,        f64::INFINITY );
ellipsoid_model!( SpherePopular,    "Sphere, Popular",              6_378_137.0,        f64::INFINITY );
ellipsoid_model!( SphereNormal,     "Sphere, Normal",               6_370_997.0,        f64::INFINITY );

#[cfg(test)]
mod tests
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    macro_rules! assert_model_nfab
    {
        ($t:tt, $n:expr, $a:expr, $finv:expr) =>
        {
            assert_eq!($n, $t::NAME);
            assert_approx_eq!(f64, $a, $t::A);
            assert_approx_eq!(f64, $finv, $t::F_INV);
            
            assert_approx_eq!(f64, 1.0, $t::F * $t::F_INV);
            assert_approx_eq!(f64, $t::A * (1.0 - $t::F), $t::B);
        }
    }

    #[rstest]
    #[case(GRS67{},     "GRS-67 (1967)",                6_378_160.0,    298.247_167_427 )]
    #[case(GRS80{},     "GRS-80 (1979)",                6_378_137.0,    298.257_222_101 )]
    #[case(IERS1989{},  "IERS (1989)",                  6_378_136.0,    298.257 )]
    #[case(IERS1992{},  "IERS (1992)",                  6_378_136.6,    298.256_42 )]
    #[case(Intl1924{},  "International (1924)",         6_378_388.0,    297.0 )]
    #[case(Intl1967{},  "New International (1967)",     6_378_157.5,    298.249_615_39 )]
    #[case(WGS66{},     "WGS66 (1966)",                 6_378_145.0,    298.25 )]
    #[case(WGS72{},     "WGS-72 (1972)", 	            6_378_135.0,    298.26 )]
    #[case(WGS84{},     "WGS-84 (1984)",                6_378_137.0,    298.257_223_563 )]
    fn test_model_worldwide<T>(#[case] _elps:T, #[case] n: &str, #[case] a: f64, #[case] finv: f64) where T: Model
    {
        assert_model_nfab!(T, n, a, finv);
    }

    #[rstest]
    #[case(WGS84{},             "WGS-84 (1984)",                6_378_137.0,        298.257_223_563 )]
    #[case(Airy1830{},          "Airy (1830)",                  6_377_563.396,      299.324_964_6 )]
    #[case(ANS66{},             "Australian National (1966)",   6_378_160.0,        298.25 )]
    #[case(Bessel1841{},        "Bessel (1841)",                6_377_397.155,      299.152_812_8 )]
    #[case(CGCS2000{},          "CGCS 2000",                    6_378_137.0,        298.257_222_1 )]
    #[case(Clarke1866{},        "Clarke (1866)",                6_378_206.4,        294.978_698_2 )]
    #[case(Clarke1878{},        "Clarke (1878)",                6_378_190.0,        293.465_998_0 )]
    #[case(Clarke1880{},        "Clarke (1880)",                6_378_249.145,      293.465 )]
    #[case(Helmert1906{},       "Helmert (1906)",               6_378_200.0,        298.3 )]
    #[case(Hayford{},           "Hayford (1910)",               6_378_388.0,        297.0 )]
    #[case(Krasov40{},          "Krassovsky (1940)",            6_378_245.0,        298.3 )]
    #[case(Maupertuis1738{},    "Maupertuis (1738)",            6_397_300.0,        191.0 )]
    #[case(Plessis1817{},       "Plessis (1817)",               6_376_523.0,        308.64 )]
    #[case(SA1969{},            "South American (1969)",        6_378_160.0,        298.25 )]
    fn test_model_regional<T>(#[case] _elps:T, #[case] n: &str, #[case] a: f64, #[case] finv: f64) where T: Model
    {
        assert_model_nfab!(T, n, a, finv);
    }

    #[rstest]
    #[case(Sphere{},            "Sphere (Mean)",        6_371_008.771_415)]
    #[case(SphereAuthalic{},    "Sphere, Authalic",     6_371_000.0)]
    #[case(SpherePopular{},     "Sphere, Popular",      6_378_137.0)]
    #[case(SphereNormal{},      "Sphere, Normal",       6_370_997.0)]
    fn test_model_sphere<T>(#[case] _elps:T, #[case] n: &str, #[case] r: f64) where T: Model
    {
        assert_eq!(n, T::NAME);
        assert_approx_eq!(f64, r, T::A);
        assert_approx_eq!(f64, f64::INFINITY, T::F_INV);

        assert_approx_eq!(f64, 0.0, T::F);
        assert_approx_eq!(f64, 1.0, T::Q + T::F);
        assert_approx_eq!(f64, r, T::B);
    }

    macro_rules! assert_model_flattening
    {
        ($t:tt) =>
        {
            let f = $t::F;
            let m = $t::M;
            let n = $t::N;
            let a = $t::A;
            let b = $t::B;
            assert_approx_eq!(f64, f, T::flattening(1));
            assert_approx_eq!(f64, m, T::flattening(2));
            assert_approx_eq!(f64, n, T::flattening(3));
            assert_approx_eq!(f64, f, (a - b) / a);
            assert_approx_eq!(f64, m, (a - b) / b);
            assert_approx_eq!(f64, n, (a - b) / (a + b));
        }
    }

    #[rstest]
    #[case(GRS67{})]
    #[case(GRS80{})]
    #[case(IERS1989{})]
    #[case(IERS1992{})]
    #[case(Intl1924{})]
    #[case(Intl1967{})]
    #[case(WGS66{})]
    #[case(WGS72{})]
    #[case(WGS84{})]
    fn test_flattening_worldwide<T>(#[case] _elps:T) where T: Model
    {
        assert_model_flattening!(T);
    }

    #[rstest]
    #[case(Airy1830{})]
    #[case(ANS66{})]
    #[case(Bessel1841{})]
    #[case(CGCS2000{})]
    #[case(Clarke1866{})]
    #[case(Clarke1878{})]
    #[case(Clarke1880{})]
    #[case(Helmert1906{})]
    #[case(Hayford{})]
    #[case(Krasov40{})]
    #[case(Maupertuis1738{})]
    #[case(Plessis1817{})]
    #[case(SA1969{})]
    fn test_flattening_regional<T>(#[case] _elps:T) where T: Model
    {
        assert_model_flattening!(T);
    }

    #[rstest]
    #[case(Sphere{})]
    #[case(SphereAuthalic{})]
    #[case(SpherePopular{})]
    #[case(SphereNormal{})]
    fn test_flattening_sphere<T>(#[case] _elps:T) where T: Model
    {
        assert_approx_eq!(f64, 0.0, T::flattening(1));
        assert_approx_eq!(f64, 0.0, T::flattening(2));
        assert_approx_eq!(f64, 0.0, T::flattening(3));
    }

    #[rstest]
    #[case(WGS84{}, 4)]
    #[case(WGS84{}, 0)]
    #[should_panic]
    fn test_flattening_panic<T>(#[case] _elps:T, #[case] i:usize) where T: Model
    {
        T::flattening(i);
    }

    macro_rules! assert_model_eccentricity
    {
        ($t:tt) =>
        {
            let f = T::F;
            let a = T::A;
            let b = T::B;
            let e1 = T::eccentricity(1);
            let e2 = T::eccentricity(2);
            let e3 = T::eccentricity(3);
            assert_approx_eq!(f64, e1 * e1, T::E1SQ);
            assert_approx_eq!(f64, e2 * e2, T::E2SQ);
            assert_approx_eq!(f64, e3 * e3, T::E3SQ);
            assert_approx_eq!(f64, f * (2.0 - f), T::E1SQ);
            assert_approx_eq!(f64, (e1 * e1 ) / (1.0 - e1 * e1), T::E2SQ);
            assert_approx_eq!(f64, (e1 * e1 ) / (2.0 - e1 * e1), T::E3SQ);
            assert_approx_eq!(f64, (a * a - b * b) / (a * a), T::E1SQ);
            assert_approx_eq!(f64, (a * a - b * b) / (b * b), T::E2SQ);
            assert_approx_eq!(f64, (a * a - b * b) / (a * a + b * b), T::E3SQ);
        }
    }

    macro_rules! assert_model_eccentricity_special
    {
        ($t:tt, $ulps:expr) =>
        {
            let a = T::A;
            let b = T::B;
            let e0 = T::eccentricity(0); 
            let e1 = T::eccentricity(1);
            let e2 = T::eccentricity(2);
            let e3 = T::eccentricity(3);
            let e4 = T::eccentricity(4);
            assert_approx_eq!(f64, a * a, b * b + T::P);
            assert_approx_eq!(f64, e0 * e0, T::P);
            assert_approx_eq!(f64, 1.0, $t::Q + $t::F);
            assert_approx_eq!(f64, f64::cos(e4), T::Q);
            assert_approx_eq!(f64, e1, f64::sin(e4), ulps=$ulps);
            assert_approx_eq!(f64, e2, f64::tan(e4), ulps=$ulps);
            assert_approx_eq!(f64, e3, f64::sin(e4) / f64::sqrt(2.0 - f64::sin(e4) * f64::sin(e4)), ulps=$ulps);
        }
    }

    macro_rules! assert_model_eccentricity_square
    {
        ($t:tt) =>
        {
            let e0 = T::eccentricity(0); 
            let e1 = T::eccentricity(1);
            let e2 = T::eccentricity(2);
            let e3 = T::eccentricity(3);
            let e0sq = T::eccentricity_square(0);
            let e1sq = T::eccentricity_square(1);
            let e2sq = T::eccentricity_square(2);
            let e3sq = T::eccentricity_square(3);
            assert_approx_eq!(f64, e0 * e0, e0sq);
            assert_approx_eq!(f64, e1 * e1, e1sq);
            assert_approx_eq!(f64, e2 * e2, e2sq);
            assert_approx_eq!(f64, e3 * e3, e3sq);
        }
    }

    #[rstest]
    #[case(GRS67{}, 80)]
    #[case(GRS80{}, 80)]
    #[case(IERS1989{}, 80)]
    #[case(IERS1992{}, 80)]
    #[case(Intl1924{}, 80)]
    #[case(Intl1967{}, 80)]
    #[case(WGS66{}, 80)]
    #[case(WGS72{}, 80)]
    #[case(WGS84{}, 80)]
    fn test_eccentricity_worldwide<T>(#[case] _elps:T, #[case] ulps:i64) where T: Model
    {
        assert_model_eccentricity!(T);
        assert_model_eccentricity_special!(T, ulps);
        assert_model_eccentricity_square!(T);
    }

    #[rstest]
    #[case(Airy1830{}, 80)]
    #[case(ANS66{}, 80)]
    #[case(Bessel1841{}, 80)]
    #[case(CGCS2000{}, 80)]
    #[case(Clarke1866{}, 80)]
    #[case(Clarke1878{}, 80)]
    #[case(Clarke1880{}, 80)]
    #[case(Helmert1906{}, 120)]
    #[case(Hayford{}, 80)]
    #[case(Krasov40{}, 80)]
    #[case(Maupertuis1738{}, 80)]
    #[case(Plessis1817{}, 80)]
    #[case(SA1969{}, 80)]
    fn test_eccentricity_regional<T>(#[case] _elps:T, #[case] ulps:i64) where T: Model
    {
        assert_model_eccentricity!(T);
        assert_model_eccentricity_special!(T, ulps);
        assert_model_eccentricity_square!(T);
    }

    #[rstest]
    #[case(Sphere{})]
    #[case(SphereAuthalic{})]
    #[case(SpherePopular{})]
    #[case(SphereNormal{})]
    fn test_eccentricity_sphere<T>(#[case] _elps:T) where T: Model
    {
        assert_approx_eq!(f64, 0.0, T::eccentricity(0));
        assert_approx_eq!(f64, 0.0, T::eccentricity(1));
        assert_approx_eq!(f64, 0.0, T::eccentricity(2));
        assert_approx_eq!(f64, 0.0, T::eccentricity(3));
        assert_approx_eq!(f64, 0.0, T::eccentricity(4));
        assert_approx_eq!(f64, 0.0, T::P);
        assert_approx_eq!(f64, 1.0, T::Q);
        assert_approx_eq!(f64, 0.0, T::E1SQ);
        assert_approx_eq!(f64, 0.0, T::E2SQ);
        assert_approx_eq!(f64, 0.0, T::E3SQ);
    }

    #[rstest]
    #[case(WGS84{}, 5)]
    #[should_panic]
    fn test_eccentricity_panic<T>(#[case] _elps:T, #[case] i:usize) where T: Model
    {
        T::eccentricity(i);
    }

    #[rstest]
    #[case(WGS84{}, 4)]
    #[should_panic]
    fn test_eccentricity_square_panic<T>(#[case] _elps:T, #[case] i:usize) where T: Model
    {
        T::eccentricity_square(i);
    }
}
