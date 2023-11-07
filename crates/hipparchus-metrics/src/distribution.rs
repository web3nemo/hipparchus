use hipparchus_mean::Fp;
use crate::metrics::Metrics;

/// Metrics for distributions
#[repr(i32)]
#[derive(Clone,PartialEq,Debug)]
pub enum DistributionMetrics
{
    /// Dot product
    CrossEntropy = 1,

    /// KL divergence
    KullbackLeiblerDivergence = 2,

    /// JS divergence
    JensenShannonDivergence = 3,

    /// Hellinger distance
    Hellinger = 4,
}

impl<T:Fp> Metrics<&[T], T> for DistributionMetrics
{
    fn measure(self, x:&[T], y:&[T]) -> T
    {
        let it = x.iter().zip(y.iter());
        match self
        {
            DistributionMetrics::CrossEntropy => it.fold(T::zero(), | agg, (&p, &q)|
            {
                agg - p.mul(q.ln())
            }),
            DistributionMetrics::KullbackLeiblerDivergence => it.fold(T::zero(), | agg, (&p, &q)|
            {
                agg + p.mul(p.ln()-q.ln())
            }),
            DistributionMetrics::JensenShannonDivergence => 
            {
                let half = T::from(0.5f64).unwrap();
                let v = it.map(|(p, &q)| p.add(q).mul(half) ).collect::<Vec<T>>();
                let m = v.as_slice().try_into().unwrap();
                let klxm = DistributionMetrics::KullbackLeiblerDivergence.measure(x, m);
                let klym = DistributionMetrics::KullbackLeiblerDivergence.measure(y, m);
                (klxm+klym) * half
            }
            DistributionMetrics::Hellinger => it.fold(T::zero(), | agg, (&p, &q)|
            {
                agg + (p.sqrt() - q.sqrt()).powi(2)
            }).div(T::from(2).unwrap()).sqrt(),
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(vec![0.5, 0.5], vec![0.5, 0.5], DistributionMetrics::CrossEntropy, 0.693147)]
    #[case(vec![0.00001, 0.99999], vec![0.99999, 0.00001], DistributionMetrics::KullbackLeiblerDivergence, 11.512684)]
    #[case(vec![0.00001, 0.99999], vec![0.99999, 0.00001], DistributionMetrics::JensenShannonDivergence, 0.6930221)]
    #[case(vec![0.0, 1.0], vec![1.0, 0.0], DistributionMetrics::Hellinger, 1.0)]
    fn test_distribution(#[case] d1: Vec<f32>, #[case] d2: Vec<f32>, #[case] m: DistributionMetrics, #[case] expected :f32)
    {
        let actual = m.measure(&d1, &d2);
        assert_approx_eq!(f32, expected, actual);
    }

    #[rstest]
    #[case(vec![0.5, 0.5], DistributionMetrics::KullbackLeiblerDivergence)]
    #[case(vec![0.5, 0.5], DistributionMetrics::JensenShannonDivergence)]
    #[case(vec![0.5, 0.5], DistributionMetrics::Hellinger)]
    fn test_distribution_zero(#[case] d: Vec<f32>, #[case] m: DistributionMetrics)
    {
        let actual = m.measure(&d, &d);
        assert_approx_eq!(f32, 0.0, actual);
    }
}
