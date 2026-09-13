# Fractal Matrix Engine Whitepaper

This directory contains the architectural whitepaper for the Fractal Matrix Engine.

## Current Whitepaper

`FRACTAL_MATRIX_ENGINE_WHITEPAPER_V1.0_DRAFT.md`

Status:

**Architecture-Defined / Pre-Production Research Specification**

## Repository Role

The whitepaper is the architectural source for FME design intent, subsystem boundaries, terminology, invariants, and future implementation direction.

Its presence in this repository does not mean that every mechanism described in the whitepaper has been implemented.

The following distinctions must remain explicit:

- architecture-permitted does not mean implemented;
- implemented does not mean tested;
- tested does not mean independently reproduced;
- benchmarked does not mean secure;
- cryptographically committed does not mean physically true.

Implementation maturity is tracked separately through repository phase documentation, profile status, conformance artifacts, tests, audits, and actual Rust source.

## Authority Relationship

The repository should interpret the major artifact classes as follows:

- **Whitepaper** — architectural specification and design source.
- **Versioned profiles** — subsystem-specific normative behavior once formally established.
- **Conformance artifacts** — expected deterministic behavior for declared profiles.
- **Rust source** — reference implementation of currently implemented profiles and mechanisms.
- **Phase documentation** — chronological implementation and verification record.

Where implementation behavior is not specified sufficiently by the whitepaper or an established profile, the gap must not be silently resolved through arbitrary implementation choices.

Such cases should be classified as one of:

1. implementation defect;
2. profile specification gap;
3. architecture amendment requirement;
4. research question.

## Draft Status

This whitepaper remains a research specification.

Descriptions of future mechanisms such as additional FER profiles, Authority Domain infrastructure, authenticated history, authenticated state, Proof Capsules, SRA processing, archival systems, execution proofs, or cross-domain coordination must not be represented as implemented unless the repository independently demonstrates that maturity.
