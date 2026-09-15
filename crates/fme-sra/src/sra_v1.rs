use fme_crypto::HashSuiteId;
use fme_fer::baseline_v1::PROFILE_ID as FER_PROFILE_ID;

use crate::SraError;

pub const SRA_MAGIC: [u8; 4] = *b"FMES";
pub const SRA_WIRE_VERSION: u8 = 0x01;
pub const SRA_ARTIFACT_VERSION: u8 = 0x01;
pub const SRA_CANONICAL_PROFILE_ID: &str = "FME-SRA-CANONICAL-V1";
pub const SRA_WIRE_FORMAT_ID: &str = "FME-SRA-CANONICAL-V1-WIRE-V1";
pub const SRA_FER_PROFILE_ID: &str = FER_PROFILE_ID;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingularityRootArtifactV1 {
    system_namespace: Vec<u8>,
    default_hash_suite_id: HashSuiteId,
    root_seed: Option<Vec<u8>>,
}

impl SingularityRootArtifactV1 {
    pub fn new(
        system_namespace: Vec<u8>,
        default_hash_suite_id: HashSuiteId,
        root_seed: Option<Vec<u8>>,
    ) -> Result<Self, SraError> {
        if system_namespace.is_empty() {
            return Err(SraError::EmptySystemNamespace);
        }

        if root_seed.as_ref().is_some_and(Vec::is_empty) {
            return Err(SraError::EmptyRootSeed);
        }

        Ok(Self {
            system_namespace,
            default_hash_suite_id,
            root_seed,
        })
    }

    pub const fn artifact_version(&self) -> u8 {
        SRA_ARTIFACT_VERSION
    }

    pub const fn fer_profile_id(&self) -> &'static str {
        SRA_FER_PROFILE_ID
    }

    pub fn system_namespace(&self) -> &[u8] {
        &self.system_namespace
    }

    pub const fn default_hash_suite_id(&self) -> HashSuiteId {
        self.default_hash_suite_id
    }

    pub fn root_seed(&self) -> Option<&[u8]> {
        self.root_seed.as_deref()
    }
}

fn append_length_prefixed(bytes: &[u8], output: &mut Vec<u8>) -> Result<(), SraError> {
    let length = u64::try_from(bytes.len()).map_err(|_| SraError::LengthOverflow)?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

pub fn encode_sra(artifact: &SingularityRootArtifactV1) -> Result<Vec<u8>, SraError> {
    let mut output = Vec::new();

    output.extend_from_slice(&SRA_MAGIC);
    output.push(SRA_WIRE_VERSION);
    append_length_prefixed(SRA_CANONICAL_PROFILE_ID.as_bytes(), &mut output)?;
    output.push(SRA_ARTIFACT_VERSION);
    append_length_prefixed(artifact.system_namespace(), &mut output)?;
    append_length_prefixed(SRA_FER_PROFILE_ID.as_bytes(), &mut output)?;
    append_length_prefixed(artifact.default_hash_suite_id().as_bytes(), &mut output)?;

    match artifact.root_seed() {
        None => output.push(0x00),
        Some(seed) => {
            output.push(0x01);
            append_length_prefixed(seed, &mut output)?;
        }
    }

    Ok(output)
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn read_u8(&mut self) -> Result<u8, SraError> {
        let value = *self.bytes.get(self.offset).ok_or(SraError::Truncated)?;
        self.offset += 1;
        Ok(value)
    }

    fn read_u64(&mut self) -> Result<u64, SraError> {
        let bytes = self.read_exact(8)?;
        let mut array = [0u8; 8];
        array.copy_from_slice(bytes);
        Ok(u64::from_be_bytes(array))
    }

    fn read_exact(&mut self, length: usize) -> Result<&'a [u8], SraError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(SraError::LengthOverflow)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(SraError::Truncated)?;
        self.offset = end;
        Ok(value)
    }

    fn read_length_prefixed(&mut self) -> Result<&'a [u8], SraError> {
        let length = self.read_u64()?;
        let length = usize::try_from(length).map_err(|_| SraError::LengthOverflow)?;
        self.read_exact(length)
    }

    fn finish(self) -> Result<(), SraError> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(SraError::TrailingBytes)
        }
    }
}

pub fn decode_sra(bytes: &[u8]) -> Result<SingularityRootArtifactV1, SraError> {
    let mut reader = Reader::new(bytes);

    if reader.read_exact(SRA_MAGIC.len())? != SRA_MAGIC {
        return Err(SraError::InvalidMagic);
    }

    let wire_version = reader.read_u8()?;
    if wire_version != SRA_WIRE_VERSION {
        return Err(SraError::UnsupportedWireVersion {
            found: wire_version,
        });
    }

    let canonical_profile = reader.read_length_prefixed()?;
    if canonical_profile != SRA_CANONICAL_PROFILE_ID.as_bytes() {
        return Err(SraError::CanonicalProfileMismatch);
    }

    let artifact_version = reader.read_u8()?;
    if artifact_version != SRA_ARTIFACT_VERSION {
        return Err(SraError::UnsupportedArtifactVersion {
            found: artifact_version,
        });
    }

    let system_namespace = reader.read_length_prefixed()?;
    if system_namespace.is_empty() {
        return Err(SraError::EmptySystemNamespace);
    }

    let fer_profile = reader.read_length_prefixed()?;
    if fer_profile != SRA_FER_PROFILE_ID.as_bytes() {
        return Err(SraError::FerProfileMismatch);
    }

    let hash_suite_bytes = reader.read_length_prefixed()?;
    let default_hash_suite_id = HashSuiteId::from_bytes(hash_suite_bytes)
        .map_err(|_| SraError::UnknownHashSuiteIdentifier)?;

    let root_seed = match reader.read_u8()? {
        0x00 => None,
        0x01 => {
            let seed = reader.read_length_prefixed()?;
            if seed.is_empty() {
                return Err(SraError::EmptyRootSeed);
            }
            Some(seed.to_vec())
        }
        found => return Err(SraError::InvalidRootSeedPresence { found }),
    };

    reader.finish()?;

    SingularityRootArtifactV1::new(system_namespace.to_vec(), default_hash_suite_id, root_seed)
}
