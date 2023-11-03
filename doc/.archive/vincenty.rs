use super::location::Location;

pub fn vincenty(c1:&Location, c2:&Location, p:f64) -> f64
{
    let u1 = ((1.0 - Location::F) * c1.latitude().to_radians().tan()).atan();
    let u2 = ((1.0 - Location::F) * c2.latitude().to_radians().tan()).atan();
    let sin_u1 = u1.sin();
    let cos_u1 = u1.cos();
    let sin_u2 = u2.sin();
    let cos_u2 = u2.cos();

    let l = (c1.longitude() - c2.longitude()).to_radians();
    let mut lambda = l;
    loop
    {
        let sin_lambda = lambda.sin();
        let cos_lambda = lambda.cos();
        let sin_sigma = ((cos_u2*sin_lambda).powi(2) + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda).powi(2)).sqrt();
        if sin_sigma == 0.0
        {
            return 0.0;
        }
        let cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
        let sigma = f64::atan2(sin_sigma, cos_sigma);

        let alpha = (cos_u1 * cos_u2 * sin_lambda / sin_sigma).asin();
        let cos_square_alpha = alpha.cos() * alpha.cos();
        let cos_sigma_m = cos_sigma - 2.0 * sin_u1 * sin_u2 / cos_square_alpha;
        let c = Location::F / 16.0 * cos_square_alpha * (4.0 + Location::F * (4.0 - 3.0 * cos_square_alpha));

        let last_lambda = lambda;
        lambda = l + ( 1.0 - c ) * Location::F * alpha.sin() * (sigma + c * (cos_sigma_m + c * cos_sigma * (-1.0 + 2.0 * cos_sigma_m * cos_sigma_m)));
        if (lambda - last_lambda).abs() <= p
        {
            let usq = cos_square_alpha * (Location::A * Location::A - Location::B * Location::B) / (Location::B * Location::B);
            let a = 1.0 + usq / 16384.0 * (4096.0 + usq * (-768.0 + usq * (320.0 - 175.0 * usq)));
            let b = usq / 1024.0 * (256.0 + usq * (-128.0 + usq * (74.0 - 47.0 * usq)));
            let delta_sigma = b * sin_sigma * (cos_sigma_m + b / 4.0 * (cos_sigma * (-1.0 + 2.0 * cos_sigma_m * cos_sigma_m) - b / 6.0 * cos_sigma_m * (-3.0 + 4.0 * sin_sigma * sin_sigma) * (-3.0 + 4.0 * cos_sigma_m * cos_sigma_m)));
            return Location::B * a * (sigma - delta_sigma);
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::vincenty;
    use super::super::location::Location;
    use float_cmp::assert_approx_eq;

    // Test vincenty distance calculation on f64 lat/lon coords
    #[test]
    fn test_vincenty()
    {
        assert_approx_eq!
        (
            f64,
            20003931.4586233,
            vincenty
            (
                &Location::new(90.0, 0.0),
                &Location::new(-90.0, 0.0),
                1e-6,
            )
        );
    }

    // Test vincenty distance calculation on f64 lat/lon coords
    #[test]
    fn test_vincenty_zero()
    {
        assert_approx_eq!
        (
            f64,
            0.0,
            vincenty
            (
                &Location::new(90.0, 0.0),
                &Location::new(90.0, 0.0),
                1e-6,
            )
        );
    }

    // Test vincenty distance calculation on f64 lat/lon coords
    #[test]
    fn test_vincenty_ny2la()
    {
        assert_approx_eq!
        (
            f64,
            3944412.145392327,
            vincenty
            (
                &Location::new(40.714268, -74.005974),      // New York
                &Location::new(34.0522, -118.2437),         // Los Angels 
                1e-6,
            )
        );
    }
}