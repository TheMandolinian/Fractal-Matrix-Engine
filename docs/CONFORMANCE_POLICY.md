# Fractal Matrix Engine Conformance Policy

Conformance artifacts are first-class implementation evidence.

## Purpose

A conformance vector states that a declared authoritative input under a
specific profile must produce a declared deterministic result or declared
deterministic failure.

Conformance vectors are intended to support reproducibility across:

- repeated local evaluation;
- independent implementations;
- implementation languages;
- supported platforms;
- future profile versions.

## Profile Binding

Every conformance artifact must identify the profile to which it applies.

A vector defined for one FER profile must not be silently interpreted under
another profile.

## Fixture Format versus Normative Wire Format

A repository fixture representation is not automatically a normative FME wire
encoding.

Test-oriented JSON, TOML, YAML, Rust structures, or other convenience formats
must be explicitly labeled when they are non-normative.

Current baseline fixtures declare:

`normative_wire_format = false`

The eventual normative path and coordinate encodings require separately
versioned specification.

## Required Vector Properties

A conforming baseline FER vector should identify sufficient information to
reproduce the expected result, including:

- profile identity;
- canonical logical test input;
- expected exact P value;
- expected exact Q value;
- expected topology depth;
- expected deterministic failure where applicable.

Future profile-specific vectors may require additional fields.

## Change Discipline

A previously published vector must not be silently changed merely to make a
modified implementation pass.

If a vector is discovered to be incorrect:

1. document the defect;
2. identify the affected profile/specification;
3. correct the source of truth;
4. record the correction in repository history;
5. determine whether a profile-version change is required.

## Independent Reproduction

Local fixture success establishes repository conformance testing.

It does not establish independent reproduction.

Independent reproduction requires a separately operated or independently
implemented evaluator to reproduce the same declared result from the same
authoritative inputs.

## Deterministic Failure Vectors

Malformed and boundary inputs are part of conformance.

Future vector sets should include declared deterministic failure cases rather
than testing successful paths only.

## Current Status

`FME-FER-AFFINE-2D-BINARY-V1` currently has implementation conformance fixtures.

The fixture representation is not the final normative FME wire format.
