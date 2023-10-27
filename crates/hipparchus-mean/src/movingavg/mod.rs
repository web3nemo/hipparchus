pub mod simple;
pub mod exponential;
pub mod weighted;
pub mod cumulative;

pub enum MovingAverage
{
    Simple = 1,
    Weighted = 2,
    Exponential = 3,
    Cumulative = 4,
}

