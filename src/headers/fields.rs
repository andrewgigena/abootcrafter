use binrw::{BinRead, BinWrite};
use std::fmt;

#[derive(Debug, BinRead, BinWrite, Clone)]
#[br()]
pub struct AndroidBootMagic(#[br(count = 8)] pub Vec<u8>);

impl fmt::Display for AndroidBootMagic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ascii_str: String = self
            .0
            .iter()
            .filter(|&&c| c.is_ascii())
            .map(|&c| c as char)
            .collect();
        write!(f, "{}", ascii_str)
    }
}

impl Default for AndroidBootMagic {
    fn default() -> Self {
        AndroidBootMagic(b"ANDROID!".to_vec())
    }
}

#[derive(Debug, Default, BinRead, BinWrite, Clone)]
pub struct Name(#[br(count = 16)] pub Vec<u8>);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ascii_str: String = self
            .0
            .iter()
            .filter(|&&c| c.is_ascii())
            .map(|&c| c as char)
            .collect();
        write!(f, "{}", ascii_str)
    }
}

impl From<String> for Name {
    fn from(s: String) -> Self {
        let mut vec = s.into_bytes();
        vec.resize(16, 0); // Ensure exactly 16 bytes
        Name(vec)
    }
}

#[derive(Debug, Default, BinRead, BinWrite, Clone)]
pub struct Cmdline(#[br(count = 512)] pub Vec<u8>);

impl fmt::Display for Cmdline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ascii_str: String = self
            .0
            .iter()
            .filter(|&&c| c.is_ascii())
            .map(|&c| c as char)
            .collect();
        write!(f, "{}", ascii_str)
    }
}

impl From<String> for Cmdline {
    fn from(s: String) -> Self {
        let mut vec = Vec::with_capacity(512);
        vec.extend(s.as_bytes().iter().take(512));
        // Pad with zeros if string is shorter than 512 bytes
        vec.resize(512, 0);
        Cmdline(vec)
    }
}

#[derive(Debug, Default, BinRead, BinWrite, Clone)]
pub struct ExtraCmdline(#[br(count = 1024)] pub Vec<u8>);

impl fmt::Display for ExtraCmdline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ascii_str: String = self
            .0
            .iter()
            .filter(|&&c| c.is_ascii())
            .map(|&c| c as char)
            .collect();
        write!(f, "{}", ascii_str)
    }
}

impl From<String> for ExtraCmdline {
    fn from(s: String) -> Self {
        let mut vec = Vec::with_capacity(1024);
        vec.extend(s.as_bytes().iter().take(1024));
        // Pad with zeros if string is shorter than 1024 bytes
        vec.resize(1024, 0);
        ExtraCmdline(vec)
    }
}

#[derive(Debug, Default, BinRead, BinWrite, Clone)]
pub struct CmdlineExtended(#[br(count = 1536)] pub Vec<u8>);
impl fmt::Display for CmdlineExtended {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ascii_str: String = self
            .0
            .iter()
            .filter(|&&c| c.is_ascii())
            .map(|&c| c as char)
            .collect();
        write!(f, "{}", ascii_str)
    }
}

impl From<String> for CmdlineExtended {
    fn from(s: String) -> Self {
        let mut vec = Vec::with_capacity(1536);
        vec.extend(s.as_bytes().iter().take(1536));
        // Pad with zeros if string is shorter than 1536 bytes
        vec.resize(1536, 0);
        CmdlineExtended(vec)
    }
}

#[derive(Debug, Default, BinRead, BinWrite, Clone)]
pub struct Id(#[br(count = 8)] pub Vec<u8>);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ascii_str: String = self
            .0
            .iter()
            .filter(|&&c| c.is_ascii())
            .map(|&c| c as char)
            .collect();
        write!(f, "{}", ascii_str)
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        let mut vec = Vec::with_capacity(8);
        vec.extend(s.as_bytes().iter().take(8));
        // Pad with zeros if string is shorter than 8 bytes
        vec.resize(8, 0);
        Id(vec)
    }
}

#[derive(Debug, Default, BinRead, BinWrite, Clone)]
pub struct AddressU32(#[br(count = 4)] pub Vec<u8>);

impl fmt::Display for AddressU32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert 4 bytes to u32, accounting for little-endian
        let addr = u32::from_le_bytes(self.0.clone().try_into().unwrap());
        write!(f, "0x{:08x}", addr)
    }
}

impl From<String> for AddressU32 {
    fn from(s: String) -> Self {
        let addr = if s.starts_with("0x") {
            // Parse hexadecimal string
            u32::from_str_radix(&s[2..], 16).unwrap_or_default()
        } else {
            // Try parsing as decimal
            s.parse::<u32>().unwrap_or_default()
        };
        AddressU32(addr.to_le_bytes().to_vec())
    }
}

#[derive(Debug, Default, BinRead, BinWrite, Clone)]
pub struct AddressU64(#[br(count = 8)] pub Vec<u8>);

impl fmt::Display for AddressU64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Handle case where we don't have exactly 8 bytes
        if self.0.len() != 8 {
            return write!(f, "0x{:016x}", 0); // Return 0 as fallback
        }

        // Convert slice to fixed size array, then to u64
        let bytes: [u8; 8] = self.0.as_slice().try_into().unwrap_or([0; 8]);
        let addr = u64::from_le_bytes(bytes);
        write!(f, "0x{:016x}", addr)
    }
}
impl From<String> for AddressU64 {
    fn from(s: String) -> Self {
        let addr = if s.starts_with("0x") {
            // Parse hexadecimal string
            u64::from_str_radix(&s[2..], 16).unwrap_or_default()
        } else {
            // Try parsing as decimal
            s.parse::<u64>().unwrap_or_default()
        };
        AddressU64(addr.to_le_bytes().to_vec())
    }
}

#[derive(Debug, Default, BinRead, BinWrite, Clone)]
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

impl From<String> for OSVersion {
    fn from(s: String) -> Self {
        // Expected format: "major.minor.patch (yyyy-mm)"
        let parts: Vec<&str> = s.split(&['.', ' ', '(', ')', '-'][..]).collect();
        if parts.len() >= 5 {
            let major = parts[0].parse::<u32>().unwrap_or_default();
            let minor = parts[1].parse::<u32>().unwrap_or_default();
            let patch = parts[2].parse::<u32>().unwrap_or_default();
            let year = parts[3].parse::<u32>().unwrap_or_default();
            let month = parts[4].parse::<u32>().unwrap_or_default();

            // Reconstruct the version number according to the bit layout
            let version = (major << 25)
                | ((minor & 0x7F) << 18)
                | ((patch & 0x7F) << 11)
                | (((year.saturating_sub(2000)) & 0xF) << 4)
                | (month & 0xF);

            OSVersion(version)
        } else {
            OSVersion(0)
        }
    }
}
