/// Obliguity of the ecliptic, the angle between the ecliptic and the celestial equator: 23.4392811° (or 23°26'21.412"), published by IERS-2022
pub const OBLIGUITY:f64 = 23.439_281_1;

/// ClimateZone is a region of the earth defined by its climate.
#[derive(Debug, PartialEq)]
pub enum ClimateZone
{
    /// North Frigid Zone, 66°33′38.588″N to 90°N
    NorthFrigidZone = 2,

    /// North Temperate Zone, 23°26′21.412″N to 66°33′38.588″N
    NorthTemperateZone = 1,

    /// Tropics, 23°26′21.412″S to 23°26′21.412″N
    Tropics = 0,

    /// South Temperate Zone, 23°26′21.412″S to 66°33′38.588″S
    SouthTemperateZone = -1,

    /// South Frigid Zone, 66°33′38.588″S to 90°S
    SouthFrigidZone = -2,
}

/// Parallel is a line of latitude.
#[derive(Debug)]
pub enum Parallel
{
    /// North Pole, 90°N
    NorthPole = 3,

    /// Arctic Circle, 66°33′N
    ArcticCircle = 2,

    /// Tropic of Cancer, 23°26′11.6″N
    TropicOfCancer = 1,

    /// Equator, 0°
    Equator = 0,

    /// Tropic of Capricorn, 23°26′11.6″S
    TropicOfCapricorn = -1,

    /// Antarctic Circle, 66°33′46.4″S
    AntarcticCircle = -2,

    /// South Pole, 90°S
    SouthPole = -3,
}

impl Parallel
{
    /// Get the latitude of the parallel.
    pub fn angle(self) -> f64
    {
        match self
        {
            Self::NorthPole => 90.0,
            Self::ArcticCircle => 90.0 - OBLIGUITY,
            Self::TropicOfCancer => OBLIGUITY,
            Self::Equator => 0.0,
            Self::TropicOfCapricorn => -OBLIGUITY,
            Self::AntarcticCircle => OBLIGUITY - 90.0,
            Self::SouthPole => -90.0,
        }
    }

    /// Get the climate zone of the parallel.
    pub fn zone(lat: f64) -> ClimateZone
    {
        match lat
        {
            lat if lat > Self::ArcticCircle.angle() && lat <= Self::NorthPole.angle() => ClimateZone::NorthFrigidZone,
            lat if lat > Self::TropicOfCancer.angle() && lat <= Self::ArcticCircle.angle() => ClimateZone::NorthTemperateZone,
            lat if lat >= Self::TropicOfCapricorn.angle() && lat <= Self::TropicOfCancer.angle() => ClimateZone::Tropics,
            lat if lat >= Self::AntarcticCircle.angle() && lat < Self::TropicOfCapricorn.angle() => ClimateZone::SouthTemperateZone,
            lat if lat >= Self::SouthPole.angle() && lat < Self::AntarcticCircle.angle() => ClimateZone::SouthFrigidZone,
            _ => panic!("Invalid latitude value"),
        }
    }
}

/// Meridian is a line of longitude.
#[derive(Debug)]
pub enum Meridian
{
    /// Greenwich (the prime meridian),  0°
    Greenwich = 0,

    /// International Date Line (the antimeridian), 180°
    InternationalDateLine = -180,
}

/// Hemisphere is a half of the earth.
#[derive(Debug, PartialEq)]
pub enum Hemisphere
{
    /// Eastern hemisphere, 0° to 180°
    Eastern = 1,

    /// Western hemisphere, 0° to -180°
    Western = -1,
}

impl Meridian
{
    /// Get the longitude of the meridian.
    pub fn angle(self) -> f64
    {
        self as i16 as f64
    }

    /// Get the hemisphere of the meridian.
    pub fn zone(lon: f64) -> Hemisphere
    {
        match lon
        {
            lon if lon > 160.0 && lon < 180.0 => Hemisphere::Western,
            lon if lon > -20.0 && lon <= 160.0 => Hemisphere::Eastern,
            lon if lon >= -180.0 && lon < -20.0 => Hemisphere::Western,
            _ => panic!("Invalid longitude value"),
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(Parallel::NorthPole, 90.0)]
    #[case(Parallel::ArcticCircle, 66.5)]
    #[case(Parallel::TropicOfCancer, 23.5)]
    #[case(Parallel::Equator, 0.0)]
    #[case(Parallel::TropicOfCapricorn, -23.5)]
    #[case(Parallel::AntarcticCircle, -66.5)]
    #[case(Parallel::SouthPole, -90.0)]
    fn test_parallel_angle(#[case] p: Parallel, #[case] lat: f64)
    {
        assert_approx_eq!(f64, lat, p.angle(), epsilon=0.1);
    }

    #[rstest]
    #[case(80.0, ClimateZone::NorthFrigidZone)]
    #[case(60.0, ClimateZone::NorthTemperateZone)]
    #[case(20.0, ClimateZone::Tropics)]
    #[case(-20.0, ClimateZone::Tropics)]
    #[case(-60.0, ClimateZone::SouthTemperateZone)]
    #[case(-80.0, ClimateZone::SouthFrigidZone)]
    fn test_parallel_zone(#[case] lat: f64, #[case] zone: ClimateZone)
    {
        assert_eq!(zone, Parallel::zone(lat));
    }

    #[rstest]
    #[case(Meridian::Greenwich, 0.0)]
    #[case(Meridian::InternationalDateLine, -180.0)]
    fn test_meridian_angle(#[case] m: Meridian, #[case] lon: f64)
    {
        assert_approx_eq!(f64, lon, m.angle(), epsilon=0.1);
    }

    #[rstest]
    #[case(170.0, Hemisphere::Western)]
    #[case(150.0, Hemisphere::Eastern)]
    #[case(30.0, Hemisphere::Eastern)]
    #[case(10.0, Hemisphere::Eastern)]
    #[case(0.0, Hemisphere::Eastern)]
    #[case(-10.0, Hemisphere::Eastern)]
    #[case(-30.0, Hemisphere::Western)]
    #[case(-150.0, Hemisphere::Western)]
    #[case(-170.0, Hemisphere::Western)]
    fn test_meridian_hemisphere(#[case] lon: f64, #[case] hemisphere: Hemisphere)
    {
        assert_eq!(hemisphere, Meridian::zone(lon));
    }
}
