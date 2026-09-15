use fme_crypto::HashSuiteId;
use fme_sra::{
    SRA_ARTIFACT_VERSION, SRA_CANONICAL_PROFILE_ID, SRA_FER_PROFILE_ID, SRA_MAGIC,
    SRA_WIRE_VERSION, SingularityRootArtifactV1, SraError, decode_sra, encode_sra,
};

fn artifact(suite: HashSuiteId, root_seed: Option<Vec<u8>>) -> SingularityRootArtifactV1 {
    SingularityRootArtifactV1::new(b"example-system".to_vec(), suite, root_seed)
        .expect("valid baseline SRA")
}

#[test]
fn rejects_empty_namespace() {
    let error = SingularityRootArtifactV1::new(Vec::new(), HashSuiteId::Sha2_256, None)
        .expect_err("empty namespace must fail");

    assert_eq!(error, SraError::EmptySystemNamespace);
}

#[test]
fn rejects_present_empty_root_seed() {
    let error = SingularityRootArtifactV1::new(
        b"example-system".to_vec(),
        HashSuiteId::Sha2_256,
        Some(Vec::new()),
    )
    .expect_err("present empty root seed must fail");

    assert_eq!(error, SraError::EmptyRootSeed);
}

#[test]
fn sha2_256_without_root_seed_round_trips() {
    let original = artifact(HashSuiteId::Sha2_256, None);
    let encoded = encode_sra(&original).expect("encode");
    let decoded = decode_sra(&encoded).expect("decode");

    assert_eq!(decoded, original);
    assert_eq!(decoded.default_hash_suite_id(), HashSuiteId::Sha2_256);
    assert_eq!(decoded.root_seed(), None);
}

#[test]
fn sha2_512_with_root_seed_round_trips() {
    let original = artifact(HashSuiteId::Sha2_512, Some(vec![0x00, 0x01, 0x02, 0xff]));
    let encoded = encode_sra(&original).expect("encode");
    let decoded = decode_sra(&encoded).expect("decode");

    assert_eq!(decoded, original);
    assert_eq!(decoded.default_hash_suite_id(), HashSuiteId::Sha2_512);
    assert_eq!(decoded.root_seed(), Some(&[0x00, 0x01, 0x02, 0xff][..]));
}

#[test]
fn encoding_is_deterministic() {
    let artifact = artifact(HashSuiteId::Sha2_512, Some(b"deterministic-seed".to_vec()));

    let first = encode_sra(&artifact).expect("first encode");
    let second = encode_sra(&artifact).expect("second encode");

    assert_eq!(first, second);
}

#[test]
fn decoded_bytes_reencode_exactly() {
    let artifact = artifact(HashSuiteId::Sha2_256, Some(vec![0xaa, 0xbb, 0xcc]));

    let encoded = encode_sra(&artifact).expect("encode");
    let decoded = decode_sra(&encoded).expect("decode");
    let reencoded = encode_sra(&decoded).expect("reencode");

    assert_eq!(reencoded, encoded);
}

fn push_length_prefixed(bytes: &[u8], output: &mut Vec<u8>) {
    let length = u64::try_from(bytes.len()).expect("test fixture length");
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(bytes);
}

struct RawSra<'a> {
    magic: &'a [u8],
    wire_version: u8,
    canonical_profile: &'a [u8],
    artifact_version: u8,
    namespace: &'a [u8],
    fer_profile: &'a [u8],
    hash_suite: &'a [u8],
    root_seed_presence: u8,
    root_seed: Option<&'a [u8]>,
}

fn baseline_raw_sra() -> RawSra<'static> {
    RawSra {
        magic: &SRA_MAGIC,
        wire_version: SRA_WIRE_VERSION,
        canonical_profile: SRA_CANONICAL_PROFILE_ID.as_bytes(),
        artifact_version: SRA_ARTIFACT_VERSION,
        namespace: b"example-system",
        fer_profile: SRA_FER_PROFILE_ID.as_bytes(),
        hash_suite: HashSuiteId::SHA2_256_BYTES,
        root_seed_presence: 0x00,
        root_seed: None,
    }
}

