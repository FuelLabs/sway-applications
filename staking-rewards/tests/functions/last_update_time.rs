
// #[tokio::test]
// async fn can_get_last_updated() {
//     let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

//     let reciepts = notify_reward_amount(&staking_contract, 5000).await;
//     let last_updated = last_update_time(&staking_contract).await;

use crate::utils::{
    abi::{last_update_time, notify_reward_amount},
    setup,
};

use tai64::Tai64;

#[tokio::test]
async fn can_get_last_updated() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    let _ = notify_reward_amount(&staking_contract, 5000).await;
    let timestamp = wallet
        .get_provider()
        .unwrap()
        .chain_info()
        .await
        .unwrap()
        .latest_block
        .header
        .time
        .unwrap()
        .timestamp() as u64;

    let last_updated = last_update_time(&staking_contract).await;

    let actual = Tai64::from_slice(&last_updated.to_be_bytes()).unwrap();
    let ts = actual.to_unix() as u64;

    assert_eq!(timestamp, ts);
}