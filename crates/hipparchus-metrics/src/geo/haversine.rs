use super::location::Location;

pub fn haversine(l1:&Location, l2:&Location, r:f64) -> f64
{
    r * haversine_radians
    (
        l1.latitude().to_radians(), l1.longitude().to_radians(),
        l2.latitude().to_radians(), l2.longitude().to_radians()
    )
}

fn haversine_radians(lat1:f64, lon1:f64, lat2:f64, lon2:f64) -> f64
{
    let hdx = (lon2 - lon1) / 2.0;
    let hdy = (lat2 - lat1) / 2.0;
    (hdy.sin().powi(2) + hdx.sin().powi(2)*lat2.cos()*lat1.cos()).sqrt().asin() * 2.0
}

#[cfg(test)]
mod tests 
{
    use super::haversine;
    use super::super::location::Location;
    use float_cmp::assert_approx_eq;

    // Test haversine distance calculation on f64 lat/lon coords
    #[test]
    fn test_haversine()
    {
        assert_approx_eq!
        (
            f64,
            std::f64::consts::PI * Location::R,
            haversine
            (
                &Location::new(90.0, 0.0),
                &Location::new(-90.0, 0.0),
                Location::R
            )
        );
    }

    // Test haversine distance calculation on f64 lat/lon coords
    #[test]
    fn test_haversine_zero()
    {
        assert_approx_eq!
        (
            f64,
            0.0,
            haversine
            (
                &Location::new(90.0, 0.0),
                &Location::new(90.0, 0.0),
                Location::R
            )
        );
    }
}