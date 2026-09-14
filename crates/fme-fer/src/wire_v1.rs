use crate::baseline_v1::PROFILE_ID;
use crate::{BaselinePath, ExactState, TopologyBit};
use num_bigint::{BigInt, Sign};
use std::error::Error;
use std::fmt;

const MAGIC: &[u8; 4] = b"FMEF";
const PROFILE_ID_BYTES: &[u8] = b"FME-FER-AFFINE-2D-BINARY-V1";

pub const WIRE_VERSION: u8 = 1;
pub const PATH_OBJECT_KIND: u8 = 1;
pub const EXACT_STATE_OBJECT_KIND: u8 = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CanonicalWireError {
    Truncated,
    InvalidMagic,
    UnsupportedVersion { found: u8 },
    UnexpectedObjectKind { expected: u8, found: u8 },
    ProfileMismatch,
    LengthOverflow,
    InvalidIntegerSign { found: u8 },
    NonCanonicalZero,
    NonCanonicalIntegerMagnitude,
    NonZeroPathPadding,
    TrailingBytes,
}

impl fmt::Display for CanonicalWireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => write!(f, "truncated canonical FER wire input"),
            Self::InvalidMagic => write!(f, "invalid canonical FER wire magic"),
            Self::UnsupportedVersion { found } => {
                write!(f, "unsupported canonical FER wire version {found}")
            }
            Self::UnexpectedObjectKind { expected, found } => write!(
                f,
                "unexpected canonical FER wire object kind {found}; expected {expected}"
            ),
            Self::ProfileMismatch => write!(f, "canonical FER wire profile mismatch"),
            Self::LengthOverflow => {
                write!(f, "canonical FER wire length exceeds implementation limits")
            }
            Self::InvalidIntegerSign { found } => {
                write!(f, "invalid canonical FER integer sign {found}")
            }
            Self::NonCanonicalZero => {
                write!(f, "noncanonical canonical FER zero integer")
            }
            Self::NonCanonicalIntegerMagnitude => {
                write!(f, "noncanonical canonical FER integer magnitude")
            }
            Self::NonZeroPathPadding => {
                write!(f, "nonzero padding bits in canonical FER path")
            }
            Self::TrailingBytes => write!(f, "trailing canonical FER wire bytes"),
        }
    }
}

impl Error for CanonicalWireError {}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn read_u8(&mut self) -> Result<u8, CanonicalWireError> {
        let value = *self
            .bytes
            .get(self.offset)
            .ok_or(CanonicalWireError::Truncated)?;
        self.offset += 1;
        Ok(value)
    }

    fn read_u64(&mut self) -> Result<u64, CanonicalWireError> {
        let bytes = self.read_exact(8)?;
        let mut array = [0u8; 8];
        array.copy_from_slice(bytes);
        Ok(u64::from_be_bytes(array))
    }

    fn read_exact(&mut self, length: usize) -> Result<&'a [u8], CanonicalWireError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(CanonicalWireError::LengthOverflow)?;

        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(CanonicalWireError::Truncated)?;

        self.offset = end;
        Ok(value)
    }

    fn finish(self) -> Result<(), CanonicalWireError> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(CanonicalWireError::TrailingBytes)
        }
    }
}

fn encode_header(object_kind: u8) -> Vec<u8> {
    debug_assert_eq!(PROFILE_ID.as_bytes(), PROFILE_ID_BYTES);
    debug_assert!(u8::try_from(PROFILE_ID_BYTES.len()).is_ok());

    let mut bytes = Vec::with_capacity(7 + PROFILE_ID_BYTES.len());
    bytes.extend_from_slice(MAGIC);
    bytes.push(WIRE_VERSION);
    bytes.push(object_kind);
    bytes.push(PROFILE_ID_BYTES.len() as u8);
    bytes.extend_from_slice(PROFILE_ID_BYTES);
    bytes
}

fn decode_header(
    reader: &mut Reader<'_>,
    expected_object_kind: u8,
) -> Result<(), CanonicalWireError> {
    if reader.read_exact(MAGIC.len())? != MAGIC {
        return Err(CanonicalWireError::InvalidMagic);
    }

    let version = reader.read_u8()?;
    if version != WIRE_VERSION {
        return Err(CanonicalWireError::UnsupportedVersion { found: version });
    }

    let object_kind = reader.read_u8()?;
    if object_kind != expected_object_kind {
        return Err(CanonicalWireError::UnexpectedObjectKind {
            expected: expected_object_kind,
            found: object_kind,
        });
    }

    let profile_length = usize::from(reader.read_u8()?);
    let profile = reader.read_exact(profile_length)?;

    if profile != PROFILE_ID_BYTES {
        return Err(CanonicalWireError::ProfileMismatch);
    }

    Ok(())
}

