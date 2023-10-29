use crate::value::Fp;
use crate::moving::traits::MovingAverage;

pub struct CumulativeMovingAverage<T:Fp>
{
    average: T,
    total: i32,
}

impl<T:Fp> CumulativeMovingAverage<T>
{
    pub fn new() -> CumulativeMovingAverage<T>
    {
        CumulativeMovingAverage
        {
            average: T::from_i32(0).unwrap(),
            total: 0,
        }
    }

    pub fn from(average: T, total: i32) -> CumulativeMovingAverage<T>
    {
        CumulativeMovingAverage
        {
            average: average,
            total: total,
        }
    }
}

impl<T:Fp> MovingAverage<T> for CumulativeMovingAverage<T>
{
    fn average(self:&Self) -> Option<T>
    {
        match self.total
        {
            0 => None,
            _ => Some(self.average)
        }
    }

    fn push(self:&mut Self, v:T) -> T
    {
        let agg = self.average * T::from_i32(self.total).unwrap();
        self.total += 1;
        self.average = (agg + v) / T::from_i32(self.total).unwrap();
        self.average
    }
}

#[cfg(test)]
mod tests 
{
    use float_cmp::assert_approx_eq;
    use crate::moving::cumulative::MovingAverage;
    use super::CumulativeMovingAverage;

    #[test]
    fn test_cma_new()
    {
        let cma = CumulativeMovingAverage::<f32>::new();
        assert_eq!(0, cma.total);
        assert_approx_eq!(f32, 0.0, cma.average);
        assert_eq!(Option::<f32>::None, cma.average());
    }

    #[test]
    fn test_cma_from()
    {
        let cma = CumulativeMovingAverage::<f32>::from(1.0, 5);
        assert_eq!(5, cma.total);
        assert_approx_eq!(f32, 1.0, cma.average);
        assert_approx_eq!(f32, 1.0, cma.average().unwrap());
    }

    #[test]
    fn test_cma()
    {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = vec![1.0, 1.5, 2.0, 2.5, 3.0];

        let mut cma = CumulativeMovingAverage::<f32>::new();
        let mut i = 0;
        for d in data
        {
            let e = expected[i];
            let actual = cma.push(d);
            let average = cma.average().unwrap();
            assert_approx_eq!(f32, actual, average);
            assert_approx_eq!(f32, e, actual);
            i += 1;
        }
    }
}
