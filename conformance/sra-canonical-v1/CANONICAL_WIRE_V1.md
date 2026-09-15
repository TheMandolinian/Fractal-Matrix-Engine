# Singularity Root Artifact Canonical Wire Encoding V1

This document defines the canonical production wire encoding introduced for the
baseline Singularity Root Artifact:

`SingularityRootArtifactV1`

Canonicalization profile:

`FME-SRA-CANONICAL-V1`

Wire format identifier:

`FME-SRA-CANONICAL-V1-WIRE-V1`

Wire version:

`1`

This specification applies only to the baseline SRA semantic object implemented
by Phase 009.

It does not define:

- SRA cryptographic commitments;
- Stable SRA identity derivation;
- Authority Domain materialization;
- Primary Trunk operational authority;
- Stable Authority Domain identity;
- structural transition encoding;
- accepted-history leaves;
- MMR commitments;
- authenticated derived state;
- Authority Domain commitments;
- Proof Capsules;
- HashHelix operational event encoding;
- cross-domain coordination.

## 1. Canonical Object Boundary

The baseline V1 SRA contains exactly:

- `artifact_version` — fixed to V1;
- `system_namespace` — required non-empty opaque bytes;
- `fer_profile_id` — fixed to `FME-FER-AFFINE-2D-BINARY-V1`;
- `default_hash_suite_id` — one registered hash-suite identifier;
- `root_seed` — optional opaque bytes which, when present, MUST be non-empty.

The following architecture-level fields are not part of the Phase 009 baseline
wire object because no normative implemented contract currently exists for them:

- HashHelix profile identifiers;
- topology policy identifiers;
- identity policy identifiers;
- materialization policy identifiers;
- authorization policy identifiers;
- archive policy identifiers;
- structural-event profile identifiers;
- creation metadata.

Their omission is intentional and MUST NOT be interpreted as implicit default
values.

## 2. Integer Byte Order and Lengths

All fixed-width integer lengths in SRA Canonical Wire V1 are unsigned
big-endian integers.

Every variable-length field uses an eight-byte unsigned `u64` length.

No byte-order mark, string terminator, whitespace, native Rust layout,
implicit padding, or platform-dependent length encoding is permitted.

## 3. Common Prefix

Every SRA Canonical Wire V1 object begins with:

| Field | Encoding |
|---|---|
| magic | four bytes: ASCII `FMES` |
| wire_version | one byte: `0x01` |
| canonical_profile_id_length | unsigned `u64`, big-endian |
| canonical_profile_id | exact ASCII bytes `FME-SRA-CANONICAL-V1` |
| artifact_version | one byte: `0x01` |

Any other magic, unsupported wire version, canonicalization profile, or
artifact version MUST be rejected.

## 4. System Namespace

After the common prefix:

1. `system_namespace_length` — unsigned `u64`, big-endian;
2. `system_namespace` — exactly that many opaque bytes.

The namespace length MUST be greater than zero.

The namespace bytes are opaque to this serialization profile. The encoder MUST
preserve them exactly and MUST NOT perform text normalization, case conversion,
Unicode normalization, trimming, or other local repair.

## 5. FER Profile Identifier

After the system namespace:

1. `fer_profile_id_length` — unsigned `u64`, big-endian;
2. `fer_profile_id` — exact ASCII bytes:

   `FME-FER-AFFINE-2D-BINARY-V1`

Phase 009 supports exactly this FER profile identifier.

A different identifier MUST be rejected rather than guessed, rewritten, or
mapped locally.


## 6. Default Hash-Suite Identifier

After the FER profile identifier:

1. `default_hash_suite_id_length` — unsigned `u64`, big-endian;
2. `default_hash_suite_id` — exact registered hash-suite identifier bytes.

Phase 009 permits exactly:

- `SHA2-256`;
- `SHA2-512`.

The identifier is interpreted through the repository's registered hash-suite
contract.

An unknown, malformed, or unregistered identifier MUST be rejected.

The presence of a hash-suite identifier in the SRA does not cause Phase 009 to
hash the SRA bytes. It records the cryptographic suite selected by the semantic
root artifact for later profiles that explicitly consume it.

## 7. Root Seed

After the default hash-suite identifier:

1. `root_seed_presence` — one byte.

The only valid values are:

- `0x00` — root seed absent;
- `0x01` — root seed present.

If `root_seed_presence` is `0x00`:

- no root-seed length follows;
- no root-seed bytes follow.

If `root_seed_presence` is `0x01`:

1. `root_seed_length` — unsigned `u64`, big-endian;
2. `root_seed` — exactly that many opaque bytes.

