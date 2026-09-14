#!/usr/bin/env python3

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

DIGEST_FIXTURE = (
    ROOT
    / "conformance"
    / "crypto-digest-v1"
    / "crypto-digest-v1.json"
)

PHASE_007_FIXTURE = (
    ROOT
    / "conformance"
    / "crypto-registry-v1"
    / "crypto-registry-v1.json"
)

fixture = json.loads(DIGEST_FIXTURE.read_text())
phase_007_fixture = json.loads(PHASE_007_FIXTURE.read_text())

MAGIC = b"FMEP"
VERSION = 1

DOMAINS = {
    "singularity_v1": b"FME/SINGULARITY/V1",
    "authority_domain_v1": b"FME/AUTHORITY_DOMAIN/V1",
}

HASH_SUITES = {
    "sha2_256": {
        "identifier": "SHA2-256",
        "digest_length": 32,
        "digest": hashlib.sha256,
    },
    "sha2_512": {
        "identifier": "SHA2-512",
        "digest_length": 64,
        "digest": hashlib.sha512,
    },
}


def fail(message: str) -> None:
    raise SystemExit(f"phase 008 registered SHA-2 digest audit: FAIL — {message}")


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
    fixture["profile_id"] == "FME-REGISTERED-SHA2-DIGEST-V1",
    "digest profile identifier mismatch",
)
require(
    fixture["preimage_profile_id"] == "FME-DOMAIN-PREIMAGE-V1",
    "preimage profile identifier mismatch",
)
require(
    fixture["hash_suite_registry_id"] == "FME-HASH-SUITE-REGISTRY-V1",
    "hash-suite registry identifier mismatch",
)

vectors = fixture["digest_vectors"]

expected_names = {
    "singularity_empty_sha2_256",
    "singularity_empty_sha2_512",
    "singularity_sample_sha2_256",
    "singularity_sample_sha2_512",
    "authority_domain_sample_sha2_256",
    "authority_domain_sample_sha2_512",
}

require(len(vectors) == 6, "expected exactly 6 digest vectors")
require(
    {row["name"] for row in vectors} == expected_names,
    "digest vector set mismatch",
)

print(
    "phase 008 registered SHA-2 digest audit: "
    "6-vector profile structure: PASS"
)

grouped = {}

for vector in vectors:
    domain_name = vector["domain"]
    suite_name = vector["hash_suite"]

    require(domain_name in DOMAINS, f"unknown fixture domain {domain_name}")
    require(suite_name in HASH_SUITES, f"unknown fixture suite {suite_name}")

    suite = HASH_SUITES[suite_name]
    payload = bytes.fromhex(vector["payload_hex"])
    reproduced_preimage = encode_preimage(DOMAINS[domain_name], payload)

    require(
        vector["hash_suite_identifier"] == suite["identifier"],
        f"{vector['name']} suite identifier mismatch",
    )
    require(
        vector["digest_length"] == suite["digest_length"],
        f"{vector['name']} digest width declaration mismatch",
    )
    require(
        vector["preimage_hex"] == reproduced_preimage.hex(),
        f"{vector['name']} independent preimage reproduction mismatch",
    )

    require(
        b"SHA2-256" not in reproduced_preimage
        and b"SHA2-512" not in reproduced_preimage,
        f"{vector['name']} incorrectly embeds hash-suite identity",
    )

    reproduced_digest = suite["digest"](reproduced_preimage).digest()

    require(
        len(reproduced_digest) == suite["digest_length"],
        f"{vector['name']} reproduced digest width mismatch",
    )
    require(
        vector["digest_hex"] == reproduced_digest.hex(),
        f"{vector['name']} independent digest reproduction mismatch",
    )

    key = (domain_name, vector["payload_hex"])
    grouped.setdefault(key, {})[suite_name] = vector

require(len(grouped) == 3, "expected three canonical preimages")

for key, rows in grouped.items():
    require(
        set(rows) == set(HASH_SUITES),
        f"{key} is not represented under both registered suites",
    )
    require(
        rows["sha2_256"]["preimage_hex"] == rows["sha2_512"]["preimage_hex"],
        f"{key} changed preimage bytes between hash suites",
    )

