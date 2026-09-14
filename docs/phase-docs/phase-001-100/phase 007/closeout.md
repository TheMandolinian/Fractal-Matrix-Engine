# Phase 007 — Baseline Cryptographic Registry and Preimage Contract

Status: **SEALED**

Implementation merge anchor:

`a237660a`

Documentation merge anchor:

`8d49bf9e`

Anchor repair merge:

`749fcd7f`

## Objective

Establish the first normative cryptographic registry and canonical
domain-separated preimage boundary for the Fractal Matrix Engine.

Phase 007 defines the deterministic bytes that a later cryptographic digest
implementation may consume without yet executing a digest algorithm.

## Implemented Scope

Phase 007 adds the `fme-crypto` workspace crate and establishes:

- a versioned cryptographic-domain registry;
- a versioned hash-suite registry;
- exact registered semantic-domain identifiers;
- exact recognized hash-suite identifiers;
- deterministic domain-separated preimage framing;
- deterministic parsing and fail-closed rejection;
- normative external conformance vectors;
- independent Python fixture reproduction;
- Rust integration tests;
- a Phase 007 repository audit.

The domain-preimage profile identifier is:

`FME-DOMAIN-PREIMAGE-V1`

## Initial Domain Registry

Phase 007 registers exactly:

- `FME/SINGULARITY/V1`;
- `FME/AUTHORITY_DOMAIN/V1`.

These exact byte strings are authoritative for this registry version.

Additional whitepaper object classes are not implicitly registered.

## Initial Hash-Suite Registry

Phase 007 recognizes:

- `SHA2-256`;
- `SHA2-512`.

The registry records exact suite identifiers and digest widths.

Phase 007 does not execute either algorithm.

Hash-suite identity remains separate from the domain-separated preimage so
that a later explicitly defined cryptographic profile may process the same
canonical semantic preimage under different registered suites.

## Canonical Preimage Boundary

The Phase 007 preimage consists of:

1. four-byte magic `FMEP`;
2. one-byte version `0x01`;
3. unsigned big-endian `u64` domain length;
4. exact registered domain bytes;
5. unsigned big-endian `u64` payload length;
6. already-canonical payload bytes.

The payload remains opaque to this layer.

Object-specific canonicalization must occur before construction of the
domain-separated preimage.

Domain separation does not repair ambiguous serialization.

## Deterministic Failure Boundary

Phase 007 rejects:

- truncated preimages;
- invalid magic;
- unsupported versions;
- unregistered domains;
- impossible or unavailable declared lengths;
- trailing bytes.

Malformed authority-bearing bytes are rejected rather than normalized or
repaired through local guesswork.

## Conformance Evidence

The normative fixture is:

`conformance/crypto-registry-v1/crypto-registry-v1.json`

It contains:

- 2 registered semantic domains;
- 2 recognized hash suites;
- 3 canonical positive preimage vectors;
- 6 malformed or rejection vectors.

The repository-local Python generator independently reproduces the declared
preimage bytes without calling the Rust implementation.

The Rust integration suite verifies registry identity, canonical preimage
bytes, round trips, semantic-domain separation, and deterministic malformed
input rejection.

## Explicit Non-Scope

Phase 007 does not implement:

- SHA-256 execution;
- SHA-512 execution;
- Singularity Root Artifact serialization or commitment;
- Stable Authority Identity derivation;
- Authority Domain materialization;
- HashHelix authority;
- MMR or authenticated-history commitments;
- authenticated state;
- Proof Capsules;
- signatures or key management;
- hash-suite transition;
- dual-commitment migration;
- cross-domain coordination.

## Verification Performed

Phase 007 passed:

- Rust formatting;
- locked dependency metadata resolution;
- workspace check;
- complete workspace tests;
- Clippy with warnings denied;
- 4 Phase 007 Rust integration tests;
- Phase 004 regression audit;
- Phase 005 regression audit;
- Phase 006 regression audit;
- Phase 007 cryptographic registry audit;
- repository baseline audit;
- fixture regeneration;
- whitespace validation.

The `fme-crypto` crate has no production dependencies.

## Resulting Repository State

The Phase 007 implementation boundary was established by implementation merge:

`a237660a`

FME now has a deterministic byte boundary between canonical semantic objects
and future cryptographic digest execution.

This phase does not itself create cryptographic commitments.

## Anchor Discipline

The implementation merge anchor above was obtained from Git after the Phase 007
implementation pull request was squash-merged into `main`.

The documentation closeout was subsequently squash-merged into `main`.

The resulting documentation merge anchor is:

`8d49bf9e`

This value was obtained from Git after the documentation merge existed.

The documentation-anchor repair was subsequently squash-merged into `main`.

The resulting anchor-repair merge is:

`749fcd7f`

This value was obtained from Git after the anchor-repair merge existed.

The subsequent record-only backfill of this repair hash is bookkeeping and
does not create a fourth Phase 007 lifecycle anchor.

No implementation, documentation, or anchor-repair merge value was predicted
or fabricated.
