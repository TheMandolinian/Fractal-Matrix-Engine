#!/usr/bin/env python3

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

WIRE_ID = "FME-SRA-CANONICAL-V1-WIRE-V1"
PROFILE_ID = "FME-SRA-CANONICAL-V1"
FER_PROFILE_ID = "FME-FER-AFFINE-2D-BINARY-V1"
HASH_SUITE_REGISTRY_ID = "FME-HASH-SUITE-REGISTRY-V1"

MAGIC = b"FMES"
WIRE_VERSION = 1
ARTIFACT_VERSION = 1

HASH_SUITES = {
    "sha2_256": b"SHA2-256",
    "sha2_512": b"SHA2-512",
}

FIXTURE = (
    ROOT
    / "conformance"
    / "sra-canonical-v1"
    / "sra-canonical-v1.json"
)

SPEC = (
    ROOT
    / "conformance"
    / "sra-canonical-v1"
    / "CANONICAL_WIRE_V1.md"
)

RUST = ROOT / "crates" / "fme-sra" / "src" / "sra_v1.rs"

LIB = ROOT / "crates" / "fme-sra" / "src" / "lib.rs"

ERROR = ROOT / "crates" / "fme-sra" / "src" / "error.rs"

CANONICAL_TEST = (
    ROOT
    / "crates"
    / "fme-sra"
    / "tests"
    / "canonical_v1.rs"
)

CONFORMANCE_TEST = (
    ROOT
    / "crates"
    / "fme-sra"
    / "tests"
    / "conformance_v1.rs"
)

GENERATOR = (
    ROOT
    / "scripts"
    / "conformance"
    / "generate_sra_canonical_v1.py"
)


def fail(message: str) -> None:
    raise SystemExit(
        f"phase 009 SRA canonical serialization audit: FAIL — {message}"
    )


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def u64(value: int) -> bytes:
    require(0 <= value <= (1 << 64) - 1, "reference u64 overflow")
    return value.to_bytes(8, "big")


def length_prefixed(value: bytes) -> bytes:
    return u64(len(value)) + value


def encode_sra(
    system_namespace: bytes,
    hash_suite_identifier: bytes,
    root_seed: bytes | None,
) -> bytes:
    require(system_namespace != b"", "reference namespace must be non-empty")
    require(
        hash_suite_identifier in HASH_SUITES.values(),
        "reference hash suite must be registered",
    )
    require(root_seed != b"", "reference root seed must not be present-empty")

    output = (
        MAGIC
        + bytes([WIRE_VERSION])
        + length_prefixed(PROFILE_ID.encode("ascii"))
        + bytes([ARTIFACT_VERSION])
        + length_prefixed(system_namespace)
        + length_prefixed(FER_PROFILE_ID.encode("ascii"))
        + length_prefixed(hash_suite_identifier)
    )

    if root_seed is None:
        return output + b"\x00"

    return output + b"\x01" + length_prefixed(root_seed)


for required in (
    FIXTURE,
    SPEC,
    RUST,
    LIB,
    ERROR,
    CANONICAL_TEST,
    CONFORMANCE_TEST,
    GENERATOR,
):
    require(
        required.is_file(),
        f"missing required file: {required.relative_to(ROOT)}",
    )


fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))

require(
    fixture["fixture_format_version"] == 1,
    "unexpected fixture format version",
)
require(
    fixture["normative_wire_format"] is True,
    "SRA wire fixture must be normative",
)
require(
    fixture["wire_format_id"] == WIRE_ID,
    "wire-format identifier mismatch",
)
require(
    fixture["profile_id"] == PROFILE_ID,
    "canonical profile identifier mismatch",
)
require(
    fixture["fer_profile_id"] == FER_PROFILE_ID,
    "FER profile identifier mismatch",
)
require(
    fixture["hash_suite_registry_id"] == HASH_SUITE_REGISTRY_ID,
    "hash-suite registry identifier mismatch",
)
require(
    fixture["magic_ascii"] == MAGIC.decode("ascii"),
    "SRA magic mismatch",
)
require(
    fixture["wire_version"] == WIRE_VERSION,
    "wire version mismatch",
)
require(
    fixture["artifact_version"] == ARTIFACT_VERSION,
    "artifact version mismatch",
)

