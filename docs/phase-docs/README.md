# Fractal Matrix Engine Phase Documentation

This directory contains the chronological implementation record for the
Fractal Matrix Engine repository.

FME uses one global implementation-phase sequence.

Subsystem labels such as FER, history commitment, authenticated state,
checkpointing, proof, archive, authorization, and coordination describe
technical scope. They do not create independent phase-number sequences.

## Phase Buckets

Phase documentation is grouped in blocks of one hundred phases:

- `phase-001-100`
- `phase-101-200`
- `phase-201-300`
- and so on as required.

Each bucket contains an `index.md`.

## Phase Closeouts

A completed implementation phase receives its own directory and `closeout.md`.

A closeout records:

- phase identity;
- scope;
- architectural boundary;
- implementation completed;
- verification performed;
- explicit non-claims;
- resulting repository state;
- and the actual commit anchor.

## Anchor Law

Commit anchors must be copied from actual repository history.

Placeholder, predicted, inferred, or fabricated commit hashes are prohibited.

A phase still under implementation must use an explicit pending marker rather
than guessing its future anchor.

## Status Vocabulary

Use:

- `PLANNED`
- `IN PROGRESS`
- `SEALED`

`SEALED` means the implementation milestone has been committed, verified, and
assigned its actual repository anchor.

It does not imply production readiness.

## Governing Principle

Phase documentation records what the repository actually established.

It must not upgrade architecture-defined, experimental, benchmarked, or
partially implemented behavior into stronger maturity claims.
