use fme_fer::baseline_v1::{PROFILE_ID, apply_transform, evaluate, root};
use fme_fer::{BaselinePath, ExactState, FerError, TopologyBit};
use num_bigint::BigInt;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ConformanceFile {
    fixture_format_version: u64,
    normative_wire_format: bool,
    profile_id: String,
    provenance: Provenance,
    vectors: Vec<ConformanceVector>,
    failure_vectors: Vec<FailureVector>,
    continuation_vectors: Vec<ContinuationVector>,
}

#[derive(Debug, Deserialize)]
struct Provenance {
    phase: String,
    reference_method: String,
    reference_script: String,
    established_vector_self_check: bool,
    independent_reproduction: bool,
}

#[derive(Debug, Deserialize)]
struct ConformanceVector {
    id: String,
    category: String,
    path: String,
    p: String,
    q: String,
    depth: u64,
}

#[derive(Debug, Deserialize)]
struct FailureVector {
    id: String,
    path: String,
    expected_error: String,
    index: usize,
    symbol: String,
}

#[derive(Debug, Deserialize)]
struct ContinuationVector {
    id: String,
    parent_path: String,
    suffix: String,
    full_path: String,
    p: String,
    q: String,
    depth: u64,
}

fn conformance_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../conformance/fer-affine-2d-binary-v1/vectors.json")
}

