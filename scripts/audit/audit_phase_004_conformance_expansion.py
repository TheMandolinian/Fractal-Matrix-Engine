#!/usr/bin/env python3

import importlib.util
import json
import sys
from pathlib import Path

sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parents[2]
FIXTURE = ROOT / "conformance/fer-affine-2d-binary-v1/vectors.json"
REFERENCE = ROOT / "scripts/conformance/reference_fer_affine_2d_binary_v1.py"

PROFILE_ID = "FME-FER-AFFINE-2D-BINARY-V1"
I128_MAX = (1 << 127) - 1


def fail(message: str) -> None:
    raise SystemExit(f"phase 004 conformance expansion audit: FAIL: {message}")


def load_reference():
    spec = importlib.util.spec_from_file_location(
        "fme_phase_004_reference_audit",
        REFERENCE,
    )

    if spec is None or spec.loader is None:
        fail("unable to load secondary exact reference evaluator")

    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main() -> None:
    if not FIXTURE.is_file():
        fail("expanded FER fixture is missing")

    if not REFERENCE.is_file():
        fail("secondary exact reference evaluator is missing")

    data = json.loads(FIXTURE.read_text(encoding="utf-8"))

    if data.get("fixture_format_version") != 2:
        fail("fixture_format_version must equal 2")

    if data.get("normative_wire_format") is not False:
        fail("fixture must remain explicitly non-normative")

    if data.get("profile_id") != PROFILE_ID:
        fail("fixture profile identifier mismatch")

    provenance = data.get("provenance")
    if not isinstance(provenance, dict):
        fail("provenance metadata is missing")

    expected_provenance = {
        "phase": "004",
        "reference_method": "exact-rational-affine-equations",
        "reference_script": (
            "scripts/conformance/reference_fer_affine_2d_binary_v1.py"
        ),
        "established_vector_self_check": True,
        "independent_reproduction": False,
    }

    for key, expected in expected_provenance.items():
        if provenance.get(key) != expected:
            fail(
                f"provenance field {key!r} mismatch: "
                f"expected={expected!r}, actual={provenance.get(key)!r}"
            )

    vectors = data.get("vectors")
    failures = data.get("failure_vectors")
    continuations = data.get("continuation_vectors")

    if not isinstance(vectors, list) or len(vectors) != 34:
        fail("expected exactly 34 success vectors")

    if not isinstance(failures, list) or len(failures) != 5:
        fail("expected exactly 5 deterministic failure vectors")

    if not isinstance(continuations, list) or len(continuations) != 8:
        fail("expected exactly 8 continuation vectors")

    all_ids: list[str] = []

    reference = load_reference()
    reference.verify_established_vectors()

    categories: set[str] = set()
    maximum_depth = -1
    depth_128_exceeds_i128 = False

    for vector in vectors:
        vector_id = vector.get("id")
        category = vector.get("category")
        path = vector.get("path")
        p = vector.get("p")
        q = vector.get("q")
        depth = vector.get("depth")

        if not isinstance(vector_id, str) or not vector_id:
            fail("success vector has missing/invalid id")

        all_ids.append(vector_id)

        if not isinstance(category, str) or not category:
            fail(f"success vector {vector_id!r} has invalid category")

        categories.add(category)

        if not isinstance(path, str):
            fail(f"success vector {vector_id!r} path is not a string")

        if set(path) - {"0", "1"}:
            fail(f"success vector {vector_id!r} path is not binary")

        if not isinstance(depth, int) or depth < 0:
            fail(f"success vector {vector_id!r} has invalid depth")

        if len(path) != depth:
            fail(f"success vector {vector_id!r} path/depth mismatch")

        try:
            p_int = int(p)
            q_int = int(q)
        except (TypeError, ValueError):
            fail(f"success vector {vector_id!r} has invalid P/Q integers")

        expected = reference.evaluate_affine(path)
        actual = (p_int, q_int, depth)

        if actual != expected:
            fail(
                f"success vector {vector_id!r} disagrees with "
                f"secondary exact reference: fixture={actual}, "
                f"reference={expected}"
            )

        maximum_depth = max(maximum_depth, depth)

        if depth == 128 and (
            abs(p_int) > I128_MAX or abs(q_int) > I128_MAX
        ):
            depth_128_exceeds_i128 = True

    required_categories = {
        "root",
        "shallow",
        "repeated",
        "alternating",
        "mixed",
    }

    if not required_categories.issubset(categories):
        fail(
            "success-vector category coverage incomplete: "
            f"found={sorted(categories)}"
        )

    if maximum_depth != 128:
        fail(f"maximum success-vector depth must equal 128, got {maximum_depth}")

    if not depth_128_exceeds_i128:
        fail("depth-128 corpus does not exceed signed 128-bit integer range")

    for vector in failures:
        vector_id = vector.get("id")
        path = vector.get("path")
        expected_error = vector.get("expected_error")
        index = vector.get("index")
        symbol = vector.get("symbol")

        if not isinstance(vector_id, str) or not vector_id:
            fail("failure vector has missing/invalid id")

        all_ids.append(vector_id)

        if expected_error != "InvalidPathSymbol":
            fail(
                f"failure vector {vector_id!r} has unsupported expected error"
            )

        if not isinstance(path, str):
            fail(f"failure vector {vector_id!r} path is not a string")

        if not isinstance(index, int) or index < 0 or index >= len(path):
            fail(f"failure vector {vector_id!r} has invalid symbol index")

        if not isinstance(symbol, str) or len(symbol) != 1:
            fail(
                f"failure vector {vector_id!r} symbol must be one character"
            )

        if path[index] != symbol:
            fail(
                f"failure vector {vector_id!r} symbol/index does not match path"
            )

        if symbol in {"0", "1"}:
            fail(
                f"failure vector {vector_id!r} expected symbol is actually valid"
            )

    for vector in continuations:
        vector_id = vector.get("id")
        parent_path = vector.get("parent_path")
        suffix = vector.get("suffix")
        full_path = vector.get("full_path")
        p = vector.get("p")
        q = vector.get("q")
        depth = vector.get("depth")

        if not isinstance(vector_id, str) or not vector_id:
            fail("continuation vector has missing/invalid id")

        all_ids.append(vector_id)

        for field_name, value in (
            ("parent_path", parent_path),
            ("suffix", suffix),
            ("full_path", full_path),
        ):
            if not isinstance(value, str):
                fail(
                    f"continuation vector {vector_id!r} "
                    f"{field_name} is not a string"
                )

            if set(value) - {"0", "1"}:
                fail(
                    f"continuation vector {vector_id!r} "
                    f"{field_name} is not binary"
                )

        if parent_path + suffix != full_path:
            fail(
                f"continuation vector {vector_id!r} path composition mismatch"
            )

        if not isinstance(depth, int) or depth != len(full_path):
            fail(
                f"continuation vector {vector_id!r} full-path/depth mismatch"
            )

        try:
            actual = (int(p), int(q), depth)
        except (TypeError, ValueError):
            fail(
                f"continuation vector {vector_id!r} has invalid P/Q integers"
            )

        expected = reference.evaluate_affine(full_path)

        if actual != expected:
            fail(
                f"continuation vector {vector_id!r} disagrees with "
                f"secondary exact reference: fixture={actual}, "
                f"reference={expected}"
            )

    if len(all_ids) != len(set(all_ids)):
        fail("fixture IDs are not globally unique")

    print(
        "phase 004 conformance expansion audit: "
        "secondary reference reproduction: PASS"
    )
    print(
        "phase 004 conformance expansion audit: "
        "34 success / 5 failure / 8 continuation vectors: PASS"
    )
    print(
        "phase 004 conformance expansion audit: "
        "depth-128 arbitrary-precision coverage: PASS"
    )
    print(
        "phase 004 conformance expansion audit: "
        "fixture provenance and non-normative boundary: PASS"
    )
    print("phase 004 conformance expansion audit: PASS")


if __name__ == "__main__":
    main()
