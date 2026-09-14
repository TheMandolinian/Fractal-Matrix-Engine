use crate::{HashSuiteId, PreimageError, decode_preimage};
use sha2::{Digest, Sha256, Sha512};
use std::error::Error;
use std::fmt;

/// Identifier for the Phase 008 registered SHA-2 digest profile.
pub const DIGEST_PROFILE_ID: &str = "FME-REGISTERED-SHA2-DIGEST-V1";

/// Deterministic failures while executing a registered digest over an FME
/// canonical domain-separated preimage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DigestError {
    InvalidPreimage(PreimageError),
}

impl fmt::Display for DigestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPreimage(error) => {
                write!(f, "invalid FME domain-separated digest preimage: {error}")
            }
        }
    }
}

impl Error for DigestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPreimage(error) => Some(error),
        }
    }
}

impl From<PreimageError> for DigestError {
    fn from(error: PreimageError) -> Self {
        Self::InvalidPreimage(error)
    }
}

/// Digest bytes together with the registered suite that produced them.
///
/// Suite identity is explicit and must not be inferred from digest length.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DigestValue {
    hash_suite: HashSuiteId,
    bytes: Vec<u8>,
}

impl DigestValue {
    /// Registered suite that produced this digest.
    pub const fn hash_suite(&self) -> HashSuiteId {
        self.hash_suite
    }

    /// Exact digest bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Execute one registered SHA-2 suite over an exact canonical Phase 007
/// domain-separated preimage.
///
/// The supplied bytes are first validated by the Phase 007 decoder. Digest
/// execution then consumes the exact original preimage bytes without changing
/// their encoding or embedding hash-suite identity into them.
pub fn digest_preimage(
    hash_suite: HashSuiteId,
    preimage: &[u8],
) -> Result<DigestValue, DigestError> {
    decode_preimage(preimage)?;

    let bytes = match hash_suite {
        HashSuiteId::Sha2_256 => Sha256::digest(preimage).to_vec(),
        HashSuiteId::Sha2_512 => Sha512::digest(preimage).to_vec(),
    };

    Ok(DigestValue { hash_suite, bytes })
}
