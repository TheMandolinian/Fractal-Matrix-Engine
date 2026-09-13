# Phase 005 — Baseline FER Benchmark Characterization

Status: **SEALED**

Commit anchor:

`f3ab055`

## Objective

Establish a reproducible benchmark harness for exact evaluation of:

`FME-FER-AFFINE-2D-BINARY-V1`

Phase 005 characterizes the implemented FER arithmetic boundary after the
expanded Phase 004 conformance corpus established stronger correctness evidence.

## Scope

Phase 005 may establish:

- a stable-Rust FER benchmark executable;
- optimized Cargo `bench`-profile benchmark execution;
- fixed profile-bound workload cases;
- multiple topology depths;
- repeated-transform workloads;
- alternating-transform workloads;
- mixed-transform workloads;
- warmup iterations;
- measured iterations;
- elapsed-time reporting;
- evaluations-per-second reporting;
- transforms-per-second reporting;
- benchmark environment metadata;
- reproducible benchmark instructions.

## Measurement Boundary

Timed FER evaluation should operate on paths parsed before the measured loop.

Path parsing, fixture loading, console output, and benchmark setup must not be
silently included in the exact FER arithmetic throughput measurement.

Benchmark values are observational measurements, not authority-bearing state.

## Terminology Boundary

A FER transform is not an application event, transaction, ledger append,
Authority Domain acceptance, proof operation, or complete FME operation.

Phase 005 may report:

- FER evaluations per second;
- FER transforms per second;
- elapsed benchmark duration.

It must not relabel those measurements as complete FME event throughput.

## Reproducibility Boundary

Benchmark output depends on implementation build mode, processor, operating
environment, toolchain, workload depth, integer magnitude, and measurement
conditions.

A local benchmark run is not independent reproduction.

## Explicit Non-Claims

Phase 005 does not establish:

- production throughput;
- complete FME event throughput;
- HashHelix throughput;
- network throughput;
- scalability superiority;
- latency guarantees;
- cryptographic throughput;
- proof throughput;
- Authority Domain throughput;
- cross-domain throughput;
- security;
- independent reproduction;
- production readiness.

## Architecture Boundary

Phase 005 does not alter authoritative FER mathematics.

No FER engine-source change is planned.

If benchmarking exposes an implementation defect, the defect must be classified
before authoritative source behavior is changed.

## Benchmark Design

Phase 005 establishes a dependency-free stable-Rust custom benchmark target
using Cargo's optimized `bench` profile.

The benchmark uses:

- `std::time::Instant`;
- `std::hint::black_box`;
- paths parsed before the timed region;
- explicit warmup iterations;
- explicit measured iterations;
- fixed deterministic workloads;
- FER evaluations-per-second reporting;
- FER transforms-per-second reporting.

The fixed workload families are:

- repeated F0;
- repeated F1;
- alternating 01;
- alternating 10;
- mixed `00101101`.

The fixed topology depths are:

`8, 16, 32, 64, 128, 256, 384, 512, 768, 1000`

This produces 50 fixed benchmark cases.

## Local Characterization Observation

The stabilized Phase 005 benchmark completed all 50 cases on:

- target architecture: `x86_64`;
- target OS: `linux`;
- Rust toolchain: `rustc 1.98.1`;
- Cargo profile: `bench`.

Observed depth-8 throughput across the fixed workloads was approximately
17.5 to 18.6 million FER transforms per second.

At depth 384, observed throughput was approximately 9.3 to 11.0 million FER
transforms per second.

At depth 1000, observed throughput was approximately 8.3 to 9.0 million FER
transforms per second.

The sampled depth-256-through-depth-512 region showed no obvious abrupt
throughput cliff.

These are local observations only. They are not throughput guarantees,
production claims, scalability claims, or independent reproduction.

The Phase 004 external conformance corpus currently reaches depth 128.
Successful benchmark execution beyond depth 128 demonstrates execution without
runtime failure for these workloads, but does not provide the same
conformance-evidence strength as the externally represented depth-128 corpus.

## Implementation Completed

Phase 005 establishes:

- a custom stable-Rust benchmark executable;
- no new benchmark dependency;
- 5 deterministic workload families;
- 10 fixed topology depths;
- 50 fixed benchmark cases;
- explicit warmup and measured-iteration policies;
- environment metadata reporting;
- explicit FER measurement terminology boundaries;
- a dedicated Phase 005 benchmark audit;
- repository baseline-audit integration.

No authoritative FER engine source was changed.

## Verification Performed

Phase 005 passed:

- Rust formatting;
- locked dependency metadata resolution;
- workspace check;
- complete workspace tests;
- 11 FER conformance tests;
- Clippy with warnings denied;
- Phase 004 regression audit;
- Phase 005 benchmark characterization audit;
- repository baseline audit;
- Python bytecode cleanliness check;
- FER engine-source scope validation;
- whitespace validation.

The stabilized benchmark run completed all 50 fixed workload/depth cases.


## Resulting Repository State

The Phase 005 implementation boundary was established at:

`f3ab055`

Phase 005 provides reproducible local characterization of the implemented
exact baseline FER evaluator across 50 fixed workload/depth cases through
topology depth 1000.

The benchmark observations remain subsystem-local measurements and do not
represent complete FME throughput, HashHelix throughput, transaction
throughput, security, production readiness, or independent reproduction.

No authoritative FER engine source was changed.

## Closeout Requirement

Phase 005 may be marked `SEALED` only after the benchmark harness, benchmark
audit, documentation, and full repository gate pass; the implementation is
committed; the actual Git anchor is obtained; and the phase ledger is repaired
with that real anchor.

No commit anchor may be predicted or fabricated.
