#!/usr/bin/env python3

import hashlib
import json
from pathlib import Path

PROFILE_ID = "FME-SRA-COMMITMENT-V1"
SRA_PROFILE_ID = b"FME-SRA-CANONICAL-V1"
FER_PROFILE_ID = b"FME-FER-AFFINE-2D-BINARY-V1"
DOMAIN = b"FME/SINGULARITY/V1"

SRA_MAGIC = b"FMES"
PREIMAGE_MAGIC = b"FMEP"

SUITES = {
    "sha2_256": (b"SHA2-256", hashlib.sha256),
    "sha2_512": (b"SHA2-512", hashlib.sha512),
}

def lp(value: bytes) -> bytes:
    return len(value).to_bytes(8, "big") + value

def encode_sra(namespace: bytes, suite: bytes, seed: bytes | None) -> bytes:
    out = (
        SRA_MAGIC + b"\x01"
        + lp(SRA_PROFILE_ID)
        + b"\x01"
        + lp(namespace)
        + lp(FER_PROFILE_ID)
        + lp(suite)
    )
    return out + (b"\x00" if seed is None else b"\x01" + lp(seed))

def encode_preimage(payload: bytes) -> bytes:
    return PREIMAGE_MAGIC + b"\x01" + lp(DOMAIN) + lp(payload)

CASES = [
    ("baseline_sha2_256_no_seed", b"example-system", "sha2_256", None),
    ("baseline_sha2_512_seed", b"example-system", "sha2_512", bytes.fromhex("000102ff")),
]

vectors = []

for case_id, namespace, suite_name, seed in CASES:
    suite_id, digest_fn = SUITES[suite_name]
    canonical = encode_sra(namespace, suite_id, seed)
    preimage = encode_preimage(canonical)
    digest = digest_fn(preimage).digest()

    vectors.append({
        "id": case_id,
        "hash_suite": suite_name,
        "hash_suite_identifier": suite_id.decode("ascii"),
        "canonical_sra_hex": canonical.hex(),
        "preimage_hex": preimage.hex(),
        "digest_hex": digest.hex(),
    })

fixture = {
    "fixture_format_version": 1,
    "profile_id": PROFILE_ID,
    "domain_identifier": DOMAIN.decode("ascii"),
    "provenance": {
        "phase": "010",
        "reference_method": "secondary-python-sra-preimage-hashlib",
        "reference_script": "scripts/conformance/generate_sra_commitment_v1.py",
        "independent_reproduction": False,
    },
    "commitment_vectors": vectors,
}

target = Path("conformance/sra-commitment-v1/sra-commitment-v1.json")
target.parent.mkdir(parents=True, exist_ok=True)
target.write_text(json.dumps(fixture, indent=2) + "\n")

print(f"wrote {target}")
print(f"commitment_vectors={len(vectors)}")
