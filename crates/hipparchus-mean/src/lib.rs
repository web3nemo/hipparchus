// re-exports

pub use self::value::*;

pub use self::sequence::arithmetic::*;
pub use self::sequence::geometric::*;
pub use self::sequence::fibonacci::*;
pub use self::sequence::padova::*;
pub use self::sequence::catalan::*;
pub use self::sequence::lookandsay::*;
pub use self::sequence::derive::*;

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

pub mod value;

pub mod sequence;
pub mod mean;
pub mod norm;
pub mod moving;
