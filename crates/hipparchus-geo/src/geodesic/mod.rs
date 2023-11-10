pub use core::{DirectGeodesic, Geodesic, InverseGeodesic};
pub use polygon::{PolygonArea, Winding};

mod core;
pub mod caps;
mod line;
mod math;
mod polygon;
mod constants;
mod coeff;