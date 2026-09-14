# Implementation Scope

The initial Fractal Matrix Engine repository implements the exact mathematical
core of the baseline Fractal Evaluation Rule profile:

`FME-FER-AFFINE-2D-BINARY-V1`

Initial scope includes:

- exact root state;
- binary transform selectors;
- logical baseline paths;
- exact `P`, `Q`, and depth state;
- exact `F0` recurrence;
- exact `F1` recurrence;
- evaluation from the root;
- continuation from retained exact parent state;
- deterministic malformed-path failure;
- versioned canonical production wire encoding for baseline paths and exact
  states;
- deterministic malformed and noncanonical wire rejection;
- conformance fixtures;
- unit/integration testing;
- later benchmark instrumentation.

The initial scope does not include:

- HashHelix accepted-event authority;
- Authority Domain materialization;
- Stable Authority Identity;
- Singularity Root Artifact implementation;
- MMR history commitments;
- authenticated state trees;
- Proof Capsules;
- archive infrastructure;
- authorization;
- cross-domain coordination;
- multidimensional FER profiles.

Architecture-permitted behavior is not treated as implemented behavior.
