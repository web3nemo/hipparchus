// re-exports
pub use self::geo::location::*;
pub use self::geo::haversine::*;
pub use self::geo::vincenty::*;
pub use self::point::manhattan::*;
pub use self::point::euclidean::*;
pub use self::point::minkowski::*;
pub use self::point::chebyshev::*;
pub use self::point::gower::*;
pub use self::point::canberra::*;

// modules
pub mod point;
pub mod geo;
pub mod vector;
pub mod text;
pub mod distribution;
pub mod sampling;
pub mod set;
