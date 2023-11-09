use std::ops::BitAnd;

use bitflags::bitflags;

bitflags!
{
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Caps: u64
    {
        // Capabilities
        const CAP_C1 = 0x0001;
        const CAP_C1p = 0x0002;
        const CAP_C2 = 0x0004;
        const CAP_C3 = 0x0008;
        const CAP_C4 = 0x0010;
        const CAP_ALL = Self::CAP_C1.bits() | Self::CAP_C1p.bits() | Self::CAP_C2.bits() | Self::CAP_C3.bits() | Self::CAP_C4.bits();

        // Output
        const LATITUDE = 0x0080;
        const LONGITUDE = 0x0100 | Self::CAP_C3.bits();
        const AZIMUTH = 0x0200;
        const DISTANCE = 0x0400 | Self::CAP_C1.bits();
        const STANDARD = Self::LATITUDE.bits() | Self::LONGITUDE.bits() | Self::AZIMUTH.bits() | Self::DISTANCE.bits();
        const DISTANCE_IN = 0x0800 | Self::CAP_C1.bits() | Self::CAP_C1p.bits();
        const REDUCEDLENGTH = 0x1000 | Self::CAP_C1.bits() | Self::CAP_C2.bits();
        const GEODESICSCALE = 0x2000 | Self::CAP_C1.bits() | Self::CAP_C2.bits();
        const AREA = 0x4000 | Self::CAP_C4.bits();
        const OUT_ALL = 0x7F80;
        const LONG_UNROLL = 0x8000;
        const POLYGONAREA = Self::LATITUDE.bits() | Self::LONGITUDE.bits() | Self::DISTANCE.bits() | Self::AREA.bits() | Self::LONG_UNROLL.bits();
                
        const ALL = Self::OUT_ALL.bits() | Self::CAP_ALL.bits();
    }
}

bitflags!
{
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Mask: u64
    {
        const CAP = Caps::CAP_ALL.bits();
        const OUT = Caps::OUT_ALL.bits() | Caps::LONG_UNROLL.bits();
    }
}

impl BitAnd<Mask> for Caps
{
    type Output = Self;

    fn bitand(self, rhs: Mask) -> Self
    {
        Self::from_bits_truncate(self.bits() & rhs.bits())
    }
}
