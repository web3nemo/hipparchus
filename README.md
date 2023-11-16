# hipparchus

[![crates.io](https://img.shields.io/crates/v/hipparchus-mean.svg)](https://crates.io/crates/hipparchus-mean)
[![docs.rs](https://docs.rs/hipparchus-mean/badge.svg)](https://docs.rs/hipparchus-mean/latest/hipparchus-mean/)
[![Downloads](https://img.shields.io/crates/d/hipparchus-mean.svg)](https://crates.io/crates/hipparchus-mean)
[![License: MIT OR Apache-2.0](https://img.shields.io/github/license/web3nemo/hipparchus.svg?style=flat-square)](#license)
[![Last Commit](https://img.shields.io/github/last-commit/web3nemo/hipparchus.svg?style=flat-square)](https://github.com/web3nemo/hipparchus)
[![CI](https://github.com/web3nemo/hipparchus/actions/workflows/ci.yml/badge.svg)](https://github.com/web3nemo/hipparchus/actions/workflows/ci.yml)
[![codecov.io](https://codecov.io/github/web3nemo/hipparchus/coverage.svg)](https://codecov.io/gh/web3nemo/hipparchus)

## What is ``hipparchus``? 

``hipparchus`` is rust library to implement various mathmatics computing, for example:
- create sequences, lpnorm, mean & moving average
- geographics orientation and distance
- distance & metrics

| Repository | Features | Crate | Documentation |
| :-- | :-- | :-- | :-- |
| ``hipparchus`` | re-exports| [![Crates.io](https://img.shields.io/crates/v/hipparchus.svg)](https://crates.io/crates/hipparchus) | [![Docs](https://docs.rs/hipparchus/badge.svg)](https://docs.rs/hipparchus-mean/latest/hipparchus/) |
| ``hipparchus-az`` | angle calculation and conversion | [![Crates.io](https://img.shields.io/crates/v/hipparchus-az.svg)](https://crates.io/crates/hipparchus-seq) | [![Docs](https://docs.rs/hipparchus-az/badge.svg)](https://docs.rs/hipparchus-mean/latest/hipparchus-az/) |
| ``hipparchus-geo`` | geospatial calculation | [![Crates.io](https://img.shields.io/crates/v/hipparchus-geo.svg)](https://crates.io/crates/hipparchus-geo) | [![Docs](https://docs.rs/hipparchus-geo/badge.svg)](https://docs.rs/hipparchus-geo/latest/hipparchus-geo/) |
| ``hipparchus-mean`` | mean & moving average | [![Crates.io](https://img.shields.io/crates/v/hipparchus-mean.svg)](https://crates.io/crates/hipparchus-mean) | [![Docs](https://docs.rs/hipparchus-mean/badge.svg)](https://docs.rs/hipparchus-mean/latest/hipparchus-mean/) |
| ``hipparchus-metrics`` | various metrics calculation | [![Crates.io](https://img.shields.io/crates/v/hipparchus-metrics.svg)](https://crates.io/crates/hipparchus-metrics) | [![Docs](https://docs.rs/hipparchus-metrics/badge.svg)](https://docs.rs/hipparchus-metrics/latest/hipparchus-metrics/) |
| ``hipparchus-seq`` | number sequence factory | [![Crates.io](https://img.shields.io/crates/v/hipparchus-seq.svg)](https://crates.io/crates/hipparchus-seq) | [![Docs](https://docs.rs/hipparchus-seq/badge.svg)](https://docs.rs/hipparchus-mean/latest/hipparchus-seq/) |

> [!WARNING] 
> ``hipparchus`` is still in the early stages of development: v0.x as prototyping version which might mean:
>   - Documentation is sparse.
>   - New version of ``hipparchus`` containing breaking changes and we can't guarantee migrations will always be easy.
>   - Important features are missing.
>   - More and more unit tests but still may not cover 100% at your usage. 
> 
> Use only if you are willing to work in this environment.

## Design Goals

TODO

## About

- TODO: Features
- TODO: News
- [Release Note](./RELEASE.md)
- [License](#license)
- [Enlistment](./ENLISTMENT.md)

## Usage

Add the following to your '''Cargo.toml''':

```toml

[dependencies]
hipparchus-mean = "0.1"

```

Here's an example to create the arithmetic sequence via hipparchus-mean:

```rust

use hipparchus_mean::Mean;

fn main()
{
    // define the input vector
    let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    
    // call arithmetic_mean to get the mean value
    let mean = v.iter().arithmetic_mean().unwrap();
    
    // print the mean value
    println!("mean = {:?}", mean);
}

```

## License

``hipparchus`` is distributed under the terms of both the MIT license and the Apache License (Version 2.0). 

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.

## Thanks

TODO
