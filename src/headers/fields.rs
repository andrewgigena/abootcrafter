use binrw::{BinRead, BinWrite};
use std::fmt;

#[derive(Debug, Default, BinRead, BinWrite)]
pub struct Magic(#[br(count = 8)] pub Vec<u8>);

impl fmt::Display for Magic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

#[derive(Debug, Default, BinRead, BinWrite)]
pub struct Name(#[br(count = 16)] pub Vec<u8>);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

#[derive(Debug, Default, BinRead, BinWrite)]
pub struct Cmdline(#[br(count = 512)] pub Vec<u8>);

impl fmt::Display for Cmdline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

#[derive(Debug, Default, BinRead, BinWrite)]
pub struct Id(#[br(count = 8)] pub Vec<u8>);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

#[derive(Debug, Default, BinRead, BinWrite)]
pub struct AddressU32(#[br(count = 4)] pub Vec<u8>);

impl fmt::Display for AddressU32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert 4 bytes to u32, accounting for little-endian
        let addr = u32::from_le_bytes(self.0.clone().try_into().unwrap());
        write!(f, "0x{:08x}", addr)
    }
}

#[derive(Debug, Default, BinRead, BinWrite)]
pub struct AddressU64(#[br(count = 8)] pub Vec<u8>);

impl fmt::Display for AddressU64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert 8 bytes to u64, accounting for little-endian
        let addr = u64::from_le_bytes(self.0.clone().try_into().unwrap());
        write!(f, "0x{:016x}", addr)
    }
}

#[derive(Debug, Default, BinRead, BinWrite)]
pub struct OSVersion(pub u32);

impl OSVersion {
    fn major(&self) -> u32 {
        self.0 >> 25
    }

    fn minor(&self) -> u32 {
        (self.0 >> 18) & 0x7F
    }

    fn patch(&self) -> u32 {
        (self.0 >> 11) & 0x7F
    }

    fn year(&self) -> u32 {
        ((self.0 >> 4) & 0xF) + 2000
    }

    fn month(&self) -> u32 {
        self.0 & 0xF
    }
}

impl fmt::Display for OSVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{} ({:04}-{:02})",
            self.major(),
            self.minor(),
            self.patch(),
            self.year(),
            self.month()
        )
    }
}
