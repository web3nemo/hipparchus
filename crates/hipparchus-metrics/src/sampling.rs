use hipparchus_mean::Fp;
use crate::metrics::Metrics;

/// Metrics for sampling
#[repr(i32)]
#[derive(Clone,PartialEq,Debug)]
pub enum SamplingMetrics
{
    /// Jaccard index
    Jaccard = 1,

    /// Sorensen-Dice index
    SorensenDice = 2,

    /// Bray-Curtis dissimilarity
    BrayCurtis = 3,
}

impl<T:Fp> Metrics<&[T], T> for SamplingMetrics
{
    fn measure(self, x:&[T], y:&[T]) -> T
    {
        let it = x.iter().zip(y.iter());
        match self
        {
            SamplingMetrics::Jaccard => 
            {
                let (intersect, union) = it.fold((T::zero(), T::zero()), | (i, u), (&p, &q)|
                {(
                    i + p.min(q),
                    u + p.max(q),
                )});
                intersect / union
            },
            SamplingMetrics::SorensenDice => 
            {
                let (intersect, agg) = it.fold((T::zero(), T::zero()), | (i, agg), (&p, &q)|
                {(
                    i + p.min(q),
                    agg + p + q,
                )});
                T::from(2).unwrap() * intersect / agg
            },
            SamplingMetrics::BrayCurtis =>
            {
                let (intersect, agg) = it.fold((T::zero(), T::zero()), | (i, agg), (&p, &q)|
                {(
                    i + p.min(q),
                    agg + p + q,
                )});
                T::one() - T::from(2).unwrap() * intersect / agg
            },
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
    #[case(vec![1.0, 0.0, 1.0], vec![0.0, 1.0, 3.0], SamplingMetrics::Jaccard, 0.2)]
    #[case(vec![1.0, 0.0, 1.0], vec![0.0, 1.0, 3.0], SamplingMetrics::SorensenDice, 0.33333334)]
    #[case(vec![1.0, 0.0, 1.0], vec![0.0, 1.0, 3.0], SamplingMetrics::BrayCurtis, 0.6666666)]
    fn test_sampling(#[case] d1: Vec<f32>, #[case] d2: Vec<f32>, #[case] m: SamplingMetrics, #[case] expected :f32)
    {
        let actual = m.measure(&d1, &d2);
        assert_approx_eq!(f32, expected, actual);
    }

    #[rstest]
    #[case(vec![5.0, 5.0], SamplingMetrics::Jaccard, 1.0)]
    #[case(vec![5.0, 5.0], SamplingMetrics::SorensenDice, 1.0)]
    #[case(vec![5.0, 5.0], SamplingMetrics::BrayCurtis, 0.0)]
    fn test_sampling_eq(#[case] d: Vec<f32>, #[case] m: SamplingMetrics, #[case] expected :f32)
    {
        let actual = m.measure(&d, &d);
        assert_approx_eq!(f32, expected, actual);
    }
}
