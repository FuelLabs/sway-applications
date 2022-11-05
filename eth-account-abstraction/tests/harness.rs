mod utils;

const VALID_SIGNER_PK: &str = "862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301";
const INVALID_SIGNER_PK: &str = "37fa81c84ccd547c30c176b118d5cb892bdb113e8e80141f266519422ef9eefd";

#[tokio::test]
async fn valid_signer() {
    utils::test_recover_and_match_address_with_parameters(VALID_SIGNER_PK).await;
}

#[ignore]
#[tokio::test]
#[should_panic]
async fn invalid_signer() {
    utils::test_recover_and_match_address_with_parameters(INVALID_SIGNER_PK).await;
}