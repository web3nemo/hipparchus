//! # The Rust library for basic mathematics solver: Lp norm, mean, moving average...

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
