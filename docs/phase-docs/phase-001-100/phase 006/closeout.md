# Phase 006 — Baseline FER Canonical Encoding

Status: **SEALED**

Implementation merge anchor:

`7322debe`

Documentation merge anchor:

`c45746aa`

Anchor repair merge:

`cd289812`

## Objective

Establish the first normative canonical production wire encoding for the
implemented baseline FER profile:

`FME-FER-AFFINE-2D-BINARY-V1`

Phase 006 closes the previously explicit gap between logical FER values and
authority-bearing serialized bytes.

## Implemented Scope

Phase 006 establishes Canonical Wire V1 for:

- `BaselinePath`;
- `ExactState`;
- explicit wire-version identification;
- explicit FER-profile binding;
- distinct path and exact-state object kinds;
- fixed-width unsigned big-endian length and depth fields;
- MSB-first binary path packing;
- authority-relevant path length;
- zeroed unused path bits;
- canonical arbitrary-precision signed integer representation;
- canonical zero representation;
- minimal unsigned big-endian integer magnitudes;
- deterministic malformed and noncanonical rejection;
- exact decode/re-encode round trips.

The normative wire-format identifier is:

`FME-FER-AFFINE-2D-BINARY-V1-WIRE-V1`

## Canonical Path Boundary

The authority-bearing baseline path encoding contains:

1. common Canonical Wire V1 header;
2. unsigned big-endian `u64` bit length;
3. exactly the required packed path bytes.

Path bits are packed most-significant-bit first.

Unused low-order bits in a partial final byte must be zero.

The root path has zero bit length and zero packed bytes.

Human-readable paths remain diagnostic and fixture inputs rather than
authority-bearing serialization.

## Canonical Exact-State Boundary

The exact FER state encoding contains:

1. common Canonical Wire V1 header;
2. unsigned big-endian `u64` depth;
3. canonical signed integer `P`;
4. canonical signed integer `Q`.

Canonical signed integers use:

- `0x00` for zero;
- `0x01` for positive;
- `0x02` for negative;
- unsigned big-endian magnitude bytes;
- shortest possible nonzero magnitude representation.

Canonical zero has zero magnitude length and no magnitude bytes.

Positive zero, negative zero, leading-zero magnitudes, unsupported signs,
truncated objects, profile mismatches, nonzero path padding, and trailing bytes
are rejected.

## Conformance Evidence

Phase 006 adds a separate normative wire fixture:

`conformance/fer-affine-2d-binary-v1/canonical-wire-v1.json`

The corpus contains:

- 8 canonical path vectors;
- 8 canonical exact-state vectors;
- 14 malformed or noncanonical rejection vectors.

A repository-local Python byte generator reproduces the declared canonical
bytes independently of the Rust serialization implementation path.

This provides secondary implementation separation.

It does **not** constitute independent reproduction by an independently
operated implementation.

The earlier Phase 004 mathematical fixture remains explicitly non-normative
with respect to production wire serialization.

## Deterministic Failure Boundary

Canonical Wire V1 decoding is fail-closed.

Malformed, ambiguous, unsupported, truncated, noncanonical, or incorrectly
profile-bound wire input is rejected rather than repaired through local
guesswork.

## Cryptographic Boundary

Phase 006 defines canonical bytes only.

It does not:

- hash those bytes;
- define cryptographic domain separation;
- derive Stable Authority Identity;
- implement the Singularity Root Artifact;
- implement MMR history commitments;
- implement authenticated state;
- implement Proof Capsules;
- implement Authority Domain materialization;
- implement HashHelix event authority;
- implement cross-domain coordination;
- implement multidimensional FER profiles.

Canonical serialization is therefore a prerequisite for later cryptographic
identity and commitment work, not an implementation of those mechanisms.

## Verification Performed

Phase 006 passed:

- Rust formatting;
- locked dependency metadata resolution;
- workspace check;
- complete workspace tests;
- 11 existing FER conformance tests;
- 4 Canonical Wire V1 integration tests;
- Clippy with warnings denied;
- Phase 004 regression audit;
- Phase 005 regression audit;
- Phase 006 canonical encoding audit;
- repository baseline audit;
- canonical fixture reproducibility;
- whitespace validation.

No new production dependency was added.

## Resulting Repository State

The Phase 006 implementation boundary was established by implementation merge:

`7322debe`

The baseline FER profile now has a declared, versioned, deterministic canonical
wire representation for paths and exact states.

This does not expand FER topology authority into application behavior and does
not alter the exact FER recurrence mathematics established by earlier phases.

## Anchor Discipline

The implementation merge anchor above was obtained from Git after the Phase 006
implementation pull request was squash-merged into `main`.

The documentation closeout was subsequently squash-merged into `main`.

The resulting documentation merge anchor is:

`c45746aa`

This value was obtained from Git after the documentation merge existed.

The documentation-anchor repair was subsequently squash-merged into `main`.

The resulting anchor-repair merge is:

`cd289812`

This value was obtained from Git after the anchor-repair merge existed.

No implementation, documentation, or anchor-repair merge value was predicted
or fabricated.
