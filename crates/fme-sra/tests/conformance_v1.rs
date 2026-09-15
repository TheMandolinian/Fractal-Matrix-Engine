use fme_crypto::{HASH_SUITE_REGISTRY_ID, HashSuiteId};
use fme_sra::{
    SRA_ARTIFACT_VERSION, SRA_CANONICAL_PROFILE_ID, SRA_FER_PROFILE_ID, SRA_MAGIC,
    SRA_WIRE_FORMAT_ID, SRA_WIRE_VERSION, SingularityRootArtifactV1, SraError, decode_sra,
    encode_sra,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    fixture_format_version: u64,
    normative_wire_format: bool,
    wire_format_id: String,
    profile_id: String,
    fer_profile_id: String,
    hash_suite_registry_id: String,
    magic_ascii: String,
    wire_version: u8,
    artifact_version: u8,
    provenance: Provenance,
    sra_vectors: Vec<SraVector>,
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
struct SraVector {
    id: String,
    system_namespace_hex: String,
    default_hash_suite: String,
    default_hash_suite_identifier: String,
    root_seed_present: bool,
    root_seed_hex: Option<String>,
    canonical_hex: String,
}

#[derive(Debug, Deserialize)]
struct FailureVector {
    id: String,
    hex: String,
    expected_error: String,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!(
        "../../../conformance/sra-canonical-v1/sra-canonical-v1.json"
    ))
    .expect("Phase 009 SRA fixture must be valid JSON")
}

fn decode_hex(vector_id: &str, value: &str) -> Vec<u8> {
    assert_eq!(
        value.len() % 2,
        0,
        "hex input must have even length for {vector_id:?}"
    );

    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let pair = std::str::from_utf8(pair)
                .unwrap_or_else(|error| panic!("invalid hex UTF-8 for {vector_id:?}: {error}"));
            u8::from_str_radix(pair, 16)
                .unwrap_or_else(|error| panic!("invalid hex for {vector_id:?}: {error}"))
        })
        .collect()
}

fn suite_from_name(name: &str) -> HashSuiteId {
    match name {
        "sha2_256" => HashSuiteId::Sha2_256,
        "sha2_512" => HashSuiteId::Sha2_512,
        other => panic!("unknown fixture hash suite {other}"),
    }
}

fn error_name(error: &SraError) -> &'static str {
    match error {
        SraError::EmptySystemNamespace => "EmptySystemNamespace",
        SraError::EmptyRootSeed => "EmptyRootSeed",
        SraError::Truncated => "Truncated",
        SraError::InvalidMagic => "InvalidMagic",
        SraError::UnsupportedWireVersion { .. } => "UnsupportedWireVersion",
        SraError::CanonicalProfileMismatch => "CanonicalProfileMismatch",
        SraError::UnsupportedArtifactVersion { .. } => "UnsupportedArtifactVersion",
        SraError::FerProfileMismatch => "FerProfileMismatch",
        SraError::UnknownHashSuiteIdentifier => "UnknownHashSuiteIdentifier",
        SraError::InvalidRootSeedPresence { .. } => "InvalidRootSeedPresence",
        SraError::LengthOverflow => "LengthOverflow",
        SraError::TrailingBytes => "TrailingBytes",
    }
}

#[test]
fn phase_009_fixture_metadata_matches_normative_contract() {
    let fixture = fixture();

    assert_eq!(fixture.fixture_format_version, 1);
    assert!(fixture.normative_wire_format);

    assert_eq!(fixture.wire_format_id.as_str(), SRA_WIRE_FORMAT_ID);
    assert_eq!(fixture.profile_id.as_str(), SRA_CANONICAL_PROFILE_ID);
    assert_eq!(fixture.fer_profile_id.as_str(), SRA_FER_PROFILE_ID);
    assert_eq!(
        fixture.hash_suite_registry_id.as_str(),
        HASH_SUITE_REGISTRY_ID
    );

    assert_eq!(fixture.magic_ascii.as_bytes(), &SRA_MAGIC);
    assert_eq!(fixture.wire_version, SRA_WIRE_VERSION);
    assert_eq!(fixture.artifact_version, SRA_ARTIFACT_VERSION);

    assert_eq!(fixture.provenance.phase, "009");
    assert_eq!(
        fixture.provenance.reference_method,
        "secondary-python-byte-encoder"
    );
    assert_eq!(
        fixture.provenance.reference_script,
        "scripts/conformance/generate_sra_canonical_v1.py"
    );
    assert!(!fixture.provenance.independent_reproduction);

    assert_eq!(fixture.sra_vectors.len(), 4);
    assert_eq!(fixture.failure_vectors.len(), 11);
}

#[test]
fn phase_009_valid_normative_vectors_match_rust_canonical_encoding() {
    let fixture = fixture();

    for vector in &fixture.sra_vectors {
        let namespace = decode_hex(&vector.id, &vector.system_namespace_hex);
        let suite = suite_from_name(&vector.default_hash_suite);

        assert_eq!(
            suite.as_str(),
            vector.default_hash_suite_identifier.as_str(),
            "hash-suite identifier mismatch for {:?}",
            vector.id
        );

        let root_seed = vector
            .root_seed_hex
            .as_deref()
            .map(|value| decode_hex(&vector.id, value));

        assert_eq!(
            vector.root_seed_present,
            root_seed.is_some(),
            "root-seed presence mismatch for {:?}",
            vector.id
        );

        let artifact =
            SingularityRootArtifactV1::new(namespace, suite, root_seed).unwrap_or_else(|error| {
                panic!(
                    "fixture semantic artifact must construct for {:?}: {error:?}",
                    vector.id
                )
            });

        let fixture_bytes = decode_hex(&vector.id, &vector.canonical_hex);

        let rust_bytes = encode_sra(&artifact).unwrap_or_else(|error| {
            panic!(
                "Rust encoder must accept fixture artifact {:?}: {error:?}",
                vector.id
            )
        });

        assert_eq!(
            rust_bytes, fixture_bytes,
            "canonical bytes mismatch for {:?}",
            vector.id
        );

        let decoded = decode_sra(&fixture_bytes).unwrap_or_else(|error| {
            panic!(
                "Rust decoder must accept canonical fixture {:?}: {error:?}",
                vector.id
            )
        });

        assert_eq!(
            decoded, artifact,
            "decoded semantic artifact mismatch for {:?}",
            vector.id
        );

        let reencoded = encode_sra(&decoded).unwrap_or_else(|error| {
            panic!(
                "Rust re-encoder must accept decoded fixture {:?}: {error:?}",
                vector.id
            )
        });

        assert_eq!(
            reencoded, fixture_bytes,
            "decode/re-encode mismatch for {:?}",
            vector.id
        );
    }
}

#[test]
fn phase_009_malformed_vectors_fail_with_expected_error() {
    let fixture = fixture();

    for vector in &fixture.failure_vectors {
        let bytes = decode_hex(&vector.id, &vector.hex);

        let error = decode_sra(&bytes).unwrap_err();

        assert_eq!(
            error_name(&error),
            vector.expected_error.as_str(),
            "unexpected decoder error for malformed vector {:?}: {error:?}",
            vector.id
        );
    }
}
