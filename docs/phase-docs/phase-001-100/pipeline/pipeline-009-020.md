# FME Implementation Pipeline — Phases 009–020

Status: **ACTIVE PLANNING ROADMAP**

This document records the current candidate implementation direction for
Fractal Matrix Engine Phases 009 through 020.

It is a planning surface, not implementation authority.

A phase listed here must not be implemented merely because it appears in this roadmap.

Every phase remains subject to repository reconnaissance, whitepaper review,
dependency verification, and a bounded implementation contract before work begins.

## Governing Pipeline Rule

The normal progression is:

pipeline candidate -> clean synchronized main -> preceding closeouts -> targeted reconnaissance -> whitepaper contract -> bounded phase definition -> implementation branch

A candidate phase may be renamed, narrowed, expanded, split, combined,
reordered, deferred, or removed when repository evidence shows that another boundary is cleaner.

Preserve:

- architecture-defined != implemented
- implemented != tested
- tested != independently reproduced
- benchmarked != secure
- pipeline-planned != implementation-authorized

## Current Architectural Position

Phase 008 is the last sealed implementation phase at creation of this roadmap.

The implemented deterministic cryptographic pipeline is:

canonical semantic payload
    -> Phase 007 domain-separated canonical preimage
    -> explicit registered hash suite
    -> Phase 008 deterministic digest bytes

Registered semantic cryptographic domains currently include:

- FME/SINGULARITY/V1
- FME/AUTHORITY_DOMAIN/V1

Registered digest suites currently include:

- SHA2-256
- SHA2-512

A Phase 008 digest does not automatically possess higher-level commitment meaning.
Higher-level semantic commitments require separately defined profiles.

---

## Phase 009 — Baseline Singularity Root Artifact Canonical Serialization

Candidate objective:

Define the first exact canonical byte representation of the baseline
Singularity Root Artifact semantic object.

Target boundary:

SRA semantic object -> canonical serialization -> canonical SRA semantic bytes

Phase 009 must not itself assign SRA commitment semantics.

Expected work includes:

- exact baseline SRA semantic schema;
- explicit serialization-profile identifier;
- exact field ordering;
- exact field types;
- required versus optional field rules;
- unambiguous length encoding;
- canonical identifier representation;
- root-seed representation if included;
- deterministic decode/re-encode behavior;
- malformed and noncanonical rejection;
- external conformance vectors;
- independent fixture reproduction;
- Phase 009 audit;
- repository-baseline integration.

The exact schema remains subject to Phase 009 reconnaissance.

Descriptive metadata must not be added merely because the whitepaper lists it illustratively.

This pipeline document is itself an explicit Phase 009 deliverable.

---

## Phase 010 — Baseline SRA Commitment Profile

Candidate objective:

Assign explicit cryptographic SRA commitment meaning to the canonical bytes
established by Phase 009.

Expected conceptual pipeline:

Phase 009 canonical SRA bytes
    -> FME/SINGULARITY/V1 domain-separated preimage
    -> explicit registered hash suite
    -> typed SRA commitment

Phase 010 should reuse the Phase 007 domain-preimage contract and Phase 008
registered digest execution rather than introduce a second hashing path.

Commitment semantics must remain distinct from serialization.

---

## Phase 011 — SRA Stable Identity Derivation

Candidate objective:

Define the stable identifier of the Singularity Root Artifact from explicitly
committed root authority material.

Preserve:

- SRA identity != human-readable system name
- SRA identity != topology coordinate
- SRA identity != physical location

The exact identity formula, suite binding, namespace binding, and failure
semantics require separate reconnaissance.

---

## Phase 012 — Authority Domain Materialization Boundary

Candidate objective:

Define the deterministic boundary through which mathematical topology
possibility becomes operationally recognized authority.

Preserve:

- mathematical possibility != materialized authority
- FER result != operational authority
- topology coordinate != Stable Authority Identity

Materialization must ultimately be explicit, accepted, replayable structural state.

---

## Phase 013 — Primary Trunk Declaration Contract

Candidate objective:

Define the canonical root-level structural declaration through which a Primary
Trunk may be materialized under SRA-governed authority.

Preserve:

- one SRA != one mandatory operational trunk
- Primary Trunk declaration != ordinary descendant-branch declaration

A Primary Trunk must not require another ordinary operational trunk to become
its artificial parent merely for identity or authority purposes.

The exact declaration schema, authorization requirements, and relationship to
root-level structural authority remain subject to reconnaissance.

---

## Phase 014 — Topology Address Descriptor Canonical Contract

Candidate objective:

Define the profile-general canonical authority representation of deterministic
FME topology placement.

Preserve the distinction among:

- Stable Authority Identity
- Topology Address Descriptor
- Matrix Coordinate
- Logical Topology
- Physical Coordinate

Do not permanently collapse general FME topology into the baseline binary
Branch Path representation.

The exact descriptor schema must remain FER-profile aware and canonically
serializable.

---

## Phase 015 — Stable Authority Domain Identity Derivation

Candidate objective:

Define deterministic Stable Authority Identity for a materialized Authority
Domain from canonical authority-bound material.

Potential dependencies include:

- SRA identity;
- namespace;
- accepted materialization declaration;
- FER Profile;
- HashHelix Profile;
- Topology Address Descriptor;
- structural relationship context;
- cryptographic suite identity.

The exact production identity formula remains subject to reconnaissance.

Preserve:

- topology position != Stable Authority Identity
- human-readable name != Stable Authority Identity
- physical location != Stable Authority Identity

---

## Phase 016 — Structural Transition Contract

