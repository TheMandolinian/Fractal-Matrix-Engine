# Phase 004 — Baseline FER Conformance Expansion

Status: **SEALED**

Commit anchor:

`b417d54`

## Objective

Strengthen deterministic exact conformance coverage for:

`FME-FER-AFFINE-2D-BINARY-V1`

before beginning FER performance benchmarking.

Phase 004 expands the externally represented conformance boundary so that
subtle errors in recurrence arithmetic, transform selection, transform order,
topology depth, continuation, arbitrary-precision growth, and malformed input
handling are more likely to be detected before performance work begins.

## Scope

Phase 004 may establish:

- deeper exact FER success vectors;
- repeated-F0 paths;
- repeated-F1 paths;
- alternating transform paths;
- deterministic mixed paths;
- arbitrary-precision exact-state cases;
- continuation cases across multiple split depths;
- externally represented malformed-input failure cases;
- fixture provenance and review metadata;
- a secondary exact reference calculation used to review expected vectors;
- expanded Rust conformance tests;
- expanded repository audit coverage for the conformance corpus.

## Conformance Truth Separation

Expected conformance values must not be copied blindly from the Rust
implementation under test and then represented as independent evidence.

Phase 004 should preserve meaningful separation between:

- the Rust FER implementation;
- the external conformance corpus;
- and any secondary exact reference calculation used to review expected values.

A secondary repository reference calculation does not establish independent
reproduction.

Independent reproduction requires a separately implemented or independently
operated evaluator.

## Fixture Boundary

Repository JSON fixtures remain test and conformance artifacts.

They must continue to declare:

`normative_wire_format = false`

Phase 004 does not establish the final normative FME path, coordinate, failure,
or profile wire encoding.

## Architecture Boundary

Phase 004 remains FER-only.

No change to authoritative FER mathematics is planned.

If expanded conformance reveals a disagreement with the currently implemented
recurrence, the discrepancy must be classified before engine source is changed.

Possible classifications include:

1. implementation defect;
2. profile specification gap;
3. architecture amendment requirement;
4. conformance-vector defect.

## Explicit Non-Claims

Phase 004 does not establish:

- FER benchmarking;
- FER performance superiority;
- SHA-256 implementation;
- SHA-512 implementation;
- cryptographic suite behavior;
- normative canonical wire encoding;
- Singularity Root Artifact behavior;
- Authority Domain materialization;
- Stable Authority Identity;
- MMR history commitments;
- authenticated state;
- Proof Capsules;
- execution proofs;
- authorization;
- cross-domain coordination;
- generic atomic distributed transactions;
- independent reproduction;
- security review;
- production readiness.

## Benchmark Boundary

Existing benchmark source may remain in the repository, but Phase 004 does not
extend, execute as evidence, or make claims from FER benchmarks.

Benchmark development is reserved for a later phase after Phase 004 closes
cleanly.

## Planned Verification

Phase 004 should verify, at minimum:

- fixture metadata;
- fixture provenance metadata;
- exact P values;
- exact Q values;
- exact topology depth;
- repeated-transform cases;
- alternating-transform cases;
- mixed-transform cases;
- longer arbitrary-precision cases;
- continuation equivalence across multiple split depths;
- deterministic malformed path rejection;
- repository fixture structure;
- formatting;
- locked dependency resolution;
- workspace build;
- complete test suite;
- Clippy with warnings denied;
- repository audit;
- whitespace integrity.

## Implementation Completed

Phase 004 established:

- fixture format version 2;
- 34 exact success vectors;
- 5 deterministic malformed-path failure vectors;
- 8 continuation vectors;
- repeated-F0 and repeated-F1 coverage;
- alternating transform coverage;
- deterministic mixed-path coverage;
- exact depth-128 arbitrary-precision cases;
- fixture provenance metadata;
- a secondary exact rational-affine reference evaluator;
- expanded Rust fixture consumption;
- a dedicated Phase 004 conformance audit;
- repository baseline-audit integration.

No authoritative FER engine source was changed.

## Verification Performed

The Phase 004 implementation passed:

- Rust formatting;
- locked dependency resolution;
- workspace check;
- complete workspace tests;
- 11 FER conformance tests;
- Clippy with warnings denied;
- Phase 004 conformance expansion audit;
- repository baseline audit;
- Python source compilation;
- whitespace validation;
- FER engine-source scope validation;
- benchmark-scope validation.

The secondary exact evaluator reproduces the established shallow corpus and
the expanded fixture values using exact rational affine evaluation.

This strengthens repository-local conformance evidence but does not establish
independent reproduction.

## Resulting Repository State

The Phase 004 implementation boundary was established at:

`b417d54`

Phase 004 strengthens correctness evidence sufficiently to permit later
consideration of FER benchmark work without making any benchmark or performance
claim in this phase.

## Closeout Requirement

Phase 004 may be marked `SEALED` only after:

1. the expanded conformance corpus is implemented;
2. the phase-specific verification passes;
3. the full repository gate passes;
4. Phase 004 implementation is committed;
5. the actual implementation commit anchor is obtained from Git;
6. the pending anchor is replaced with the real anchor;
7. the phase index is changed from `IN PROGRESS` to `SEALED`;
8. any required closeout repair is committed;
9. main is pushed;
10. remote parity is confirmed.

No commit anchor may be predicted or fabricated.
