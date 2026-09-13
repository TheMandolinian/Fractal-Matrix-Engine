# Fractal Matrix Engine Maturity Model

Fractal Matrix Engine uses explicit maturity language so architectural intent,
implementation status, test evidence, and production claims are never conflated.

## Maturity States

### Architecture-Defined

The behavior is described by the governing FME architecture or an applicable
profile specification.

This does not imply that an implementation exists.

### Implemented

Executable code exists for the behavior.

This does not imply that the behavior has been exhaustively tested,
benchmarked, independently reproduced, security-reviewed, or approved for
production use.

### Tested

Automated tests exercise declared behavior and currently pass under the
repository's verification gate.

Testing does not by itself establish performance, security, or production
readiness.

### Conformance-Tested

The implementation reproduces published conformance fixtures or vectors under
the declared profile.

Conformance to local fixtures is not equivalent to independent
cross-implementation reproduction.

### Benchmarked

Performance or resource behavior has been measured through a declared,
reproducible benchmark procedure.

Benchmark results must identify relevant environment, workload, profile, and
measurement boundaries.

### Independently Reproduced

A separately implemented or independently operated implementation has
reproduced the declared normative result from the same authoritative inputs.

This status requires actual evidence and must not be inferred from local tests.

### Security-Reviewed

A defined security review has been completed against the relevant scope.

Ordinary code review does not automatically qualify as security review.

### Production-Ready

Production readiness requires a separately declared release gate covering the
relevant correctness, conformance, operational, security, compatibility,
recovery, and support requirements.

No current FME component should be described as production-ready unless that
gate has explicitly been satisfied.

## Governing Rule

Architecture-permitted does not mean implemented.

Implemented does not mean tested.

Tested does not mean independently reproduced.

Benchmarked does not mean secure.

Cryptographically committed does not mean physically true.

No maturity state may be silently inferred from another.
