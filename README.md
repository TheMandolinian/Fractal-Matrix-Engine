Fractal Matrix Engine (FME).

FME is a sparse, deterministic, profile-defined architecture for topology evaluation, authority-domain organization, cryptographic commitments, checkpointing, selective proof, and replayable verification built on top of HashHelix.

FME does not replace HashHelix.

HashHelix remains responsible for accepted-event progression within an Authority Domain. The Fractal Evaluation Rule (FER) provides deterministic topology evaluation under an explicitly versioned topology profile. Authenticated structures provide bounded proof surfaces, and cryptographic digest functions bind canonical authority artifacts.

This repository is currently a pre-production research implementation.

Initial implementation scope

The first implementation target is the baseline topology profile:

FME-FER-AFFINE-2D-BINARY-V1

with:

F0(z) = ((1 + i) / 3)z - 1

F1(z) = ((1 - i) / 3)z + 1

and root topology state:

zε = 0

Authoritative evaluation uses exact arithmetic rather than floating-point complex arithmetic.

For a path of depth d, the exact topology state is represented as:

z_w = (P_w + iQ_w) / 3^d

with integer recurrence:

For transform F0:

P' = P - Q - 3^(d+1)

Q' = P + Q

For transform F1:

P' = P + Q + 3^(d+1)

Q' = Q - P

The initial implementation exists to prove that identical profile inputs produce identical exact topology results across repeated evaluation and continuation from retained exact parent state.

Architectural boundaries

This repository must preserve the separation between:

HashHelix accepted-event authority;
FER topology evaluation;
structural materialization;
logical/application relationships;
cryptographic commitments;
authenticated proof structures;
checkpointing;
archival evidence;
and cross-domain coordination.

The first milestone implements FER only.

It does not yet implement MMR history commitments, authenticated state trees, Proof Capsules, Authority Domain materialization, authorization, archives, cross-domain transactions, multidimensional FER profiles, or production HashHelix integration.

Architecture-permitted does not mean implemented.

Implemented does not mean benchmarked.

Benchmarked does not mean production-ready.

See the Fractal Matrix Engine V1.0 whitepaper for the governing architecture.
