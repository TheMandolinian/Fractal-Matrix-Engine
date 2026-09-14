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
    "scripts/audit/audit_phase_004_conformance_expansion.py"
    "scripts/audit/audit_phase_005_benchmark_characterization.py"
    "scripts/audit/audit_phase_006_baseline_fer_canonical_encoding.py"
    "scripts/audit/audit_phase_007_cryptographic_registry_preimage.py"
    "scripts/audit/audit_phase_008_registered_sha2_digest.py"
    "scripts/conformance/generate_crypto_registry_v1.py"
    "scripts/conformance/generate_crypto_digest_v1.py"
    "conformance/crypto-registry-v1/CRYPTO_REGISTRY_V1.md"
    "conformance/crypto-registry-v1/crypto-registry-v1.json"
    "conformance/crypto-digest-v1/CRYPTO_DIGEST_V1.md"
    "conformance/crypto-digest-v1/crypto-digest-v1.json"
    "crates/fme-crypto/src/digest_v1.rs"
    "crates/fme-crypto/tests/digest_v1.rs"
    "crates/fme-crypto/Cargo.toml"
    "crates/fme-crypto/src/lib.rs"
    "crates/fme-crypto/tests/registry_preimage_v1.rs"
    "crates/fme-fer/benches/baseline_v1.rs"
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
    "docs/phase-docs/phase-001-100/phase 004/closeout.md"
    "docs/phase-docs/phase-001-100/phase 005/closeout.md"
    "docs/phase-docs/phase-001-100/phase 006/closeout.md"
    "docs/phase-docs/phase-001-100/phase 007/closeout.md"
    "docs/phase-docs/phase-001-100/phase 008/closeout.md"
    "conformance/fer-affine-2d-binary-v1/vectors.json"
    "scripts/conformance/reference_fer_affine_2d_binary_v1.py"
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
phase_004="docs/phase-docs/phase-001-100/phase 004/closeout.md"
phase_005="docs/phase-docs/phase-001-100/phase 005/closeout.md"
phase_006="docs/phase-docs/phase-001-100/phase 006/closeout.md"
phase_007="docs/phase-docs/phase-001-100/phase 007/closeout.md"
phase_008="docs/phase-docs/phase-001-100/phase 008/closeout.md"

grep -Fq '| Phase | Scope | Status | Implementation Anchor | Documentation Anchor | Anchor Repair |' "$phase_index" \
    || fail "lifecycle-anchor phase ledger header missing"

grep -Fq '| 001 | Exact Baseline FER Core | SEALED | `cb7861b9` | `aa7aaa84` |' "$phase_index" \
    || fail "Phase 001 dual-anchor index entry is incorrect"

grep -Fq '| 002 | External Conformance Vector Consumption | SEALED | `c5e996b5` | `aa7aaa84` |' "$phase_index" \
    || fail "Phase 002 dual-anchor index entry is incorrect"

grep -Fq '| 003 | Institutional Repository Baseline | SEALED | `aa7aaa84` | `dc359eed` |' "$phase_index" \
    || fail "Phase 003 dual-anchor index entry is incorrect"

grep -Fq '| 004 | Baseline FER Conformance Expansion | SEALED | `b417d542` | `b0ad7139` |' "$phase_index" \
    || fail "Phase 004 dual-anchor index entry is incorrect"

grep -Fq '| 005 | Baseline FER Benchmark Characterization | SEALED | `f3ab055b` | `80f66946` |' "$phase_index" \
    || fail "Phase 005 dual-anchor index entry is incorrect"

grep -Fq '| 006 | Baseline FER Canonical Encoding | SEALED | `7322debe` | `c45746aa` | `cd289812` |' "$phase_index" \
    || fail "Phase 006 lifecycle-anchor index entry is incorrect"

for phase_file in \
    "$phase_001" \
    "$phase_002" \
    "$phase_003" \
    "$phase_004" \
    "$phase_005" \
    "$phase_006"
do
    grep -Fq 'Status: **SEALED**' "$phase_file" \
        || fail "sealed phase closeout status missing: $phase_file"
done

grep -Fq '`cb7861b9`' "$phase_001" \
    || fail "Phase 001 implementation anchor is incorrect"

