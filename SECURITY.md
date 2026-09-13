# Security Policy

Fractal Matrix Engine is currently a pre-production research implementation.

## Current Security Posture

The repository must not be interpreted as production security infrastructure.

Current implementation work prioritizes:

- deterministic correctness;
- profile conformance;
- reproducibility;
- explicit failure behavior;
- and architectural boundary enforcement.

No claim of production security has been made.

## Reporting Security Issues

Do not publish exploit details, private keys, credentials, or sensitive
vulnerability information in a public issue.

Where GitHub private vulnerability reporting is available, use that private
channel for security-sensitive disclosures.

If no private disclosure channel is available, open only a minimal public issue
requesting a private contact path without including exploit details.

## Security-Relevant Scope

Security-relevant areas include:

- authority-bound canonicalization;
- FER arithmetic correctness;
- deterministic parsing and rejection;
- cryptographic commitment construction;
- authenticated data structures;
- authorization;
- profile transitions;
- archive integrity;
- proof verification;
- cross-domain coordination.

Many of these mechanisms are not yet implemented.

## Supported Versions

There is currently no production release line.

A production security-support policy will be established before any production
support claim is made.
