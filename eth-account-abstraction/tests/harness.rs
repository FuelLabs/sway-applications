mod utils;

const VALID_SPENDER_PK: &str = "862512a2363db2b3a375c0d4bbbd27172180d89f23f2e259bac850ab02619301";
const INVALID_SPENDER_PK: &str = "37fa81c84ccd547c30c176b118d5cb892bdb113e8e80141f266519422ef9eefd";

// #[tokio::test]
// async fn valid_spender() {
//     utils::test_predicate_spend_with_parameters(VALID_SPENDER_PK).await;
// }

// #[tokio::test]
// #[should_panic]
// async fn invalid_spender() {
//     utils::test_predicate_spend_with_parameters(INVALID_SPENDER_PK).await;
// }





// #[tokio::test]
// async fn run_ecr_script() {
//     utils::test_ecr_script(VALID_SPENDER_PK).await;
// }

// #[tokio::test]
// async fn check_address_hashing() {
//     utils::test_addresses(VALID_SPENDER_PK).await;
// }

#[tokio::test]
async fn check_ec_recover() {
    utils::test_ecr(VALID_SPENDER_PK).await;
}