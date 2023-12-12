//! [![github]](https://github.com/web3nemo/hipparchus)&ensp;
//! [![crates-io]](https://crates.io/crates/hipparchus-wrap)&ensp;
//! [![docs-rs]](https://docs.rs/hipparchus-wrap)
//! 
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! The crate provides macros to create wrapped type and implement operation and traits for it.
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
//! hipparchus-wrap = "0.1"
//! ```
//! 
//! # Examples
//! 
//! ``` rs
//! // TODO
//! ````
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
pub use self::nt::*;
pub use self::op_unary::*;
pub use self::op_binary::*;
pub use self::arithmatic::*;
pub use self::bitwise::*;

// modules
pub mod nt;
pub mod convert;
pub mod op_unary;
pub mod op_binary;
pub mod arithmatic;
pub mod bitwise;

// TODO: Convert
//  - TryFrom, TryInto
//  - Primitive shortcut of From/Into, TryFrom/TryInto

// TODO: From Str Debug, Display
// TODO: Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp, Pointer
// TODO: AsRef, AsMut, Borrow, BorrowMut, Deref, DerefMut
// TODO: Into Iterator, index, index_mut
// TODO: Error
// TODO: PartialEq, Eq, PartialOrd, Ord, Hash
// TODO: Default
// TODO: Clone, Copy
// TODO: Serialize, Deserialize
