use crate::value::Fp;

pub trait MovingAverage<T:Fp>
{
    fn average(self:&Self) -> Option<T>;
    fn push(self:&mut Self, v:T) -> T;
}

pub struct CumulativeMovingAverage<T:Fp>
{
    average: T,
    total: i32,
}

impl<T:Fp> CumulativeMovingAverage<T>
{
    pub fn new() -> CumulativeMovingAverage<T>
    {
        Self::from(T::zero(), 0)
    }

    pub fn from(average: T, total: i32) -> CumulativeMovingAverage<T>
    {
        CumulativeMovingAverage
        {
            average,
            total,
        }
    }
}

impl<T:Fp> MovingAverage<T> for CumulativeMovingAverage<T>
{
    fn average(self:&Self) -> Option<T>
    {
        if self.total <= 0 { None } else { Some(self.average) }
    }

    fn push(self:&mut Self, v:T) -> T
    {
        let total = T::from_i32(self.total).unwrap();
        self.average = (self.average * total + v) / (total + T::one());
        self.total += 1;
        self.average
    }
}

pub struct WeightedMovingAverage<T:Fp>
{
    total: u64,
    agg: T,
    weight: T,
}

impl<T:Fp> WeightedMovingAverage<T>
{
    pub fn new() -> WeightedMovingAverage<T>
    {
        Self::from(0, T::zero())
    }

    pub fn from(total: u64, agg:T) -> WeightedMovingAverage<T>
    {
        WeightedMovingAverage
        { 
            total, agg,
            weight: T::from_u64( (total + 1) * total / 2 ).unwrap()
        }
    }
}

impl<T:Fp> MovingAverage<T> for WeightedMovingAverage<T>
{
    fn average(self:&Self) -> Option<T>
    {
        if self.total <= 0 { None } else { Some(self.agg / self.weight) }
    }

    fn push(self:&mut Self, v:T) -> T
    {
        self.total += 1;
        let w = T::from_u64(self.total).unwrap();
        self.agg = self.agg + w * v;
        self.weight = self.weight + w;
        self.agg / self.weight
    }
}

pub struct ExponentialMovingAverage<T:Fp>
{
    average: Option<T>,
    decay: T,
}

impl<T:Fp> ExponentialMovingAverage<T>
{
    pub fn new(decay:f32) -> ExponentialMovingAverage<T>
    {
        Self::from(None, decay)
    }

    pub fn from(init: Option<T>, decay: f32) -> ExponentialMovingAverage<T>
    {
        ExponentialMovingAverage
        {
            average: init,
            decay: T::from_f32(decay).unwrap(),
        }
    }
}

impl<T:Fp> MovingAverage<T> for ExponentialMovingAverage<T>
{
    fn average(self:&Self) -> Option<T>
    {
        self.average
    }

    fn push(self:&mut Self, v:T) -> T
    {
        let current = self.average.unwrap_or(v);
        let next = current * self.decay + v * (T::one() - self.decay);
        self.average = Some(next);
        next
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_cma_new()
    {
        let cma = CumulativeMovingAverage::<f32>::new();
        assert_eq!(0, cma.total);
        assert_eq!(0.0, cma.average);
        assert_eq!(None, cma.average());
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
            let actual_push = cma.push(d);
            let actual_average = cma.average().unwrap();
            assert_approx_eq!(f32, actual_push, actual_average);
            assert_approx_eq!(f32, e, actual_push);
            i += 1;
        }
    }

    #[test]
    fn test_wma_new()
    {
        let wma = WeightedMovingAverage::<f32>::new();
        assert_eq!(0, wma.total);
        assert_eq!(0.0, wma.agg);
        assert_eq!(0.0, wma.weight);
        assert_eq!(None, wma.average());
    }

    #[test]
    fn test_wma_from()
    {
        let wma = WeightedMovingAverage::<f32>::from(4, 30.0);
        assert_eq!(4, wma.total);
        assert_eq!(30.0, wma.agg);
        assert_eq!(10.0, wma.weight);
        assert_eq!(3.0, wma.average().unwrap());
    }

    #[test]
    fn test_wma()
    {
        //  weight:  1,  3,  6, 10
        //       w:  1,  2,  3,  4
        //    item:  6,  3,  4,  4
        //   delta:  6,  6, 12, 16
        //     agg:  6, 12, 24, 40
        // average:  6,  4,  4,  4
        let data = vec![6.0, 3.0, 4.0, 4.0];
        let expected = vec![6.0, 4.0, 4.0, 4.0];
        let mut wma = WeightedMovingAverage::<f32>::new();
        let mut i = 0;
        for d in data
        {
            let e = expected[i];
            let actual_push = wma.push(d);
            let actual_average = wma.average().unwrap();
            assert_approx_eq!(f32, actual_push, actual_average);
            assert_approx_eq!(f32, e, actual_push);
            i += 1;
        }
    }
    
    #[test]
    fn test_ema_new()
    {
        let ema = ExponentialMovingAverage::<f32>::new(0.9);
        assert_eq!(0.9, ema.decay);
        assert_eq!(None, ema.average);
        assert_eq!(None, ema.average());
    }

    #[test]
    fn test_ema_from()
    {
        let ema = ExponentialMovingAverage::<f32>::from(Some(1.0), 0.9);
        assert_eq!(0.9, ema.decay);
        assert_eq!(1.0, ema.average.unwrap());
        assert_eq!(1.0, ema.average().unwrap());
    }

    #[test]
    fn test_ema()
    {
        let decay = 0.9;
        let data = vec![1.0, 0.0, 0.0, 0.0];
        let expected = vec![1.0, decay, decay * decay, decay * decay * decay];
        let mut ema = ExponentialMovingAverage::<f32>::new(decay);
        let mut i = 0;
        for d in data
        {
            let e = expected[i];
            let actual_push = ema.push(d);
            let actual_average = ema.average().unwrap();
            assert_approx_eq!(f32, actual_push, actual_average);
            assert_approx_eq!(f32, e, actual_push);
            i += 1;
        }
    }
}
