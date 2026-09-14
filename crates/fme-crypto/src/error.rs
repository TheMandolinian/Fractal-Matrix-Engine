use std::error::Error;
use std::fmt;

/// Deterministic failures for FME cryptographic registry identifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistryError {
    UnknownDomainIdentifier,
    UnknownHashSuiteIdentifier,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownDomainIdentifier => {
                f.write_str("unknown FME cryptographic domain identifier")
            }
            Self::UnknownHashSuiteIdentifier => f.write_str("unknown FME hash-suite identifier"),
        }
    }
}

impl Error for RegistryError {}
