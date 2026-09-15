use fme_crypto::{
    DigestError, DomainId, HashSuiteId, PreimageError, digest_preimage, encode_preimage,
};
use std::error::Error;
use std::fmt;

use crate::{SingularityRootArtifactV1, SraError, encode_sra};

/// Identifier for the Phase 010 baseline SRA commitment profile.
pub const SRA_COMMITMENT_PROFILE_ID: &str = "FME-SRA-COMMITMENT-V1";

/// Deterministic failures while producing a Phase 010 SRA commitment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SraCommitmentError {
    CanonicalSra(SraError),
    Preimage(PreimageError),
    Digest(DigestError),
}

impl fmt::Display for SraCommitmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CanonicalSra(error) => write!(f, "canonical SRA encoding failed: {error}"),
            Self::Preimage(error) => {
                write!(f, "SRA commitment preimage construction failed: {error}")
            }
            Self::Digest(error) => write!(f, "SRA commitment digest execution failed: {error}"),
        }
    }
}

impl Error for SraCommitmentError {}

/// Typed cryptographic commitment to one canonical baseline SRA.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SraCommitmentV1 {
    hash_suite: HashSuiteId,
    digest: Vec<u8>,
}

impl SraCommitmentV1 {
    pub const fn profile_id(&self) -> &'static str {
        SRA_COMMITMENT_PROFILE_ID
    }

    pub const fn domain(&self) -> DomainId {
        DomainId::SingularityV1
    }

    pub const fn hash_suite(&self) -> HashSuiteId {
        self.hash_suite
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.digest
    }
}

/// Commit one baseline SRA under its declared default hash suite.
pub fn commit_sra(
    artifact: &SingularityRootArtifactV1,
) -> Result<SraCommitmentV1, SraCommitmentError> {
    let canonical_sra = encode_sra(artifact).map_err(SraCommitmentError::CanonicalSra)?;

    let preimage = encode_preimage(DomainId::SingularityV1, &canonical_sra)
        .map_err(SraCommitmentError::Preimage)?;

    let hash_suite = artifact.default_hash_suite_id();

    let digest = digest_preimage(hash_suite, &preimage).map_err(SraCommitmentError::Digest)?;

    Ok(SraCommitmentV1 {
        hash_suite,
        digest: digest.as_bytes().to_vec(),
    })
}