fn encode_bigint(value: &BigInt, output: &mut Vec<u8>) -> Result<(), CanonicalWireError> {
    let (sign, magnitude) = value.to_bytes_be();

    if sign == Sign::NoSign {
        output.push(0);
        output.extend_from_slice(&0u64.to_be_bytes());
        return Ok(());
    }

    if magnitude.is_empty() || magnitude[0] == 0 {
        return Err(CanonicalWireError::NonCanonicalIntegerMagnitude);
    }

    let sign_byte = match sign {
        Sign::Plus => 1u8,
        Sign::Minus => 2u8,
        Sign::NoSign => unreachable!("zero handled above"),
    };

    let magnitude_length =
        u64::try_from(magnitude.len()).map_err(|_| CanonicalWireError::LengthOverflow)?;

    output.push(sign_byte);
    output.extend_from_slice(&magnitude_length.to_be_bytes());
    output.extend_from_slice(&magnitude);

    Ok(())
}

fn decode_bigint(reader: &mut Reader<'_>) -> Result<BigInt, CanonicalWireError> {
    let sign_byte = reader.read_u8()?;
    let length = reader.read_u64()?;
    let length = usize::try_from(length).map_err(|_| CanonicalWireError::LengthOverflow)?;
    let magnitude = reader.read_exact(length)?;

    match sign_byte {
        0 => {
            if !magnitude.is_empty() {
                return Err(CanonicalWireError::NonCanonicalZero);
            }

            Ok(BigInt::from(0u8))
        }
        1 | 2 => {
            if magnitude.is_empty() {
                return Err(CanonicalWireError::NonCanonicalZero);
            }

            if magnitude[0] == 0 {
                return Err(CanonicalWireError::NonCanonicalIntegerMagnitude);
            }

            let sign = if sign_byte == 1 {
                Sign::Plus
            } else {
                Sign::Minus
            };

            Ok(BigInt::from_bytes_be(sign, magnitude))
        }
        found => Err(CanonicalWireError::InvalidIntegerSign { found }),
    }
}

pub fn encode_path(path: &BaselinePath) -> Result<Vec<u8>, CanonicalWireError> {
    let bit_length = u64::try_from(path.len()).map_err(|_| CanonicalWireError::LengthOverflow)?;

    let packed_length = (path.len() / 8)
        .checked_add(usize::from(!path.len().is_multiple_of(8)))
        .ok_or(CanonicalWireError::LengthOverflow)?;

    let mut output = encode_header(PATH_OBJECT_KIND);
    output.extend_from_slice(&bit_length.to_be_bytes());

    let output_length = output
        .len()
        .checked_add(packed_length)
        .ok_or(CanonicalWireError::LengthOverflow)?;

    output.resize(output_length, 0);

    let packed_start = output.len() - packed_length;

    for (index, bit) in path.iter().enumerate() {
        if bit == TopologyBit::One {
            let byte_index = packed_start + index / 8;
            let bit_index = 7 - (index % 8);
            output[byte_index] |= 1u8 << bit_index;
        }
    }

    Ok(output)
}

pub fn decode_path(bytes: &[u8]) -> Result<BaselinePath, CanonicalWireError> {
    let mut reader = Reader::new(bytes);
    decode_header(&mut reader, PATH_OBJECT_KIND)?;

    let bit_length = reader.read_u64()?;
    let bit_length = usize::try_from(bit_length).map_err(|_| CanonicalWireError::LengthOverflow)?;

    let packed_length = (bit_length / 8)
        .checked_add(usize::from(!bit_length.is_multiple_of(8)))
        .ok_or(CanonicalWireError::LengthOverflow)?;
    let packed = reader.read_exact(packed_length)?;

    if !bit_length.is_multiple_of(8) {
        let unused_bits = 8 - (bit_length % 8);
        let padding_mask = (1u8 << unused_bits) - 1;

        if packed.last().is_some_and(|last| last & padding_mask != 0) {
            return Err(CanonicalWireError::NonZeroPathPadding);
        }
    }

    let mut bits = Vec::with_capacity(bit_length);

    for index in 0..bit_length {
        let byte = packed[index / 8];
        let bit_index = 7 - (index % 8);

        bits.push(if byte & (1u8 << bit_index) == 0 {
            TopologyBit::Zero
        } else {
            TopologyBit::One
        });
    }

    reader.finish()?;
    Ok(BaselinePath::from_bits(bits))
}

pub fn encode_exact_state(state: &ExactState) -> Result<Vec<u8>, CanonicalWireError> {
    let mut output = encode_header(EXACT_STATE_OBJECT_KIND);

    output.extend_from_slice(&state.depth().to_be_bytes());
    encode_bigint(state.p(), &mut output)?;
    encode_bigint(state.q(), &mut output)?;

    Ok(output)
}

pub fn decode_exact_state(bytes: &[u8]) -> Result<ExactState, CanonicalWireError> {
    let mut reader = Reader::new(bytes);
    decode_header(&mut reader, EXACT_STATE_OBJECT_KIND)?;

    let depth = reader.read_u64()?;
    let p = decode_bigint(&mut reader)?;
    let q = decode_bigint(&mut reader)?;

    reader.finish()?;

    Ok(ExactState::from_parts(p, q, depth))
}
