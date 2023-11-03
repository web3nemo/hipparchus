use hipparchus_mean::Fp;
use crate::metrics::Metrics;

/// Trait for simularity metrics between two vectors
#[repr(i32)]
#[derive(Clone,PartialEq,Debug)]
pub enum VectorMetrics
{
    /// Dot product
    DotProduct = 1,

    /// Cosine similarity
    Cosine = 2,
}

impl<T:Fp> Metrics<&[T], T> for VectorMetrics
{
    fn measure(self, x:&[T], y:&[T]) -> T
    {
        let it = x.iter().zip(y.iter());
        match self
        {
            VectorMetrics::DotProduct => it.fold( T::zero(), | agg, (&a, &b) | agg + a * b),
            VectorMetrics::Cosine => 
            {
                let mut aa = T::zero();
                let mut bb = T::zero();
                let ab = it.fold( T::zero(), | agg, (&a, &b) |
                {
                    aa = aa + a * a;
                    bb = bb + b * b;
                    agg + a * b
                });
                ab / (aa.sqrt() * bb.sqrt())
            },
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use float_cmp::assert_approx_eq;
    use hipparchus_mean::LpNorm;

    #[test]
    fn test_dotproduct_overlap()
    {
        let v = 1.0;
        let x = [v, v];
        let y = [2.0 * v, 2.0 * v];
        let l2x = x.iter().l2norm().unwrap();
        let l2y = y.iter().l2norm().unwrap();
        let expected =  l2x * l2y;
        assert_approx_eq!(f32, expected, VectorMetrics::DotProduct.measure(&x, &y));
        assert_approx_eq!(f32, expected, VectorMetrics::DotProduct.measure(&y, &x));
    }

    #[test]
    fn test_dotproduct_orthogonal()
    {
        let v = 1.0;
        let x = [v, v];
        let y = [v, -v];
        let expected = 0.0;
        assert_approx_eq!(f32, expected, VectorMetrics::DotProduct.measure(&x, &y));
        assert_approx_eq!(f32, expected, VectorMetrics::DotProduct.measure(&y, &x));
    }

    #[test]
    fn test_cosine_opposite()
    {
        let v = 1.0;
        let x = [v, v];
        let y = [-v, -v];
        let expected = f32::cos(180.0f32.to_radians());
        assert_approx_eq!(f32, expected, VectorMetrics::Cosine.measure(&x, &y));
        assert_approx_eq!(f32, expected, VectorMetrics::Cosine.measure(&x, &y));
    }

    #[test]
    fn test_cosine_overlap()
    {
        let v = 1.0;
        let x = [v, v];
        let y = [2.0 * v, 2.0 * v];
        let expected = f32::cos(0.0f32.to_radians());
        assert_approx_eq!(f32, expected, VectorMetrics::Cosine.measure(&x, &y));
        assert_approx_eq!(f32, expected, VectorMetrics::Cosine.measure(&y, &x));
    }

    #[test]
    fn test_cosine_eq()
    {
        let x = [1.0, 2.0];
        let expected = f32::cos(0.0f32.to_radians());
        assert_approx_eq!(f32, expected, VectorMetrics::Cosine.measure(&x, &x));
    }
}
