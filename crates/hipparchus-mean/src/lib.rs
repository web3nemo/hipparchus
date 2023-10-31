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
//! ## Create A Sequence
//! 
//! Here's an example for how to create the arithmetic sequence via hipparchus-mean:
//! 
//! ```rust
//! 
//! use hipparchus_mean::Sequence;
//! 
//! let v = Sequence::Arithmetic { init: 1, difference: 1 }.vec(5);
//! 
//! ```
//! 
//! Below is a full list of all sequences hipperchus has supported:
//! 
//! | Sequence | Syntax | Feature |
//! | :-- | :-- | :-- |
//! | Arithmetic | { init:T, difference:T } | arithmetic sequence with init value and difference |
//! | Geometric | { init:T, ratio:T } | geometric sequence with init value and ratio |
//! | Natural | (bool) | natural sequence starting with 0/1 |
//! | Odd | - | odd sequence starting with 1 |
//! | Even | (bool) | even sequence starting with 0/1 |
//! | Power | (T) | power sequence starting with 1 with radix |
//! | Triangular | - | triangular sequence starting with 1 |
//! | Square | - | square sequence starting with 1 |
//! | Cubic | - | cubic sequence starting with 1 |
//! | Harmonic | { init:T, difference:T } | harmonic sequence with init value and difference |
//! | Fibonacci | - | fibonacci sequence starting with 0, 1 |
//! | Lucas | - | lucas sequence starting with 2, 1 |
//! | Padova | - | padova sequence |
//! | Catalan | - | catalan sequence |
//! | LookAndSay | (usize) | look and say sequence starting with a usize value | 
//! 
//! And hipparchus-mean support recursive, map and fold OPs to generate complicated or derived sequences. 
//! Please refer to codes written in unit tests of sequence module.
//! 
//! ## Lp Norm
//! 
//! Here's an example for how to integrate the L1 norm with hipparchus-mean:
//! 
//! ```rust
//! 
//! use hipparchus_mean::LpNorm;
//! 
//! let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let norm = v.iter().l0norm.unwrap();
//! 
//! ``` 
//! 
//! Below is a full list of all norm algorithms hipperchus has supported:
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
//! Here's an example for how to integrate the arithmetic mean with hipparchus-mean:
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
//! Below is a full list of all mean algorithm hipperchus supported now:
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
pub use self::lpnorm::*;
pub use self::mean::*;
pub use self::movingavg::*;
pub use self::sequence::*;

// modules
pub mod value;
pub mod lpnorm;
pub mod mean;
pub mod movingavg;
pub mod sequence;
