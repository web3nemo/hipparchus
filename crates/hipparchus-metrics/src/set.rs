use crate::metrics::Metrics;
use crate::sortedvec::sorted_intersect;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SetMetrics
{
    KumarHassebrook = 1,
    Jaccard = 2,
    Sorensen = 3,
}

impl<T:PartialOrd> Metrics<&[T], f64> for SetMetrics
{
    fn measure(self, x:&[T], y:&[T]) -> f64
    {
        match self
        {
            SetMetrics::KumarHassebrook => SetMetrics::kumar_hassebrook(x, y),
            SetMetrics::Jaccard => SetMetrics::jaccard(x, y),
            SetMetrics::Sorensen => SetMetrics::sorensen(x, y),
        }
    }
}

impl SetMetrics
{
    pub fn kumar_hassebrook<T:PartialOrd>(x:&[T], y:&[T]) -> f64
    {
        let total_x = x.len();
        let total_y = y.len();
    
        match (total_x, total_y)
        {
            (0, 0) => 1.0,
            (0, _) => 0.0,
            (_, 0) => 0.0,
            (_, _) =>
            {
                let i = sorted_intersect(&x, &y);
                let u = total_x + total_y - i;
                (i as f64) / (u as f64)
            }
        }
    }
    
    pub fn jaccard<T:PartialOrd>(x:&[T], y:&[T]) -> f64
    {
        1.0 - SetMetrics::kumar_hassebrook(x, y)
    }

    pub fn sorensen<T:PartialOrd>(x:&[T], y:&[T]) -> f64
    {
        let total_x = x.len();
        let total_y = y.len();

        match (total_x, total_y)
        {
            (0, 0) => 0.0,
            (0, _) => 1.0,
            (_, 0) => 1.0,
            (_, _) =>
            {
                let i = sorted_intersect(&x, &y);
                1.0 - 2.0 * (i as f64) / (total_x + total_y) as f64
            }
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
    #[case(vec![1, 2, 3, 5, 7], vec![1, 2, 4, 6, 8], SetMetrics::KumarHassebrook, 0.25)]
    #[case(vec![1, 3, 5, 7, 8], vec![2, 4, 6, 7, 8], SetMetrics::KumarHassebrook, 0.25)]
    #[case(vec![1, 2, 3, 5], vec![1, 2, 4, 6, 7, 8], SetMetrics::KumarHassebrook, 0.25)]
    #[case(vec![1, 3, 7, 8], vec![2, 4, 5, 6, 7, 8], SetMetrics::KumarHassebrook, 0.25)]
    #[case(vec![1, 2, 3, 5, 7], vec![1, 2, 4, 6, 8], SetMetrics::Jaccard, 0.75)]
    #[case(vec![1, 3, 5, 7, 8], vec![2, 4, 6, 7, 8], SetMetrics::Jaccard, 0.75)]
    #[case(vec![1, 2, 3, 5], vec![1, 2, 4, 6, 7, 8], SetMetrics::Jaccard, 0.75)]
    #[case(vec![1, 3, 7, 8], vec![2, 4, 5, 6, 7, 8], SetMetrics::Jaccard, 0.75)]
    #[case(vec![1, 2, 3, 5, 7], vec![1, 2, 4, 6, 8], SetMetrics::Sorensen, 0.6)]
    #[case(vec![1, 3, 5, 7, 8], vec![2, 4, 6, 7, 8], SetMetrics::Sorensen, 0.6)]
    #[case(vec![1, 2, 3, 5], vec![1, 2, 4, 6, 7, 8], SetMetrics::Sorensen, 0.6)]
    #[case(vec![1, 3, 7, 8], vec![2, 4, 5, 6, 7, 8], SetMetrics::Sorensen, 0.6)]
    fn test_set_metrics(#[case] v1: Vec<i32>, #[case] v2: Vec<i32>, #[case] metrics: SetMetrics, #[case] distance: f64)
    {
        assert_approx_eq!(f64, distance, metrics.measure(&v1, &v2));
    }

    #[rstest]
    #[case(vec![1, 2, 3], SetMetrics::KumarHassebrook, 0.0)]
    #[case(vec![1, 2, 3], SetMetrics::Jaccard, 1.0)]
    #[case(vec![1, 2, 3], SetMetrics::Sorensen, 1.0)]
    fn test_set_metrics_empty(#[case] v: Vec<i32>, #[case] metrics: SetMetrics, #[case] distance: f64)
    {
        assert_approx_eq!(f64, distance, metrics.measure(&v, &[]));
        assert_approx_eq!(f64, distance, metrics.measure(&[], &v));
    }

    #[rstest]
    #[case(vec![] as Vec<i32>, SetMetrics::KumarHassebrook, 1.0)]
    #[case(vec![1, 2, 3], SetMetrics::KumarHassebrook, 1.0)]
    #[case(vec![] as Vec<i32>, SetMetrics::Jaccard, 0.0)]
    #[case(vec![1, 2, 3], SetMetrics::Jaccard, 0.0)]
    #[case(vec![] as Vec<i32>, SetMetrics::Sorensen, 0.0)]
    #[case(vec![1, 2, 3], SetMetrics::Sorensen, 0.0)]
    fn test_set_metrics_eq(#[case] v: Vec<i32>, #[case] metrics: SetMetrics, #[case] distance: f64)
    {
        assert_approx_eq!(f64, distance, metrics.measure(&v, &v));
    }
}