print(
    "phase 008 registered SHA-2 digest audit: "
    "independent SHA2-256/SHA2-512 reproduction: PASS"
)

rows_by_name = {row["name"]: row for row in vectors}

for suite_name in HASH_SUITES:
    singularity = bytes.fromhex(
        rows_by_name[f"singularity_sample_{suite_name}"]["digest_hex"]
    )
    authority = bytes.fromhex(
        rows_by_name[f"authority_domain_sample_{suite_name}"]["digest_hex"]
    )

    require(
        singularity != authority,
        f"domain separation failed for {suite_name}",
    )

print(
    "phase 008 registered SHA-2 digest audit: "
    "domain-separated digest distinction: PASS"
)

phase_007_rows = {
    row["name"]: row
    for row in phase_007_fixture["preimage_vectors"]
}

require(
    set(phase_007_rows)
    == {
        "singularity_empty",
        "singularity_sample",
        "authority_domain_sample",
    },
    "Phase 007 positive preimage vector set changed unexpectedly",
)

for base_name, phase_007_row in phase_007_rows.items():
    sha256_row = rows_by_name[f"{base_name}_sha2_256"]
    sha512_row = rows_by_name[f"{base_name}_sha2_512"]

    require(
        sha256_row["preimage_hex"] == phase_007_row["preimage_hex"],
        f"{base_name} SHA2-256 preimage diverges from Phase 007",
    )
    require(
        sha512_row["preimage_hex"] == phase_007_row["preimage_hex"],
        f"{base_name} SHA2-512 preimage diverges from Phase 007",
    )

preimage_source = (
    ROOT / "crates" / "fme-crypto" / "src" / "preimage_v1.rs"
).read_text()

require(
    "HashSuiteId" not in preimage_source,
    "Phase 007 preimage implementation became coupled to hash-suite selection",
)

print(
    "phase 008 registered SHA-2 digest audit: "
    "Phase 007 preimage preservation: PASS"
)

digest_source = (
    ROOT / "crates" / "fme-crypto" / "src" / "digest_v1.rs"
).read_text()

decode_position = digest_source.find("decode_preimage(preimage)?;")
suite_position = digest_source.find("match hash_suite")

require(
    decode_position >= 0,
    "digest implementation does not validate Phase 007 preimage bytes",
)
require(
    suite_position >= 0,
    "digest implementation does not explicitly select registered suite",
)
require(
    decode_position < suite_position,
    "digest execution selects suite before validating canonical preimage",
)
require(
    "Sha256::digest(preimage)" in digest_source,
    "SHA2-256 does not process exact supplied preimage bytes",
)
require(
    "Sha512::digest(preimage)" in digest_source,
    "SHA2-512 does not process exact supplied preimage bytes",
)

test_source = (
    ROOT / "crates" / "fme-crypto" / "tests" / "digest_v1.rs"
).read_text()

require(
    "../../../conformance/crypto-registry-v1/crypto-registry-v1.json"
    in test_source,
    "Phase 008 tests do not consume Phase 007 malformed-preimage evidence",
)
require(
    "malformed_phase_007_preimages_fail_before_digest_execution"
    in test_source,
    "malformed preimage rejection test missing",
)

manifest = (
    ROOT / "crates" / "fme-crypto" / "Cargo.toml"
).read_text()

dependencies_block = manifest.split("[dependencies]", 1)[1].split(
    "[dev-dependencies]", 1
)[0]

dependency_lines = [
    line.strip()
    for line in dependencies_block.splitlines()
    if line.strip()
]

require(
    dependency_lines
    == [
        'sha2 = { version = "=0.11.0", default-features = false }'
    ],
    "fme-crypto production dependency boundary is not the expected pinned sha2 dependency",
)

print(
    "phase 008 registered SHA-2 digest audit: "
    "validation and dependency boundary: PASS"
)

print("phase 008 registered SHA-2 digest audit: PASS")
