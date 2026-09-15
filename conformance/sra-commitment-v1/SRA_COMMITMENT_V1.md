# FME SRA Commitment Profile V1

Profile identifier:

`FME-SRA-COMMITMENT-V1`

## Scope

This profile assigns cryptographic commitment semantics to the canonical
baseline Singularity Root Artifact bytes defined by:

`FME-SRA-CANONICAL-V1`

It composes existing FME contracts rather than defining a second
serialization, preimage, or digest path.

## Commitment Pipeline

For a valid `SingularityRootArtifactV1`:

1. encode the artifact using the Phase 009 canonical SRA encoder;
2. construct a Phase 007 domain-separated preimage using:
   `FME/SINGULARITY/V1`;
3. select the artifact's declared `default_hash_suite_id`;
4. execute that registered suite through the Phase 008 digest profile;
5. interpret the resulting typed digest as the Phase 010 SRA commitment.

Conceptually:

`SRA semantic object`
→ `canonical SRA bytes`
→ `FME/SINGULARITY/V1 preimage`
→ `declared registered hash suite`
→ `SRA commitment`

## Hash-Suite Binding

The baseline SRA commitment uses the hash suite declared by the SRA itself.

Phase 010 does not accept an independent suite override during commitment
generation.

Supported suites remain exactly:

- `SHA2-256`;
- `SHA2-512`.

Hash-suite identity must remain explicit and must not be inferred from digest
length.

## Domain Binding

The semantic domain is exactly:

`FME/SINGULARITY/V1`

No other registered domain produces a conformant Phase 010 SRA commitment.

## Canonicality

Phase 010 does not redefine SRA canonical serialization.

The committed payload is exactly the canonical byte sequence produced by the
Phase 009 SRA profile.

Phase 010 does not normalize, repair, reinterpret, or independently rebuild
malformed SRA bytes.

## Result Semantics

The resulting commitment is interpreted under:

- commitment profile `FME-SRA-COMMITMENT-V1`;
- domain `FME/SINGULARITY/V1`;
- the SRA-declared registered hash suite;
- the resulting digest bytes.

The commitment does not create operational authority.

Accordingly:

`serialization != commitment`

`digest != commitment semantics`

`SRA commitment != SRA stable identity`

`root artifact != operational trunk`

`mathematical possibility != materialization`

## Conformance Fixture

The normative fixture is:

`conformance/sra-commitment-v1/sra-commitment-v1.json`

Each vector exposes:

- hash-suite identity;
- canonical Phase 009 SRA bytes;
- exact Phase 007 domain-separated preimage bytes;
- expected digest bytes.

The repository-local Python generator is:

`scripts/conformance/generate_sra_commitment_v1.py`

It reconstructs the SRA bytes, domain-separated preimage, and SHA-2 digest
without invoking the Rust implementation.

Its provenance is a secondary repository-local reference implementation, not
independent external reproduction.

## Explicit Non-Scope

This profile does not define:

- SRA Stable Identity;
- Authority Domain identity;
- Authority Domain materialization;
- Primary Trunk authority;
- accepted-history commitments;
- MMR history;
- authenticated derived state;
- Authority Domain commitments;
- Proof Capsules;
- signatures or key management;
- authorization;
- cross-domain coordination;
- cross-domain atomicity.