Candidate objective:

Establish replayable authority-bearing transitions for structural state.

Candidate transition classes may eventually include:

- reserve;
- materialize;
- activate;
- retire;
- migrate;
- reparent;
- relationship creation;
- relationship removal.

A topology or identity transition must not exist solely as hidden mutable
database, filesystem, runtime, or UI state.

Preserve:

- topology changes are accepted history
- structural authority changes must remain replayable
- local implementation order does not create structural authority

---

## Phase 017 — Accepted History Leaf Contract

Candidate objective:

Define the exact canonical cryptographic leaf material representing an event
that HashHelix has already accepted.

Preserve:

- HashHelix acceptance -> eligible history leaf
- candidate submission != history leaf
- rejected candidate != accepted-history leaf
- MMR does not create event authority

Phase 017 should establish leaf semantics before the complete authenticated
history accumulator is introduced.

The final leaf schema must explicitly bind accepted local order and relevant
profile context without inventing a second sequencing mechanism.

---

## Phase 018 — Baseline MMR History Commitment Profile

Candidate objective:

Implement the first normative append-oriented authenticated-history profile.

Expected areas include:

- leaf hashing;
- internal-node hashing;
- node domain separation;
- indexing;
- append behavior;
- peak ordering;
- peak commitment or bagging;
- leaf-count binding;
- empty-history representation;
- proof encoding;
- proof verification;
- deterministic malformed-proof rejection;
- external conformance vectors.

Preserve:

- HashHelix determines accepted order
- MMR commits that accepted order
- MMR does not create event authority

---

## Phase 019 — Authenticated Derived-State Foundation

Candidate objective:

Establish the canonical key/value commitment boundary required for authenticated derived state.

Expected areas may include:

- canonical state-key material;
- canonical state-value material;
- schema and profile binding;
- deterministic key/value commitment semantics;
- selection of the baseline authenticated-state structure.

Preserve:

- authenticated state != proof of correct state derivation
- state commitment != event authority

The exact authenticated-state structure remains subject to reconnaissance.

---

## Phase 020 — Authority Domain Commitment Profile

Candidate objective:

Define the first unified cryptographic commitment to a materialized Authority Domain declared state.

Candidate committed material may include:

- Stable Authority Identity;
- accepted-history commitment;
- authenticated derived-state commitment;
- HashHelix accepted head;
- topology and profile context;
- epoch or checkpoint context;
- predecessor commitment;
- profile identifiers.

The final schema must be determined through Phase 020 reconnaissance.

Preserve:

- unified commitment != event authority
- unified commitment != physical truth

---

# Candidate Dependency Direction

Current candidate direction:

009 SRA canonical bytes
    -> 010 SRA commitment
    -> 011 SRA stable identity
    -> 012 materialization boundary
    -> 013 Primary Trunk declaration
    -> 014 topology descriptor
    -> 015 Stable Authority Domain identity
    -> 016 structural transitions
    -> 017 accepted-history leaf
    -> 018 MMR authenticated history
    -> 019 authenticated derived state
    -> 020 Authority Domain commitment

This sequence is provisional.

Repository evidence controls the final phase boundaries.

---

# Explicitly Deferred Beyond This Pipeline

This roadmap does not attempt to complete every FME subsystem.

Later work is expected to address carefully separated areas such as:

- checkpoints;
- Proof Capsules;
- topology proofs;
- relationship proofs;
- ancestry proofs where applicable;
- archive manifests;
- HOT/WARM/COLD evidence lifecycle;
- Proof Mode;
- Full Audit Mode;
- HashHelix operational integration;
- cryptographic migration;
- signatures and key management;
- cross-domain coordination.

Cross-domain atomic coordination remains explicitly unfinished.

Preserve:

- Remote Request + Valid Proof != Local State Mutation
- asynchronous checkpointing != atomicity
- asynchronous rollups != synchronization

---

# Pipeline Renewal Rule

Before implementation proceeds beyond the final phase covered by this roadmap,
create the next repository-local pipeline document.

The expected next planning block is approximately:

pipeline-021-030.md

Later blocks should continue in roughly ten-phase segments unless a natural
architectural boundary justifies a slightly different range.

The next pipeline must be grounded in what the repository has actually
implemented by that point.

Do not blindly copy unimplemented assumptions from this roadmap forward.

---

# Repository Governance

All implementation phases continue to use the established lifecycle:

Stage 0 — reconnaissance from clean synchronized main

Stage 1 — implementation branch -> PR -> squash merge -> capture real implementation merge hash

Stage 2 — documentation branch -> PR -> squash merge -> capture real documentation merge hash

Stage 3 — anchor-repair branch -> PR -> squash merge -> capture real anchor-repair merge hash

Stage 4 — record-only anchor-repair hash backfill; bookkeeping only; no fourth lifecycle anchor

Final lifecycle reporting for a phase contains exactly:

- Implementation merge
- Documentation merge
- Anchor repair merge

Only real squash commits observed on main qualify as lifecycle anchors.

Never predict Git hashes.
Never edit directly on main.

---

# Reconnaissance Procedure

For each future phase:

1. verify clean synchronized main;
2. read the active pipeline;
3. read the immediately preceding relevant 2–3 phase closeouts;
4. reconstruct the exact implemented dependency boundary;
5. inspect only relevant source, fixture, generator, and audit files;
6. review the relevant whitepaper sections;
7. define a bounded phase contract;
8. only then create the implementation branch.

Do not begin every phase with unnecessary broad repository archaeology.

The roadmap preserves direction.

Repository evidence determines what is real.
