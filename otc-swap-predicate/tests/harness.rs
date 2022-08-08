mod utils;

use fuels::prelude::*;

// These constants should match those hard-coded in the predicate
const ASK_AMOUNT: u64 = 42;
const ASK_TOKEN: AssetId = AssetId::new([1u8; 32]);
const RECEIVER: &str = "fuel1p8qt95dysmzrn2rmewntg6n6rg3l8ztueqafg5s6jmd9cgautrdslwdqdw";

#[tokio::test]
async fn valid_predicate_spend_with_swap() {
    utils::test_predicate_spend_with_parameters(ASK_AMOUNT, ASK_TOKEN, RECEIVER).await;
}

#[tokio::test]
#[should_panic]
async fn incorrect_ask_amount() {
    utils::test_predicate_spend_with_parameters(41, ASK_TOKEN, RECEIVER).await;
}

#[tokio::test]
#[should_panic]
async fn incorrect_ask_token() {
    utils::test_predicate_spend_with_parameters(ASK_AMOUNT, AssetId::new([42u8; 32]), RECEIVER)
        .await;
}

#[tokio::test]
#[should_panic]
async fn incorrect_receiver_address() {
    utils::test_predicate_spend_with_parameters(
        ASK_AMOUNT,
        ASK_TOKEN,
        "fuel1p8qt95dysmzrn2rmewntg6n6rg3l8ztueqafg5s6jmd9cgautrdslwdqdq", // Note incorrect end
    )
    .await;
}
