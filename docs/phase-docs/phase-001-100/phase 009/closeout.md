# Phase 009 — Baseline Singularity Root Artifact Canonical Serialization

Status: **SEALED**

Implementation merge anchor:

`ff1ca940`

Documentation merge anchor:

`4dab2f5b`

Anchor repair merge:

`5b2b547`

## Objective

Establish the first implemented baseline Singularity Root Artifact semantic
object and its exact canonical production wire representation.

Phase 009 defines the deterministic bytes representing the baseline SRA.

It does not assign cryptographic commitment meaning to those bytes.

## Implemented Scope

Phase 009 adds the `fme-sra` workspace crate and establishes:

- `SingularityRootArtifactV1`;
- fixed artifact version V1;
- required non-empty opaque system namespace bytes;
- fixed FER profile binding to `FME-FER-AFFINE-2D-BINARY-V1`;
- explicit registered default hash-suite identity;
- optional non-empty opaque root-seed bytes;
- exact canonical SRA wire encoding V1;
- deterministic canonical decoding;
- fail-closed malformed and noncanonical rejection;
- normative canonical-byte conformance vectors;
- repository-local secondary Python byte reproduction;
- Rust canonical and conformance tests;
- a dedicated Phase 009 audit;
- repository-baseline audit integration.

The canonicalization profile identifier is:

`FME-SRA-CANONICAL-V1`

The wire-format identifier is:

`FME-SRA-CANONICAL-V1-WIRE-V1`

## Baseline SRA Semantic Boundary

The Phase 009 baseline semantic SRA contains exactly:

- `artifact_version` — fixed to V1;
- `system_namespace` — required non-empty opaque bytes;
- `fer_profile_id` — fixed to `FME-FER-AFFINE-2D-BINARY-V1`;
- `default_hash_suite_id` — one registered hash-suite identifier;
- `root_seed` — optional opaque bytes which, when present, must be non-empty.

Architecture-level fields without a normative implemented contract are not
silently added or defaulted.

Their omission does not imply implementation.

## Canonical Wire Boundary

The canonical V1 byte sequence contains, in exact order:

1. four-byte magic `FMES`;
2. one-byte wire version `0x01`;
3. canonical-profile identifier length as unsigned big-endian `u64`;
4. exact canonical-profile identifier bytes;
5. one-byte artifact version `0x01`;
6. system-namespace length as unsigned big-endian `u64`;
7. exact system-namespace bytes;
8. FER-profile identifier length as unsigned big-endian `u64`;
9. exact FER-profile identifier bytes;
10. hash-suite identifier length as unsigned big-endian `u64`;
11. exact registered hash-suite identifier bytes;
12. one-byte root-seed presence value;
13. if present, root-seed length as unsigned big-endian `u64`;
14. if present, exact root-seed bytes.

Phase 009 recognizes exactly the already registered hash-suite identifiers:

- `SHA2-256`;
- `SHA2-512`.

The selected suite is represented in the semantic SRA.

Phase 009 does not hash the SRA bytes.

## Deterministic Failure Boundary

Phase 009 rejects:

- empty system namespace;
- present-but-empty root seed;
- truncated input;
- invalid magic;
- unsupported wire version;
- canonical-profile mismatch;
- unsupported artifact version;
- FER-profile mismatch;
- unknown hash-suite identifier;
- invalid root-seed presence values;
- lengths that exceed implementation limits;
- trailing bytes.

Malformed or noncanonical SRA bytes are rejected rather than normalized,
repaired, or inferred through local policy.

## Conformance Evidence

The normative fixture is:

`conformance/sra-canonical-v1/sra-canonical-v1.json`

It contains:

- 4 normative canonical SRA vectors;
- 11 malformed or rejection vectors.

The repository-local Python generator:

`scripts/conformance/generate_sra_canonical_v1.py`

reconstructs the canonical wire bytes without invoking the Rust encoder.

Its provenance explicitly records that this is a secondary repository-local
reference implementation and not independent external reproduction.

The Rust conformance suite verifies fixture metadata, canonical bytes,
decode/re-encode equality, semantic round trips, and deterministic malformed
input rejection.

## Cryptographic Boundary

The Phase 009 pipeline is:

`SRA semantic object -> Phase 009 canonical SRA bytes`

Accordingly:

`serialization != commitment`

and:

`digest execution != commitment semantics`

A later separately versioned profile may consume the canonical SRA bytes and
assign explicit cryptographic commitment meaning.

## Explicit Non-Scope

Phase 009 does not implement:

- SRA cryptographic commitment;
- Stable SRA identity derivation;
- Stable Authority Identity derivation;
- Authority Domain materialization;
- Primary Trunk operational authority;
- structural transition encoding;
- accepted-history leaves;
- MMR history commitments;
- authenticated derived state;
- Authority Domain commitments;
- Proof Capsules;
- HashHelix operational authority or integration;
- authorization;
- signatures or key management;
- cross-domain coordination;
- cross-domain atomicity.

Successful SRA decoding does not create operational authority.

Accordingly:

`root artifact != operational trunk`

and:

`mathematical possibility != materialization`

## Implementation Pipeline Roadmap

Phase 009 also introduces:

`docs/phase-docs/phase-001-100/pipeline/pipeline-009-020.md`

That file records the candidate implementation direction through Phase 020.

It is explicitly a planning surface and is not implementation authority.

Every listed future phase remains subject to clean-main verification, prior
closeout review, targeted reconnaissance, whitepaper grounding, and a bounded
phase contract before implementation begins.

## Verification Performed

Phase 009 passed:

- locked dependency metadata resolution;
- Rust formatting;
- complete workspace check;
- complete workspace tests;
- Clippy across all targets with warnings denied;
- 17 canonical SRA Rust tests;
- 3 SRA conformance tests;
- Phase 009 canonical serialization audit;
- regression audits for preceding implemented phases;
- repository baseline audit;
- secondary fixture regeneration;
- whitespace validation;
- clean Python cache verification.

The Phase 009 audit specifically verified:

- independent reconstruction of all 4 normative canonical vectors;
- exact coverage of all 11 malformed/rejection vectors;
- Rust/spec/test/fixture boundary alignment;
- canonical serialization and commitment separation.

## Resulting Repository State

The Phase 009 implementation boundary was established by implementation merge:

`ff1ca940`

FME now has the deterministic pipeline:

`baseline SRA semantic object -> Phase 009 canonical SRA serialization -> canonical SRA semantic bytes`

Those bytes do not yet possess SRA commitment semantics.

## Anchor Discipline

The implementation merge anchor above was obtained from Git after the Phase 009
implementation pull request was squash-merged into `main`.

The documentation closeout was subsequently squash-merged into `main`.

The resulting documentation merge anchor is:

`4dab2f5b`

This value was obtained from Git after the documentation merge existed.

The documentation-anchor repair was subsequently squash-merged into `main`.

The resulting anchor-repair merge is:

`5b2b547`

This value was obtained from Git after the anchor-repair merge existed.

This record-only backfill is bookkeeping and does not create a fourth Phase 009
lifecycle anchor.

No implementation, documentation, or anchor-repair merge value was predicted
or fabricated.
