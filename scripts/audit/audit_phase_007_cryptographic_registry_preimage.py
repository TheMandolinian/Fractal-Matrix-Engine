#!/usr/bin/env python3

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

fixture_path = (
    ROOT
    / "conformance"
    / "crypto-registry-v1"
    / "crypto-registry-v1.json"
)

fixture = json.loads(fixture_path.read_text())

MAGIC = b"FMEP"
VERSION = 1

DOMAINS = {
    "singularity_v1": b"FME/SINGULARITY/V1",
    "authority_domain_v1": b"FME/AUTHORITY_DOMAIN/V1",
}

HASH_SUITES = {
    "sha2_256": (b"SHA2-256", 32),
    "sha2_512": (b"SHA2-512", 64),
}


def fail(message: str) -> None:
    raise SystemExit(f"phase 007 cryptographic registry audit: FAIL — {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def encode_preimage(domain: bytes, payload: bytes) -> bytes:
    return (
        MAGIC
        + bytes([VERSION])
        + len(domain).to_bytes(8, "big")
        + domain
        + len(payload).to_bytes(8, "big")
        + payload
    )


require(
    fixture["profile_id"] == "FME-DOMAIN-PREIMAGE-V1",
    "preimage profile identifier mismatch",
)
require(
    fixture["domain_registry_id"] == "FME-CRYPTO-DOMAIN-REGISTRY-V1",
    "domain registry identifier mismatch",
)
require(
    fixture["hash_suite_registry_id"] == "FME-HASH-SUITE-REGISTRY-V1",
    "hash-suite registry identifier mismatch",
)
require(fixture["magic_ascii"] == "FMEP", "preimage magic mismatch")
require(fixture["version"] == 1, "preimage version mismatch")

domain_rows = {
    row["name"]: row
    for row in fixture["domains"]
}

require(set(domain_rows) == set(DOMAINS), "registered domain set mismatch")

for name, identifier in DOMAINS.items():
    row = domain_rows[name]
    require(row["identifier"] == identifier.decode("ascii"), f"{name} identifier mismatch")
    require(row["identifier_hex"] == identifier.hex(), f"{name} identifier hex mismatch")

suite_rows = {
    row["name"]: row
    for row in fixture["hash_suites"]
}

require(set(suite_rows) == set(HASH_SUITES), "registered hash-suite set mismatch")

for name, (identifier, digest_length) in HASH_SUITES.items():
    row = suite_rows[name]
    require(row["identifier"] == identifier.decode("ascii"), f"{name} identifier mismatch")
    require(row["identifier_hex"] == identifier.hex(), f"{name} identifier hex mismatch")
    require(row["digest_length"] == digest_length, f"{name} digest width mismatch")

print(
    "phase 007 cryptographic registry audit: "
    "2 domains / 2 hash suites: PASS"
)

require(len(fixture["preimage_vectors"]) == 3, "expected 3 positive preimage vectors")

for vector in fixture["preimage_vectors"]:
    domain = DOMAINS[vector["domain"]]
    payload = bytes.fromhex(vector["payload_hex"])
    reproduced = encode_preimage(domain, payload).hex()

    require(
        reproduced == vector["preimage_hex"],
        f"{vector['name']} secondary preimage reproduction mismatch",
    )

print(
    "phase 007 cryptographic registry audit: "
    "secondary preimage reproduction: PASS"
)

valid_sample = encode_preimage(
    DOMAINS["singularity_v1"],
    bytes.fromhex("000102ff"),
)

unknown_domain = b"FME/UNKNOWN/V1"

expected_malformed = {
    "invalid_magic": (
        b"X" + valid_sample[1:],
        "InvalidMagic",
    ),
    "unsupported_version": (
        valid_sample[:4] + b"\x02" + valid_sample[5:],
        "UnsupportedVersion",
    ),
    "unknown_domain": (
        MAGIC
        + bytes([VERSION])
        + len(unknown_domain).to_bytes(8, "big")
        + unknown_domain
        + (0).to_bytes(8, "big"),
        "UnknownDomain",
    ),
    "truncated_domain": (
        MAGIC
        + bytes([VERSION])
        + len(DOMAINS["singularity_v1"]).to_bytes(8, "big")
        + DOMAINS["singularity_v1"][:-1],
        "Truncated",
    ),
    "truncated_payload": (
        valid_sample[:-1],
        "Truncated",
    ),
    "trailing_bytes": (
        valid_sample + b"\x00",
        "TrailingBytes",
    ),
}

malformed_rows = {
    row["name"]: row
    for row in fixture["malformed_vectors"]
}

require(
    set(malformed_rows) == set(expected_malformed),
    "malformed vector set mismatch",
)

for name, (expected_bytes, expected_error) in expected_malformed.items():
    row = malformed_rows[name]

    require(
        row["preimage_hex"] == expected_bytes.hex(),
        f"{name} malformed bytes mismatch",
    )
    require(
        row["expected_error"] == expected_error,
        f"{name} expected error mismatch",
    )

print(
    "phase 007 cryptographic registry audit: "
    "6 malformed/rejection vectors: PASS"
)

for vector in fixture["preimage_vectors"]:
    preimage = bytes.fromhex(vector["preimage_hex"])

    require(
        b"SHA2-256" not in preimage and b"SHA2-512" not in preimage,
        f"{vector['name']} incorrectly embeds hash-suite identity",
    )

preimage_source = (
    ROOT / "crates" / "fme-crypto" / "src" / "preimage_v1.rs"
).read_text()

require(
    "HashSuiteId" not in preimage_source,
    "preimage implementation is coupled to hash-suite selection",
)

require(
    "sha2::" not in preimage_source
    and "Sha256" not in preimage_source
    and "Sha512" not in preimage_source,
    "Phase 007 preimage implementation executes digest algorithms",
)

print(
    "phase 007 cryptographic registry audit: "
    "domain/hash-suite separation and preimage boundary: PASS"
)

print("phase 007 cryptographic registry audit: PASS")
