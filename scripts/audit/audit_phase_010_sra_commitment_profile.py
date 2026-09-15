#!/usr/bin/env python3

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

PROFILE_ID = "FME-SRA-COMMITMENT-V1"
DOMAIN = b"FME/SINGULARITY/V1"
SRA_PROFILE = b"FME-SRA-CANONICAL-V1"
FER_PROFILE = b"FME-FER-AFFINE-2D-BINARY-V1"

FIXTURE = ROOT / "conformance/sra-commitment-v1/sra-commitment-v1.json"
SPEC = ROOT / "conformance/sra-commitment-v1/SRA_COMMITMENT_V1.md"
RUST = ROOT / "crates/fme-sra/src/commitment_v1.rs"
LIB = ROOT / "crates/fme-sra/src/lib.rs"
UNIT_TEST = ROOT / "crates/fme-sra/tests/commitment_v1.rs"
CONF_TEST = ROOT / "crates/fme-sra/tests/commitment_conformance_v1.rs"
GENERATOR = ROOT / "scripts/conformance/generate_sra_commitment_v1.py"

SUITES = {
    "sha2_256": (b"SHA2-256", hashlib.sha256),
    "sha2_512": (b"SHA2-512", hashlib.sha512),
}

CASES = {
    "baseline_sha2_256_no_seed": (
        b"example-system",
        "sha2_256",
        None,
    ),
    "baseline_sha2_512_seed": (
        b"example-system",
        "sha2_512",
        bytes.fromhex("000102ff"),
    ),
}


def fail(message: str) -> None:
    raise SystemExit(f"phase 010 SRA commitment profile audit: FAIL — {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def lp(value: bytes) -> bytes:
    return len(value).to_bytes(8, "big") + value


def encode_sra(namespace: bytes, suite: bytes, seed: bytes | None) -> bytes:
    out = (
        b"FMES"
        + b"\x01"
        + lp(SRA_PROFILE)
        + b"\x01"
        + lp(namespace)
        + lp(FER_PROFILE)
        + lp(suite)
    )
    return out + (b"\x00" if seed is None else b"\x01" + lp(seed))


def encode_preimage(payload: bytes) -> bytes:
    return b"FMEP" + b"\x01" + lp(DOMAIN) + lp(payload)


for path in (
    FIXTURE,
    SPEC,
    RUST,
    LIB,
    UNIT_TEST,
    CONF_TEST,
    GENERATOR,
):
    require(path.is_file(), f"missing required file: {path.relative_to(ROOT)}")

fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))

require(fixture["fixture_format_version"] == 1, "fixture version mismatch")
require(fixture["profile_id"] == PROFILE_ID, "commitment profile mismatch")
require(
    fixture["domain_identifier"] == DOMAIN.decode("ascii"),
    "domain identifier mismatch",
)

provenance = fixture["provenance"]
require(provenance["phase"] == "010", "fixture phase mismatch")
require(
    provenance["reference_method"] == "secondary-python-sra-preimage-hashlib",
    "reference method mismatch",
)
require(
    provenance["reference_script"]
    == "scripts/conformance/generate_sra_commitment_v1.py",
    "reference script mismatch",
)
require(
    provenance["independent_reproduction"] is False,
    "repository-local generator must not claim independent reproduction",
)

vectors = fixture["commitment_vectors"]
require(len(vectors) == 2, "expected exactly 2 commitment vectors")

for vector in vectors:
    vector_id = vector["id"]
    require(vector_id in CASES, f"unknown vector: {vector_id}")

    namespace, suite_name, seed = CASES[vector_id]
    require(suite_name in SUITES, f"unknown suite: {suite_name}")

    suite_id, digest_fn = SUITES[suite_name]

    require(vector["hash_suite"] == suite_name, f"suite mismatch: {vector_id}")
    require(
        vector["hash_suite_identifier"] == suite_id.decode("ascii"),
        f"suite identifier mismatch: {vector_id}",
    )

    canonical = encode_sra(namespace, suite_id, seed)
    preimage = encode_preimage(canonical)
    digest = digest_fn(preimage).digest()

    require(
        vector["canonical_sra_hex"] == canonical.hex(),
        f"canonical SRA mismatch: {vector_id}",
    )
    require(
        vector["preimage_hex"] == preimage.hex(),
        f"preimage mismatch: {vector_id}",
    )
    require(
        vector["digest_hex"] == digest.hex(),
        f"digest mismatch: {vector_id}",
    )

print(
    "phase 010 SRA commitment profile audit: "
    "2 commitment vectors independently reproduced: PASS"
)

rust = RUST.read_text(encoding="utf-8")

for token in (
    'pub const SRA_COMMITMENT_PROFILE_ID: &str = "FME-SRA-COMMITMENT-V1";',
    "pub struct SraCommitmentV1",
    "pub fn commit_sra",
    "encode_sra(artifact)",
    "encode_preimage(DomainId::SingularityV1",
    "artifact.default_hash_suite_id()",
    "digest_preimage(hash_suite, &preimage)",
):
    require(token in rust, f"Rust commitment implementation missing: {token}")

for forbidden in (
    "Sha256::digest",
    "Sha512::digest",
    "sha2::",
    "StableAuthority",
    "AuthorityDomain",
    "MMR",
):
    require(
        forbidden not in rust,
        f"Phase 010 implementation crosses bounded scope: {forbidden}",
    )

lib = LIB.read_text(encoding="utf-8")
require(
    "Phase 010 adds the baseline cryptographic SRA commitment profile." in lib,
    "crate documentation does not acknowledge Phase 010",
)

unit_test = UNIT_TEST.read_text(encoding="utf-8")
for token in (
    "commitment_uses_declared_sra_hash_suite",
    "commitment_exactly_composes_phases_009_007_and_008",
    "commitment_is_deterministic",
):
    require(token in unit_test, f"unit test missing: {token}")

conf_test = CONF_TEST.read_text(encoding="utf-8")
for token in (
    "../../../conformance/sra-commitment-v1/sra-commitment-v1.json",
    "canonical_sra_hex",
    "preimage_hex",
    "digest_hex",
    "phase_010_commitment_vectors_match_rust",
):
    require(token in conf_test, f"conformance test missing: {token}")

spec = SPEC.read_text(encoding="utf-8")
for token in (
    "FME-SRA-COMMITMENT-V1",
    "FME/SINGULARITY/V1",
    "`serialization != commitment`",
    "`digest != commitment semantics`",
    "`SRA commitment != SRA stable identity`",
    "`root artifact != operational trunk`",
    "`mathematical possibility != materialization`",
):
    require(token in spec, f"spec boundary missing: {token}")

generator = GENERATOR.read_text(encoding="utf-8")
for token in (
    'PROFILE_ID = "FME-SRA-COMMITMENT-V1"',
    'DOMAIN = b"FME/SINGULARITY/V1"',
    "hashlib.sha256",
    "hashlib.sha512",
    '"independent_reproduction": False',
):
    require(token in generator, f"generator contract missing: {token}")

for forbidden in ("subprocess", "cargo"):
    require(
        forbidden not in generator,
        f"secondary generator improperly delegates to tooling: {forbidden}",
    )

print(
    "phase 010 SRA commitment profile audit: "
    "Rust/spec/test/fixture boundary alignment: PASS"
)
print(
    "phase 010 SRA commitment profile audit: "
    "serialization/preimage/digest/commitment boundaries: PASS"
)
print("phase 010 SRA commitment profile audit: PASS")
