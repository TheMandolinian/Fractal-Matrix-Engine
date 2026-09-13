#!/usr/bin/env python3

import re
import sys
from pathlib import Path

sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parents[2]
CARGO = ROOT / "crates/fme-fer/Cargo.toml"
BENCH = ROOT / "crates/fme-fer/benches/baseline_v1.rs"
BASELINE = ROOT / "crates/fme-fer/src/baseline_v1.rs"

PROFILE_ID = "FME-FER-AFFINE-2D-BINARY-V1"
EXPECTED_DEPTHS = [8, 16, 32, 64, 128, 256, 384, 512, 768, 1000]

EXPECTED_WORKLOADS = {
    "repeated_f0",
    "repeated_f1",
    "alternating_01",
    "alternating_10",
    "mixed_00101101",
}


def fail(message: str) -> None:
    raise SystemExit(
        f"phase 005 benchmark characterization audit: FAIL: {message}"
    )


def main() -> None:
    if not CARGO.is_file():
        fail("fme-fer Cargo.toml is missing")

    if not BENCH.is_file():
        fail("baseline FER benchmark source is missing")

    if not BASELINE.is_file():
        fail("authoritative baseline FER source is missing")

    cargo = CARGO.read_text(encoding="utf-8")
    bench = BENCH.read_text(encoding="utf-8")
    baseline = BASELINE.read_text(encoding="utf-8")

    required_bench_target = """[[bench]]
name = "baseline_v1"
harness = false"""

    if required_bench_target not in cargo:
        fail("dependency-free custom benchmark target is not configured")

    depth_match = re.search(
        r"const DEPTHS:\s*\[usize;\s*\d+\]\s*=\s*\[([^\]]+)\];",
        bench,
    )

    if depth_match is None:
        fail("unable to locate fixed benchmark depth matrix")

    actual_depths = [
        int(value.strip())
        for value in depth_match.group(1).split(",")
        if value.strip()
    ]

    if actual_depths != EXPECTED_DEPTHS:
        fail(
            f"unexpected depth matrix: "
            f"expected={EXPECTED_DEPTHS}, actual={actual_depths}"
        )

    for workload in EXPECTED_WORKLOADS:
        if f'"{workload}"' not in bench:
            fail(f"required workload missing: {workload}")

    required_tokens = [
        "use std::hint::black_box;",
        "use std::time::Instant;",
        "evaluate(black_box(&workload.path))",
        'println!("cargo_profile=bench");',
        'println!("target_arch={ARCH}");',
        'println!("target_os={OS}");',
        'println!("rust_toolchain={}", rustc_version());',
        'println!("timing_scope=exact_complete_path_fer_evaluation_only");',
        'println!("path_parsing_inside_timed_region=false");',
        'println!("fixture_loading_inside_timed_region=false");',
        "evaluations_per_second",
        "transforms_per_second",
    ]

    for token in required_tokens:
        if token not in bench:
            fail(f"required benchmark contract token missing: {token!r}")

    profile_import = re.search(
        r"use\s+fme_fer::baseline_v1::\{[^}]*\bPROFILE_ID\b[^}]*\};",
        bench,
        flags=re.MULTILINE | re.DOTALL,
    )

    if profile_import is None:
        fail("benchmark does not import PROFILE_ID from baseline_v1")

    expected_profile_declaration = (
        f"pub const PROFILE_ID: &str = \"{PROFILE_ID}\";"
    )

    if expected_profile_declaration not in baseline:
        fail("authoritative FER profile declaration mismatch")

    if "profile={PROFILE_ID}" not in bench:
        fail("benchmark does not report the authoritative FER profile identifier")

    warmup_position = bench.find(
        "for _ in 0..workload.warmup_iterations"
    )
    timer_position = bench.find("let started = Instant::now();")
    measured_position = bench.find(
        "for _ in 0..workload.measured_iterations"
    )

    if min(warmup_position, timer_position, measured_position) < 0:
        fail("unable to locate warmup/timed-loop structure")

    if not warmup_position < timer_position < measured_position:
        fail("warmup must precede timer and measured loop")

    if 'let source = kind.path_string(depth);' not in bench:
        fail("fixed path construction is missing")

    if 'BaselinePath::from_str(&source)' not in bench:
        fail("path parsing is missing from workload construction")

    if bench.find("BaselinePath::from_str(&source)") > timer_position:
        fail("path parsing appears after timed-region start")

    expected_iteration_policy = {
        8: (20_000, 1_000_000),
        16: (10_000, 500_000),
        32: (5_000, 250_000),
        64: (2_000, 100_000),
        128: (1_000, 50_000),
        256: (500, 25_000),
        384: (300, 15_000),
        512: (200, 10_000),
        768: (100, 5_000),
        1000: (100, 5_000),
    }

    for depth, (warmup, measured) in expected_iteration_policy.items():
        expected = (
            f"{depth} => ({warmup:_}, {measured:_}),"
        )

        if expected not in bench:
            fail(
                f"iteration policy mismatch for depth {depth}: "
                f"expected source token {expected!r}"
            )

    if "events_per_second" in bench or "transactions_per_second" in bench:
        fail("benchmark must not relabel FER work as events or transactions")

    print(
        "phase 005 benchmark characterization audit: "
        "custom stable-Rust benchmark target: PASS"
    )
    print(
        "phase 005 benchmark characterization audit: "
        "5 workloads / 10 depths / 50 fixed cases: PASS"
    )
    print(
        "phase 005 benchmark characterization audit: "
        "warmup and timed-region separation: PASS"
    )
    print(
        "phase 005 benchmark characterization audit: "
        "FER measurement terminology boundary: PASS"
    )
    print("phase 005 benchmark characterization audit: PASS")


if __name__ == "__main__":
    main()
