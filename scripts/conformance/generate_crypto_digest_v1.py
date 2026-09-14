#!/usr/bin/env python3

import hashlib
import json
from pathlib import Path

PROFILE_ID = "FME-REGISTERED-SHA2-DIGEST-V1"
PREIMAGE_PROFILE_ID = "FME-DOMAIN-PREIMAGE-V1"
HASH_SUITE_REGISTRY_ID = "FME-HASH-SUITE-REGISTRY-V1"

MAGIC = b"FMEP"
VERSION = 1

DOMAINS = {
    "singularity_v1": b"FME/SINGULARITY/V1",
    "authority_domain_v1": b"FME/AUTHORITY_DOMAIN/V1",
}

HASH_SUITES = {
    "sha2_256": {
        "identifier": b"SHA2-256",
        "digest_length": 32,
        "digest": hashlib.sha256,
    },
    "sha2_512": {
        "identifier": b"SHA2-512",
        "digest_length": 64,
        "digest": hashlib.sha512,
    },
}


def encode_preimage(domain: bytes, payload: bytes) -> bytes:
    return (
        MAGIC
        + bytes([VERSION])
        + len(domain).to_bytes(8, "big")
        + domain
        + len(payload).to_bytes(8, "big")
        + payload
    )


base_vectors = [
    ("singularity_empty", "singularity_v1", b""),
    ("singularity_sample", "singularity_v1", bytes.fromhex("000102ff")),
    ("authority_domain_sample", "authority_domain_v1", bytes.fromhex("000102ff")),
]

digest_vectors = []

for base_name, domain_name, payload in base_vectors:
    preimage = encode_preimage(DOMAINS[domain_name], payload)

    for suite_name, suite in HASH_SUITES.items():
        digest = suite["digest"](preimage).digest()

        if len(digest) != suite["digest_length"]:
            raise SystemExit(
                f"{suite_name} digest width mismatch: "
                f"{len(digest)} != {suite['digest_length']}"
            )

        if suite["identifier"] in preimage:
            raise SystemExit(
                f"{base_name} preimage unexpectedly embeds {suite_name}"
            )

        digest_vectors.append(
            {
                "name": f"{base_name}_{suite_name}",
                "hash_suite": suite_name,
                "hash_suite_identifier": suite["identifier"].decode("ascii"),
                "digest_length": suite["digest_length"],
                "domain": domain_name,
                "payload_hex": payload.hex(),
                "preimage_hex": preimage.hex(),
                "digest_hex": digest.hex(),
            }
        )

document = {
    "profile_id": PROFILE_ID,
    "preimage_profile_id": PREIMAGE_PROFILE_ID,
    "hash_suite_registry_id": HASH_SUITE_REGISTRY_ID,
    "digest_vectors": digest_vectors,
}

output = Path("conformance/crypto-digest-v1/crypto-digest-v1.json")
output.write_text(json.dumps(document, indent=2) + "\n")

print(f"wrote {output}")
print(f"digest_vectors={len(digest_vectors)}")
print("sha2_256_vectors=3")
print("sha2_512_vectors=3")
