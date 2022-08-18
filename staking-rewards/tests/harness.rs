mod utils;

use fuels::prelude::*;
use utils::{get_balance, setup, stakingrewards_mod, ONE, REWARDS_ASSET, STAKING_ASSET};

// Until timestamp supported in Sway, timestamps of each action must be specified. Contract is deployed at t=0
const INITIAL_STAKE: u64 = 10 * ONE;
const INITIAL_TIMESTAMP: u64 = 0;

#[tokio::test]
async fn stake_tokens() {
    let (staking_contract, _id, wallet) = setup(INITIAL_STAKE, INITIAL_TIMESTAMP).await;

    // User balance has updated
    let wallet_identity = stakingrewards_mod::Identity::Address(Address::from(wallet.address()));
    let user_balance = staking_contract
        .balance_of(wallet_identity)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(user_balance, INITIAL_STAKE);

    // Total_supply has updated
    let total_supply = staking_contract.total_supply().call().await.unwrap().value;
    assert_eq!(total_supply, INITIAL_STAKE);
}

#[tokio::test]
async fn calculate_earned_tokens() {
    let (staking_contract, _id, wallet) = setup(INITIAL_STAKE, INITIAL_TIMESTAMP).await;

    let timestamp = 123;

    // Total accrued per token is time_elapsed * rate / total_supply
    let expected_reward_per_token: u64 =
        ((timestamp - INITIAL_TIMESTAMP) * 42 * ONE) / INITIAL_STAKE;
    let reward_per_token = staking_contract
        .reward_per_token(timestamp)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(reward_per_token, expected_reward_per_token);

    let wallet_identity = stakingrewards_mod::Identity::Address(Address::from(wallet.address()));
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

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
    let (staking_contract, _id, wallet) = setup(INITIAL_STAKE, INITIAL_TIMESTAMP).await;

    let balance_before = get_balance(&wallet, REWARDS_ASSET).await;
    let timestamp = 123;

    let expected_reward_per_token: u64 =
        ((timestamp - INITIAL_TIMESTAMP) * 42 * ONE) / INITIAL_STAKE;
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

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
    let (staking_contract, _id, wallet) = setup(INITIAL_STAKE, INITIAL_TIMESTAMP).await;

    let timestamp = 123;

    let expected_reward_per_token: u64 =
        ((timestamp - INITIAL_TIMESTAMP) * 42 * ONE) / INITIAL_STAKE;
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

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
        INITIAL_STAKE
    );
}
