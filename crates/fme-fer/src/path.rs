use crate::error::FerError;
use std::fmt;
use std::str::FromStr;

/// One transform selector in `FME-FER-AFFINE-2D-BINARY-V1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TopologyBit {
    Zero,
    One,
}

impl fmt::Display for TopologyBit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => f.write_str("0"),
            Self::One => f.write_str("1"),
        }
    }
}

/// Logical binary path for the baseline FER profile.
///
/// This type represents topology semantics only. Its in-memory representation
/// is NOT the normative FME wire encoding.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BaselinePath {
    bits: Vec<TopologyBit>,
}

impl BaselinePath {
    /// Construct the empty/root path.
    pub fn root() -> Self {
        Self::default()
    }

    /// Construct a path from explicit transform selectors.
    pub fn from_bits(bits: impl IntoIterator<Item = TopologyBit>) -> Self {
        Self {
            bits: bits.into_iter().collect(),
        }
    }

    /// Number of topology transforms represented by this path.
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Whether this is the empty/root path.
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// Iterate over transform selectors from root toward the target state.
    pub fn iter(&self) -> impl Iterator<Item = TopologyBit> + '_ {
        self.bits.iter().copied()
    }
}

impl FromStr for BaselinePath {
    type Err = FerError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut bits = Vec::with_capacity(value.len());

        for (index, symbol) in value.chars().enumerate() {
            let bit = match symbol {
                '0' => TopologyBit::Zero,
                '1' => TopologyBit::One,
                _ => {
                    return Err(FerError::InvalidPathSymbol { index, symbol });
                }
            };

            bits.push(bit);
        }

        Ok(Self { bits })
    }
}

impl fmt::Display for BaselinePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.bits.is_empty() {
            return f.write_str("ε");
        }

        for bit in &self.bits {
            write!(f, "{bit}")?;
        }

        Ok(())
    }
}
