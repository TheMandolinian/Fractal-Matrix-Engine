use fme_fer::baseline_v1::{PROFILE_ID, apply_transform, evaluate, root};
use fme_fer::{BaselinePath, FerError, TopologyBit};
use num_bigint::BigInt;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ConformanceFile {
    fixture_format_version: u64,
    normative_wire_format: bool,
    profile_id: String,
    vectors: Vec<ConformanceVector>,
}

#[derive(Debug, Deserialize)]
struct ConformanceVector {
    path: String,
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

    assert_eq!(fixture.fixture_format_version, 1);
    assert!(!fixture.normative_wire_format);
    assert_eq!(fixture.profile_id, PROFILE_ID);
    assert!(!fixture.vectors.is_empty());
}

#[test]
fn external_conformance_vectors_match_exact_recurrence() {
    let fixture = load_conformance_file();

    for vector in fixture.vectors {
        let path: BaselinePath = vector
            .path
            .parse()
            .unwrap_or_else(|error| panic!("invalid fixture path {:?}: {error}", vector.path));

        let state = evaluate(&path)
            .unwrap_or_else(|error| panic!("evaluation failed for {:?}: {error}", vector.path));

        let expected_p = vector
            .p
            .parse::<BigInt>()
            .unwrap_or_else(|error| panic!("invalid P value {:?}: {error}", vector.p));

        let expected_q = vector
            .q
            .parse::<BigInt>()
            .unwrap_or_else(|error| panic!("invalid Q value {:?}: {error}", vector.q));

        assert_eq!(
            state.p(),
            &expected_p,
            "P mismatch for path {:?}",
            vector.path
        );

        assert_eq!(
            state.q(),
            &expected_q,
            "Q mismatch for path {:?}",
            vector.path
        );

        assert_eq!(
            state.depth(),
            vector.depth,
            "depth mismatch for path {:?}",
            vector.path
        );
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
