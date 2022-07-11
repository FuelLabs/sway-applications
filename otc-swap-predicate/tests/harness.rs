mod utils;

use fuels::tx::{Address, AssetId};

// These constants should match those hard-coded in the predicate
const ASK_AMOUNT: u64 = 42;
const ASK_TOKEN: AssetId = AssetId::new([0u8; 32]);
const RECEIVER_ADDRESS: Address = Address::new([3u8; 32]);

#[tokio::test]
async fn valid_predicate_spend() {
    utils::test_predicate_with_parameters(ASK_AMOUNT, ASK_TOKEN, RECEIVER_ADDRESS).await;
}

#[tokio::test]
#[should_panic]
async fn incorrect_ask_amount() {
    utils::test_predicate_with_parameters(41, ASK_TOKEN, RECEIVER_ADDRESS).await;
}

#[tokio::test]
#[should_panic]
async fn incorrect_ask_token() {
    utils::test_predicate_with_parameters(ASK_AMOUNT, AssetId::new([1u8; 32]), RECEIVER_ADDRESS)
        .await;
}

#[tokio::test]
#[should_panic]
async fn incorrect_receiver_address() {
    utils::test_predicate_with_parameters(ASK_AMOUNT, ASK_TOKEN, Address::new([2u8; 32])).await;
}
