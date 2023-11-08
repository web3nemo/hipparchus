use crate::value::Fp;

/// The algorithm to use for computing the mean
#[repr(i32)]
pub enum MeanAlgorithm
{
    /// Arithmetic mean
    ArithmeticMean = 0,

    /// Geometric mean
    GeometricMean = 1,

    /// Quadratic mean
    QuadraticMean = 2,

    /// Harmonic mean
    HarmonicMean = 3,

    /// Simple moving average
    SimpleMovingAverage = 10,

    /// Cumulative moving average
    CumulativeMovingAverage = 11,

    /// Weighted moving average
    WeightedMovingAverage = 12,

    /// Exponential moving average with decay factor
    ExponentialMovingAverage(f32) = 13,
}

/// Compute the mean of a vector
pub trait Mean<'a, T>
where
    T: Fp + 'a,
    Self: Iterator<Item = &'a T> + 'a
{
    /// Compute the mean of a vector with the specified algorithm
    fn mean(self, algo:MeanAlgorithm) -> Option<T>;

    /// Compute the arithmetic mean of a vector
    fn arithmetic_mean(self) -> Option<T>;

    /// Compute the geometric mean of a vector
    fn geometric_mean(self) -> Option<T>;

    /// Compute the quadratic mean of a vector
    fn quadratic_mean(self) -> Option<T>;

    /// Compute the harmonic mean of a vector
    fn harmonic_mean(self) -> Option<T>;

    /// Compute the weighted moving average of a vector
    fn weighted_moving_avg(self) -> Option<T>;

    /// Compute the exponential moving average of a vector with the specified decay factor
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
            MeanAlgorithm::SimpleMovingAverage => self.arithmetic_mean(),
            MeanAlgorithm::CumulativeMovingAverage => self.arithmetic_mean(),
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
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(vec![1.0, 2.0, 3.0, 4.0, 5.0], MeanAlgorithm::ArithmeticMean, 3.0)]
    #[case(vec![1.0, 1.0, 1.0, 2.0, 4.0, 8.0], MeanAlgorithm::GeometricMean, 2.0)]
    #[case(vec![1.0, 7.0], MeanAlgorithm::QuadraticMean, 5.0)]
    #[case(vec![1.0, 1.0, 0.5, 0.25], MeanAlgorithm::HarmonicMean, 0.5)]
    #[case(vec![1.0, 2.0, 3.0, 4.0, 5.0], MeanAlgorithm::SimpleMovingAverage, 3.0)]
    #[case(vec![1.0, 2.0, 3.0, 4.0, 5.0], MeanAlgorithm::CumulativeMovingAverage, 3.0)]
    #[case(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0], MeanAlgorithm::WeightedMovingAverage, 5.0)]
    #[case(vec![1.0, 0.0, 0.0, 0.0, 0.0], MeanAlgorithm::ExponentialMovingAverage(0.9), 0.9f32.powi(4))]
    #[case(vec![0.0, 1.0, 1.0, 1.0, 1.0], MeanAlgorithm::ExponentialMovingAverage(0.9), 1.0 - 0.9f32.powi(4))]
    fn test_mean(#[case] v: Vec<f32>, #[case] algo: MeanAlgorithm, #[case] expected: f32)
    {
        assert_approx_eq!(f32, expected, v.iter().mean(algo).unwrap());
    }

    #[rstest]
    #[case(MeanAlgorithm::ArithmeticMean)]
    #[case(MeanAlgorithm::GeometricMean)]
    #[case(MeanAlgorithm::QuadraticMean)]
    #[case(MeanAlgorithm::HarmonicMean)]
    #[case(MeanAlgorithm::SimpleMovingAverage)]
    #[case(MeanAlgorithm::CumulativeMovingAverage)]
    #[case(MeanAlgorithm::WeightedMovingAverage)]
    #[case(MeanAlgorithm::ExponentialMovingAverage(0.5))]
    fn test_mean_eq(#[case] algo: MeanAlgorithm)
    {
        let expected = 1.0;
        let v = vec![expected; 10];
        assert_approx_eq!(f32, expected, v.iter().mean(algo).unwrap());
    }

    #[rstest]
    #[case(MeanAlgorithm::ArithmeticMean)]
    #[case(MeanAlgorithm::GeometricMean)]
    #[case(MeanAlgorithm::QuadraticMean)]
    #[case(MeanAlgorithm::HarmonicMean)]
    #[case(MeanAlgorithm::SimpleMovingAverage)]
    #[case(MeanAlgorithm::CumulativeMovingAverage)]
    #[case(MeanAlgorithm::WeightedMovingAverage)]
    #[case(MeanAlgorithm::ExponentialMovingAverage(0.5))]
    fn test_mean_with_zero(#[case] algo: MeanAlgorithm)
    {
        let expected = 0.0;
        let v = vec![expected; 10];
        assert_approx_eq!(f32, expected, v.iter().mean(algo).unwrap());
    }

    #[rstest]
    #[rstest]
    #[case(MeanAlgorithm::ArithmeticMean)]
    #[case(MeanAlgorithm::GeometricMean)]
    #[case(MeanAlgorithm::QuadraticMean)]
    #[case(MeanAlgorithm::HarmonicMean)]
    #[case(MeanAlgorithm::SimpleMovingAverage)]
    #[case(MeanAlgorithm::CumulativeMovingAverage)]
    #[case(MeanAlgorithm::WeightedMovingAverage)]
    #[case(MeanAlgorithm::ExponentialMovingAverage(0.5))]
    fn test_mean_empty(#[case] algo: MeanAlgorithm)
    {
        assert_eq!(None, std::iter::empty::<&f32>().mean(algo));
    }
}
