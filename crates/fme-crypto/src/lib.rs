#![forbid(unsafe_code)]

//! Cryptographic registry and canonical commitment-preimage framing for the
//! Fractal Matrix Engine.
//!
//! Phase 007 establishes:
//!
//! - exact semantic-domain identifiers;
//! - exact recognized hash-suite identifiers;
//! - deterministic domain-separated preimage framing.
//!
//! Phase 008 adds deterministic execution of the registered SHA2-256 and
//! SHA2-512 suites over exact validated Phase 007 preimage bytes.
//!
//! This crate does not create SRA commitments, Stable Authority Identities,
//! history commitments, state commitments, Proof Capsules, signatures, key
//! material, or hash-suite transitions.

pub mod digest_v1;
pub mod domain;
pub mod error;
pub mod hash_suite;
pub mod preimage_v1;

pub use digest_v1::{DIGEST_PROFILE_ID, DigestError, DigestValue, digest_preimage};
pub use domain::{DOMAIN_REGISTRY_ID, DomainId};
pub use error::RegistryError;
pub use hash_suite::{HASH_SUITE_REGISTRY_ID, HashSuiteId};
pub use preimage_v1::{
    DomainSeparatedPreimage, PREIMAGE_PROFILE_ID, PREIMAGE_VERSION, PreimageError, decode_preimage,
    encode_preimage,
};
