mod utils;

use fuels::prelude::*;
use utils::{get_balance, setup, stakingrewards_mod, ONE, REWARDS_ASSET, STAKING_ASSET};

// For testing, staking and reward token will both be native asset
// Timestamps of each action must be specified. Contract is deployed at t=0

#[tokio::test]
async fn stake_tokens() {
    let initial_stake = 10 * ONE;
    let initial_timestamp = 0;
    let (staking_contract, _id, wallet) = setup(initial_stake, initial_timestamp).await;

    // User balance has updated
    let wallet_identity = stakingrewards_mod::Identity::Address(Address::from(wallet.address()));
    let user_balance = staking_contract
        .balance_of(wallet_identity)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(user_balance, initial_stake);

    // Total_supply has updated
    let total_supply = staking_contract.total_supply().call().await.unwrap().value;
    assert_eq!(total_supply, initial_stake);
}

#[tokio::test]
async fn calculate_earned_tokens() {
    let initial_stake = 10 * ONE;
    let initial_timestamp = 0;
    let (staking_contract, _id, wallet) = setup(initial_stake, initial_timestamp).await;

    let timestamp = 123;

    // Total accrued per token is time_elapsed * rate / total_supply
    let expected_reward_per_token: u64 =
        ((timestamp - initial_timestamp) * 42 * ONE) / initial_stake;
    let reward_per_token = staking_contract
        .reward_per_token(timestamp)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(reward_per_token, expected_reward_per_token);

    let wallet_identity = stakingrewards_mod::Identity::Address(Address::from(wallet.address()));
    let expected_reward = expected_reward_per_token * initial_stake / ONE;

    let earned = staking_contract
        .earned(wallet_identity, timestamp)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(earned, expected_reward);
}

#[tokio::test]
async fn claim_reward() {
    let initial_stake = 10 * ONE;
    let initial_timestamp = 0;
    let (staking_contract, _id, wallet) = setup(initial_stake, initial_timestamp).await;

    let balance_before = get_balance(&wallet, REWARDS_ASSET).await;
    let timestamp = 123;

    let expected_reward_per_token: u64 =
        ((timestamp - initial_timestamp) * 42 * ONE) / initial_stake;
    let expected_reward = expected_reward_per_token * initial_stake / ONE;

    let _receipts = staking_contract
        .get_reward(timestamp)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let balance_after = get_balance(&wallet, REWARDS_ASSET).await;
    assert_eq!(balance_after - balance_before, expected_reward);
}

#[tokio::test]
async fn exit_with_reward() {
    let initial_stake = 10 * ONE;
    let initial_timestamp = 0;
    let (staking_contract, _id, wallet) = setup(initial_stake, initial_timestamp).await;

    let timestamp = 123;

    let expected_reward_per_token: u64 =
        ((timestamp - initial_timestamp) * 42 * ONE) / initial_stake;
    let expected_reward = expected_reward_per_token * initial_stake / ONE;

    let staking_balance_before = get_balance(&wallet, STAKING_ASSET).await;
    let rewards_balance_before = get_balance(&wallet, REWARDS_ASSET).await;

    let _receipts = staking_contract
        .exit(timestamp)
        .append_variable_outputs(2)
        .call()
        .await
        .unwrap();

    let staking_balance_after = get_balance(&wallet, STAKING_ASSET).await;
    let rewards_balance_after = get_balance(&wallet, REWARDS_ASSET).await;

    assert_eq!(
        rewards_balance_after - rewards_balance_before,
        expected_reward
    );
    assert_eq!(
        staking_balance_after - staking_balance_before,
        initial_stake
    );
}
