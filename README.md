# hipparchus

[![crates.io](https://img.shields.io/crates/v/hipparchus-mean.svg)](https://crates.io/crates/hipparchus-mean)
[![docs.rs](https://docs.rs/hipparchus-mean/badge.svg)](https://docs.rs/hipparchus-mean/latest/hipparchus-mean/)
[![Downloads](https://img.shields.io/crates/d/hipparchus-mean.svg)](https://crates.io/crates/hipparchus-mean)
[![License: MIT OR Apache-2.0](https://img.shields.io/github/license/web3nemo/hipparchus.svg?style=flat-square)](#license)
[![Last Commit](https://img.shields.io/github/last-commit/web3nemo/hipparchus.svg?style=flat-square)](https://github.com/web3nemo/hipparchus)
[![CI](https://github.com/web3nemo/hipparchus/actions/workflows/ci.yml/badge.svg)](https://github.com/web3nemo/hipparchus/actions/workflows/ci.yml)
[![codecov.io](https://codecov.io/github/web3nemo/hipparchus/coverage.svg?branch=main)]
(https://codecov.io/gh/web3nemo/hipparchus?branch=master)

## What is ``hipparchus``? 

``hipparchus-mean`` is a rust library to implement various mathmatics OP, for example:
- create sequences
- lpnorm, mean & moving average
- distance metrics

| Repository | Crate | Documentation |
| ---------- | ----- | ------------- |
| ``hipparchus-mean`` | [![Crates.io](https://img.shields.io/crates/v/hipparchus-mean.svg)](https://crates.io/crates/hipparchus-mean) | [![Docs](https://docs.rs/hipparchus-mean/badge.svg)](https://docs.rs/hipparchus-mean/latest/hipparchus-mean/) |
| ``hipparchus-metrics`` | [![Crates.io](https://img.shields.io/crates/v/hipparchus-metrics.svg)](https://crates.io/crates/hipparchus-metrics) | [![Docs](https://docs.rs/hipparchus-metrics/badge.svg)](https://docs.rs/hipparchus-metrics/latest/hipparchus-metrics/) |

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

``hipparchus-mean`` is distributed under the terms of both the MIT license and the Apache License (Version 2.0). 

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.

## Thanks

TODO