fn raw_sra(raw: RawSra<'_>) -> Vec<u8> {
    let mut output = Vec::new();
    output.extend_from_slice(raw.magic);
    output.push(raw.wire_version);
    push_length_prefixed(raw.canonical_profile, &mut output);
    output.push(raw.artifact_version);
    push_length_prefixed(raw.namespace, &mut output);
    push_length_prefixed(raw.fer_profile, &mut output);
    push_length_prefixed(raw.hash_suite, &mut output);
    output.push(raw.root_seed_presence);

    if let Some(seed) = raw.root_seed {
        push_length_prefixed(seed, &mut output);
    }

    output
}

#[test]
fn rejects_invalid_magic() {
    let bytes = raw_sra(RawSra {
        magic: b"BAD!",
        ..baseline_raw_sra()
    });

    assert_eq!(decode_sra(&bytes), Err(SraError::InvalidMagic));
}

#[test]
fn rejects_unsupported_wire_version() {
    let bytes = raw_sra(RawSra {
        wire_version: 0x02,
        ..baseline_raw_sra()
    });

    assert_eq!(
        decode_sra(&bytes),
        Err(SraError::UnsupportedWireVersion { found: 0x02 })
    );
}

#[test]
fn rejects_canonical_profile_mismatch() {
    let bytes = raw_sra(RawSra {
        canonical_profile: b"FME-SRA-CANONICAL-V2",
        ..baseline_raw_sra()
    });

    assert_eq!(decode_sra(&bytes), Err(SraError::CanonicalProfileMismatch));
}

#[test]
fn rejects_unsupported_artifact_version() {
    let bytes = raw_sra(RawSra {
        artifact_version: 0x02,
        ..baseline_raw_sra()
    });

    assert_eq!(
        decode_sra(&bytes),
        Err(SraError::UnsupportedArtifactVersion { found: 0x02 })
    );
}

#[test]
fn rejects_fer_profile_mismatch() {
    let bytes = raw_sra(RawSra {
        fer_profile: b"FME-FER-UNKNOWN-V1",
        ..baseline_raw_sra()
    });

    assert_eq!(decode_sra(&bytes), Err(SraError::FerProfileMismatch));
}

#[test]
fn rejects_unknown_hash_suite() {
    let bytes = raw_sra(RawSra {
        hash_suite: b"SHA3-256",
        ..baseline_raw_sra()
    });

    assert_eq!(
        decode_sra(&bytes),
        Err(SraError::UnknownHashSuiteIdentifier)
    );
}

#[test]
fn rejects_invalid_root_seed_presence() {
    let bytes = raw_sra(RawSra {
        root_seed_presence: 0x02,
        ..baseline_raw_sra()
    });

    assert_eq!(
        decode_sra(&bytes),
        Err(SraError::InvalidRootSeedPresence { found: 0x02 })
    );
}

#[test]
fn rejects_encoded_empty_namespace() {
    let bytes = raw_sra(RawSra {
        namespace: b"",
        ..baseline_raw_sra()
    });

    assert_eq!(decode_sra(&bytes), Err(SraError::EmptySystemNamespace));
}

#[test]
fn rejects_encoded_present_empty_root_seed() {
    let bytes = raw_sra(RawSra {
        root_seed_presence: 0x01,
        root_seed: Some(b""),
        ..baseline_raw_sra()
    });

    assert_eq!(decode_sra(&bytes), Err(SraError::EmptyRootSeed));
}

#[test]
fn rejects_truncated_input() {
    let artifact = artifact(HashSuiteId::Sha2_512, Some(b"root-seed".to_vec()));
    let mut bytes = encode_sra(&artifact).expect("encode");
    bytes.pop();

    assert_eq!(decode_sra(&bytes), Err(SraError::Truncated));
}

#[test]
fn rejects_trailing_bytes() {
    let artifact = artifact(HashSuiteId::Sha2_256, None);
    let mut bytes = encode_sra(&artifact).expect("encode");
    bytes.push(0x00);

    assert_eq!(decode_sra(&bytes), Err(SraError::TrailingBytes));
}
