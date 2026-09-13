use fme_fer::baseline_v1::{PROFILE_ID, apply_transform, evaluate, root};
use fme_fer::{BaselinePath, FerError, TopologyBit};
use num_bigint::BigInt;

fn assert_state(path: &str, expected_p: i64, expected_q: i64, expected_depth: u64) {
    let path: BaselinePath = path.parse().expect("test path must be valid");
    let state = evaluate(&path).expect("FER evaluation must succeed");

    assert_eq!(state.p(), &BigInt::from(expected_p));
    assert_eq!(state.q(), &BigInt::from(expected_q));
    assert_eq!(state.depth(), expected_depth);
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
fn published_shallow_vectors_match_exact_recurrence() {
    let vectors = [
        ("", 0, 0, 0),
        ("0", -3, 0, 1),
        ("1", 3, 0, 1),
        ("00", -12, -3, 2),
        ("01", 6, 3, 2),
        ("10", -6, 3, 2),
        ("11", 12, -3, 2),
        ("000", -36, -15, 3),
        ("111", 36, -15, 3),
        ("0101", 66, 33, 4),
        ("1010", -66, 33, 4),
        ("001101", 690, 237, 6),
    ];

    for (path, p, q, depth) in vectors {
        assert_state(path, p, q, depth);
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
