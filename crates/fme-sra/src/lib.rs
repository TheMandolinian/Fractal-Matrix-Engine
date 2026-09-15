#![forbid(unsafe_code)]

//! Baseline Singularity Root Artifact semantics and canonical serialization
//! for the Fractal Matrix Engine.
//!
//! Phase 009 establishes the exact baseline SRA semantic boundary and its
//! canonical byte representation.
//!
//! Phase 010 adds the baseline cryptographic SRA commitment profile.
//!
//! This crate does not create Stable Authority Identities or Authority Domain materialization.
//! It also does not create accepted-history commitments, MMRs, authenticated state,
//! Proof Capsules, authorization, or cross-domain coordination.

pub mod commitment_v1;
pub mod error;
pub mod sra_v1;

pub use commitment_v1::{
    SRA_COMMITMENT_PROFILE_ID, SraCommitmentError, SraCommitmentV1, commit_sra,
};
pub use error::SraError;
pub use sra_v1::{
    SRA_ARTIFACT_VERSION, SRA_CANONICAL_PROFILE_ID, SRA_FER_PROFILE_ID, SRA_MAGIC,
    SRA_WIRE_FORMAT_ID, SRA_WIRE_VERSION, SingularityRootArtifactV1, decode_sra, encode_sra,
};
