// Required for criterion.rs benchmark suites
#![allow(unused)]

use criterion::{criterion_group, criterion_main, Criterion};
use hipparchus_mean::*;
use hipparchus_seq::Sequence;

pub fn bench_mean(c: &mut Criterion)
{
    let size = 1024;
    let v:Vec<f32> = Sequence::Lucas.vec(size);

    c.bench_function("mean.arithmetic.f32", |b| b.iter(||
    {
        let m = v.iter().mean(MeanAlgorithm::ArithmeticMean).unwrap();
    }));

    c.bench_function("mean.geometric.f32", |b| b.iter(||
    {
        let m = v.iter().mean(MeanAlgorithm::GeometricMean).unwrap();
    }));

    c.bench_function("mean.quadratic.f32", |b| b.iter(||
    {
        let m = v.iter().mean(MeanAlgorithm::QuadraticMean).unwrap();
    }));
    
    c.bench_function("mean.harmonic.f32", |b| b.iter(||
    {
        let m = v.iter().mean(MeanAlgorithm::HarmonicMean).unwrap();
    }));

    c.bench_function("mean.wma.f32", |b| b.iter(||
    {
        let m = v.iter().mean(MeanAlgorithm::WeightedMovingAverage).unwrap();
    }));
    
    c.bench_function("mean.ema.f32", |b| b.iter(||
    {
        let m = v.iter().mean(MeanAlgorithm::ExponentialMovingAverage(0.9)).unwrap();
    }));
}

criterion_group!
(
    name = benches;
    config = Criterion::default();
    targets = bench_mean
);

criterion_main!(benches);
