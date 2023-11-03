use hipparchus_mean::Fp;
use crate::metrics::Metrics;

#[repr(i32)]
#[derive(Clone,PartialEq,Debug)]
pub enum PointMetrics<T:Fp>
{
    /// Manhattan distance, equivalent to Minkowski distance with p=1
    Manhattan = 1,

    /// Euclidean distance, equivalent to Minkowski distance with p=2
    Euclidean = 2,

    /// Chebyshev distance, equivalent to Minkowski distance with p=Inf
    Chebyshev = 3,

    /// Minkowski distance where p=1, 2, ..., Inf
    Minkowski(T) = 4,

    /// Canberra distance, d=∑|Pi−Qi|/(|Pi|+|Qi|)
    Canberra = 11,

    /// Gower distance, d=∑|Pi−Qi|/n
    Gower = 12,
}

impl<T:Fp> Metrics<&[T], T> for PointMetrics<T>
{
    fn measure(self, x:&[T], y:&[T]) -> T
    {
        let it = x.iter().zip(y.iter());
        match self
        {
            PointMetrics::Manhattan => it.fold( T::zero(), | agg, (&a, &b) |
            {
                let d = a.sub(b).abs();
                agg + d
            }),
            PointMetrics::Euclidean => it.fold( T::zero(), | agg, (&a, &b) |
            {
                let d = a.sub(b);
                agg + d * d
            }).sqrt(),
            PointMetrics::Chebyshev => it.fold( T::zero(), | agg, (&a, &b) |
            {
                let d = a.sub(b).abs();
                agg.max(d)
            }),
            PointMetrics::Minkowski(p) => it.fold( T::zero(), | agg, (&a, &b) |
            {
                let d = a.sub(b).abs();
                agg + d.powf(p)
            }).powf(p.inv()),
            PointMetrics::Canberra => it.fold( T::zero(), | agg, (&a, &b) |
            {
                let d = a.sub(b).abs();
                let s = a.abs() + b.abs();
                agg + d / s
            }),
            PointMetrics::Gower => 
            {
                let mut total = 0;
                it.fold( T::zero(), | agg, (&a, &b) |
                {
                    let d = a.sub(b).abs();
                    total += 1;
                    agg + d
                }) / T::from(total).unwrap()
            },
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_manhattan()
    {
        let x = [0.0, 1.0];
        let y = [1.0, -1.0];
        let expected = 3.0;

        assert_approx_eq!(f32, expected, PointMetrics::Manhattan.measure(&x, &y));
        assert_approx_eq!(f32, expected, PointMetrics::Manhattan.measure(&y, &x));
    }

    #[test]
    fn test_euclidean()
    {
        let x = [0.0, 1.0];
        let y = [1.0, -1.0];
        let expected = 5.0;

        assert_approx_eq!(f32, expected, PointMetrics::Euclidean::<f32>.measure(&x, &y).powi(2));
        assert_approx_eq!(f32, expected, PointMetrics::Euclidean::<f32>.measure(&y, &x).powi(2));
    }

    #[test]
    fn test_chebyshev()
    {
        let x = [0.0, 1.0];
        let y = [1.0, -1.0];
        let expected = 2.0;

        assert_approx_eq!(f32, expected, PointMetrics::Chebyshev.measure(&x, &y));
        assert_approx_eq!(f32, expected, PointMetrics::Chebyshev.measure(&y, &x));
    }

    #[test]
    fn test_minkowski()
    {
        let x = [0.0, 1.0];
        let y = [1.0, -1.0];

        assert_approx_eq!(f32, 3.0, PointMetrics::Minkowski(1.0f32).measure(&x, &y));
        assert_approx_eq!(f32, 3.0, PointMetrics::Minkowski(1.0f32).measure(&y, &x));
        assert_approx_eq!(f32, 5.0, PointMetrics::Minkowski(2.0f32).measure(&x, &y).powi(2));
        assert_approx_eq!(f32, 5.0, PointMetrics::Minkowski(2.0f32).measure(&y, &x).powi(2));
        assert_approx_eq!(f32, 9.0, PointMetrics::Minkowski(3.0f32).measure(&x, &y).powi(3));
        assert_approx_eq!(f32, 9.0, PointMetrics::Minkowski(3.0f32).measure(&y, &x).powi(3));
    }

    #[test]
    fn test_canberra()
    {
        let x = [0.0, 1.0];
        let y = [1.0, -1.0];
        let expected = 2.0;
        
        assert_approx_eq!(f32, expected, PointMetrics::Canberra.measure(&x, &y));
        assert_approx_eq!(f32, expected, PointMetrics::Canberra.measure(&y, &x));
    }

    #[test]
    fn test_gower()
    {
        let x = [0.0, 1.0];
        let y = [1.0, -1.0];
        let expected = 1.5;

        assert_approx_eq!(f32, expected, PointMetrics::Gower.measure(&x, &y));
        assert_approx_eq!(f32, expected, PointMetrics::Gower.measure(&y, &x));
    }

    #[test]
    fn test_measure_eq()
    {
        let x = [1.0, 2.0];

        assert_approx_eq!(f32, 0.0, PointMetrics::Manhattan.measure(&x, &x));
        assert_approx_eq!(f32, 0.0, PointMetrics::Euclidean.measure(&x, &x));
        assert_approx_eq!(f32, 0.0, PointMetrics::Chebyshev.measure(&x, &x));
        assert_approx_eq!(f32, 0.0, PointMetrics::Minkowski(1.0).measure(&x, &x));
        assert_approx_eq!(f32, 0.0, PointMetrics::Canberra.measure(&x, &x));
        assert_approx_eq!(f32, 0.0, PointMetrics::Gower.measure(&x, &x));
    }
}
