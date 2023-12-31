//! [![github]](https://github.com/web3nemo/hipparchus)&ensp;
//! [![crates-io]](https://crates.io/crates/hipparchus-mean)&ensp;
//! [![docs-rs]](https://docs.rs/hipparchus-mean)
//! 
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! The crate provides various mathematics solver:
//! - factory to create various type of sequences with map/fold OPs for derived sequences
//! - utility to calculate various Lp norm for n-dimension vectors
//! - utility to calculate various mean & average moving for n-dimension vectors
//! 
//! # License
//! 
//! This project is licensed under either of
//! - [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](https://github.com/web3nemo/hipparchus/blob/HEAD/LICENSE-APACHE))
//! - [MIT License](https://opensource.org/license/mit/) ([LICENSE-MIT](https://github.com/rust-lang/libc/blob/HEAD/LICENSE-MIT))
//! at your option.
//! 
//! # Usage
//! 
//! Add the following to your '''Cargo.toml''':
//! 
//! ```toml
//! [dependencies]
//! hipparchus-mean = "0.1"
//! ```
//! 
//! # Example
//!
//! ## Lp Norm
//! 
//! Here's an example to calculate L1 norm of a vector via ``hipparchus``:
//! 
//! ```rust
//! 
//! use hipparchus_mean::LpNorm;
//! 
//! let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let norm = v.iter().l0norm().unwrap();
//! 
//! ``` 
//! 
//! Below is a full list of all norm algorithms ``hipperchus`` has supported:
//! 
//! | Norm | Syntax | Feature |
//! | :-- | :-- | :-- |
//! | l0norm | - | L0 norm |
//! | l1norm | - | L1 norm |
//! | l2norm | - | L2 norm |
//! | lpnorm | (f32) | Lp norm with p factor |
//! | lpnorm_inf | - | Lp norm (p=inf) |
//! 
//! ## Mean & Moving Average 
//! 
//! Here's an example to calculate the arithmetic mean of a vector via ``hipparchus``:
//! 
//! ```rust
//! 
//! use hipparchus_mean::Mean;
//! 
//! let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let mean = v.iter().arithmetic_mean().unwrap();
//! 
//! ```
//! 
//! Below is a full list of all mean algorithm ``hipperchus`` has supported:
//! 
//! | Mean | Syntax | Feature |
//! | :-- | :-- | :-- |
//! | ArithmeticMean | - | arithmetic mean |
//! | GeometricMean | - | geometric mean |
//! | QuadraticMean | - | quadratic mean |
//! | HarmonicMean | - | harmonic mean |
//! | SimpleMovingAverage | - | simple moving average |
//! | CumulativeMovingAverage | - | cumulative moving average |
//! | WeightedMovingAverage | - | weighted moving average |
//! | ExponentialMovingAverage | (f32) | exponential moving average with decay |
//! 
//! # Contributing
//! 
//! We welcome all people who want to contribute. Please see the contributing instructions for more information.
//! 
//! Contributions in any form (issues, pull requests, etc.) to this project must adhere to Rust's Code of Conduct.
//! 
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in hipparchus-* by you, 
//! as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//! 
//! 

// re-exports
pub use self::value::*;
pub use self::two::*;
pub use self::power::*;
pub use self::lpnorm::*;
pub use self::mean::*;
pub use self::movingavg::*;

// modules
pub mod value;
pub mod two;
pub mod power;
pub mod lpnorm;
pub mod mean;
pub mod movingavg;
