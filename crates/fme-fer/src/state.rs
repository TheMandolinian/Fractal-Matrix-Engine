use num_bigint::BigInt;

/// Exact authoritative topology state for
/// `FME-FER-AFFINE-2D-BINARY-V1`.
///
/// The represented coordinate is:
///
/// `z = (P + iQ) / 3^depth`
///
/// `P`, `Q`, and `depth` are retained exactly. The representation is not
/// reduced as an ordinary rational fraction because depth is part of the
/// baseline topology state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactState {
    p: BigInt,
    q: BigInt,
    depth: u64,
}

impl ExactState {
    /// Exact root state `z_ε = 0`.
    pub fn root() -> Self {
        Self {
            p: BigInt::from(0u8),
            q: BigInt::from(0u8),
            depth: 0,
        }
    }

    /// Exact numerator of the real component.
    pub fn p(&self) -> &BigInt {
        &self.p
    }

    /// Exact numerator of the imaginary component.
    pub fn q(&self) -> &BigInt {
        &self.q
    }

    /// Recursive baseline topology depth.
    pub fn depth(&self) -> u64 {
        self.depth
    }

    pub(crate) fn from_parts(p: BigInt, q: BigInt, depth: u64) -> Self {
        Self { p, q, depth }
    }
}
