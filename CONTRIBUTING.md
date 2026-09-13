# Contributing to Fractal Matrix Engine

Fractal Matrix Engine is an architecture-driven deterministic systems project.

Contributions must preserve the distinction between:

- architectural intent;
- normative profile behavior;
- implementation behavior;
- conformance fixtures;
- benchmarks;
- experimental research;
- and demonstrated production properties.

## Authority-Bearing Changes

Before changing authority-bearing behavior, identify the governing architecture
or profile rule.

If the required behavior is unspecified, do not silently choose an
implementation-specific answer.

Classify the issue explicitly as:

1. an implementation defect;
2. a profile specification gap;
3. an architecture amendment requirement; or
4. a research question.

## Change Discipline

Prefer narrow, reviewable changes.

Avoid unrelated refactoring inside authority-sensitive changes.

A contribution should include the tests, conformance evidence, documentation,
or benchmark evidence required to justify its behavior.

## Required Local Gate

Before committing an implementation milestone, run:

    cargo fmt --all --check
    cargo check --workspace --locked
    cargo test --workspace --locked
    cargo clippy --workspace --all-targets --locked -- -D warnings

Repository-specific audits must also pass where applicable.

## Conformance Changes

Existing conformance vectors must not be rewritten merely to make modified
implementation behavior pass.

Any change to a published expected result requires explicit technical
justification and repository history.

## Commit Discipline

Use coherent commits with meaningful messages.

Milestone anchors must reference actual commits.

Predicted, placeholder, or fabricated commit anchors are prohibited.

## Unsafe Rust

The current `fme-fer` crate forbids unsafe Rust.

Introducing unsafe code into an authority-bearing component requires explicit
technical justification and review.

## Production Claims

Do not describe code as production-ready, secure, atomic, independently
reproduced, or performance-superior unless the corresponding evidence exists.
