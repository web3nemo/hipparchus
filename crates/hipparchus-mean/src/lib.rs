//! # The Rust library for basic mathematics solver: Lp norm, mean, moving average...
//!
//! This library provides smart pointers and collections for managing
//! heap-allocated values.
//!
//! This library, like libcore, normally doesn’t need to be used directly
//! since its contents are re-exported in the [`std` crate](../std/index.html).
//! Crates that use the `#![no_std]` attribute however will typically
//! not depend on `std`, so they’d use this crate instead.
//!
//! ## Boxed values
//!
//! The [`Box`] type is a smart pointer type. There can only be one owner of a
//! [`Box`], and the owner can decide to mutate the contents, which live on the
//! heap.
//! 
// re-exports
pub use self::value::*;
pub use self::lpnorm::*;
pub use self::mean::*;
pub use self::movingavg::*;

// modules
pub mod value;
pub mod lpnorm;
pub mod mean;
pub mod movingavg;
pub mod sequence;
