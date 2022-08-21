mod utils;

use fuels::prelude::*;
use utils::{
    get_balance, reward_per_token, setup, stakingrewards_mod::Identity, ONE, REWARDS_ASSET,
    STAKING_ASSET, balance_of, total_supply, earned, get_reward, exit,
};

// Until timestamp supported in Sway, timestamps of each action must be specified. Contract is deployed at t=0
const INITIAL_STAKE: u64 = 10 * ONE;
const INITIAL_TIMESTAMP: u64 = 0;

const TIMESTAMP: u64 = 123;

#[tokio::test]
async fn constructed() {
    let (staking_contract, _id, wallet) = setup(INITIAL_STAKE, INITIAL_TIMESTAMP).await;
    let wallet_identity = Identity::Address(Address::from(wallet.address()));

    let owner_identity = staking_contract.owner().call().await.unwrap().value;
    assert_eq!(wallet_identity, owner_identity);
}

#[tokio::test]
async fn stake_tokens() {
    let (staking_contract, _id, wallet) = setup(INITIAL_STAKE, INITIAL_TIMESTAMP).await;

    // User balance has updated
    let wallet_identity = Identity::Address(Address::from(wallet.address()));
    let user_balance = balance_of(&staking_contract, wallet_identity).await;
    assert_eq!(user_balance, INITIAL_STAKE);

    // Total_supply has updated
    let total_supply = total_supply(&staking_contract).await;
    assert_eq!(total_supply, INITIAL_STAKE);
}

#[tokio::test]
async fn calculate_earned_tokens() {
    let (staking_contract, _id, wallet) = setup(INITIAL_STAKE, INITIAL_TIMESTAMP).await;

    // Total accrued per token is time_elapsed * rate / total_supply
    let expected_reward_per_token: u64 =
        ((TIMESTAMP - INITIAL_TIMESTAMP) * 42 * ONE) / INITIAL_STAKE;
    let reward_per_token = reward_per_token(&staking_contract, TIMESTAMP).await;

    assert_eq!(reward_per_token, expected_reward_per_token);

    let wallet_identity = Identity::Address(Address::from(wallet.address()));
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

    let earned = earned(&staking_contract, wallet_identity, TIMESTAMP).await;
    assert_eq!(earned, expected_reward);
}

#[tokio::test]
async fn claim_reward() {
    let (staking_contract, _id, wallet) = setup(INITIAL_STAKE, INITIAL_TIMESTAMP).await;

    let balance_before = get_balance(&wallet, REWARDS_ASSET).await;

    let expected_reward_per_token: u64 =
        ((TIMESTAMP - INITIAL_TIMESTAMP) * 42 * ONE) / INITIAL_STAKE;
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

    let _receipts = get_reward(&staking_contract, TIMESTAMP).await;

    let balance_after = get_balance(&wallet, REWARDS_ASSET).await;
    assert_eq!(balance_after - balance_before, expected_reward);
}

#[tokio::test]
async fn exit_with_reward() {
    let (staking_contract, _id, wallet) = setup(INITIAL_STAKE, INITIAL_TIMESTAMP).await;

    let expected_reward_per_token: u64 =
        ((TIMESTAMP - INITIAL_TIMESTAMP) * 42 * ONE) / INITIAL_STAKE;
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

    let staking_balance_before = get_balance(&wallet, STAKING_ASSET).await;
    let rewards_balance_before = get_balance(&wallet, REWARDS_ASSET).await;

    let _receipts = exit(&staking_contract, TIMESTAMP).await;

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
