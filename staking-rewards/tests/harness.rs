mod utils;

use fuels::prelude::*;
use utils::{get_balance, setup, stake, stakingrewards_mod, BASE_ASSET, ONE};

// For testing, staking and reward token will both be native asset
// Timestamps of each action must be specified. Contract is deployed at t=0

#[tokio::test]
async fn stake_tokens() {
    let (staking_contract, _id, wallet) = setup().await;

    let amount_to_stake = 10 * ONE;
    stake(&staking_contract, amount_to_stake, 0).await;

    // Check user balance has updated
    let wallet_identity = stakingrewards_mod::Identity::Address(wallet.address());
    let user_balance = staking_contract
        .balance_of(wallet_identity)
        .call()
        .await
        .unwrap()
        .value;

    // Check total_supply has updated
    let total_supply = staking_contract.total_supply().call().await.unwrap().value;
    assert_eq!(total_supply, amount_to_stake);
    assert_eq!(user_balance, amount_to_stake)
}

#[tokio::test]
async fn calculate_earned_tokens() {
    let (staking_contract, _id, wallet) = setup().await;
    let amount_to_stake = 10 * ONE;
    stake(&staking_contract, amount_to_stake, 0).await;

    let timestamp = 123;

    // Total accrued per token is time_elapsed * rate / total_supply
    // So expect (123 * 2 // 10) = 24 reward per token
    let reward_per_token = staking_contract
        .reward_per_token(timestamp)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(reward_per_token, 24);

    // Our wallet staked 10 tokens, so expect 24 * 10 = 240 tokens earned
    let wallet_identity = stakingrewards_mod::Identity::Address(wallet.address());
    let earned = staking_contract
        .earned(wallet_identity, timestamp)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(earned, 240);
}

#[tokio::test]
async fn claim_reward() {
    let (staking_contract, _id, wallet) = setup().await;

    let amount_to_stake = 10 * ONE;
    let mut timestamp = 0;
    let staking_call_params = CallParameters::new(Some(amount_to_stake), None, None);
    let _receipts = staking_contract
        .stake(timestamp)
        .call_params(staking_call_params)
        .call()
        .await
        .unwrap();

    timestamp = 123;

    let provider = wallet.get_provider().unwrap();
    let balance_before = get_balance(&provider, wallet.address(), BASE_ASSET).await;

    let _receipts = staking_contract
        .get_reward(timestamp)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let balance_after = get_balance(&provider, wallet.address(), BASE_ASSET).await;
    assert_eq!(balance_after - balance_before, 240);
}
