#!/usr/bin/env python3

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

PROFILE = "FME-FER-AFFINE-2D-BINARY-V1"
WIRE_ID = "FME-FER-AFFINE-2D-BINARY-V1-WIRE-V1"

FIXTURE = (
    ROOT
    / "conformance"
    / "fer-affine-2d-binary-v1"
    / "canonical-wire-v1.json"
)
SPEC = (
    ROOT
    / "conformance"
    / "fer-affine-2d-binary-v1"
    / "CANONICAL_WIRE_V1.md"
)
RUST = ROOT / "crates" / "fme-fer" / "src" / "wire_v1.rs"
TEST = ROOT / "crates" / "fme-fer" / "tests" / "canonical_wire_v1.rs"
GENERATOR = (
    ROOT
    / "scripts"
    / "conformance"
    / "generate_fer_canonical_wire_v1.py"
)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"phase 006 canonical encoding audit: FAIL — {message}")


def u64(value: int) -> bytes:
    require(0 <= value <= (1 << 64) - 1, "reference u64 overflow")
    return value.to_bytes(8, "big")


def header(kind: int) -> bytes:
    profile = PROFILE.encode("ascii")
    return b"FMEF" + bytes([1, kind, len(profile)]) + profile


def encode_path(path: str) -> bytes:
    require(all(bit in "01" for bit in path), "invalid reference path")

    packed = bytearray((len(path) + 7) // 8)

    for index, bit in enumerate(path):
        if bit == "1":
            packed[index // 8] |= 1 << (7 - (index % 8))

    return header(1) + u64(len(path)) + bytes(packed)


def encode_integer(value: int) -> bytes:
    if value == 0:
        return b"\x00" + u64(0)

    sign = b"\x01" if value > 0 else b"\x02"
    magnitude_value = abs(value)
    length = (magnitude_value.bit_length() + 7) // 8
    magnitude = magnitude_value.to_bytes(length, "big")

    require(bool(magnitude), "empty nonzero magnitude")
    require(magnitude[0] != 0, "leading-zero reference magnitude")

    return sign + u64(len(magnitude)) + magnitude


def encode_state(p: int, q: int, depth: int) -> bytes:
    return header(2) + u64(depth) + encode_integer(p) + encode_integer(q)


def evaluate(path: str) -> tuple[int, int, int]:
    p = 0
    q = 0
    depth = 0

    for bit in path:
        next_depth = depth + 1
        denominator = 3 ** next_depth

        if bit == "0":
            p, q = p - q - denominator, p + q
        elif bit == "1":
            p, q = p + q + denominator, q - p
        else:
            raise SystemExit("phase 006 canonical encoding audit: invalid path")

        depth = next_depth

    return p, q, depth


for required in (FIXTURE, SPEC, RUST, TEST, GENERATOR):
    require(required.is_file(), f"missing required file: {required.relative_to(ROOT)}")

fixture = json.loads(FIXTURE.read_text())

require(fixture["fixture_format_version"] == 1, "unexpected fixture version")
require(fixture["normative_wire_format"] is True, "wire fixture must be normative")
require(fixture["wire_format_id"] == WIRE_ID, "wire-format identifier mismatch")
require(fixture["profile_id"] == PROFILE, "profile identifier mismatch")

provenance = fixture["provenance"]
require(provenance["phase"] == "006", "fixture phase mismatch")
require(
    provenance["reference_method"] == "secondary-python-byte-encoder",
    "unexpected reference method",
)
require(
    provenance["reference_script"]
    == "scripts/conformance/generate_fer_canonical_wire_v1.py",
    "reference script mismatch",
)
require(
    provenance["independent_reproduction"] is False,
    "repository-local generator must not claim independent reproduction",
)

paths = fixture["path_vectors"]
states = fixture["state_vectors"]
failures = fixture["failure_vectors"]

require(len(paths) == 8, "expected 8 path vectors")
require(len(states) == 8, "expected 8 state vectors")
require(len(failures) == 14, "expected 14 failure vectors")

for vector in paths:
    expected = encode_path(vector["path"]).hex()
    require(
        vector["canonical_hex"] == expected,
        f"path bytes mismatch: {vector['id']}",
    )
    require(
        vector["bit_length"] == len(vector["path"]),
        f"path length mismatch: {vector['id']}",
    )

for vector in states:
    p, q, depth = evaluate(vector["source_path"])

    require(str(p) == vector["p"], f"P mismatch: {vector['id']}")
    require(str(q) == vector["q"], f"Q mismatch: {vector['id']}")
    require(depth == vector["depth"], f"depth mismatch: {vector['id']}")

    expected = encode_state(p, q, depth).hex()
    require(
        vector["canonical_hex"] == expected,
        f"state bytes mismatch: {vector['id']}",
    )

required_failures = {
    "path-invalid-magic": "InvalidMagic",
    "path-unsupported-version": "UnsupportedVersion",
    "path-wrong-object-kind": "UnexpectedObjectKind",
    "path-profile-mismatch": "ProfileMismatch",
    "path-truncated-header": "Truncated",
    "path-nonzero-padding": "NonZeroPathPadding",
    "path-trailing-byte": "TrailingBytes",
    "state-invalid-sign": "InvalidIntegerSign",
    "state-positive-zero": "NonCanonicalZero",
    "state-negative-zero": "NonCanonicalZero",
    "state-zero-with-magnitude": "NonCanonicalZero",
    "state-leading-zero-magnitude": "NonCanonicalIntegerMagnitude",
    "state-truncated": "Truncated",
    "state-trailing-byte": "TrailingBytes",
}

actual_failures = {
    vector["id"]: vector["expected_error"]
    for vector in failures
}

require(
    actual_failures == required_failures,
    "negative-vector coverage mismatch",
)

rust = RUST.read_text()

for token in (
    'b"FMEF"',
    'b"FME-FER-AFFINE-2D-BINARY-V1"',
    "pub const WIRE_VERSION: u8 = 1;",
    "pub fn encode_path",
    "pub fn decode_path",
    "pub fn encode_exact_state",
    "pub fn decode_exact_state",
    "NonCanonicalZero",
    "NonCanonicalIntegerMagnitude",
    "NonZeroPathPadding",
    "TrailingBytes",
):
    require(token in rust, f"Rust codec missing required token: {token}")

spec = SPEC.read_text()

for token in (
    "most-significant-bit first",
    "unsigned big-endian",
    "Canonical zero",
    "positive zero",
    "negative zero",
    "leading-zero magnitude",
    "It does not hash those bytes.",
    "It does not define cryptographic domain separation.",
    "It does not derive Stable Authority Identity.",
):
    require(token in spec, f"wire specification missing boundary: {token}")

print(
    "phase 006 canonical encoding audit: "
    "8 path / 8 state / 14 failure vectors: PASS"
)
print(
    "phase 006 canonical encoding audit: "
    "secondary reference-byte reproduction: PASS"
)
print(
    "phase 006 canonical encoding audit: "
    "canonical rejection and profile binding: PASS"
)
print("phase 006 canonical encoding audit: PASS")
