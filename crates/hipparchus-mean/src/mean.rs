use crate::value::Fp;

#[repr(i32)]
pub enum MeanAlgorithm
{
    ArithmeticMean = 0,
    GeometricMean = 1,
    QuadraticMean = 2,
    HarmonicMean = 3,
    WeightedMovingAverage = 4,
    ExponentialMovingAverage(f32)
}

pub trait Mean<'a, T>
where
    T: Fp + 'a,
    Self: Iterator<Item = &'a T> + 'a
{
    fn mean(self, algo:MeanAlgorithm) -> Option<T>;
    fn arithmetic_mean(self) -> Option<T>;
    fn geometric_mean(self) -> Option<T>;
    fn quadratic_mean(self) -> Option<T>;
    fn harmonic_mean(self) -> Option<T>;
    fn weighted_moving_avg(self) -> Option<T>;
    fn exponential_moving_avg(self, decay:f32) -> Option<T>;
}

impl<'a, T, I> Mean<'a, T> for I
where
    T: Fp + 'a,
    I: Iterator<Item = &'a T> + 'a,
{
    fn mean(self, algo:MeanAlgorithm) -> Option<T>
    {
        match algo
        {
            MeanAlgorithm::ArithmeticMean => self.arithmetic_mean(),
            MeanAlgorithm::GeometricMean => self.geometric_mean(),
            MeanAlgorithm::QuadraticMean => self.quadratic_mean(),
            MeanAlgorithm::HarmonicMean => self.harmonic_mean(),
            MeanAlgorithm::WeightedMovingAverage => self.weighted_moving_avg(),
            MeanAlgorithm::ExponentialMovingAverage(decay) => self.exponential_moving_avg(decay),
        }
    }

    fn arithmetic_mean(self) -> Option<T>
    {
        let mut total = 0;
        let sum = self.fold(T::zero(), |s,&x|
        {
            total += 1;
            s + x
        });
    
        if total <= 0 { None } else { Some(sum / T::from_i32(total).unwrap()) }
    }

    fn geometric_mean(self) -> Option<T>
    {
        let mut total = 0;
        let sum = self.fold(T::one(), |s,&x|
        {
            total += 1;
            s * x
        });

        if total <= 0 { None } else { Some(sum.powf(T::one()/T::from_i32(total).unwrap())) }
    }

    fn harmonic_mean(self) -> Option<T>
    {
        let mut total = 0;
        let sum = self.fold(T::zero(), |s,&x|
        {
            total += 1;
            s + T::one() / x
        });

        if total <= 0 { None } else { Some(T::from_i32(total).unwrap() / sum) }
    }

    fn quadratic_mean(self) -> Option<T>
    {
        let mut total = 0;
        let sum = self.fold(T::zero(), |s,&x|
        {
            total += 1;
            s + x * x
        });
    
        if total <= 0 { None } else { Some(sum.div(T::from_i32(total).unwrap()).sqrt()) }
    }

    fn weighted_moving_avg(self) -> Option<T>
    {
        let mut total = 0;
        let (sum, weight) = self.fold((T::zero(), T::zero()), |(s, w), &x|
        {
            total += 1;
            let k = T::from_i32(total).unwrap();
            (s + x * k, w + k)
        });
    
        if total <= 0 { None } else { Some(sum / weight) }
    }

    fn exponential_moving_avg(self, decay:f32) -> Option<T>
    {
        let mut empty = true;
        let decay = T::from_f32(decay).unwrap();
        let complement = T::one() - decay;
        let sum = self.fold(T::zero(), |s, &x|
        {
            let base = if empty { x } else { s };
            empty = false;
            decay * base + complement * x
        });
    
        if empty { None } else { Some(sum) }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use float_cmp::assert_approx_eq;
    use num::ToPrimitive;

    #[test]
    fn test_mean_arithmetic()
    {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_approx_eq!(f32, 3.0, v.iter().arithmetic_mean().unwrap());
        assert_approx_eq!(f32, 3.0, v.iter().mean(MeanAlgorithm::ArithmeticMean).unwrap());
    }

    #[test]
    fn test_mean_geometric()
    {
        let v = vec![1.0, 1.0, 1.0, 2.0, 4.0, 8.0];
        assert_approx_eq!(f32, 2.0, v.iter().geometric_mean().unwrap());
        assert_approx_eq!(f32, 2.0, v.iter().mean(MeanAlgorithm::GeometricMean).unwrap());
    }

    #[test]
    fn test_mean_quadratic()
    {
        let v = vec![1.0, 7.0];
        assert_approx_eq!(f32, 5.0, v.iter().quadratic_mean().unwrap());
        assert_approx_eq!(f32, 5.0, v.iter().mean(MeanAlgorithm::QuadraticMean).unwrap());
    }

    #[test]
    fn test_mean_harmonic()
    {
        let v = vec![1.0, 1.0, 0.5, 0.25];
        assert_approx_eq!(f32, 0.5, v.iter().harmonic_mean().unwrap());
        assert_approx_eq!(f32, 0.5, v.iter().mean(MeanAlgorithm::HarmonicMean).unwrap());
    }

    #[test]
    fn test_wma()
    {
        let n = 5;
        let v = (1..n+1).collect::<Vec<i32>>().iter().map(|&x|x.to_f32().unwrap()).collect::<Vec<f32>>();
        let expected = (2.0 * n.to_f32().unwrap() + 1.0) / 3.0;
        assert_approx_eq!(f32, expected, v.iter().weighted_moving_avg().unwrap());
        assert_approx_eq!(f32, expected, v.iter().mean(MeanAlgorithm::WeightedMovingAverage).unwrap());
    }

    #[test]
    fn test_ema()
    {
        let decay:f32 = 0.9;
        let mut v = vec![0.0f32; 4];
        v.insert(0, 1.0);
        let expected = decay.powi(v.len() as i32 - 1);
        assert_approx_eq!(f32, expected, v.iter().exponential_moving_avg(decay).unwrap());
        assert_approx_eq!(f32, expected, v.iter().mean(MeanAlgorithm::ExponentialMovingAverage(decay)).unwrap());
    }

    #[test]
    fn test_ema2()
    {
        let decay:f32 = 0.9;
        let mut v = vec![1.0f32; 4];
        v.insert(0, 0.0);
        let expected = 1.0 - decay.powi(v.len() as i32 - 1);
        assert_approx_eq!(f32, expected, v.iter().exponential_moving_avg(decay).unwrap());
        assert_approx_eq!(f32, expected, v.iter().mean(MeanAlgorithm::ExponentialMovingAverage(decay)).unwrap());
    }

    #[test]
    fn test_mean_eq()
    {
        let v = vec![1.0; 10];
        assert_approx_eq!(f32, 1.0, v.iter().arithmetic_mean().unwrap());
        assert_approx_eq!(f32, 1.0, v.iter().geometric_mean().unwrap());
        assert_approx_eq!(f32, 1.0, v.iter().quadratic_mean().unwrap());
        assert_approx_eq!(f32, 1.0, v.iter().harmonic_mean().unwrap());
        assert_approx_eq!(f32, 1.0, v.iter().weighted_moving_avg().unwrap());
        assert_approx_eq!(f32, 1.0, v.iter().exponential_moving_avg(0.9).unwrap());
    }

    #[test]
    fn test_mean_empty()
    {
        assert_eq!(None, std::iter::empty::<&f32>().arithmetic_mean());
        assert_eq!(None, std::iter::empty::<&f32>().geometric_mean());
        assert_eq!(None, std::iter::empty::<&f32>().quadratic_mean());
        assert_eq!(None, std::iter::empty::<&f32>().harmonic_mean());
        assert_eq!(None, std::iter::empty::<&f32>().weighted_moving_avg());
        assert_eq!(None, std::iter::empty::<&f32>().exponential_moving_avg(0.9));
    }
}
