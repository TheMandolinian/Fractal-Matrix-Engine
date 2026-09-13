use std::error::Error;
use std::fmt;

/// Deterministic failures produced by baseline FER evaluation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FerError {
    /// A human-readable baseline path contained a symbol other than `0` or `1`.
    InvalidPathSymbol { index: usize, symbol: char },

    /// Advancing the topology depth would exceed the representation of `u64`.
    DepthOverflow,
}

impl fmt::Display for FerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPathSymbol { index, symbol } => {
                write!(
                    f,
                    "invalid baseline FER path symbol '{symbol}' at index {index}"
                )
            }
            Self::DepthOverflow => write!(f, "baseline FER topology depth overflow"),
        }
    }
}

impl Error for FerError {}
