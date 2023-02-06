use crate::utils::{setup, ONE, INITIAL_STAKE, get_balance, REWARDS_ASSET, abi::{get_reward}};

#[tokio::test]
async fn claim_reward() {
    let (staking_contract, _id, wallet, _wallet2, inittimestamp) = setup().await;

    let balance_before = get_balance(&wallet, REWARDS_ASSET).await;

    get_reward(&staking_contract).await;

    // wallet.get_provider().unwrap().chain_info().await.unwrap().latest_block.header.time.unwrap().timestamp() as u64;
    let timestamp = wallet.get_provider().unwrap().chain_info().await.unwrap().latest_block.header.time.unwrap().timestamp() as u64;

    let expected_reward_per_token: u64 =
    ((timestamp - inittimestamp) * 42 * ONE) / INITIAL_STAKE;
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

    let balance_after = get_balance(&wallet, REWARDS_ASSET).await;
    assert_eq!(balance_after - balance_before, expected_reward);
}