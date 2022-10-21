mod utils;

// const VALID_SPENDER_PK: &str = "";
// const INVALID_SPENDER_PK: &str = "";

#[tokio::test]
async fn valid_spender() {
    utils::test_predicate_spend_with_parameters().await;
}