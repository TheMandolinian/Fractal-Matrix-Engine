use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SraError {
    EmptySystemNamespace,
    EmptyRootSeed,
    Truncated,
    InvalidMagic,
    UnsupportedWireVersion { found: u8 },
    CanonicalProfileMismatch,
    UnsupportedArtifactVersion { found: u8 },
    FerProfileMismatch,
    UnknownHashSuiteIdentifier,
    InvalidRootSeedPresence { found: u8 },
    LengthOverflow,
    TrailingBytes,
}

impl fmt::Display for SraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySystemNamespace => f.write_str("system namespace must not be empty"),
            Self::EmptyRootSeed => f.write_str("present root seed must not be empty"),
            Self::Truncated => f.write_str("truncated canonical SRA input"),
            Self::InvalidMagic => f.write_str("invalid canonical SRA magic"),
            Self::UnsupportedWireVersion { found } => {
                write!(f, "unsupported canonical SRA wire version {found}")
            }
            Self::CanonicalProfileMismatch => f.write_str("canonical SRA profile mismatch"),
            Self::UnsupportedArtifactVersion { found } => {
                write!(f, "unsupported SRA artifact version {found}")
            }
            Self::FerProfileMismatch => f.write_str("SRA FER profile mismatch"),
            Self::UnknownHashSuiteIdentifier => f.write_str("unknown SRA hash-suite identifier"),
            Self::InvalidRootSeedPresence { found } => {
                write!(f, "invalid root-seed presence value {found}")
            }
            Self::LengthOverflow => {
                f.write_str("canonical SRA length exceeds implementation limits")
            }
            Self::TrailingBytes => f.write_str("trailing canonical SRA bytes"),
        }
    }
}

impl std::error::Error for SraError {}