provenance = fixture["provenance"]

require(provenance["phase"] == "009", "fixture phase mismatch")
require(
    provenance["reference_method"] == "secondary-python-byte-encoder",
    "unexpected reference method",
)
require(
    provenance["reference_script"]
    == "scripts/conformance/generate_sra_canonical_v1.py",
    "reference script mismatch",
)
require(
    provenance["independent_reproduction"] is False,
    "repository-local generator must not claim independent reproduction",
)

vectors = fixture["sra_vectors"]
failures = fixture["failure_vectors"]

require(len(vectors) == 4, "expected exactly 4 normative SRA vectors")
require(len(failures) == 11, "expected exactly 11 malformed SRA vectors")


for vector in vectors:
    vector_id = vector["id"]
    suite_name = vector["default_hash_suite"]

    require(
        suite_name in HASH_SUITES,
        f"unknown fixture hash suite: {suite_name}",
    )

    suite_identifier = HASH_SUITES[suite_name]

    require(
        vector["default_hash_suite_identifier"]
        == suite_identifier.decode("ascii"),
        f"hash-suite identifier mismatch: {vector_id}",
    )

    namespace = bytes.fromhex(vector["system_namespace_hex"])

    root_seed = (
        None
        if vector["root_seed_hex"] is None
        else bytes.fromhex(vector["root_seed_hex"])
    )

    require(
        vector["root_seed_present"] == (root_seed is not None),
        f"root-seed presence mismatch: {vector_id}",
    )

    expected = encode_sra(
        namespace,
        suite_identifier,
        root_seed,
    )

    require(
        vector["canonical_hex"] == expected.hex(),
        f"canonical byte mismatch: {vector_id}",
    )


print(
    "phase 009 SRA canonical serialization audit: "
    "4 normative vectors independently reproduced: PASS"
)


required_failures = {
    "invalid_magic": "InvalidMagic",
    "unsupported_wire_version": "UnsupportedWireVersion",
    "canonical_profile_mismatch": "CanonicalProfileMismatch",
    "unsupported_artifact_version": "UnsupportedArtifactVersion",
    "empty_system_namespace": "EmptySystemNamespace",
    "fer_profile_mismatch": "FerProfileMismatch",
    "unknown_hash_suite": "UnknownHashSuiteIdentifier",
    "invalid_root_seed_presence": "InvalidRootSeedPresence",
    "present_empty_root_seed": "EmptyRootSeed",
    "truncated": "Truncated",
    "trailing_bytes": "TrailingBytes",
}

actual_failures = {
    vector["id"]: vector["expected_error"]
    for vector in failures
}

require(
    actual_failures == required_failures,
    "malformed vector coverage mismatch",
)

print(
    "phase 009 SRA canonical serialization audit: "
    "11 malformed/rejection vectors: PASS"
)


rust = RUST.read_text(encoding="utf-8")

for token in (
    'pub const SRA_MAGIC: [u8; 4] = *b"FMES";',
    'pub const SRA_WIRE_VERSION: u8 = 0x01;',
    'pub const SRA_ARTIFACT_VERSION: u8 = 0x01;',
    'pub const SRA_CANONICAL_PROFILE_ID: &str = "FME-SRA-CANONICAL-V1";',
    'pub const SRA_WIRE_FORMAT_ID: &str = "FME-SRA-CANONICAL-V1-WIRE-V1";',
    "pub fn encode_sra",
    "pub fn decode_sra",
    "append_length_prefixed",
    "to_be_bytes()",
    "u64::from_be_bytes",
    "HashSuiteId::from_bytes",
    "SraError::InvalidMagic",
    "SraError::CanonicalProfileMismatch",
    "SraError::FerProfileMismatch",
    "SraError::UnknownHashSuiteIdentifier",
    "SraError::InvalidRootSeedPresence",
    "SraError::TrailingBytes",
):
    require(
        token in rust,
        f"Rust SRA implementation missing required token: {token}",
    )


