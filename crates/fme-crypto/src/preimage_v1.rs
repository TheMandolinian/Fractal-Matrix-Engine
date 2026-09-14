use crate::DomainId;
use std::error::Error;
use std::fmt;

const MAGIC: &[u8; 4] = b"FMEP";

pub const PREIMAGE_VERSION: u8 = 1;
pub const PREIMAGE_PROFILE_ID: &str = "FME-DOMAIN-PREIMAGE-V1";

/// Deterministic failures while parsing or constructing a Phase 007
/// domain-separated canonical preimage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreimageError {
    Truncated,
    InvalidMagic,
    UnsupportedVersion { found: u8 },
    UnknownDomain,
    LengthOverflow,
    TrailingBytes,
}

impl fmt::Display for PreimageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => f.write_str("truncated FME domain-separated preimage"),
            Self::InvalidMagic => f.write_str("invalid FME domain-separated preimage magic"),
            Self::UnsupportedVersion { found } => {
                write!(
                    f,
                    "unsupported FME domain-separated preimage version {found}"
                )
            }
            Self::UnknownDomain => f.write_str("unknown FME cryptographic domain"),
            Self::LengthOverflow => {
                f.write_str("FME domain-separated preimage length exceeds implementation limits")
            }
            Self::TrailingBytes => f.write_str("trailing bytes in FME domain-separated preimage"),
        }
    }
}

impl Error for PreimageError {}

/// Parsed view of one canonical domain-separated preimage.
///
/// Hash-suite identity is intentionally not embedded in these bytes. A later
/// commitment profile selects the registered hash suite that processes this
/// exact preimage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DomainSeparatedPreimage<'a> {
    domain: DomainId,
    canonical_payload: &'a [u8],
}

impl<'a> DomainSeparatedPreimage<'a> {
    pub const fn domain(&self) -> DomainId {
        self.domain
    }

    pub const fn canonical_payload(&self) -> &'a [u8] {
        self.canonical_payload
    }
}

/// Construct the unique Phase 007 preimage framing for an already-canonical
/// authority payload.
pub fn encode_preimage(
    domain: DomainId,
    canonical_payload: &[u8],
) -> Result<Vec<u8>, PreimageError> {
    let domain_bytes = domain.as_bytes();

    let domain_len =
        u64::try_from(domain_bytes.len()).map_err(|_| PreimageError::LengthOverflow)?;
    let payload_len =
        u64::try_from(canonical_payload.len()).map_err(|_| PreimageError::LengthOverflow)?;

    let capacity = MAGIC
        .len()
        .checked_add(1)
        .and_then(|n| n.checked_add(8))
        .and_then(|n| n.checked_add(domain_bytes.len()))
        .and_then(|n| n.checked_add(8))
        .and_then(|n| n.checked_add(canonical_payload.len()))
        .ok_or(PreimageError::LengthOverflow)?;

    let mut output = Vec::with_capacity(capacity);

    output.extend_from_slice(MAGIC);
    output.push(PREIMAGE_VERSION);
    output.extend_from_slice(&domain_len.to_be_bytes());
    output.extend_from_slice(domain_bytes);
    output.extend_from_slice(&payload_len.to_be_bytes());
    output.extend_from_slice(canonical_payload);

    Ok(output)
}

/// Parse and validate one canonical Phase 007 preimage.
///
/// Unknown semantic domains fail closed.
pub fn decode_preimage(bytes: &[u8]) -> Result<DomainSeparatedPreimage<'_>, PreimageError> {
    let mut reader = Reader::new(bytes);

    if reader.read_exact(MAGIC.len())? != MAGIC {
        return Err(PreimageError::InvalidMagic);
    }

    let version = reader.read_u8()?;
    if version != PREIMAGE_VERSION {
        return Err(PreimageError::UnsupportedVersion { found: version });
    }

    let domain_len = reader.read_u64()?;
    let domain_len = usize::try_from(domain_len).map_err(|_| PreimageError::LengthOverflow)?;
    let domain_bytes = reader.read_exact(domain_len)?;

    let domain = DomainId::from_bytes(domain_bytes).map_err(|_| PreimageError::UnknownDomain)?;

    let payload_len = reader.read_u64()?;
    let payload_len = usize::try_from(payload_len).map_err(|_| PreimageError::LengthOverflow)?;
    let canonical_payload = reader.read_exact(payload_len)?;

    reader.finish()?;

    Ok(DomainSeparatedPreimage {
        domain,
        canonical_payload,
    })
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn read_u8(&mut self) -> Result<u8, PreimageError> {
        let value = *self
            .bytes
            .get(self.offset)
            .ok_or(PreimageError::Truncated)?;

        self.offset += 1;
        Ok(value)
    }

    fn read_u64(&mut self) -> Result<u64, PreimageError> {
        let bytes = self.read_exact(8)?;
        let mut value = [0u8; 8];
        value.copy_from_slice(bytes);
        Ok(u64::from_be_bytes(value))
    }

    fn read_exact(&mut self, length: usize) -> Result<&'a [u8], PreimageError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(PreimageError::LengthOverflow)?;

        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(PreimageError::Truncated)?;

        self.offset = end;
        Ok(value)
    }

    fn finish(self) -> Result<(), PreimageError> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(PreimageError::TrailingBytes)
        }
    }
}
