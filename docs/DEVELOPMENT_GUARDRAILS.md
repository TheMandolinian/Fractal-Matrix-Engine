# Fractal Matrix Engine Development Guardrails

These guardrails apply to implementation work in this repository.

## 1. Architecture Is a Contract

The Fractal Matrix Engine whitepaper and subsequently adopted normative profile
specifications define the intended architectural boundaries.

Implementation must not silently invent authority-bearing behavior when the
architecture leaves a question unresolved.

When implementation exposes a genuine gap, classify it explicitly as one of:

1. implementation correction;
2. profile specification requirement;
3. architecture amendment;
4. candidate simplification or removal.

## 2. Candidate Behavior Is Not Normative Behavior

Experimental code, benchmark code, visualization code, examples, test fixture
formats, and exploratory APIs do not become normative FME behavior merely
because they exist in the repository.

## 3. Exact Arithmetic Remains Authoritative

For FER profiles requiring exact arithmetic, floating-point values are
non-authoritative unless a future profile explicitly states otherwise.

Visualization must not become an authority source.

## 4. Deterministic Failure

Malformed, unsupported, ambiguous, numerically invalid, or noncanonical
authority-bearing input must fail through declared deterministic behavior.

The implementation must not repair authority state through local guesswork.

## 5. No Hidden Authority

UI ordering, database ordering, hash-map iteration, filesystem ordering,
runtime discovery, memory location, wall-clock arrival time, and transport
success must not silently become authority mechanisms.

## 6. Narrow Milestones

Prefer small coherent milestones over speculative scaffolding.

A module or crate should exist because an implemented boundary requires it,
not because a future architecture section mentions it.

## 7. Verification Before Commit

Implementation milestones should pass the repository gate before commit.

At minimum, for the current Rust workspace:

- formatting;
- locked dependency resolution;
- build/check;
- tests;
- Clippy with warnings denied;
- conformance fixture validation;
- repository-specific audits.

## 8. Commit Anchors Are Evidence

Milestone documentation must reference actual repository commit identifiers
after the relevant commit exists.

Placeholder, predicted, or fabricated commit anchors are prohibited.

## 9. Replayability

Authority-affecting behavior must preserve the ability to reproduce declared
results from the required retained evidence and governing profile context.

## 10. Implementation Honesty

Do not describe a mechanism as secure, scalable, atomic, consensus-bearing,
production-ready, performant, or independently verifiable without the evidence
required for that claim.
