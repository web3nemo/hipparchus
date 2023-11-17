//! [![github]](https://github.com/web3nemo/hipparchus)&ensp;
//! [![crates-io]](https://crates.io/crates/hipparchus-az)&ensp;
//! [![docs-rs]](https://docs.rs/hipparchus-az)
//! 
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! The crate provides various utility for angle representation and conversion:
//! - DMS representation of an angle
//! - Azimuth (slope of y/x) representation of an angle
//! - Modulo-based angle normalization
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
//! hipparchus-az = "0.1"
//! ```
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
pub use self::modulo::*;
pub use self::degrees::*;
pub use self::radians::*;
pub use self::sign::*;
pub use self::unit::*;
pub use self::dms::*;
pub use self::azimuth::*;

// modules
pub mod modulo;
pub mod degrees;
pub mod radians;
pub mod sign;
pub mod unit;
pub mod dms;
pub mod azimuth;
pub mod azimuth_norm;
pub mod azimuth_ops;
