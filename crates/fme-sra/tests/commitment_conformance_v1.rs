use fme_crypto::{DomainId, HashSuiteId, encode_preimage};
use fme_sra::{SRA_COMMITMENT_PROFILE_ID, SingularityRootArtifactV1, commit_sra, encode_sra};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    fixture_format_version: u64,
    profile_id: String,
    domain_identifier: String,
    commitment_vectors: Vec<Vector>,
}

#[derive(Debug, Deserialize)]
struct Vector {
    id: String,
    hash_suite: String,
    hash_suite_identifier: String,
    canonical_sra_hex: String,
    preimage_hex: String,
    digest_hex: String,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!(
        "../../../conformance/sra-commitment-v1/sra-commitment-v1.json"
    ))
    .expect("Phase 010 commitment fixture must be valid JSON")
}

fn decode_hex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0);

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

fn suite_from_name(name: &str) -> HashSuiteId {
    match name {
        "sha2_256" => HashSuiteId::Sha2_256,
        "sha2_512" => HashSuiteId::Sha2_512,
        other => panic!("unknown fixture suite {other}"),
    }
}

#[test]
fn phase_010_commitment_vectors_match_rust() {
    let fixture = fixture();

    assert_eq!(fixture.fixture_format_version, 1);
    assert_eq!(fixture.profile_id, SRA_COMMITMENT_PROFILE_ID);
    assert_eq!(fixture.domain_identifier, DomainId::SingularityV1.as_str());
    assert_eq!(fixture.commitment_vectors.len(), 2);

    for vector in fixture.commitment_vectors {
        let suite = suite_from_name(&vector.hash_suite);
        assert_eq!(suite.as_str(), vector.hash_suite_identifier);

        let artifact = match vector.id.as_str() {
            "baseline_sha2_256_no_seed" => {
                SingularityRootArtifactV1::new(b"example-system".to_vec(), suite, None)
            }
            "baseline_sha2_512_seed" => SingularityRootArtifactV1::new(
                b"example-system".to_vec(),
                suite,
                Some(vec![0x00, 0x01, 0x02, 0xff]),
            ),
            other => panic!("unknown commitment vector {other}"),
        }
        .expect("valid fixture SRA");

        let canonical = encode_sra(&artifact).expect("canonical encoding");
        assert_eq!(canonical, decode_hex(&vector.canonical_sra_hex));

        let preimage =
            encode_preimage(DomainId::SingularityV1, &canonical).expect("canonical preimage");
        assert_eq!(preimage, decode_hex(&vector.preimage_hex));

        let commitment = commit_sra(&artifact).expect("SRA commitment");
        assert_eq!(commitment.hash_suite(), suite);
        assert_eq!(commitment.as_bytes(), decode_hex(&vector.digest_hex));
    }
}
