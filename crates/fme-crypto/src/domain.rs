use crate::RegistryError;

/// Identifier for the Phase 007 cryptographic-domain registry.
pub const DOMAIN_REGISTRY_ID: &str = "FME-CRYPTO-DOMAIN-REGISTRY-V1";

/// Exact registered semantic domains established by Phase 007.
///
/// This registry is intentionally narrow. Future authority-bearing object
/// classes require explicit later registration rather than implicit use of
/// whitepaper examples.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DomainId {
    SingularityV1,
    AuthorityDomainV1,
}

impl DomainId {
    pub const SINGULARITY_V1_BYTES: &'static [u8] = b"FME/SINGULARITY/V1";
    pub const AUTHORITY_DOMAIN_V1_BYTES: &'static [u8] = b"FME/AUTHORITY_DOMAIN/V1";

    /// Exact authoritative bytes for this semantic domain.
    pub const fn as_bytes(self) -> &'static [u8] {
        match self {
            Self::SingularityV1 => Self::SINGULARITY_V1_BYTES,
            Self::AuthorityDomainV1 => Self::AUTHORITY_DOMAIN_V1_BYTES,
        }
    }

    /// Human-readable form of the exact registered identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingularityV1 => "FME/SINGULARITY/V1",
            Self::AuthorityDomainV1 => "FME/AUTHORITY_DOMAIN/V1",
        }
    }

    /// Parse only exact registered domain bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RegistryError> {
        match bytes {
            Self::SINGULARITY_V1_BYTES => Ok(Self::SingularityV1),
            Self::AUTHORITY_DOMAIN_V1_BYTES => Ok(Self::AuthorityDomainV1),
            _ => Err(RegistryError::UnknownDomainIdentifier),
        }
    }
}