for forbidden in (
    "Sha256::digest",
    "Sha512::digest",
    "sha2::",
    "encode_preimage",
    "FME/SINGULARITY/V1",
):
    require(
        forbidden not in rust,
        f"Phase 009 serialization incorrectly crosses commitment boundary: {forbidden}",
    )


lib = LIB.read_text(encoding="utf-8")

for token in (
    "Phase 010 adds the baseline cryptographic SRA commitment profile.",
    "Stable Authority Identities",
    "Authority Domain materialization",
    "accepted-history commitments",
    "MMRs",
    "Proof Capsules",
):
    require(
        token in lib,
        f"crate boundary documentation missing: {token}",
    )


error_source = ERROR.read_text(encoding="utf-8")

for token in (
    "EmptySystemNamespace",
    "EmptyRootSeed",
    "Truncated",
    "InvalidMagic",
    "UnsupportedWireVersion",
    "CanonicalProfileMismatch",
    "UnsupportedArtifactVersion",
    "FerProfileMismatch",
    "UnknownHashSuiteIdentifier",
    "InvalidRootSeedPresence",
    "LengthOverflow",
    "TrailingBytes",
):
    require(
        token in error_source,
        f"SRA error contract missing: {token}",
    )


canonical_test = CANONICAL_TEST.read_text(encoding="utf-8")

for token in (
    "sha2_256_without_root_seed_round_trips",
    "sha2_512_with_root_seed_round_trips",
    "encoding_is_deterministic",
    "decoded_bytes_reencode_exactly",
    "rejects_invalid_magic",
    "rejects_unsupported_wire_version",
    "rejects_canonical_profile_mismatch",
    "rejects_unsupported_artifact_version",
    "rejects_fer_profile_mismatch",
    "rejects_unknown_hash_suite",
    "rejects_invalid_root_seed_presence",
    "rejects_encoded_empty_namespace",
    "rejects_encoded_present_empty_root_seed",
    "rejects_truncated_input",
    "rejects_trailing_bytes",
):
    require(
        token in canonical_test,
        f"canonical Rust test missing: {token}",
    )


conformance_test = CONFORMANCE_TEST.read_text(encoding="utf-8")

for token in (
    "../../../conformance/sra-canonical-v1/sra-canonical-v1.json",
    "phase_009_fixture_metadata_matches_normative_contract",
    "phase_009_valid_normative_vectors_match_rust_canonical_encoding",
    "phase_009_malformed_vectors_fail_with_expected_error",
):
    require(
        token in conformance_test,
        f"conformance test boundary missing: {token}",
    )


spec = SPEC.read_text(encoding="utf-8")

for token in (
    "FME-SRA-CANONICAL-V1-WIRE-V1",
    "unsigned big-endian",
    "system namespace",
    "SHA2-256",
    "SHA2-512",
    "root-seed presence",
    "one valid baseline SRA semantic object",
    "`serialization != commitment`",
    "`digest execution != commitment semantics`",
    "`root artifact != operational trunk`",
    "`mathematical possibility != materialization`",
    "It does not constitute independent reproduction",
):
    require(
        token in spec,
        f"normative wire specification missing boundary: {token}",
    )


generator = GENERATOR.read_text(encoding="utf-8")

for token in (
    'MAGIC = b"FMES"',
    "WIRE_VERSION = 1",
    "ARTIFACT_VERSION = 1",
    'b"SHA2-256"',
    'b"SHA2-512"',
    'reference_method": "secondary-python-byte-encoder"',
    '"independent_reproduction": False',
):
    require(
        token in generator,
        f"secondary generator missing required contract token: {token}",
    )


for forbidden in (
    "subprocess",
    "cargo",
    "encode_sra(",
):
    if forbidden == "encode_sra(":
        continue

    require(
        forbidden not in generator,
        f"secondary generator improperly delegates to Rust/tooling: {forbidden}",
    )


print(
    "phase 009 SRA canonical serialization audit: "
    "Rust/spec/test/fixture boundary alignment: PASS"
)

print(
    "phase 009 SRA canonical serialization audit: "
    "serialization/commitment separation: PASS"
)

print("phase 009 SRA canonical serialization audit: PASS")
