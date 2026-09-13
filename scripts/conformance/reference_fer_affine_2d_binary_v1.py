#!/usr/bin/env python3

from fractions import Fraction

PROFILE_ID = "FME-FER-AFFINE-2D-BINARY-V1"


def evaluate_affine(path: str) -> tuple[int, int, int]:
    """
    Secondary exact reference calculation.

    This evaluates the affine equations directly over exact rational
    real/imaginary components. It intentionally does not implement the
    Rust P/Q integer recurrence.
    """
    x = Fraction(0, 1)
    y = Fraction(0, 1)

    for index, symbol in enumerate(path):
        if symbol == "0":
            x, y = (
                (x - y) / 3 - 1,
                (x + y) / 3,
            )
        elif symbol == "1":
            x, y = (
                (x + y) / 3 + 1,
                (y - x) / 3,
            )
        else:
            raise ValueError(
                f"invalid baseline FER path symbol {symbol!r} at index {index}"
            )

    depth = len(path)
    denominator = 3 ** depth

    p = x * denominator
    q = y * denominator

    if p.denominator != 1 or q.denominator != 1:
        raise AssertionError(
            f"exact affine result did not normalize to integer numerators: {path!r}"
        )

    return p.numerator, q.numerator, depth


ESTABLISHED_VECTORS = {
    "": (0, 0, 0),
    "0": (-3, 0, 1),
    "1": (3, 0, 1),
    "00": (-12, -3, 2),
    "01": (6, 3, 2),
    "10": (-6, 3, 2),
    "11": (12, -3, 2),
    "000": (-36, -15, 3),
    "111": (36, -15, 3),
    "0101": (66, 33, 4),
    "1010": (-66, 33, 4),
    "001101": (690, 237, 6),
}


PROPOSED_EXPANSION = [
    ("repeat_f0_d8", "0" * 8),
    ("repeat_f1_d8", "1" * 8),
    ("repeat_f0_d16", "0" * 16),
    ("repeat_f1_d16", "1" * 16),
    ("alternate_01_d16", "01" * 8),
    ("alternate_10_d16", "10" * 8),
    ("mixed_a_d16", "0011010111001011"),
    ("mixed_b_d16", "1100101000110100"),
    ("repeat_f0_d32", "0" * 32),
    ("repeat_f1_d32", "1" * 32),
    ("alternate_01_d32", "01" * 16),
    ("alternate_10_d32", "10" * 16),
    ("mixed_a_d32", "00110101110010111001010001101001"),
    ("mixed_b_d32", "11001010001101000110101110010110"),
    ("repeat_f0_d64", "0" * 64),
    ("repeat_f1_d64", "1" * 64),
    ("alternate_01_d64", "01" * 32),
    ("alternate_10_d64", "10" * 32),
    ("repeat_f0_d128", "0" * 128),
    ("repeat_f1_d128", "1" * 128),
    ("alternate_01_d128", "01" * 64),
    ("alternate_10_d128", "10" * 64),
]


def verify_established_vectors() -> None:
    for path, expected in ESTABLISHED_VECTORS.items():
        actual = evaluate_affine(path)
        if actual != expected:
            raise AssertionError(
                f"established vector mismatch for {path!r}: "
                f"expected={expected}, actual={actual}"
            )


def main() -> None:
    verify_established_vectors()

    print(f"REFERENCE PROFILE: {PROFILE_ID}")
    print("REFERENCE METHOD: exact rational affine evaluation")
    print("ESTABLISHED VECTOR SELF-CHECK: PASS")
    print()
    print("PROPOSED PHASE 004 SUCCESS VECTORS:")
    print("label|path|p|q|depth")

    for label, path in PROPOSED_EXPANSION:
        p, q, depth = evaluate_affine(path)
        print(f"{label}|{path}|{p}|{q}|{depth}")


if __name__ == "__main__":
    main()
