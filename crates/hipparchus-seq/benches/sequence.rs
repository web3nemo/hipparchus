// Required for criterion.rs benchmark suites
#![allow(unused)]

use criterion::{criterion_group, criterion_main, Criterion};
use hipparchus_seq::*;

pub fn bench_seq(c: &mut Criterion)
{
    let size = 10;

    c.bench_function("seq.natural.i64", |b| b.iter(||
    {
        let v:Vec<i64> = Sequence::Natural(true).vec(size);
    }));

    c.bench_function("seq.fibonacci.i64", |b| b.iter(||
    {
        let v:Vec<i64> = Sequence::Fibonacci.vec(size);
    }));
        
    c.bench_function("seq.catalan.i64", |b| b.iter(||
    {
        let v:Vec<i64> = Sequence::Catalan.vec(size);
    }));
    
    c.bench_function("seq.lookandsay.u64", |b| b.iter(||
    {
        let v:Vec<u64> = Sequence::LookAndSay(1).vec(size);
    }));
}

criterion_group!
(
    name = benches;
    config = Criterion::default();
    targets = bench_seq
);

criterion_main!(benches);
