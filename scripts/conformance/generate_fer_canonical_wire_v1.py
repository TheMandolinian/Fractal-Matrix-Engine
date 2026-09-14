#!/usr/bin/env python3

import json
from pathlib import Path

PROFILE_ID = b"FME-FER-AFFINE-2D-BINARY-V1"
MAGIC = b"FMEF"
WIRE_VERSION = 1
PATH_KIND = 1
STATE_KIND = 2


def header(kind: int) -> bytes:
    if len(PROFILE_ID) > 255:
        raise ValueError("profile identifier exceeds u8 length")
    return (
        MAGIC
        + bytes([WIRE_VERSION, kind, len(PROFILE_ID)])
        + PROFILE_ID
    )


def u64(value: int) -> bytes:
    if not 0 <= value <= (1 << 64) - 1:
        raise ValueError("value outside u64")
    return value.to_bytes(8, "big")


def encode_path(path: str) -> bytes:
    if any(bit not in "01" for bit in path):
        raise ValueError(f"invalid path {path!r}")

    packed = bytearray((len(path) + 7) // 8)

    for index, bit in enumerate(path):
        if bit == "1":
            packed[index // 8] |= 1 << (7 - (index % 8))

    return header(PATH_KIND) + u64(len(path)) + bytes(packed)


def encode_integer(value: int) -> bytes:
    if value == 0:
        return b"\x00" + u64(0)

    sign = 1 if value > 0 else 2
    magnitude_value = abs(value)
    magnitude_length = (magnitude_value.bit_length() + 7) // 8
    magnitude = magnitude_value.to_bytes(magnitude_length, "big")

    if not magnitude or magnitude[0] == 0:
        raise AssertionError("noncanonical generated magnitude")

    return bytes([sign]) + u64(len(magnitude)) + magnitude


def encode_state(p: int, q: int, depth: int) -> bytes:
    return (
        header(STATE_KIND)
        + u64(depth)
        + encode_integer(p)
        + encode_integer(q)
    )


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
            raise ValueError(f"invalid path bit {bit!r}")

        depth = next_depth

    return p, q, depth


def hx(value: bytes) -> str:
    return value.hex()


PATH_CASES = [
    ("path-root", ""),
    ("path-f0", "0"),
    ("path-f1", "1"),
    ("path-7-bit", "0101101"),
    ("path-8-bit", "01000001"),
    ("path-9-bit", "010000011"),
    ("path-mixed-16", "0010110100101101"),
    ("path-mixed-33", "001011010010110100101101001011010"),
]

STATE_CASES = [
    ("state-root", ""),
    ("state-f0", "0"),
    ("state-f1", "1"),
    ("state-01", "01"),
    ("state-10", "10"),
    ("state-mixed-8", "00101101"),
    ("state-mixed-32", "00101101" * 4),
    ("state-mixed-128", "00101101" * 16),
]


path_vectors = [
    {
        "id": case_id,
        "path": path,
        "bit_length": len(path),
        "canonical_hex": hx(encode_path(path)),
    }
    for case_id, path in PATH_CASES
]

state_vectors = []

for case_id, path in STATE_CASES:
    p, q, depth = evaluate(path)
    state_vectors.append(
        {
            "id": case_id,
            "source_path": path,
            "p": str(p),
            "q": str(q),
            "depth": depth,
            "canonical_hex": hx(encode_state(p, q, depth)),
        }
    )


root_path = bytearray(encode_path(""))
zero_path = bytearray(encode_path("0"))
root_state = bytearray(encode_state(0, 0, 0))
positive_state = bytearray(encode_state(3, 0, 1))

P_SIGN_OFFSET = len(header(STATE_KIND)) + 8
P_LENGTH_OFFSET = P_SIGN_OFFSET + 1
P_MAGNITUDE_OFFSET = P_LENGTH_OFFSET + 8

failure_vectors = []

bad = bytearray(root_path)
bad[0] ^= 0x01
failure_vectors.append(
    {
        "id": "path-invalid-magic",
        "decode_as": "path",
        "hex": hx(bytes(bad)),
        "expected_error": "InvalidMagic",
    }
)

bad = bytearray(root_path)
bad[4] = 2
failure_vectors.append(
    {
        "id": "path-unsupported-version",
        "decode_as": "path",
        "hex": hx(bytes(bad)),
        "expected_error": "UnsupportedVersion",
    }
)

bad = bytearray(root_path)
bad[5] = STATE_KIND
failure_vectors.append(
    {
        "id": "path-wrong-object-kind",
        "decode_as": "path",
        "hex": hx(bytes(bad)),
        "expected_error": "UnexpectedObjectKind",
    }
)

bad = bytearray(root_path)
bad[7] ^= 0x01
failure_vectors.append(
    {
        "id": "path-profile-mismatch",
        "decode_as": "path",
        "hex": hx(bytes(bad)),
        "expected_error": "ProfileMismatch",
    }
)

failure_vectors.append(
    {
        "id": "path-truncated-header",
        "decode_as": "path",
        "hex": hx(bytes(root_path[:3])),
        "expected_error": "Truncated",
    }
)

bad = bytearray(zero_path)
bad[-1] |= 0x01
failure_vectors.append(
    {
        "id": "path-nonzero-padding",
        "decode_as": "path",
        "hex": hx(bytes(bad)),
        "expected_error": "NonZeroPathPadding",
    }
)

failure_vectors.append(
    {
        "id": "path-trailing-byte",
        "decode_as": "path",
        "hex": hx(bytes(root_path) + b"\x00"),
        "expected_error": "TrailingBytes",
    }
)

bad = bytearray(root_state)
bad[P_SIGN_OFFSET] = 3
failure_vectors.append(
    {
        "id": "state-invalid-sign",
        "decode_as": "exact_state",
        "hex": hx(bytes(bad)),
        "expected_error": "InvalidIntegerSign",
    }
)

bad = bytearray(root_state)
bad[P_SIGN_OFFSET] = 1
failure_vectors.append(
    {
        "id": "state-positive-zero",
        "decode_as": "exact_state",
        "hex": hx(bytes(bad)),
        "expected_error": "NonCanonicalZero",
    }
)

bad = bytearray(root_state)
bad[P_SIGN_OFFSET] = 2
failure_vectors.append(
    {
        "id": "state-negative-zero",
        "decode_as": "exact_state",
        "hex": hx(bytes(bad)),
        "expected_error": "NonCanonicalZero",
    }
)

bad = bytearray(root_state)
bad[P_SIGN_OFFSET] = 0
bad[P_LENGTH_OFFSET:P_MAGNITUDE_OFFSET] = u64(1)
bad[P_MAGNITUDE_OFFSET:P_MAGNITUDE_OFFSET] = b"\x00"
failure_vectors.append(
    {
        "id": "state-zero-with-magnitude",
        "decode_as": "exact_state",
        "hex": hx(bytes(bad)),
        "expected_error": "NonCanonicalZero",
    }
)

bad = bytearray(positive_state)
bad[P_LENGTH_OFFSET:P_MAGNITUDE_OFFSET] = u64(2)
bad[P_MAGNITUDE_OFFSET:P_MAGNITUDE_OFFSET] = b"\x00"
failure_vectors.append(
    {
        "id": "state-leading-zero-magnitude",
        "decode_as": "exact_state",
        "hex": hx(bytes(bad)),
        "expected_error": "NonCanonicalIntegerMagnitude",
    }
)

failure_vectors.append(
    {
        "id": "state-truncated",
        "decode_as": "exact_state",
        "hex": hx(bytes(root_state[:-1])),
        "expected_error": "Truncated",
    }
)

failure_vectors.append(
    {
        "id": "state-trailing-byte",
        "decode_as": "exact_state",
        "hex": hx(bytes(root_state) + b"\x00"),
        "expected_error": "TrailingBytes",
    }
)

fixture = {
    "fixture_format_version": 1,
    "normative_wire_format": True,
    "wire_format_id": "FME-FER-AFFINE-2D-BINARY-V1-WIRE-V1",
    "profile_id": PROFILE_ID.decode("ascii"),
    "provenance": {
        "phase": "006",
        "reference_method": "secondary-python-byte-encoder",
        "reference_script": "scripts/conformance/generate_fer_canonical_wire_v1.py",
        "independent_reproduction": False,
    },
    "path_vectors": path_vectors,
    "state_vectors": state_vectors,
    "failure_vectors": failure_vectors,
}

target = Path(
    "conformance/fer-affine-2d-binary-v1/canonical-wire-v1.json"
)
target.write_text(json.dumps(fixture, indent=2) + "\n")

print(f"wrote {target}")
print(f"path_vectors={len(path_vectors)}")
print(f"state_vectors={len(state_vectors)}")
print(f"failure_vectors={len(failure_vectors)}")
