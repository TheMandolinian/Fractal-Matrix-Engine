# Canonical Wire Encoding V1

This document defines the canonical authority-bearing wire encoding introduced
for the baseline FER profile:

`FME-FER-AFFINE-2D-BINARY-V1`

Wire version:

`1`

This specification applies only to the baseline FER path and exact-state
objects defined below.

It does not define:

- Stable Authority Identity;
- Singularity Root Artifact encoding;
- domain-separated cryptographic commitments;
- Authority Domain commitments;
- MMR encoding;
- Proof Capsules;
- HashHelix event encoding;
- higher-dimensional FER encoding.

## 1. Common Header

Every Canonical Wire V1 object begins with:

| Field | Encoding |
|---|---|
| magic | four bytes: ASCII `FMEF` |
| wire_version | one byte: `0x01` |
| object_kind | one byte |
| profile_id_length | one unsigned byte |
| profile_id | exact ASCII profile identifier bytes |

The required profile identifier is:

`FME-FER-AFFINE-2D-BINARY-V1`

Object kinds are:

- `0x01` — baseline recursive path;
- `0x02` — baseline exact FER state.

No byte-order mark, string terminator, whitespace, or implicit padding is
permitted.

## 2. Integer Byte Order

All fixed-width integers in Canonical Wire V1 use unsigned big-endian byte
order.

The V1 fixed-width length and depth fields use eight bytes (`u64`).

## 3. Baseline Path Encoding

After the common header, a baseline path contains:

1. `bit_length` — unsigned `u64`, big-endian;
2. `packed_bits` — exactly `ceil(bit_length / 8)` bytes.

Transform selectors are encoded as:

- `F0` → bit `0`;
- `F1` → bit `1`.

Bits are packed most-significant-bit first.

For example, logical path:

`01000001`

is represented by one packed byte:

`0x41`

When `bit_length` is not divisible by eight, unused low-order bits in the final
byte MUST be zero.

The root path has:

- `bit_length = 0`;
- zero packed path bytes.

Trailing bytes are forbidden.

## 4. Exact FER State Encoding

After the common header, an exact FER state contains:

1. `depth` — unsigned `u64`, big-endian;
2. canonical signed integer `P`;
3. canonical signed integer `Q`.

The authoritative state remains:

`(P, Q, depth)`

representing:

`z = (P + iQ) / 3^depth`

The serialization MUST NOT reduce the state as an ordinary mathematical
fraction because topology depth is authority-relevant state.

## 5. Canonical Signed Integer Encoding

Each signed integer contains:

1. `sign` — one byte;
2. `magnitude_length` — unsigned `u64`, big-endian;
3. `magnitude` — unsigned big-endian magnitude bytes.

Sign values are:

- `0x00` — zero;
- `0x01` — positive;
- `0x02` — negative.

Canonical zero is exactly:

- sign `0x00`;
- magnitude length `0`;
- no magnitude bytes.

For nonzero integers:

- sign MUST be `0x01` or `0x02`;
- magnitude length MUST be greater than zero;
- magnitude MUST be unsigned big-endian;
- the first magnitude byte MUST NOT be `0x00`;
- magnitude MUST use the shortest representation capable of representing the
  absolute value.

The following are noncanonical and MUST be rejected:

- positive zero;
- negative zero;
- zero with magnitude bytes;
- nonzero integer with zero-length magnitude;
- leading-zero magnitude;
- unsupported sign values.

## 6. Canonical Parsing

Canonical Wire V1 parsing is fail-closed.

A decoder MUST reject:

- invalid magic;
- unsupported wire version;
- unexpected object kind;
- profile mismatch;
- truncated input;
- invalid or unsupported integer sign;
- noncanonical integer zero;
- non-minimal integer magnitude;
- nonzero unused path bits;
- trailing bytes;
- lengths that cannot be processed safely by the implementation.

A decoder MUST NOT repair malformed or noncanonical authority bytes.

## 7. Canonical Equivalence

Within this wire profile:

one logical supported object → one canonical byte representation.

A decoder accepting multiple distinct byte encodings for the same logical V1
object would be nonconformant unless a future profile explicitly defines a
different normalization boundary.

## 8. Cryptographic Boundary

Canonical Wire V1 produces canonical bytes.

It does not hash those bytes.

It does not define cryptographic domain separation.

It does not derive Stable Authority Identity.

Later cryptographic profiles may consume Canonical Wire V1 bytes as inputs,
but those mechanisms remain outside Phase 006.

## 9. Human-Readable Boundary

Human-readable paths such as:

`00101101`

are diagnostic and test-input representations.

They are not the authority-bearing wire representation.

Likewise, decimal strings used in JSON conformance fixtures for `P` and `Q`
are fixture representations, not canonical production wire integers.
