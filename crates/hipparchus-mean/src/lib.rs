// re-exports
pub use self::mean::arithmetic::*;
pub use self::mean::geometric::*;
pub use self::mean::harmonic::*;
pub use self::mean::quadratic::*;
pub use self::mean::traits::*;
pub use self::norm::l0norm::*;
pub use self::norm::l1norm::*;
pub use self::norm::l2norm::*;
pub use self::norm::lpnorm::*;
pub use self::norm::lpnorm_inf::*;
pub use self::norm::traits::*;

// modules
pub mod mean;
pub mod norm;
pub mod movingavg;
