use fme_fer::BaselinePath;
use fme_fer::baseline_v1::{PROFILE_ID, evaluate};
use fme_fer::wire_v1::{
    CanonicalWireError, decode_exact_state, decode_path, encode_exact_state, encode_path,
};
use num_bigint::BigInt;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct WireFixture {
    fixture_format_version: u64,
    normative_wire_format: bool,
    wire_format_id: String,
    profile_id: String,
    provenance: Provenance,
    path_vectors: Vec<PathVector>,
    state_vectors: Vec<StateVector>,
    failure_vectors: Vec<FailureVector>,
}

#[derive(Debug, Deserialize)]
struct Provenance {
    phase: String,
    reference_method: String,
    reference_script: String,
    independent_reproduction: bool,
}

#[derive(Debug, Deserialize)]
struct PathVector {
    id: String,
    path: String,
    bit_length: usize,
    canonical_hex: String,
}

#[derive(Debug, Deserialize)]
struct StateVector {
    id: String,
    source_path: String,
    p: String,
    q: String,
    depth: u64,
    canonical_hex: String,
}

#[derive(Debug, Deserialize)]
struct FailureVector {
    id: String,
    decode_as: String,
    hex: String,
    expected_error: String,
}

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../conformance/fer-affine-2d-binary-v1/canonical-wire-v1.json")
}

fn load_fixture() -> WireFixture {
    let path = fixture_path();
    let bytes = fs::read(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

    serde_json::from_slice(&bytes)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
}

fn decode_hex(vector_id: &str, value: &str) -> Vec<u8> {
    assert!(
        value.len().is_multiple_of(2),
        "odd-length hex in vector {vector_id:?}"
    );

    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let text = std::str::from_utf8(pair)
                .unwrap_or_else(|error| panic!("invalid UTF-8 hex in {vector_id:?}: {error}"));

            u8::from_str_radix(text, 16)
                .unwrap_or_else(|error| panic!("invalid hex in {vector_id:?}: {error}"))
        })
        .collect()
}

fn error_name(error: &CanonicalWireError) -> &'static str {
    match error {
        CanonicalWireError::Truncated => "Truncated",
        CanonicalWireError::InvalidMagic => "InvalidMagic",
        CanonicalWireError::UnsupportedVersion { .. } => "UnsupportedVersion",
        CanonicalWireError::UnexpectedObjectKind { .. } => "UnexpectedObjectKind",
        CanonicalWireError::ProfileMismatch => "ProfileMismatch",
        CanonicalWireError::LengthOverflow => "LengthOverflow",
        CanonicalWireError::InvalidIntegerSign { .. } => "InvalidIntegerSign",
        CanonicalWireError::NonCanonicalZero => "NonCanonicalZero",
        CanonicalWireError::NonCanonicalIntegerMagnitude => "NonCanonicalIntegerMagnitude",
        CanonicalWireError::NonZeroPathPadding => "NonZeroPathPadding",
        CanonicalWireError::TrailingBytes => "TrailingBytes",
    }
}

#[test]
fn canonical_wire_fixture_metadata_is_expected() {
    let fixture = load_fixture();

    assert_eq!(fixture.fixture_format_version, 1);
    assert!(fixture.normative_wire_format);
    assert_eq!(
        fixture.wire_format_id,
        "FME-FER-AFFINE-2D-BINARY-V1-WIRE-V1"
    );
    assert_eq!(fixture.profile_id, PROFILE_ID);

    assert_eq!(fixture.provenance.phase, "006");
    assert_eq!(
        fixture.provenance.reference_method,
        "secondary-python-byte-encoder"
    );
    assert_eq!(
        fixture.provenance.reference_script,
        "scripts/conformance/generate_fer_canonical_wire_v1.py"
    );
    assert!(!fixture.provenance.independent_reproduction);

    assert_eq!(fixture.path_vectors.len(), 8);
    assert_eq!(fixture.state_vectors.len(), 8);
    assert_eq!(fixture.failure_vectors.len(), 14);
}

#[test]
fn canonical_path_vectors_match_normative_bytes_and_round_trip() {
    let fixture = load_fixture();

    for vector in fixture.path_vectors {
        let path: BaselinePath = vector
            .path
            .parse()
            .unwrap_or_else(|error| panic!("invalid logical path for {:?}: {error}", vector.id));

        assert_eq!(
            path.len(),
            vector.bit_length,
            "bit length mismatch for {:?}",
            vector.id
        );

        let expected = decode_hex(&vector.id, &vector.canonical_hex);
        let encoded = encode_path(&path)
            .unwrap_or_else(|error| panic!("encode failed for {:?}: {error}", vector.id));

        assert_eq!(
            encoded, expected,
            "canonical path bytes mismatch for {:?}",
            vector.id
        );

        let decoded = decode_path(&expected)
            .unwrap_or_else(|error| panic!("decode failed for {:?}: {error}", vector.id));

        assert_eq!(
            decoded, path,
            "canonical path round trip mismatch for {:?}",
            vector.id
        );

        assert_eq!(
            encode_path(&decoded).unwrap(),
            expected,
            "canonical path re-encoding mismatch for {:?}",
            vector.id
        );
    }
}

#[test]
fn canonical_state_vectors_match_normative_bytes_and_round_trip() {
    let fixture = load_fixture();

    for vector in fixture.state_vectors {
        let path: BaselinePath = vector
            .source_path
            .parse()
            .unwrap_or_else(|error| panic!("invalid source path for {:?}: {error}", vector.id));

        let state = evaluate(&path)
            .unwrap_or_else(|error| panic!("FER evaluation failed for {:?}: {error}", vector.id));

        assert_eq!(
            state.p(),
            &vector.p.parse::<BigInt>().unwrap(),
            "P mismatch for {:?}",
            vector.id
        );
        assert_eq!(
            state.q(),
            &vector.q.parse::<BigInt>().unwrap(),
            "Q mismatch for {:?}",
            vector.id
        );
        assert_eq!(
            state.depth(),
            vector.depth,
            "depth mismatch for {:?}",
            vector.id
        );

        let expected = decode_hex(&vector.id, &vector.canonical_hex);
        let encoded = encode_exact_state(&state)
            .unwrap_or_else(|error| panic!("encode failed for {:?}: {error}", vector.id));

        assert_eq!(
            encoded, expected,
            "canonical state bytes mismatch for {:?}",
            vector.id
        );

        let decoded = decode_exact_state(&expected)
            .unwrap_or_else(|error| panic!("decode failed for {:?}: {error}", vector.id));

        assert_eq!(
            decoded, state,
            "canonical state round trip mismatch for {:?}",
            vector.id
        );

        assert_eq!(
            encode_exact_state(&decoded).unwrap(),
            expected,
            "canonical state re-encoding mismatch for {:?}",
            vector.id
        );
    }
}

#[test]
fn malformed_and_noncanonical_wire_vectors_fail_deterministically() {
    let fixture = load_fixture();

    for vector in fixture.failure_vectors {
        let bytes = decode_hex(&vector.id, &vector.hex);

        let error = match vector.decode_as.as_str() {
            "path" => decode_path(&bytes).expect_err("malformed path vector unexpectedly decoded"),
            "exact_state" => {
                decode_exact_state(&bytes).expect_err("malformed state vector unexpectedly decoded")
            }
            other => panic!(
                "unsupported decode target {other:?} in failure vector {:?}",
                vector.id
            ),
        };

        assert_eq!(
            error_name(&error),
            vector.expected_error,
            "unexpected error for failure vector {:?}: {:?}",
            vector.id,
            error
        );
    }
}
