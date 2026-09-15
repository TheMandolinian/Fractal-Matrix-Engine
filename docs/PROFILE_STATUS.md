# FER Profile Status

## FME-FER-AFFINE-2D-BINARY-V1

Status: **implementation research / not production normative**

Currently targeted:

- exact two-dimensional baseline recurrence;
- binary logical path semantics;
- arbitrary-precision integer numerators;
- deterministic root evaluation;
- deterministic continuation;
- implementation conformance vectors.

Frozen and implemented for the baseline FER profile:

- normative canonical wire encoding for baseline logical paths;
- normative canonical wire encoding for exact baseline `P`, `Q`, and depth states.

Not yet frozen beyond that baseline wire profile:

- general topology-address descriptor wire encoding;
- additional coordinate or profile representations;
- maximum authoritative topology depth;
- production resource limits;
- profile registry representation;
- cryptographic binding;
- profile-transition wire semantics.

Implementation convenience must not silently define any unresolved normative
profile rule.
