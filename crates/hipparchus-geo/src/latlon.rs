/// Latitude and longitude in degrees.

pub struct LatLon(f64, f64);

impl LatLon
{
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
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_latlon()
    {
        let latlon = LatLon(39.908823,116.397470);
        assert_approx_eq!(f64, 39.908823, latlon.latitude());
        assert_approx_eq!(f64, 116.397470, latlon.longitude());
    }

    #[test]
    fn test_latlon_value()
    {
        let latlon = LatLon(39.908823,116.397470);
        assert_approx_eq!(f64, 39.908823, latlon.latitude());
        assert_approx_eq!(f64, 116.397470, latlon.longitude());
    }
}    
