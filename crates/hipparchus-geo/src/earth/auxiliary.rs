use num_enum::{IntoPrimitive, TryFromPrimitive};

// See in  https://arxiv.org/pdf/2212.05818.pdf
// See in https://github.com/geographiclib/geographiclib
#[repr(i8)]
#[derive(Debug, PartialEq, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
pub enum AuxiliaryLatitude
{
    Geographic = 0,
    Parametric = 1,
    Geocentric = 2,
    Rectifying = 3,
    Conformal = 4,
    Authalic = 5,
}

impl AuxiliaryLatitude
{
    // Alias to enum values
    pub const PHI: Self = Self::Geographic;
    pub const BETA: Self = Self::Parametric;
    pub const THETA: Self = Self::Geocentric;
    pub const MU: Self = Self::Rectifying;
    pub const CHI: Self = Self::Conformal;
    pub const XI: Self = Self::Authalic;
    pub const COMMON: Self = Self::Geographic;
    pub const GEODESIC: Self = Self::Geographic;
    pub const REDUCED: Self = Self::Parametric;

    
}
