#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

fail() {
    printf 'repository baseline audit: FAIL: %s\n' "$1" >&2
    exit 1
}

required_files=(
    "Cargo.toml"
    "Cargo.lock"
    "rust-toolchain.toml"
    "README.md"
    "LICENSE"
    "CONTRIBUTING.md"
    "SECURITY.md"
    "CHANGELOG.md"
    ".github/workflows/ci.yml"
    "scripts/audit/audit_repository_baseline.sh"
    "docs/IMPLEMENTATION_SCOPE.md"
    "docs/PROFILE_STATUS.md"
    "docs/MATURITY_MODEL.md"
    "docs/CONFORMANCE_POLICY.md"
    "docs/DEVELOPMENT_GUARDRAILS.md"
    "docs/phase-docs/README.md"
    "docs/phase-docs/phase-001-100/index.md"
    "docs/phase-docs/phase-001-100/phase 001/closeout.md"
    "docs/phase-docs/phase-001-100/phase 002/closeout.md"
    "docs/phase-docs/phase-001-100/phase 003/closeout.md"
    "conformance/fer-affine-2d-binary-v1/vectors.json"
)

for file in "${required_files[@]}"; do
    [[ -f "$file" ]] || fail "missing required file: $file"
done

grep -Fq 'channel = "1.98.1"' rust-toolchain.toml \
    || fail "Rust toolchain is not pinned to 1.98.1"

grep -Fq 'license = "Apache-2.0"' Cargo.toml \
    || fail "Apache-2.0 workspace license declaration missing"

grep -Fq '#![forbid(unsafe_code)]' crates/fme-fer/src/lib.rs \
    || fail "fme-fer unsafe-code prohibition missing"

grep -Fq 'FME-FER-AFFINE-2D-BINARY-V1' \
    conformance/fer-affine-2d-binary-v1/vectors.json \
    || fail "baseline FER profile binding missing"

phase_index="docs/phase-docs/phase-001-100/index.md"
phase_001="docs/phase-docs/phase-001-100/phase 001/closeout.md"
phase_002="docs/phase-docs/phase-001-100/phase 002/closeout.md"
phase_003="docs/phase-docs/phase-001-100/phase 003/closeout.md"

grep -Fq '| 001 | Exact Baseline FER Core | SEALED | `cb7861b9` |' "$phase_index" \
    || fail "Phase 001 index entry or anchor is incorrect"

grep -Fq '| 002 | External Conformance Vector Consumption | SEALED | `c5e996b5` |' "$phase_index" \
    || fail "Phase 002 index entry or anchor is incorrect"

grep -Fq '| 003 | Institutional Repository Baseline | IN PROGRESS | `PENDING_AFTER_IMPLEMENTATION_COMMIT` |' "$phase_index" \
    || fail "Phase 003 index entry is not correctly marked IN PROGRESS"

grep -Fq 'Status: **SEALED**' "$phase_001" \
    || fail "Phase 001 closeout is not SEALED"

grep -Fq '`cb7861b9`' "$phase_001" \
    || fail "Phase 001 closeout anchor is incorrect"

grep -Fq 'Status: **SEALED**' "$phase_002" \
    || fail "Phase 002 closeout is not SEALED"

grep -Fq '`c5e996b5`' "$phase_002" \
    || fail "Phase 002 closeout anchor is incorrect"

grep -Fq 'Status: **IN PROGRESS**' "$phase_003" \
    || fail "Phase 003 closeout must remain IN PROGRESS before implementation commit"

grep -Fq '`PENDING_AFTER_IMPLEMENTATION_COMMIT`' "$phase_003" \
    || fail "Phase 003 pending implementation anchor is missing"

printf '%s\n' "repository baseline audit: phase ledger semantics: PASS"

python3 - <<'PY'
import json
from pathlib import Path

path = Path("conformance/fer-affine-2d-binary-v1/vectors.json")
data = json.loads(path.read_text(encoding="utf-8"))

assert data["fixture_format_version"] == 1
assert data["normative_wire_format"] is False
assert data["profile_id"] == "FME-FER-AFFINE-2D-BINARY-V1"
assert isinstance(data["vectors"], list)
assert data["vectors"]

for vector in data["vectors"]:
    path_value = vector["path"]

    assert isinstance(path_value, str)
    assert set(path_value) <= {"0", "1"}
    assert isinstance(vector["depth"], int)
    assert vector["depth"] >= 0
    assert len(path_value) == vector["depth"]

    int(vector["p"])
    int(vector["q"])

print("repository baseline audit: conformance fixture structure: PASS")
PY

printf '%s\n' \
    "repository baseline audit: required files: PASS" \
    "repository baseline audit: toolchain pin: PASS" \
    "repository baseline audit: Apache-2.0 posture: PASS" \
    "repository baseline audit: FER profile binding: PASS" \
    "repository baseline audit: unsafe-code guard: PASS" \
    "repository baseline audit: PASS"
