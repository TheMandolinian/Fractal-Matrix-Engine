# Phase 008 — Baseline Registered SHA-2 Digest Execution

Status: **IN PROGRESS**

Implementation merge anchor:

`443275a8`

Documentation merge anchor:

`—`

Anchor repair merge:

`—`

## Objective

Establish deterministic execution of the hash suites registered by Phase 007
over exact validated `FME-DOMAIN-PREIMAGE-V1` bytes.

Phase 007 defines the canonical bytes that enter the cryptographic layer.

Phase 008 defines how those exact bytes are processed by the registered
SHA-2 algorithms without assigning higher-level commitment meaning to the
resulting digest.

## Implemented Scope

Phase 008 establishes:

- deterministic `SHA2-256` execution;
- deterministic `SHA2-512` execution;
- explicit registered hash-suite selection;
- typed digest output carrying explicit suite identity;
- fail-closed validation of Phase 007 preimages before digest execution;
- normative external digest conformance vectors;
- independent Python `hashlib` reproduction;
- Rust integration tests;
- a dedicated Phase 008 repository audit;
- repository-baseline integration.

The Phase 008 digest profile identifier is:

`FME-REGISTERED-SHA2-DIGEST-V1`

The production cryptographic dependency is narrowly pinned as:

`sha2 = { version = "=0.11.0", default-features = false }`

## Registered Digest Execution

Phase 008 executes exactly the suites already registered by Phase 007:

- `SHA2-256`;
- `SHA2-512`.

`SHA2-256` produces exactly 32 digest bytes.

`SHA2-512` produces exactly 64 digest bytes.

Hash-suite identity remains explicit and must not be inferred from digest
width.

For a valid Phase 007 preimage `P`:

```text
SHA2-256 result = SHA-256(P)
SHA2-512 result = SHA-512(P)
```

The complete exact preimage is processed.

The decoded payload alone is not the digest input.

Hash-suite identity is not inserted into the Phase 007 preimage.

The same canonical preimage may therefore be processed independently under
both registered suites.

## Validation and Failure Boundary

The authority-facing digest operation validates supplied bytes through the
Phase 007 canonical preimage decoder before digest execution.

Malformed or nonconformant preimages therefore fail before hashing.

This preserves deterministic rejection for:

- invalid magic;
- unsupported versions;
- unregistered domains;
- truncation;
- impossible or unavailable declared lengths;
- trailing bytes.

No local normalization or guesswork repairs malformed digest input.

## Domain Separation

Equal canonical payload bytes framed under different registered semantic
domains produce different Phase 007 preimages.

Under the same registered hash suite, those different domain-separated
preimages produce different digest results.

Phase 008 does not perform object-specific canonicalization.

## Conformance Evidence

The normative Phase 008 fixture is:

`conformance/crypto-digest-v1/crypto-digest-v1.json`

It contains six digest vectors:

- three canonical Phase 007 preimages processed under `SHA2-256`;
- the same three canonical Phase 007 preimages processed under `SHA2-512`.

The repository-local Python generator independently reconstructs the Phase 007
preimage framing and computes expected digest values using Python `hashlib`.

The Rust implementation consumes those independently produced expected values.

The Rust integration suite verifies:

- all six registered SHA-2 vectors;
- explicit suite identity and digest width;
- deterministic repeated execution;
- the same preimage under both registered suites;
- semantic-domain digest separation;
- rejection of all Phase 007 malformed-preimage vectors before digest execution.

## Phase 007 Compatibility

Phase 007 originally had no production cryptographic dependency because it did
not execute digest algorithms.

Phase 008 legitimately introduces the pinned SHA-2 implementation dependency.

The Phase 007 audit therefore continues to enforce its actual architectural
boundary: canonical preimage construction remains independent from digest
execution and does not itself execute SHA-256 or SHA-512.

Phase 007 canonical preimage bytes and registry identifiers remain unchanged.

## Explicit Non-Scope

Phase 008 does not implement:

- new semantic cryptographic domains;
- Singularity Root Artifact serialization;
- Singularity Root Artifact commitment;
- Stable Authority Identity derivation;
- Authority Domain materialization;
- HashHelix authority or integration;
- MMR or authenticated-history commitments;
- authenticated state;
- FER cryptographic binding;
- Proof Capsules;
- signatures;
- public or private keys;
- key storage or key management;
- authorization;
- hash-suite transition;
- dual-commitment migration;
- cryptographic migration;
- cross-domain coordination;
- cross-domain atomicity.

A Phase 008 digest is not automatically an SRA commitment, Authority Domain
identity, history root, state root, Proof Capsule, or HashHelix commitment.

Those meanings require separately specified later profiles.

## Verification Performed

Phase 008 passed:

- Rust formatting;
- locked workspace dependency resolution;
- complete workspace check;
- complete workspace tests;
- Clippy across all targets with warnings denied;
- 4 Phase 008 Rust integration tests;
- Phase 007 regression audit;
- Phase 008 registered SHA-2 digest audit;
- repository baseline audit;
- independent fixture regeneration;
- whitespace validation.

The Phase 008 audit specifically verified:

- the six-vector profile structure;
- independent SHA2-256 and SHA2-512 reproduction;
- domain-separated digest distinction;
- preservation of Phase 007 preimage bytes;
- validation before digest execution;
- the pinned production dependency boundary.

## Resulting Repository State

The Phase 008 implementation boundary was established by implementation merge:

`443275a8`

FME now has the deterministic pipeline:

```text
canonical semantic payload
    ↓
Phase 007 domain-separated canonical preimage
    ↓
explicit registered hash suite
    ↓
Phase 008 deterministic digest bytes
```

The resulting digest has no higher-level commitment meaning unless a later
profile explicitly defines that meaning.

## Anchor Discipline

The implementation merge anchor above was obtained from Git after the Phase
008 implementation pull request was squash-merged into `main`.

The documentation merge does not yet exist.

Therefore the documentation merge anchor remains:

`—`

The anchor-repair merge also does not yet exist.

Therefore the anchor-repair merge remains:

`—`

After this documentation closeout is squash-merged, its actual Git-derived
documentation merge hash must be recorded on a dedicated anchor-repair branch.

No documentation or anchor-repair hash may be predicted or fabricated.

The phase remains **IN PROGRESS** until the lifecycle-anchor repair is
completed.
