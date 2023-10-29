use crate::value::Fp;

pub trait MovingAverage<T:Fp>
{
    fn average(self:&Self) -> Option<T>;
    fn push(self:&mut Self, v:T) -> T;
}

pub enum MovingAverageAlgorithm
{
    Simple,
    Weighted,
    Exponential,
    Cumulative,
}