grep -Fq '`c5e996b5`' "$phase_002" \
    || fail "Phase 002 implementation anchor is incorrect"

grep -Fq '`aa7aaa84`' "$phase_003" \
    || fail "Phase 003 implementation anchor is incorrect"

grep -Fq '`b417d54`' "$phase_004" \
    || fail "Phase 004 implementation anchor is incorrect"

grep -Fq '`f3ab055`' "$phase_005" \
    || fail "Phase 005 implementation anchor is incorrect"

grep -Fq '`7322debe`' "$phase_006" \
    || fail "Phase 006 implementation merge anchor is incorrect"

grep -Fq '`c45746aa`' "$phase_006" \
    || fail "Phase 006 documentation merge anchor is incorrect"

grep -Fq '`cd289812`' "$phase_006" \
    || fail "Phase 006 anchor-repair merge is incorrect"

grep -Fq '| 007 | Baseline Cryptographic Registry and Preimage Contract | SEALED | `a237660a` | `8d49bf9e` | `749fcd7f` |' "$phase_index" \
    || fail "Phase 007 sealed ledger entry is incorrect"

grep -Fq 'Status: **SEALED**' "$phase_007" \
    || fail "Phase 007 closeout status is not SEALED"

grep -Fq '`a237660a`' "$phase_007" \
    || fail "Phase 007 implementation merge anchor is incorrect"

grep -Fq '`8d49bf9e`' "$phase_007" \
    || fail "Phase 007 documentation merge anchor is incorrect"

grep -Fq '`749fcd7f`' "$phase_007" \
    || fail "Phase 007 anchor-repair merge is incorrect"

grep -Fq '| 008 | Baseline Registered SHA-2 Digest Execution | SEALED | `443275a8` | `07130bf3` | `40b31897` |' "$phase_index" \
    || fail "Phase 008 sealed ledger entry is incorrect"

grep -Fq 'Status: **SEALED**' "$phase_008" \
    || fail "Phase 008 closeout status is not SEALED"

grep -Fq '`443275a8`' "$phase_008" \
    || fail "Phase 008 implementation merge anchor is incorrect"

grep -Fq '`07130bf3`' "$phase_008" \
    || fail "Phase 008 documentation merge anchor is incorrect"

grep -Fq '`40b31897`' "$phase_008" \
    || fail "Phase 008 anchor-repair merge is incorrect"

grep -Fq 'Last sealed phase: **008**' "$phase_index" \
    || fail "Phase 008 is not recorded as the last sealed phase"

grep -Fq 'Current phase: **none**' "$phase_index" \
    || fail "Phase 008 closeout position is incorrect"

if grep -RIn 'PENDING_AFTER_' \
    docs/phase-docs/phase-001-100 >/dev/null
then
    fail "pending phase anchor remains in phase documentation"
fi

printf '%s\n' "repository baseline audit: lifecycle-anchor phase ledger semantics: PASS"

python3 - <<'PY'
import json
from pathlib import Path

path = Path("conformance/fer-affine-2d-binary-v1/vectors.json")
data = json.loads(path.read_text(encoding="utf-8"))

assert data["fixture_format_version"] == 2
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

PYTHONDONTWRITEBYTECODE=1 \
    python3 scripts/audit/audit_phase_004_conformance_expansion.py

PYTHONDONTWRITEBYTECODE=1 \
    python3 scripts/audit/audit_phase_005_benchmark_characterization.py

PYTHONDONTWRITEBYTECODE=1 \
    python3 scripts/audit/audit_phase_006_baseline_fer_canonical_encoding.py

PYTHONDONTWRITEBYTECODE=1 \
    python3 scripts/audit/audit_phase_007_cryptographic_registry_preimage.py

PYTHONDONTWRITEBYTECODE=1 \
    python3 scripts/audit/audit_phase_008_registered_sha2_digest.py


printf '%s\n' \
    "repository baseline audit: required files: PASS" \
    "repository baseline audit: toolchain pin: PASS" \
    "repository baseline audit: Apache-2.0 posture: PASS" \
    "repository baseline audit: FER profile binding: PASS" \
    "repository baseline audit: unsafe-code guard: PASS" \
    "repository baseline audit: PASS"
