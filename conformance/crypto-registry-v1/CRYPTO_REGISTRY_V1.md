# FME Cryptographic Registry and Domain Preimage V1

## Status

Phase 007 normative baseline.

This specification defines:

- the initial FME cryptographic-domain registry;
- the initial recognized hash-suite identifiers;
- the canonical domain-separated preimage framing used before digest execution.

It does not define or execute a cryptographic digest algorithm.

## Profile identifiers

Domain registry:

`FME-CRYPTO-DOMAIN-REGISTRY-V1`

Hash-suite registry:

`FME-HASH-SUITE-REGISTRY-V1`

Domain-preimage profile:

`FME-DOMAIN-PREIMAGE-V1`

## Registered semantic domains

Phase 007 registers exactly two semantic domains:

| Semantic domain | Exact identifier bytes |
|---|---|
| Singularity Root Artifact commitment | `FME/SINGULARITY/V1` |
| Authority Domain identity/commitment | `FME/AUTHORITY_DOMAIN/V1` |

The byte strings are authoritative exactly as shown.

Capitalization, separators, underscores, and version suffixes are significant.

Unregistered or differently encoded domain identifiers are invalid.

## Recognized hash-suite identifiers

Phase 007 recognizes exactly:

| Suite | Exact identifier bytes | Digest width |
|---|---|---:|
| SHA2-256 | `SHA2-256` | 32 bytes |
| SHA2-512 | `SHA2-512` | 64 bytes |

Recognition does not mean Phase 007 executes either digest algorithm.

Hash-suite identity is not embedded inside the domain-separated preimage.

A later commitment profile selects the registered hash suite that processes
the exact preimage bytes.

## Domain-separated preimage encoding

The canonical encoding is:

    magic          4 bytes   ASCII "FMEP"
    version        1 byte    0x01
    domain_length  8 bytes   unsigned big-endian
    domain         N bytes   exact registered domain identifier
    payload_length 8 bytes   unsigned big-endian
    payload        M bytes   already-canonical payload

There are no optional fields, terminators, padding bytes, or trailing bytes.

Lengths are unsigned 64-bit integers encoded in network byte order.

For one registered domain and one payload byte sequence there is exactly one
Phase 007 preimage encoding.

## Canonical payload boundary

The payload is opaque to this framing layer.

Phase 007 does not decide whether an object-specific payload is canonical.
The producer must supply bytes already canonicalized by the applicable object
or wire profile.

Domain separation does not repair ambiguous object serialization.

## Deterministic rejection

A decoder must reject:

- truncated input;
- incorrect magic;
- unsupported preimage version;
- unregistered domain identifier;
- declared lengths exceeding implementation limits;
- declared lengths exceeding available input;
- trailing bytes.

No local guesswork or normalization may repair malformed authority-bearing
input.

## Explicit non-scope

Phase 007 does not implement:

- SHA-256 execution;
- SHA-512 execution;
- Singularity Root Artifact serialization;
- Singularity Root Artifact commitment;
- Stable Authority Identity derivation;
- Authority Domain materialization;
- history or MMR commitments;
- authenticated state;
- Proof Capsules;
- signatures or key management;
- hash-suite transition;
- dual-commitment migration;
- cross-domain coordination.
