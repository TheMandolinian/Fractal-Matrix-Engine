use crate::RegistryError;

/// Identifier for the Phase 007 hash-suite registry.
pub const HASH_SUITE_REGISTRY_ID: &str = "FME-HASH-SUITE-REGISTRY-V1";

/// Hash-suite identifiers recognized by the initial FME registry.
///
/// Phase 007 establishes identifier semantics only. It does not execute these
/// algorithms and does not create cryptographic commitments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HashSuiteId {
    Sha2_256,
    Sha2_512,
}

impl HashSuiteId {
    pub const SHA2_256_BYTES: &'static [u8] = b"SHA2-256";
    pub const SHA2_512_BYTES: &'static [u8] = b"SHA2-512";

    /// Exact authoritative bytes for this registered suite identifier.
    pub const fn as_bytes(self) -> &'static [u8] {
        match self {
            Self::Sha2_256 => Self::SHA2_256_BYTES,
            Self::Sha2_512 => Self::SHA2_512_BYTES,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sha2_256 => "SHA2-256",
            Self::Sha2_512 => "SHA2-512",
        }
    }

    /// Digest width associated with the named algorithm.
    ///
    /// This describes registry semantics only; Phase 007 does not calculate
    /// a digest.
    pub const fn digest_len(self) -> usize {
        match self {
            Self::Sha2_256 => 32,
            Self::Sha2_512 => 64,
        }
    }

    /// Parse only exact registered suite bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RegistryError> {
        match bytes {
            Self::SHA2_256_BYTES => Ok(Self::Sha2_256),
            Self::SHA2_512_BYTES => Ok(Self::Sha2_512),
            _ => Err(RegistryError::UnknownHashSuiteIdentifier),
        }
    }
}
