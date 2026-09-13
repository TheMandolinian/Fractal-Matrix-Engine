use crate::error::FerError;
use crate::path::{BaselinePath, TopologyBit};
use crate::state::ExactState;
use num_bigint::BigInt;

/// Baseline FER profile identifier.
pub const PROFILE_ID: &str = "FME-FER-AFFINE-2D-BINARY-V1";

/// Return the exact root topology state.
pub fn root() -> ExactState {
    ExactState::root()
}

/// Apply baseline transform `F0`.
///
/// For parent state `(P, Q, d)`:
///
/// `P' = P - Q - 3^(d+1)`
///
/// `Q' = P + Q`
pub fn apply_f0(parent: &ExactState) -> Result<ExactState, FerError> {
    apply_transform(parent, TopologyBit::Zero)
}

/// Apply baseline transform `F1`.
///
/// For parent state `(P, Q, d)`:
///
/// `P' = P + Q + 3^(d+1)`
///
/// `Q' = Q - P`
pub fn apply_f1(parent: &ExactState) -> Result<ExactState, FerError> {
    apply_transform(parent, TopologyBit::One)
}

/// Continue exact FER evaluation from a retained exact parent state.
pub fn apply_transform(
    parent: &ExactState,
    transform: TopologyBit,
) -> Result<ExactState, FerError> {
    let next_depth = parent
        .depth()
        .checked_add(1)
        .ok_or(FerError::DepthOverflow)?;

    let denominator = power_of_three(next_depth);

    Ok(apply_with_denominator(
        parent,
        transform,
        next_depth,
        &denominator,
    ))
}

/// Evaluate a complete baseline path from the root state.
///
/// Denominator progression is retained incrementally during root evaluation so
/// each path step does not independently recompute `3^depth`.
pub fn evaluate(path: &BaselinePath) -> Result<ExactState, FerError> {
    let mut state = root();
    let mut denominator = BigInt::from(1u8);

    for transform in path.iter() {
        let next_depth = state
            .depth()
            .checked_add(1)
            .ok_or(FerError::DepthOverflow)?;

        denominator *= 3u8;

        state = apply_with_denominator(&state, transform, next_depth, &denominator);
    }

    Ok(state)
}

fn apply_with_denominator(
    parent: &ExactState,
    transform: TopologyBit,
    next_depth: u64,
    denominator: &BigInt,
) -> ExactState {
    let (next_p, next_q) = match transform {
        TopologyBit::Zero => (
            parent.p() - parent.q() - denominator,
            parent.p() + parent.q(),
        ),
        TopologyBit::One => (
            parent.p() + parent.q() + denominator,
            parent.q() - parent.p(),
        ),
    };

    ExactState::from_parts(next_p, next_q, next_depth)
}

fn power_of_three(mut exponent: u64) -> BigInt {
    let mut result = BigInt::from(1u8);
    let mut base = BigInt::from(3u8);

    while exponent > 0 {
        if exponent & 1 == 1 {
            result *= &base;
        }

        exponent >>= 1;

        if exponent > 0 {
            base = &base * &base;
        }
    }

    result
}
