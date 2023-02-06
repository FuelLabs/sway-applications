use crate::utils::{
    abi::exit, get_balance, setup, INITIAL_STAKE, ONE, REWARDS_ASSET, STAKING_ASSET,
};

#[tokio::test]
async fn exit_with_reward() {
    let (staking_contract, _id, wallet, _wallet2, inittimestamp) = setup().await;

    let staking_balance_before = get_balance(&wallet, STAKING_ASSET).await;
    let rewards_balance_before = get_balance(&wallet, REWARDS_ASSET).await;

    exit(&staking_contract).await;
    let exit_ts = wallet
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

    let staking_balance_after = get_balance(&wallet, STAKING_ASSET).await;
    let rewards_balance_after = get_balance(&wallet, REWARDS_ASSET).await;

    let expected_reward_per_token: u64 = ((exit_ts - inittimestamp) * 42 * ONE) / INITIAL_STAKE;
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

    assert_eq!(
        rewards_balance_after - rewards_balance_before,
        expected_reward
    );
    assert_eq!(
        staking_balance_after - staking_balance_before,
        INITIAL_STAKE
    );
}
