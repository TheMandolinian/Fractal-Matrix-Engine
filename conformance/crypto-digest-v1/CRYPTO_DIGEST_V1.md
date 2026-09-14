# FME Registered SHA-2 Digest Profile V1

Profile identifier:

`FME-REGISTERED-SHA2-DIGEST-V1`

## Purpose

This profile defines deterministic execution of the hash suites registered by
Phase 007 over exact validated `FME-DOMAIN-PREIMAGE-V1` bytes.

Phase 007 defines the canonical domain-separated bytes.

Phase 008 defines how those exact bytes are processed by the registered SHA-2
algorithms.

This profile does not assign higher-level object semantics to the resulting
digest.

## Registered suites

The profile executes exactly the Phase 007 registered suites:

- `SHA2-256`
- `SHA2-512`

The registry identifiers remain authoritative.

`SHA2-256` produces exactly 32 digest bytes.

`SHA2-512` produces exactly 64 digest bytes.

Hash-suite identity is explicit and must not be inferred from digest width.

## Digest input

The complete validated Phase 007 preimage is the digest input.

Its format remains:

```text
magic          4 bytes   "FMEP"
version        1 byte    0x01
domain_length  8 bytes   unsigned big-endian
domain         N bytes   exact registered domain bytes
payload_length 8 bytes   unsigned big-endian
payload        M bytes   already-canonical payload
```

Phase 008 does not modify this format.

In particular, hash-suite identity is not inserted into the Phase 007
preimage.

Therefore the same canonical preimage may be processed independently by both
registered suites.

## Execution

For a valid Phase 007 preimage `P`:

```text
SHA2-256 result = SHA-256(P)
SHA2-512 result = SHA-512(P)
```

The entire preimage `P` is processed.

The decoded payload alone is not the digest input.

## Validation boundary

The authority-facing digest operation first validates the supplied bytes using
the Phase 007 canonical preimage decoder.

Malformed or nonconformant preimages fail before registered digest execution.

Phase 008 therefore preserves the Phase 007 rejection boundary for:

- invalid magic;
- unsupported version;
- unknown registered domain;
- truncation;
- impossible or unavailable declared lengths;
- trailing bytes.

No local normalization or guesswork repairs malformed digest input.

## Domain separation

Equal payload bytes framed under different registered semantic domains produce
different Phase 007 preimages and therefore different registered digest
results under the same hash suite.

The cryptographic digest layer does not itself decide object-specific
canonicalization.

## Suite identity

A digest result remains associated with the registered suite that produced it.

Digest length is not an authority mechanism for discovering suite identity.

A verifier must receive or derive suite identity from the applicable declared
cryptographic context.

## Conformance evidence

The normative Phase 008 fixture is:

`crypto-digest-v1.json`

It contains six integration vectors:

- three canonical Phase 007 preimages processed under `SHA2-256`;
- the same three canonical Phase 007 preimages processed under `SHA2-512`.

The fixture generator reconstructs Phase 007 preimages independently in
Python and computes expected digests with Python `hashlib`.

The Rust implementation does not produce the fixture values used as its own
expected answers.

## Explicit non-scope

This profile does not implement or define:

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

A digest produced under this profile is not automatically an SRA commitment,
Authority Domain identity, history root, state root, Proof Capsule, or
HashHelix commitment.

Those meanings require separately specified later profiles.

## Governing boundary

The Phase 008 rule is:

```text
Phase 007 defines what bytes are hashed.
Phase 008 defines how those bytes are hashed.
Later profiles define what the resulting digest means.
```