A present root seed MUST have length greater than zero.

Therefore:

- absent seed is valid;
- present non-empty seed is valid;
- present empty seed is invalid;
- any presence value other than `0x00` or `0x01` is invalid.

Root-seed bytes are opaque to this serialization profile and MUST be preserved
exactly.

Phase 009 does not define what later topology, identity, commitment, or
materialization profiles may do with the root seed.

## 8. Complete Canonical Field Order

The complete V1 byte sequence is exactly:

1. four-byte magic `FMES`;
2. one-byte wire version `0x01`;
3. canonical-profile identifier length as unsigned big-endian `u64`;
4. exact canonical-profile identifier bytes `FME-SRA-CANONICAL-V1`;
5. one-byte artifact version `0x01`;
6. system-namespace length as unsigned big-endian `u64`;
7. exact system-namespace bytes;
8. FER-profile identifier length as unsigned big-endian `u64`;
9. exact FER-profile identifier bytes `FME-FER-AFFINE-2D-BINARY-V1`;
10. hash-suite identifier length as unsigned big-endian `u64`;
11. exact registered hash-suite identifier bytes;
12. one-byte root-seed presence value;
13. if present, root-seed length as unsigned big-endian `u64`;
14. if present, exact root-seed bytes.

Fields MUST appear in this order.

No maps, tags, implicit field discovery, implementation-native struct layout, or
unordered serialization participates in the authoritative encoding.

## 9. Canonical Parsing

SRA Canonical Wire V1 parsing is fail-closed.

A decoder MUST reject:

- truncated input;
- invalid magic;
- unsupported wire version;
- canonicalization-profile mismatch;
- unsupported artifact version;
- empty system namespace;
- FER-profile mismatch;
- unknown hash-suite identifier;
- invalid root-seed presence value;
- present but empty root seed;
- lengths that cannot be processed safely by the implementation;
- trailing bytes.

A decoder MUST NOT:

- guess missing values;
- substitute another profile;
- infer a hash suite from digest length or local configuration;
- normalize namespace bytes;
- repair malformed root-seed representation;
- ignore trailing bytes;
- silently reinterpret unsupported versions.

Malformed or noncanonical authority bytes fail deterministically.

## 10. Canonical Equivalence

Within `FME-SRA-CANONICAL-V1-WIRE-V1`:

one valid baseline SRA semantic object → one canonical byte representation.

A conformant encoder presented with the same semantic SRA MUST produce identical
bytes.

A conformant decoder that accepts canonical SRA bytes and then re-encodes the
result MUST reproduce those bytes exactly.

Multiple distinct byte encodings for the same baseline semantic SRA are not
permitted by this profile.

## 11. Cryptographic Boundary

Phase 009 produces canonical SRA semantic bytes.

It does not:

- hash those bytes;
- construct an SRA commitment;
- apply cryptographic domain separation;
- derive Stable SRA identity;
- derive Stable Authority Domain identity;
- create operational authority;
- materialize a Primary Trunk;
- create a HashHelix accepted-event sequence.

The architectural pipeline remains:

`SRA semantic object`

→

`Phase 009 canonical SRA bytes`

A later separately versioned commitment profile may consume those canonical
bytes.

Accordingly:

`serialization != commitment`

and:

`digest execution != commitment semantics`.

## 12. Root Artifact and Operational Authority Boundary

The Singularity Root Artifact is the deterministic origin artifact represented
by this wire format.

Successful decoding of an SRA does not create an operational HashHelix trunk or
Authority Domain.

Likewise, the SRA's FER profile and hash-suite identifiers establish declared
interpretation context; they do not independently create event authority,
topology materialization, or accepted structural state.

Accordingly:

`root artifact != operational trunk`

and:

`mathematical possibility != materialization`.

## 13. Conformance Fixture Boundary

The repository fixture:

`conformance/sra-canonical-v1/sra-canonical-v1.json`

contains normative canonical-byte vectors for this wire profile.

The JSON document itself is a conformance-fixture representation.

It is not the production SRA wire format.

Fields such as hexadecimal strings, fixture identifiers, provenance metadata,
and expected error names exist to support conformance testing and MUST NOT be
interpreted as additional fields in the production SRA byte sequence.

The fixture's canonical hexadecimal vectors are independently generated by:

`scripts/conformance/generate_sra_canonical_v1.py`

That secondary generator reproduces the Phase 009 byte layout without calling
the Rust encoder.

This strengthens implementation separation for repository conformance testing.

It does not constitute independent reproduction by an external implementation
or independent operator.
