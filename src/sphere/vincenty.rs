use super::geo::Location;

pub struct VincentyParameters(u32, f64, i32);

impl VincentyParameters
{
    pub fn new<I:Into<u32>, T:Into<f64>, P:Into<i32>>(iterations:I, convergence:T, precision:P) -> Self
    {
        VincentyParameters(iterations.into(), convergence.into(), precision.into())
    }

    // Create a new const Location with its degree values of latitude and longitude.
    pub const fn new_const(iterations:u32, convergence:f64, precision:i32) -> Self
    {
        VincentyParameters(iterations, convergence, precision)
    }

    // Get the maximum iterations
    pub fn iterations(&self) -> u32
    {
        self.0
    }

    // Get the convergence threshold
    pub fn convergence(&self) -> f64
    {
        self.1
    }

    // Get the precision
    pub fn precision(&self) -> i32
    {
        self.2
    }
}

pub fn vincenty(c1:&Location, c2:&Location, settings:&VincentyParameters) -> f64
{
    let u1 = ((1.0 - Location::F) * c1.latitude().to_radians().tan()).atan();
    let u2 = ((1.0 - Location::F) * c2.latitude().to_radians().tan()).atan();
    let sin_u1 = u1.sin();
    let cos_u1 = u1.cos();
    let sin_u2 = u2.sin();
    let cos_u2 = u2.cos();

    let l = (c1.longitude() - c2.longitude()).to_radians();
    let mut lambda = l;
    let mut i = settings.iterations();
    let p = 10f64.powi(-settings.precision());
    while i>0
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

        i -= 1;
    }

    panic!("Reached maximum iterations but still did not meet required precision and convergence threshold.")
}

#[cfg(test)]
mod tests 
{
    use crate::sphere::vincenty::VincentyParameters;

    use super::vincenty;
    use super::super::geo::Location;
    use float_cmp::assert_approx_eq;

    // Test vincenty distance calculation on f64 lat/lon coords
    #[test]
    fn test_vincenty()
    {
        assert_approx_eq!
        (
            f64,
            // std::f64::consts::PI * Location::R,
            20003931.4586233,
            vincenty
            (
                &Location::new(90.0, 0.0),
                &Location::new(-90.0, 0.0),
                &VincentyParameters::new(200u32, 0.000_000_000_001f64, 6)
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
                &VincentyParameters::new(200u32, 0.000_000_000_001f64, 6)
            )
        );
    }
}