fn load_conformance_file() -> ConformanceFile {
    let path = conformance_path();

    let bytes = fs::read(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

    serde_json::from_slice(&bytes)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
}

fn parse_bigint(vector_id: &str, field: &str, value: &str) -> BigInt {
    value.parse::<BigInt>().unwrap_or_else(|error| {
        panic!(
            "invalid {field} value for conformance vector {vector_id:?}: \
             {value:?}: {error}"
        )
    })
}

fn assert_exact_state(
    vector_id: &str,
    state: &ExactState,
    expected_p: &str,
    expected_q: &str,
    expected_depth: u64,
) {
    let expected_p = parse_bigint(vector_id, "P", expected_p);
    let expected_q = parse_bigint(vector_id, "Q", expected_q);

    assert_eq!(
        state.p(),
        &expected_p,
        "P mismatch for conformance vector {vector_id:?}"
    );

    assert_eq!(
        state.q(),
        &expected_q,
        "Q mismatch for conformance vector {vector_id:?}"
    );

    assert_eq!(
        state.depth(),
        expected_depth,
        "depth mismatch for conformance vector {vector_id:?}"
    );
}

#[test]
fn baseline_profile_identifier_is_stable() {
    assert_eq!(PROFILE_ID, "FME-FER-AFFINE-2D-BINARY-V1");
}

#[test]
fn root_is_exact_zero_state() {
    let state = root();

    assert_eq!(state.p(), &BigInt::from(0));
    assert_eq!(state.q(), &BigInt::from(0));
    assert_eq!(state.depth(), 0);
}

#[test]
fn external_conformance_fixture_metadata_is_expected() {
    let fixture = load_conformance_file();

    assert_eq!(fixture.fixture_format_version, 2);
    assert!(!fixture.normative_wire_format);
    assert_eq!(fixture.profile_id, PROFILE_ID);

    assert_eq!(fixture.provenance.phase, "004");
    assert_eq!(
        fixture.provenance.reference_method,
        "exact-rational-affine-equations"
    );
    assert_eq!(
        fixture.provenance.reference_script,
        "scripts/conformance/reference_fer_affine_2d_binary_v1.py"
    );
    assert!(fixture.provenance.established_vector_self_check);
    assert!(!fixture.provenance.independent_reproduction);

    assert!(!fixture.vectors.is_empty());
    assert!(!fixture.failure_vectors.is_empty());
    assert!(!fixture.continuation_vectors.is_empty());
}

#[test]
fn expanded_fixture_has_phase_004_coverage() {
    let fixture = load_conformance_file();

    assert_eq!(fixture.vectors.len(), 34);
    assert_eq!(fixture.failure_vectors.len(), 5);
    assert_eq!(fixture.continuation_vectors.len(), 8);

    assert!(
        fixture
            .vectors
            .iter()
            .any(|vector| vector.category == "repeated")
    );
    assert!(
        fixture
            .vectors
            .iter()
            .any(|vector| vector.category == "alternating")
    );
    assert!(
        fixture
            .vectors
            .iter()
            .any(|vector| vector.category == "mixed")
    );

    let maximum_depth = fixture
        .vectors
        .iter()
        .map(|vector| vector.depth)
        .max()
        .expect("success vector corpus must not be empty");

    assert_eq!(maximum_depth, 128);

    let depth_128_exceeds_i128 = fixture
        .vectors
        .iter()
        .filter(|vector| vector.depth == 128)
        .any(|vector| vector.p.parse::<i128>().is_err() || vector.q.parse::<i128>().is_err());

    assert!(
        depth_128_exceeds_i128,
        "depth-128 corpus should exercise arbitrary-precision integer values"
    );
}

#[test]
fn external_conformance_vectors_match_exact_recurrence() {
    let fixture = load_conformance_file();

    for vector in fixture.vectors {
        let path: BaselinePath = vector.path.parse().unwrap_or_else(|error| {
            panic!(
                "invalid fixture path for vector {:?}: {:?}: {error}",
                vector.id, vector.path
            )
        });

        let state = evaluate(&path).unwrap_or_else(|error| {
            panic!(
                "evaluation failed for conformance vector {:?}: {error}",
                vector.id
            )
        });

        assert_exact_state(&vector.id, &state, &vector.p, &vector.q, vector.depth);
    }
}

#[test]
fn external_failure_vectors_match_deterministic_path_rejection() {
    let fixture = load_conformance_file();

    for vector in fixture.failure_vectors {
        assert_eq!(
            vector.expected_error, "InvalidPathSymbol",
            "unsupported expected error in failure vector {:?}",
            vector.id
        );

        let mut symbols = vector.symbol.chars();
        let expected_symbol = symbols.next().unwrap_or_else(|| {
            panic!(
                "failure vector {:?} contains an empty expected symbol",
                vector.id
            )
        });

        assert!(
            symbols.next().is_none(),
            "failure vector {:?} expected symbol must contain exactly one character",
            vector.id
        );

        let result = vector.path.parse::<BaselinePath>();

        assert_eq!(
            result,
            Err(FerError::InvalidPathSymbol {
                index: vector.index,
                symbol: expected_symbol,
            }),
            "failure mismatch for conformance vector {:?}",
            vector.id
        );
    }
}

#[test]
fn external_continuation_vectors_match_full_root_evaluation() {
    let fixture = load_conformance_file();

    for vector in fixture.continuation_vectors {
        assert_eq!(
            format!("{}{}", vector.parent_path, vector.suffix),
            vector.full_path,
            "continuation path composition mismatch for {:?}",
            vector.id
        );

        let parent: BaselinePath = vector.parent_path.parse().unwrap_or_else(|error| {
            panic!(
                "invalid parent path for continuation vector {:?}: {error}",
                vector.id
            )
        });

        let suffix: BaselinePath = vector.suffix.parse().unwrap_or_else(|error| {
            panic!(
                "invalid suffix for continuation vector {:?}: {error}",
                vector.id
            )
        });

        let full: BaselinePath = vector.full_path.parse().unwrap_or_else(|error| {
            panic!(
                "invalid full path for continuation vector {:?}: {error}",
                vector.id
            )
        });

        let mut continued = evaluate(&parent).unwrap_or_else(|error| {
            panic!(
                "parent evaluation failed for continuation vector {:?}: {error}",
                vector.id
            )
        });

        for transform in suffix.iter() {
            continued = apply_transform(&continued, transform).unwrap_or_else(|error| {
                panic!(
                    "continuation failed for conformance vector {:?}: {error}",
                    vector.id
                )
            });
        }

        let reevaluated = evaluate(&full).unwrap_or_else(|error| {
            panic!(
                "full evaluation failed for continuation vector {:?}: {error}",
                vector.id
            )
        });

        assert_eq!(
            continued, reevaluated,
            "continuation/root reevaluation mismatch for {:?}",
            vector.id
        );

        assert_exact_state(&vector.id, &continued, &vector.p, &vector.q, vector.depth);
    }
}

#[test]
fn continuation_matches_re_evaluation_from_root() {
    let full: BaselinePath = "001101".parse().unwrap();
    let parent: BaselinePath = "00110".parse().unwrap();

    let expected = evaluate(&full).unwrap();
    let parent_state = evaluate(&parent).unwrap();

    let continued = apply_transform(&parent_state, TopologyBit::One).unwrap();

    assert_eq!(continued, expected);
}

#[test]
fn continuation_matches_root_evaluation_across_multiple_paths() {
    let cases = [
        ("0", TopologyBit::Zero, "00"),
        ("0", TopologyBit::One, "01"),
        ("1", TopologyBit::Zero, "10"),
        ("1", TopologyBit::One, "11"),
        ("010", TopologyBit::One, "0101"),
        ("101", TopologyBit::Zero, "1010"),
        ("00110", TopologyBit::One, "001101"),
    ];

    for (parent_path, transform, full_path) in cases {
        let parent: BaselinePath = parent_path.parse().unwrap();
        let full: BaselinePath = full_path.parse().unwrap();

        let parent_state = evaluate(&parent).unwrap();
        let continued = apply_transform(&parent_state, transform).unwrap();
        let reevaluated = evaluate(&full).unwrap();

        assert_eq!(continued, reevaluated, "failed for {full_path}");
    }
}

#[test]
fn malformed_human_readable_path_fails_deterministically() {
    let result = "0102".parse::<BaselinePath>();

    assert_eq!(
        result,
        Err(FerError::InvalidPathSymbol {
            index: 3,
            symbol: '2',
        })
    );
}

#[test]
fn path_depth_is_distinct_from_bit_prefix_value() {
    let root: BaselinePath = "".parse().unwrap();
    let one_zero: BaselinePath = "0".parse().unwrap();
    let two_zeroes: BaselinePath = "00".parse().unwrap();
    let three_zeroes: BaselinePath = "000".parse().unwrap();

    assert_ne!(root, one_zero);
    assert_ne!(one_zero, two_zeroes);
    assert_ne!(two_zeroes, three_zeroes);

    assert_eq!(root.len(), 0);
    assert_eq!(one_zero.len(), 1);
    assert_eq!(two_zeroes.len(), 2);
    assert_eq!(three_zeroes.len(), 3);
}
