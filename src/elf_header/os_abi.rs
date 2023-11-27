use crate::prelude::AsBytes;

pub enum OsAbi {
    SystemV,
    HpUx,
    NetBsd,
    Linux,
    GnuHurd,
    Solaris,
    Aix,
    Irix,
    FreeBsd,
    Tru64,
    NovellModesto,
    OpenBsd,
    OpenVms,
    NonStopKernel,
    Aros,
    FenixOs,
    NuxiCloudAbi,
    StratusTechnologiesOpenVos,
}

impl AsBytes for OsAbi {
    fn as_bytes(&self) -> Vec<u8> {
        vec![match self {
            OsAbi::SystemV => 0,
            OsAbi::HpUx => 1,
            OsAbi::NetBsd => 2,
            OsAbi::Linux => 3,
            OsAbi::GnuHurd => 4,
            OsAbi::Solaris => 6,
            OsAbi::Aix => 7,
            OsAbi::Irix => 8,
            OsAbi::FreeBsd => 9,
            OsAbi::Tru64 => 0x0A,
            OsAbi::NovellModesto => 0x0B,
            OsAbi::OpenBsd => 0x0C,
            OsAbi::OpenVms => 0x0D,
            OsAbi::NonStopKernel => 0x0E,
            OsAbi::Aros => 0x0F,
            OsAbi::FenixOs => 0x10,
            OsAbi::NuxiCloudAbi => 0x11,
            OsAbi::StratusTechnologiesOpenVos => 0x12,
        }]
    }
}
