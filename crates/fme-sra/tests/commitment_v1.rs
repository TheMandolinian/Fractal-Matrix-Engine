use fme_crypto::{DomainId, HashSuiteId, digest_preimage, encode_preimage};
use fme_sra::{SRA_COMMITMENT_PROFILE_ID, SingularityRootArtifactV1, commit_sra, encode_sra};

fn artifact(suite: HashSuiteId) -> SingularityRootArtifactV1 {
    SingularityRootArtifactV1::new(
        b"phase-010-system".to_vec(),
        suite,
        Some(b"phase-010-seed".to_vec()),
    )
    .expect("valid baseline SRA")
}

#[test]
fn commitment_uses_declared_sra_hash_suite() {
    for suite in [HashSuiteId::Sha2_256, HashSuiteId::Sha2_512] {
        let artifact = artifact(suite);
        let commitment = commit_sra(&artifact).expect("commitment must succeed");

        assert_eq!(commitment.profile_id(), SRA_COMMITMENT_PROFILE_ID);
        assert_eq!(commitment.domain(), DomainId::SingularityV1);
        assert_eq!(commitment.hash_suite(), suite);
        assert_eq!(commitment.as_bytes().len(), suite.digest_len());
    }
}

#[test]
fn commitment_exactly_composes_phases_009_007_and_008() {
    let artifact = artifact(HashSuiteId::Sha2_512);

    let canonical = encode_sra(&artifact).expect("canonical SRA encoding");
    let preimage =
        encode_preimage(DomainId::SingularityV1, &canonical).expect("canonical preimage");
    let expected =
        digest_preimage(artifact.default_hash_suite_id(), &preimage).expect("registered digest");

    let commitment = commit_sra(&artifact).expect("SRA commitment");

    assert_eq!(commitment.hash_suite(), expected.hash_suite());
    assert_eq!(commitment.as_bytes(), expected.as_bytes());
}

#[test]
fn commitment_is_deterministic() {
    let artifact = artifact(HashSuiteId::Sha2_256);

    let first = commit_sra(&artifact).expect("first commitment");
    let second = commit_sra(&artifact).expect("second commitment");

    assert_eq!(first, second);
}
