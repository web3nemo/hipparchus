pub struct Location(f64, f64);

impl Location
{
    // sphere constants of earth
    pub const R:f64 = 6371393f64;

    // WGS84, ellipsoids constants of earth
    pub const A:f64 = 6378137.0f64;
    pub const B:f64 = 6356752.314245f64;
    pub const F:f64 = 1.0f64 / 298.257223563f64;

    // Create a new Location with its degree values of latitude and longitude.
    pub fn new<T:Into<f64>>(lat:T, lon:T) -> Self
    {
        Location(lat.into(), lon.into())
    }

    // Create a new const Location with its degree values of latitude and longitude.
    pub const fn new_const(lat:f64, lon:f64) -> Self
    {
        Location(lat, lon)
    }

    // Get the latitude.
    pub fn latitude(&self) -> f64
    {
        self.0
    }

    // Get the longitude.
    pub fn longitude(&self) -> f64
    {
        self.1
    }
}

#[cfg(test)]
mod tests 
{
    use super::Location;
    use float_cmp::assert_approx_eq;

    // Test to create new instance of Geo-location
    #[test]
    fn test_location_new()
    {
        let l = Location::new(39.908823,116.397470);
        assert_approx_eq!(f64, 39.908823, l.latitude());
        assert_approx_eq!(f64, 116.397470, l.longitude());
    }

    // Test to create new const of Geo-location
    #[test]
    fn test_location_newconst()
    {
        const L:Location = Location::new_const(39.908823,116.397470);
        assert_approx_eq!(f64, 39.908823, L.latitude());
        assert_approx_eq!(f64, 116.397470, L.longitude());
    }
}
