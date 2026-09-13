#![forbid(unsafe_code)]

//! Exact Fractal Evaluation Rule implementation for the
//! Fractal Matrix Engine (FME).
//!
//! Initial implementation scope:
//!
//! `FME-FER-AFFINE-2D-BINARY-V1`
//!
//! This crate does not implement HashHelix event authority, Authority Domain
//! materialization, MMR history commitments, authenticated state,
//! Proof Capsules, archives, authorization, cross-domain coordination,
//! or multidimensional FER profiles.

pub mod baseline_v1;
pub mod error;
pub mod path;
pub mod state;

pub use error::FerError;
pub use path::{BaselinePath, TopologyBit};
pub use state::ExactState;
