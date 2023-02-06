
use crate::utils::{
    abi::{last_time_reward_applicable, period_finish},
    setup,
};

#[tokio::test]
async fn can_get_last_time_reward_applicable() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    let period_finish = period_finish(&staking_contract).await;
    let last_called_ts = wallet
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
    let expected = std::cmp::min(last_called_ts, period_finish);
    let actual = last_time_reward_applicable(&staking_contract).await;

    assert_eq!(actual, expected);
}