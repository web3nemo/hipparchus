// Required for criterion.rs benchmark suites
#![allow(unused)]

use criterion::{criterion_group, criterion_main, Criterion};
use hipparchus_mean::*;
use hipparchus_seq::Sequence;

pub fn bench_norm(c: &mut Criterion)
{
    let size = 10;
    let v:Vec<f32> = Sequence::Fibonacci.vec(size);

    c.bench_function("norm.l0.f32", |b| b.iter(||
    {
        let n = v.iter().l0norm();
    }));
    
    c.bench_function("norm.l1.f32", |b| b.iter(||
    {
        let n = v.iter().l1norm();
    }));

    c.bench_function("norm.l2.f32", |b| b.iter(||
    {
        let n = v.iter().l2norm();
    }));

    c.bench_function("norm.lpinf.f32", |b| b.iter(||
    {
        let n = v.iter().lpnorm_inf();
    }));

    c.bench_function("norm.lpinf.f32", |b| b.iter(||
    {
        let n = v.iter().lpnorm(3.0);
    }));

}

criterion_group!
(
    name = benches;
    config = Criterion::default();
    targets = bench_norm
);

criterion_main!(benches);
