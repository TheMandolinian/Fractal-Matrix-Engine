#!/usr/bin/env python3

import json
from pathlib import Path

PROFILE_ID = "FME-DOMAIN-PREIMAGE-V1"
DOMAIN_REGISTRY_ID = "FME-CRYPTO-DOMAIN-REGISTRY-V1"
HASH_SUITE_REGISTRY_ID = "FME-HASH-SUITE-REGISTRY-V1"

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


def encode_preimage(domain: bytes, payload: bytes) -> bytes:
    return (
        MAGIC
        + bytes([VERSION])
        + len(domain).to_bytes(8, "big")
        + domain
        + len(payload).to_bytes(8, "big")
        + payload
    )


vectors = [
    ("singularity_empty", "singularity_v1", b""),
    ("singularity_sample", "singularity_v1", bytes.fromhex("000102ff")),
    ("authority_domain_sample", "authority_domain_v1", bytes.fromhex("000102ff")),
]

valid_sample = encode_preimage(
    DOMAINS["singularity_v1"],
    bytes.fromhex("000102ff"),
)

unknown_domain = b"FME/UNKNOWN/V1"

malformed_vectors = [
    {
        "name": "invalid_magic",
        "preimage_hex": (b"X" + valid_sample[1:]).hex(),
        "expected_error": "InvalidMagic",
    },
    {
        "name": "unsupported_version",
        "preimage_hex": (valid_sample[:4] + b"\x02" + valid_sample[5:]).hex(),
        "expected_error": "UnsupportedVersion",
    },
    {
        "name": "unknown_domain",
        "preimage_hex": (
            MAGIC
            + bytes([VERSION])
            + len(unknown_domain).to_bytes(8, "big")
            + unknown_domain
            + (0).to_bytes(8, "big")
        ).hex(),
        "expected_error": "UnknownDomain",
    },
    {
        "name": "truncated_domain",
        "preimage_hex": (
            MAGIC
            + bytes([VERSION])
            + len(DOMAINS["singularity_v1"]).to_bytes(8, "big")
            + DOMAINS["singularity_v1"][:-1]
        ).hex(),
        "expected_error": "Truncated",
    },
    {
        "name": "truncated_payload",
        "preimage_hex": encode_preimage(
            DOMAINS["singularity_v1"],
            bytes.fromhex("000102ff"),
        )[:-1].hex(),
        "expected_error": "Truncated",
    },
    {
        "name": "trailing_bytes",
        "preimage_hex": (valid_sample + b"\x00").hex(),
        "expected_error": "TrailingBytes",
    },
]

document = {
    "profile_id": PROFILE_ID,
    "domain_registry_id": DOMAIN_REGISTRY_ID,
    "hash_suite_registry_id": HASH_SUITE_REGISTRY_ID,
    "magic_ascii": MAGIC.decode("ascii"),
    "version": VERSION,
    "domains": [
        {
            "name": name,
            "identifier": value.decode("ascii"),
            "identifier_hex": value.hex(),
        }
        for name, value in DOMAINS.items()
    ],
    "hash_suites": [
        {
            "name": name,
            "identifier": identifier.decode("ascii"),
            "identifier_hex": identifier.hex(),
            "digest_length": digest_length,
        }
        for name, (identifier, digest_length) in HASH_SUITES.items()
    ],
    "preimage_vectors": [
        {
            "name": name,
            "domain": domain_name,
            "payload_hex": payload.hex(),
            "preimage_hex": encode_preimage(DOMAINS[domain_name], payload).hex(),
        }
        for name, domain_name, payload in vectors
    ],
    "malformed_vectors": malformed_vectors,
}

output = Path("conformance/crypto-registry-v1/crypto-registry-v1.json")
output.write_text(json.dumps(document, indent=2) + "\n")

print(f"wrote {output}")
print(f"domains={len(document['domains'])}")
print(f"hash_suites={len(document['hash_suites'])}")
print(f"preimage_vectors={len(document['preimage_vectors'])}")
print(f"malformed_vectors={len(document['malformed_vectors'])}")
