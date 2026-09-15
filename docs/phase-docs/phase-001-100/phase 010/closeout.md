# Phase 010 — Baseline SRA Commitment Profile

Status: **DOCUMENTATION CLOSEOUT IN PROGRESS**

Implementation merge anchor:

`96276106`

Documentation merge anchor:

`—`

Anchor repair merge:

`—`

## Objective

Assign explicit cryptographic commitment semantics to the canonical baseline
Singularity Root Artifact bytes established by Phase 009.

Phase 010 composes the existing canonical serialization, domain-preimage, and
registered digest contracts rather than introducing a second cryptographic path.

## Implemented Scope

Phase 010 establishes:

- commitment profile `FME-SRA-COMMITMENT-V1`;
- typed `SraCommitmentV1`;
- deterministic `commit_sra` generation;
- fixed semantic domain `FME/SINGULARITY/V1`;
- hash-suite selection from the SRA's declared `default_hash_suite_id`;
- SHA2-256 and SHA2-512 commitment execution through the existing registered
  digest implementation;
- explicit commitment profile, domain, hash-suite, and digest accessors;
- Rust unit coverage;
- Rust conformance-fixture consumption;
- repository-local Python secondary reproduction;
- a dedicated Phase 010 audit;
- repository-baseline audit integration.

## Commitment Pipeline

The implemented pipeline is:

`SingularityRootArtifactV1`

→ Phase 009 `encode_sra`

→ canonical `FME-SRA-CANONICAL-V1` bytes

→ Phase 007 `FME/SINGULARITY/V1` domain-separated preimage

→ SRA-declared registered hash suite

→ Phase 008 registered digest execution

→ typed Phase 010 SRA commitment

Phase 010 reuses the existing Phase 007 and Phase 008 cryptographic contracts.

It does not duplicate SRA serialization, preimage construction, or SHA-2
execution inside `fme-sra`.

## Hash-Suite Binding

The commitment hash suite is selected from the canonical SRA's declared
`default_hash_suite_id`.

The implemented suites remain exactly:

- `SHA2-256`;
- `SHA2-512`.

Phase 010 does not provide an independent hash-suite override during commitment
generation.

Hash-suite identity remains explicit and is not inferred from digest length.

## Domain Binding

The commitment domain is exactly:

`FME/SINGULARITY/V1`

Phase 010 does not introduce another Singularity commitment domain or locally
construct an alternate preimage format.

## Commitment Semantics

The resulting value is interpreted under:

- profile `FME-SRA-COMMITMENT-V1`;
- domain `FME/SINGULARITY/V1`;
- the SRA-declared registered hash suite;
- the resulting digest bytes.

Accordingly:

`serialization != commitment`

`preimage != digest`

`digest != commitment semantics`

`SRA commitment != SRA stable identity`

`root artifact != operational trunk`

`mathematical possibility != materialization`

## Conformance Evidence

The normative Phase 010 fixture is:

`conformance/sra-commitment-v1/sra-commitment-v1.json`

It contains two commitment vectors covering:

- SHA2-256 without a root seed;
- SHA2-512 with a root seed.

Each vector exposes:

- explicit hash-suite identity;
- canonical Phase 009 SRA bytes;
- exact Phase 007 domain-separated preimage bytes;
- expected Phase 008 digest bytes.

The repository-local secondary generator is:

`scripts/conformance/generate_sra_commitment_v1.py`

It reconstructs the canonical SRA bytes, domain-separated preimage, and SHA-2
digest without invoking the Rust implementation.

Its provenance explicitly records that it is not independent external
reproduction.

## Verification Performed

Phase 010 passed:

- Rust formatting;
- `fme-sra` tests;
- complete workspace check;
- complete workspace tests;
- Clippy across all targets with warnings denied;
- deterministic Phase 010 fixture regeneration;
- Phase 009 regression audit;
- Phase 010 commitment-profile audit;
- repository baseline audit;
- staged whitespace validation.

The complete workspace Rust suite passed 47 tests with zero failures.

The dedicated Phase 010 audit verified:

- independent Python reconstruction of both commitment vectors;
- exact canonical SRA bytes;
- exact domain-separated preimage bytes;
- exact SHA2-256 and SHA2-512 digest bytes;
- Rust/spec/test/fixture alignment;
- serialization/preimage/digest/commitment boundary preservation.

## Phase 009 Compatibility

Phase 009 remains the canonical SRA serialization authority.

Its serialization implementation continues to prohibit local hashing or
commitment behavior inside the Phase 009 serialization path.

The Phase 009 audit was updated only where its crate-level documentation
expectation became stale after Phase 010 legitimately added commitment support
to `fme-sra`.

The Phase 009 serialization/commitment separation audit remains green.

## Explicit Non-Scope

Phase 010 does not implement:

- Stable SRA identity derivation;
- Stable Authority Identity derivation;
- Authority Domain materialization;
- Primary Trunk operational authority;
- structural transition semantics;
- accepted-history leaves;
- MMR history commitments;
- authenticated derived state;
- Authority Domain commitments;
- Proof Capsules;
- signatures or key management;
- authorization;
- cross-domain coordination;
- cross-domain atomicity.

Phase 011 remains the candidate boundary for SRA Stable Identity Derivation.

## Resulting Repository State

The Phase 010 implementation boundary was established by implementation merge:

`96276106`

FME now has the deterministic pipeline:

`baseline SRA semantic object`

→ `canonical SRA serialization`

→ `FME/SINGULARITY/V1 preimage`

→ `registered digest execution`

→ `typed SRA commitment`

The resulting commitment does not create stable SRA identity or operational
authority.

## Anchor Discipline

The implementation merge anchor above was obtained from Git after the Phase 010
implementation pull request was squash-merged into `main`.

The documentation merge does not yet exist.

Therefore the documentation merge anchor remains:

`—`

The anchor-repair merge also does not yet exist and remains:

`—`

Neither value will be predicted or fabricated.

After this documentation closeout is squash-merged, the real documentation
merge hash must be read from updated `main`.

A dedicated anchor-repair branch will then record that real documentation
anchor and seal the Phase 010 lifecycle.
