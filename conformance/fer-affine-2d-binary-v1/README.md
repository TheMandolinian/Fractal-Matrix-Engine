# FME-FER-AFFINE-2D-BINARY-V1 Conformance Fixtures

This directory contains implementation conformance fixtures for the baseline
Fractal Evaluation Rule profile:

`FME-FER-AFFINE-2D-BINARY-V1`

## Fixture Boundary

The JSON representation used here is a repository test-fixture format only.

It is **not** the normative FME topology-path, exact-coordinate, failure, or
profile wire encoding.

The fixture therefore declares:

`normative_wire_format = false`

A production canonical wire specification must be explicitly defined and
versioned before these logical values are used as authority-bearing serialized
bytes.

## Phase 004 Expansion

Phase 004 expands the fixture corpus to include:

- root and shallow exact-state vectors;
- repeated F0 vectors;
- repeated F1 vectors;
- alternating transform vectors;
- mixed transform vectors;
- longer arbitrary-precision vectors;
- deterministic malformed human-readable path failures;
- continuation cases from multiple split depths.

The expanded corpus includes exact states through topology depth 128.

The depth-128 cases require integer magnitudes beyond ordinary fixed-width
128-bit signed integer capacity and therefore exercise the baseline profile's
arbitrary-precision implementation boundary.

## Expected-Value Provenance

Phase 004 expected values were reviewed using:

`scripts/conformance/reference_fer_affine_2d_binary_v1.py`

That evaluator computes the baseline affine transformations directly over
Python exact rational arithmetic.

For exact real and imaginary components `(x, y)`, it evaluates:

For F0:

`x' = (x - y) / 3 - 1`

`y' = (x + y) / 3`

For F1:

`x' = (x + y) / 3 + 1`

`y' = (y - x) / 3`

Only after complete path evaluation does the reference calculation recover
the baseline integer numerators by multiplying the exact rational components
by `3^depth`.

This intentionally differs from the Rust implementation's direct P/Q
recurrence path.

Before generating the Phase 004 expansion, the secondary evaluator reproduces
the previously established shallow fixture vectors.

## Evidence Boundary

The repository-local secondary evaluator strengthens review separation between
the Rust implementation and expected fixture values.

It does **not** establish independent reproduction.

Independent reproduction requires a separately implemented or independently
operated evaluator reproducing the declared results from the same authoritative
profile inputs.
