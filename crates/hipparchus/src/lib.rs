//! The crate provides a collection of utilities via re-exports on various `hipparchus-*` crates.
//! 
//! | Repository | Features | Crate | Documentation |
//! | :-- | :-- | :-- | :-- |
//! | ``hipparchus`` | re-exports| [![Crates.io](https://img.shields.io/crates/v/hipparchus.svg)](https://crates.io/crates/hipparchus) | [![Docs](https://docs.rs/hipparchus/badge.svg)](https://docs.rs/hipparchus-mean/latest/hipparchus/) |
//! | ``hipparchus-az`` | angle calculation and conversion | [![Crates.io](https://img.shields.io/crates/v/hipparchus-az.svg)](https://crates.io/crates/hipparchus-seq) | [![Docs](https://docs.rs/hipparchus-az/badge.svg)](https://docs.rs/hipparchus-mean/latest/hipparchus-az/) |
//! | ``hipparchus-geo`` | geospatial calculation | [![Crates.io](https://img.shields.io/crates/v/hipparchus-geo.svg)](https://crates.io/crates/hipparchus-geo) | [![Docs](https://docs.rs/hipparchus-geo/badge.svg)](https://docs.rs/hipparchus-geo/latest/hipparchus-geo/) |
//! | ``hipparchus-mean`` | mean & moving average | [![Crates.io](https://img.shields.io/crates/v/hipparchus-mean.svg)](https://crates.io/crates/hipparchus-mean) | [![Docs](https://docs.rs/hipparchus-mean/badge.svg)](https://docs.rs/hipparchus-mean/latest/hipparchus-mean/) |
//! | ``hipparchus-metrics`` | various metrics calculation | [![Crates.io](https://img.shields.io/crates/v/hipparchus-metrics.svg)](https://crates.io/crates/hipparchus-metrics) | [![Docs](https://docs.rs/hipparchus-metrics/badge.svg)](https://docs.rs/hipparchus-metrics/latest/hipparchus-metrics/) |
//! | ``hipparchus-seq`` | number sequence factory | [![Crates.io](https://img.shields.io/crates/v/hipparchus-seq.svg)](https://crates.io/crates/hipparchus-seq) | [![Docs](https://docs.rs/hipparchus-seq/badge.svg)](https://docs.rs/hipparchus-mean/latest/hipparchus-seq/) |
//! 
//! # License
//! 
//! All `hipparchus-*` crates are licensed under either of
//! - [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](https://github.com/web3nemo/hipparchus/blob/HEAD/LICENSE-APACHE))
//! - [MIT License](https://opensource.org/license/mit/) ([LICENSE-MIT](https://github.com/rust-lang/libc/blob/HEAD/LICENSE-MIT))
//! at your option.
//! 

// re-exports
pub use hipparchus_az::*;
pub use hipparchus_geo::*;
pub use hipparchus_mean::*;
pub use hipparchus_metrics::*;
pub use hipparchus_seq::*;
