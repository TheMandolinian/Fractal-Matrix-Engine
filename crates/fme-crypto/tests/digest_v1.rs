use fme_crypto::{
    DIGEST_PROFILE_ID, DigestError, DomainId, HASH_SUITE_REGISTRY_ID, HashSuiteId,
    PREIMAGE_PROFILE_ID, PreimageError, digest_preimage, encode_preimage,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DigestFixture {
    profile_id: String,
    preimage_profile_id: String,
    hash_suite_registry_id: String,
    digest_vectors: Vec<DigestVector>,
}

#[derive(Debug, Deserialize)]
struct DigestVector {
    name: String,
    hash_suite: String,
    hash_suite_identifier: String,
    digest_length: usize,
    domain: String,
    payload_hex: String,
    preimage_hex: String,
    digest_hex: String,
}

#[derive(Debug, Deserialize)]
struct Phase007Fixture {
    malformed_vectors: Vec<MalformedVector>,
}

#[derive(Debug, Deserialize)]
struct MalformedVector {
    name: String,
    preimage_hex: String,
    expected_error: String,
}

fn digest_fixture() -> DigestFixture {
    serde_json::from_str(include_str!(
        "../../../conformance/crypto-digest-v1/crypto-digest-v1.json"
    ))
    .expect("Phase 008 digest fixture must be valid JSON")
}

fn phase_007_fixture() -> Phase007Fixture {
    serde_json::from_str(include_str!(
        "../../../conformance/crypto-registry-v1/crypto-registry-v1.json"
    ))
    .expect("Phase 007 fixture must remain valid JSON")
}

fn decode_hex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0, "hex input must have even length");

    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let pair = std::str::from_utf8(pair).expect("hex must be ASCII");
            u8::from_str_radix(pair, 16).expect("fixture must contain valid hex")
        })
        .collect()
}

fn domain_from_name(name: &str) -> DomainId {
    match name {
        "singularity_v1" => DomainId::SingularityV1,
        "authority_domain_v1" => DomainId::AuthorityDomainV1,
        other => panic!("unknown fixture domain {other}"),
    }
}

fn suite_from_name(name: &str) -> HashSuiteId {
    match name {
        "sha2_256" => HashSuiteId::Sha2_256,
        "sha2_512" => HashSuiteId::Sha2_512,
        other => panic!("unknown fixture hash suite {other}"),
    }
}

#[test]
fn registered_sha2_vectors_match_independent_fixture() {
    let fixture = digest_fixture();

    assert_eq!(fixture.profile_id, DIGEST_PROFILE_ID);
    assert_eq!(fixture.preimage_profile_id, PREIMAGE_PROFILE_ID);
    assert_eq!(fixture.hash_suite_registry_id, HASH_SUITE_REGISTRY_ID);
    assert_eq!(fixture.digest_vectors.len(), 6);

    for vector in fixture.digest_vectors {
        let domain = domain_from_name(&vector.domain);
        let suite = suite_from_name(&vector.hash_suite);
        let payload = decode_hex(&vector.payload_hex);
        let expected_preimage = decode_hex(&vector.preimage_hex);
        let expected_digest = decode_hex(&vector.digest_hex);

        assert_eq!(suite.as_str(), vector.hash_suite_identifier);
        assert_eq!(suite.digest_len(), vector.digest_length);

        let encoded = encode_preimage(domain, &payload)
            .unwrap_or_else(|error| panic!("{} failed to encode: {error}", vector.name));

        assert_eq!(encoded, expected_preimage, "vector {}", vector.name);

        let digest = digest_preimage(suite, &expected_preimage)
            .unwrap_or_else(|error| panic!("{} failed to digest: {error}", vector.name));

        assert_eq!(digest.hash_suite(), suite, "vector {}", vector.name);
        assert_eq!(
            digest.as_bytes().len(),
            suite.digest_len(),
            "vector {}",
            vector.name
        );
        assert_eq!(digest.as_bytes(), expected_digest, "vector {}", vector.name);

        let repeated = digest_preimage(suite, &expected_preimage)
            .unwrap_or_else(|error| panic!("{} repeat failed: {error}", vector.name));

        assert_eq!(digest, repeated, "vector {}", vector.name);
    }
}

#[test]
fn same_preimage_can_be_processed_by_both_registered_suites() {
    let preimage = encode_preimage(DomainId::SingularityV1, &[0x00, 0x01, 0x02, 0xff])
        .expect("encoding must succeed");

    let sha256 = digest_preimage(HashSuiteId::Sha2_256, &preimage).expect("SHA2-256 must succeed");
    let sha512 = digest_preimage(HashSuiteId::Sha2_512, &preimage).expect("SHA2-512 must succeed");

    assert_eq!(sha256.hash_suite(), HashSuiteId::Sha2_256);
    assert_eq!(sha512.hash_suite(), HashSuiteId::Sha2_512);
    assert_eq!(sha256.as_bytes().len(), 32);
    assert_eq!(sha512.as_bytes().len(), 64);
    assert_ne!(sha256.as_bytes(), sha512.as_bytes());
}

#[test]
fn equal_payloads_in_distinct_domains_produce_distinct_digests() {
    let payload = [0x00, 0x01, 0x02, 0xff];

    let singularity =
        encode_preimage(DomainId::SingularityV1, &payload).expect("encoding must succeed");
    let authority =
        encode_preimage(DomainId::AuthorityDomainV1, &payload).expect("encoding must succeed");

    for suite in [HashSuiteId::Sha2_256, HashSuiteId::Sha2_512] {
        let singularity_digest =
            digest_preimage(suite, &singularity).expect("singularity digest must succeed");
        let authority_digest =
            digest_preimage(suite, &authority).expect("authority digest must succeed");

        assert_ne!(
            singularity_digest.as_bytes(),
            authority_digest.as_bytes(),
            "domain separation failed for {}",
            suite.as_str()
        );
    }
}

#[test]
fn malformed_phase_007_preimages_fail_before_digest_execution() {
    let fixture = phase_007_fixture();

    assert_eq!(fixture.malformed_vectors.len(), 6);

    for vector in fixture.malformed_vectors {
        let bytes = decode_hex(&vector.preimage_hex);

        for suite in [HashSuiteId::Sha2_256, HashSuiteId::Sha2_512] {
            let error = digest_preimage(suite, &bytes).unwrap_err();

            match vector.expected_error.as_str() {
                "InvalidMagic" => assert_eq!(
                    error,
                    DigestError::InvalidPreimage(PreimageError::InvalidMagic)
                ),
                "UnsupportedVersion" => assert!(matches!(
                    error,
                    DigestError::InvalidPreimage(PreimageError::UnsupportedVersion { .. })
                )),
                "UnknownDomain" => assert_eq!(
                    error,
                    DigestError::InvalidPreimage(PreimageError::UnknownDomain)
                ),
                "Truncated" => assert_eq!(
                    error,
                    DigestError::InvalidPreimage(PreimageError::Truncated)
                ),
                "TrailingBytes" => assert_eq!(
                    error,
                    DigestError::InvalidPreimage(PreimageError::TrailingBytes)
                ),
                other => panic!("unknown expected error {other} for {}", vector.name),
            }
        }
    }
}
