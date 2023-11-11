pub use core::{DirectGeodesic, Geodesic, InverseGeodesic};
pub use polygon::{PolygonArea, Winding};

mod core;
mod caps;
mod line;
mod trig;
mod math;
mod polygon;
mod constants;
mod coeff;