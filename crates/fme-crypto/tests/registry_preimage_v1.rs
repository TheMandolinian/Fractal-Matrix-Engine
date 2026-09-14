use fme_crypto::{
    DOMAIN_REGISTRY_ID, DomainId, HASH_SUITE_REGISTRY_ID, HashSuiteId, PREIMAGE_PROFILE_ID,
    PREIMAGE_VERSION, PreimageError, RegistryError, decode_preimage, encode_preimage,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    profile_id: String,
    domain_registry_id: String,
    hash_suite_registry_id: String,
    magic_ascii: String,
    version: u8,
    domains: Vec<DomainVector>,
    hash_suites: Vec<HashSuiteVector>,
    preimage_vectors: Vec<PreimageVector>,
    malformed_vectors: Vec<MalformedVector>,
}

#[derive(Debug, Deserialize)]
struct DomainVector {
    name: String,
    identifier: String,
    identifier_hex: String,
}

#[derive(Debug, Deserialize)]
struct HashSuiteVector {
    name: String,
    identifier: String,
    identifier_hex: String,
    digest_length: usize,
}

#[derive(Debug, Deserialize)]
struct PreimageVector {
    name: String,
    domain: String,
    payload_hex: String,
    preimage_hex: String,
}

#[derive(Debug, Deserialize)]
struct MalformedVector {
    name: String,
    preimage_hex: String,
    expected_error: String,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!(
        "../../../conformance/crypto-registry-v1/crypto-registry-v1.json"
    ))
    .expect("Phase 007 fixture must be valid JSON")
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

#[test]
fn fixture_metadata_and_registry_identifiers_are_exact() {
    let fixture = fixture();

    assert_eq!(fixture.profile_id, PREIMAGE_PROFILE_ID);
    assert_eq!(fixture.domain_registry_id, DOMAIN_REGISTRY_ID);
    assert_eq!(fixture.hash_suite_registry_id, HASH_SUITE_REGISTRY_ID);
    assert_eq!(fixture.magic_ascii, "FMEP");
    assert_eq!(fixture.version, PREIMAGE_VERSION);

    assert_eq!(fixture.domains.len(), 2);
    assert_eq!(fixture.hash_suites.len(), 2);

    for vector in fixture.domains {
        let domain = domain_from_name(&vector.name);

        assert_eq!(domain.as_str(), vector.identifier);
        assert_eq!(domain.as_bytes(), decode_hex(&vector.identifier_hex));
        assert_eq!(DomainId::from_bytes(domain.as_bytes()), Ok(domain));
    }

    for vector in fixture.hash_suites {
        let suite = match vector.name.as_str() {
            "sha2_256" => HashSuiteId::Sha2_256,
            "sha2_512" => HashSuiteId::Sha2_512,
            other => panic!("unknown fixture hash suite {other}"),
        };

        assert_eq!(suite.as_str(), vector.identifier);
        assert_eq!(suite.as_bytes(), decode_hex(&vector.identifier_hex));
        assert_eq!(suite.digest_len(), vector.digest_length);
        assert_eq!(HashSuiteId::from_bytes(suite.as_bytes()), Ok(suite));
    }

    assert_eq!(
        DomainId::from_bytes(b"FME/UNKNOWN/V1"),
        Err(RegistryError::UnknownDomainIdentifier)
    );

    assert_eq!(
        HashSuiteId::from_bytes(b"SHA2-384"),
        Err(RegistryError::UnknownHashSuiteIdentifier)
    );
}

#[test]
fn canonical_preimage_vectors_match_and_round_trip() {
    let fixture = fixture();

    assert_eq!(fixture.preimage_vectors.len(), 3);

    for vector in fixture.preimage_vectors {
        let domain = domain_from_name(&vector.domain);
        let payload = decode_hex(&vector.payload_hex);
        let expected = decode_hex(&vector.preimage_hex);

        let encoded = encode_preimage(domain, &payload)
            .unwrap_or_else(|error| panic!("{} failed to encode: {error}", vector.name));

        assert_eq!(encoded, expected, "vector {}", vector.name);

        let decoded = decode_preimage(&expected)
            .unwrap_or_else(|error| panic!("{} failed to decode: {error}", vector.name));

        assert_eq!(decoded.domain(), domain, "vector {}", vector.name);
        assert_eq!(
            decoded.canonical_payload(),
            payload,
            "vector {}",
            vector.name
        );
    }
}

#[test]
fn equal_payloads_in_distinct_domains_produce_distinct_preimages() {
    let payload = [0x00, 0x01, 0x02, 0xff];

    let singularity =
        encode_preimage(DomainId::SingularityV1, &payload).expect("encoding must succeed");

    let authority =
        encode_preimage(DomainId::AuthorityDomainV1, &payload).expect("encoding must succeed");

    assert_ne!(singularity, authority);
}

#[test]
fn malformed_preimage_vectors_fail_deterministically() {
    let fixture = fixture();

    assert_eq!(fixture.malformed_vectors.len(), 6);

    for vector in fixture.malformed_vectors {
        let bytes = decode_hex(&vector.preimage_hex);
        let error = decode_preimage(&bytes).unwrap_err();

        match vector.expected_error.as_str() {
            "InvalidMagic" => assert_eq!(error, PreimageError::InvalidMagic),
            "UnsupportedVersion" => {
                assert!(matches!(error, PreimageError::UnsupportedVersion { .. }))
            }
            "UnknownDomain" => assert_eq!(error, PreimageError::UnknownDomain),
            "Truncated" => assert_eq!(error, PreimageError::Truncated),
            "TrailingBytes" => assert_eq!(error, PreimageError::TrailingBytes),
            other => panic!("unknown expected error {other} for {}", vector.name),
        }
    }
}
