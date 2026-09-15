# FER Profile Status

## FME-FER-AFFINE-2D-BINARY-V1

Status: **implementation research / not production normative**

Currently targeted:

- exact two-dimensional baseline recurrence;
- binary logical path semantics;
- arbitrary-precision integer numerators;
- deterministic root evaluation;
- deterministic continuation;
- implementation conformance vectors.

Frozen and implemented for the baseline FER profile:

- normative canonical wire encoding for baseline logical paths;
- normative canonical wire encoding for exact baseline `P`, `Q`, and depth states.

Not yet frozen beyond that baseline wire profile:

- general topology-address descriptor wire encoding;
- additional coordinate or profile representations;
- maximum authoritative topology depth;
- production resource limits;
- profile registry representation;
- cryptographic binding;
- profile-transition wire semantics.

Implementation convenience must not silently define any unresolved normative
profile rule.

## FME-SRA-COMMITMENT-V1

Status: **implemented baseline commitment profile / not operational authority**

Phase 010 freezes and implements:

- commitment over exact `FME-SRA-CANONICAL-V1` bytes;
- semantic domain `FME/SINGULARITY/V1`;
- hash-suite selection from the SRA's declared `default_hash_suite_id`;
- registered `SHA2-256` and `SHA2-512` digest execution;
- typed `SraCommitmentV1` output;
- deterministic Rust and secondary Python conformance evidence.

The commitment profile does not define:

- Stable SRA identity;
- Stable Authority Identity;
- Authority Domain materialization;
- Primary Trunk authority;
- accepted-history commitments;
- MMR history;
- authenticated state;
- Proof Capsules;
- authorization;
- cross-domain coordination.

`SRA commitment != SRA stable identity`

`root artifact != operational trunk`
