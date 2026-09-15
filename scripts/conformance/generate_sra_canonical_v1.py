#!/usr/bin/env python3

import json
from pathlib import Path

PROFILE_ID = b"FME-SRA-CANONICAL-V1"
WIRE_FORMAT_ID = "FME-SRA-CANONICAL-V1-WIRE-V1"
FER_PROFILE_ID = b"FME-FER-AFFINE-2D-BINARY-V1"
HASH_SUITE_REGISTRY_ID = "FME-HASH-SUITE-REGISTRY-V1"

MAGIC = b"FMES"
WIRE_VERSION = 1
ARTIFACT_VERSION = 1

HASH_SUITES = {
    "sha2_256": b"SHA2-256",
    "sha2_512": b"SHA2-512",
}

def u64(value: int) -> bytes:
    if not 0 <= value <= (1 << 64) - 1:
        raise ValueError("value outside u64")
    return value.to_bytes(8, "big")


def length_prefixed(value: bytes) -> bytes:
    return u64(len(value)) + value


def encode_sra(
    namespace: bytes,
    hash_suite: bytes,
    root_seed: bytes | None,
) -> bytes:
    if not namespace:
        raise ValueError("namespace must not be empty")

    if root_seed == b"":
        raise ValueError("present root seed must not be empty")

    output = (
        MAGIC
        + bytes([WIRE_VERSION])
        + length_prefixed(PROFILE_ID)
        + bytes([ARTIFACT_VERSION])
        + length_prefixed(namespace)
        + length_prefixed(FER_PROFILE_ID)
        + length_prefixed(hash_suite)
    )

    if root_seed is None:
        return output + b"\x00"

    return output + b"\x01" + length_prefixed(root_seed)


def hx(value: bytes) -> str:
    return value.hex()


VALID_CASES = [
    (
        "baseline_sha2_256_no_seed",
        b"example-system",
        "sha2_256",
        None,
    ),
    (
        "baseline_sha2_512_seed",
        b"example-system",
        "sha2_512",
        bytes.fromhex("000102ff"),
    ),
    (
        "binary_namespace",
        bytes.fromhex("000102ff"),
        "sha2_256",
        None,
    ),
    (
        "binary_root_seed",
        b"binary-seed-system",
        "sha2_512",
        bytes.fromhex("00ff1080007f"),
    ),
]


sra_vectors = []

for case_id, namespace, suite_name, root_seed in VALID_CASES:
    suite_bytes = HASH_SUITES[suite_name]
    canonical = encode_sra(namespace, suite_bytes, root_seed)

    sra_vectors.append(
        {
            "id": case_id,
            "system_namespace_hex": hx(namespace),
            "default_hash_suite": suite_name,
            "default_hash_suite_identifier": suite_bytes.decode("ascii"),
            "root_seed_present": root_seed is not None,
            "root_seed_hex": None if root_seed is None else hx(root_seed),
            "canonical_hex": hx(canonical),
        }
    )

def raw_sra(
    magic: bytes = MAGIC,
    wire_version: int = WIRE_VERSION,
    profile_id: bytes = PROFILE_ID,
    artifact_version: int = ARTIFACT_VERSION,
    namespace: bytes = b"example-system",
    fer_profile_id: bytes = FER_PROFILE_ID,
    hash_suite: bytes = HASH_SUITES["sha2_256"],
    root_seed_presence: int = 0,
    root_seed: bytes | None = None,
) -> bytes:
    output = (
        magic
        + bytes([wire_version])
        + length_prefixed(profile_id)
        + bytes([artifact_version])
        + length_prefixed(namespace)
        + length_prefixed(fer_profile_id)
        + length_prefixed(hash_suite)
        + bytes([root_seed_presence])
    )

    if root_seed is not None:
        output += length_prefixed(root_seed)

    return output


valid_seeded = encode_sra(
    b"example-system",
    HASH_SUITES["sha2_512"],
    bytes.fromhex("000102ff"),
)

failure_vectors = [
    {
        "id": "invalid_magic",
        "hex": hx(raw_sra(magic=b"BAD!")),
        "expected_error": "InvalidMagic",
    },
    {
        "id": "unsupported_wire_version",
        "hex": hx(raw_sra(wire_version=2)),
        "expected_error": "UnsupportedWireVersion",
    },
    {
        "id": "canonical_profile_mismatch",
        "hex": hx(raw_sra(profile_id=b"FME-SRA-CANONICAL-V2")),
        "expected_error": "CanonicalProfileMismatch",
    },
    {
        "id": "unsupported_artifact_version",
        "hex": hx(raw_sra(artifact_version=2)),
        "expected_error": "UnsupportedArtifactVersion",
    },
    {
        "id": "empty_system_namespace",
        "hex": hx(raw_sra(namespace=b"")),
        "expected_error": "EmptySystemNamespace",
    },
    {
        "id": "fer_profile_mismatch",
        "hex": hx(raw_sra(fer_profile_id=b"FME-FER-UNKNOWN-V1")),
        "expected_error": "FerProfileMismatch",
    },
    {
        "id": "unknown_hash_suite",
        "hex": hx(raw_sra(hash_suite=b"SHA3-256")),
        "expected_error": "UnknownHashSuiteIdentifier",
    },
    {
        "id": "invalid_root_seed_presence",
        "hex": hx(raw_sra(root_seed_presence=2)),
        "expected_error": "InvalidRootSeedPresence",
    },
    {
        "id": "present_empty_root_seed",
        "hex": hx(raw_sra(root_seed_presence=1, root_seed=b"")),
        "expected_error": "EmptyRootSeed",
    },
    {
        "id": "truncated",
        "hex": hx(valid_seeded[:-1]),
        "expected_error": "Truncated",
    },
    {
        "id": "trailing_bytes",
        "hex": hx(raw_sra() + b"\x00"),
        "expected_error": "TrailingBytes",
    },
]


fixture = {
    "fixture_format_version": 1,
    "normative_wire_format": True,
    "wire_format_id": WIRE_FORMAT_ID,
    "profile_id": PROFILE_ID.decode("ascii"),
    "fer_profile_id": FER_PROFILE_ID.decode("ascii"),
    "hash_suite_registry_id": HASH_SUITE_REGISTRY_ID,
    "magic_ascii": MAGIC.decode("ascii"),
    "wire_version": WIRE_VERSION,
    "artifact_version": ARTIFACT_VERSION,
    "provenance": {
        "phase": "009",
        "reference_method": "secondary-python-byte-encoder",
        "reference_script": "scripts/conformance/generate_sra_canonical_v1.py",
        "independent_reproduction": False,
    },
    "sra_vectors": sra_vectors,
    "failure_vectors": failure_vectors,
}

target = Path("conformance/sra-canonical-v1/sra-canonical-v1.json")
target.write_text(json.dumps(fixture, indent=2) + "\n")

print(f"wrote {target}")
print(f"sra_vectors={len(sra_vectors)}")
print(f"failure_vectors={len(failure_vectors)}")
