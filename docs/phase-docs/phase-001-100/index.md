# Fractal Matrix Engine Phase Index — 001–100

This index is the chronological implementation ledger for FME phases 001–100.

| Phase | Scope | Status | Implementation Anchor | Documentation Anchor | Anchor Repair |
|---|---|---|---|---|---|
| 001 | Exact Baseline FER Core | SEALED | `cb7861b9` | `aa7aaa84` | — |
| 002 | External Conformance Vector Consumption | SEALED | `c5e996b5` | `aa7aaa84` | — |
| 003 | Institutional Repository Baseline | SEALED | `aa7aaa84` | `dc359eed` | — |
| 004 | Baseline FER Conformance Expansion | SEALED | `b417d542` | `b0ad7139` | — |
| 005 | Baseline FER Benchmark Characterization | SEALED | `f3ab055b` | `80f66946` | — |
| 006 | Baseline FER Canonical Encoding | SEALED | `7322debe` | `c45746aa` | `cd289812` |
| 007 | Baseline Cryptographic Registry and Preimage Contract | SEALED | `a237660a` | `8d49bf9e` | `749fcd7f` |
| 008 | Baseline Registered SHA-2 Digest Execution | SEALED | `443275a8` | `07130bf3` | `40b31897` |

The distinct anchor-repair lifecycle begins with Phase 006. For phases that
predate that lifecycle, a dash in the Anchor Repair column means that no
separate anchor-repair merge exists. During an active anchor-repair branch, a
dash may also temporarily mean that the repair merge does not yet exist.

A repair anchor must be replaced only with the real Git-derived merge hash
after the corresponding repair pull request is merged. No repair hash may be
predicted or fabricated.

Phases 001 and 002 predate the documentation-anchor procedure. Their closeout
documents were introduced together by `aa7aaa84` during the institutional
repository-baseline work, so that commit is recorded as their historical
documentation anchor rather than implying separate documentation pull
requests.

## Phase 001 — Exact Baseline FER Core

Historical working label: `FER-001`

Established the first executable exact implementation of
`FME-FER-AFFINE-2D-BINARY-V1`.

See:

`phase 001/closeout.md`

## Phase 002 — External Conformance Vector Consumption

Historical working label: `FER-002`

Moved baseline FER expected results into external repository conformance
fixtures consumed by the Rust test suite.

See:

`phase 002/closeout.md`

## Phase 003 — Institutional Repository Baseline

Establishes the repository-level maturity, conformance, security,
development, CI, audit, and phase-documentation posture.

See:

`phase 003/closeout.md`

## Phase 004 — Baseline FER Conformance Expansion

Strengthens externally represented deterministic exact conformance for
`FME-FER-AFFINE-2D-BINARY-V1` before FER benchmarking begins.

See:

`phase 004/closeout.md`

## Phase 005 — Baseline FER Benchmark Characterization

Establishes a reproducible measurement harness for exact baseline FER
evaluation without making system-level performance or scalability claims.

See:

`phase 005/closeout.md`

## Phase 006 — Baseline FER Canonical Encoding

Establishes the first normative, versioned canonical production wire encoding
for baseline FER paths and exact states, including deterministic rejection of
malformed and noncanonical representations.

See:

`phase 006/closeout.md`

## Phase 007 — Baseline Cryptographic Registry and Preimage Contract

Establishes exact initial cryptographic-domain and hash-suite registries plus
deterministic domain-separated preimage framing without executing digest
algorithms.

See:

`phase 007/closeout.md`

## Phase 008 — Baseline Registered SHA-2 Digest Execution

Establishes deterministic execution of the registered SHA2-256 and SHA2-512
algorithms over exact validated Phase 007 canonical preimage bytes without
assigning higher-level commitment semantics to the resulting digest.

See:

`phase 008/closeout.md`

## Current Position

Last sealed phase: **008**

Current phase: **none**
