mod utils;

use fuels::prelude::*;
use utils::{
    balance_of, earned, exit, get_balance, get_reward, get_reward_for_duration,
    last_time_reward_applicable, last_update_time, notify_reward_amount, owner, period_finish,
    recover_tokens, reward_duration, reward_per_token, reward_per_token_paid,
    reward_per_token_stored, reward_rate, rewards, rewards_distribution, rewards_duration,
    rewards_token, set_rewards_duration, setup, stake, staking_token, total_supply, withdraw, ONE,
    RANDOM_ASSET, REWARDS_ASSET, STAKING_ASSET,
};

// Until timestamp supported in Sways of each action must be specified. Contract is deployed at t=0
const INITIAL_STAKE: u64 = 10 * ONE;
const INITIAL_TIMESTAMP: u64 = 0;

const TIMESTAMP: u64 = 123;

#[tokio::test]
async fn constructed() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;
    let wallet_identity = Identity::Address(Address::from(wallet.address()));

    let owner_identity = owner(&staking_contract).await;
    assert_eq!(wallet_identity, owner_identity.0.value);
}

#[tokio::test]
async fn stake_tokens() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    // User balance has updated
    let wallet_identity = Identity::Address(Address::from(wallet.address()));
    let user_balance = balance_of(&staking_contract, &wallet_identity).await;
    assert_eq!(user_balance.0.value, INITIAL_STAKE);

    // Total_supply has updated
    let total_supply = total_supply(&staking_contract).await;
    assert_eq!(total_supply.0.value, INITIAL_STAKE);
}

#[tokio::test]
async fn can_get_balance_of() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    // User balance has updated
    let wallet_identity = Identity::Address(Address::from(wallet.address()));
    let user_balance = balance_of(&staking_contract, &wallet_identity).await;
    assert_eq!(user_balance.0.value, INITIAL_STAKE);

    // User balance updates again
    stake(&staking_contract, 50000).await;
    let user_balance = balance_of(&staking_contract, &wallet_identity).await;
    assert_eq!(user_balance.0.value, INITIAL_STAKE + 50000);
}

#[tokio::test]
async fn calculate_earned_tokens() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    // Total accrued per token is time_elapsed * rate / total_supply
    let expected_reward_per_token: u64 =
        ((TIMESTAMP - INITIAL_TIMESTAMP) * 42 * ONE) / INITIAL_STAKE;
    let reward_per_token = reward_per_token(&staking_contract).await;

    assert_eq!(reward_per_token.0.value, expected_reward_per_token);

    let wallet_identity = Identity::Address(Address::from(wallet.address()));
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

    let earned = earned(&staking_contract, wallet_identity).await;
    assert_eq!(earned.0.value, expected_reward);
}

#[tokio::test]
async fn claim_reward() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    let balance_before = get_balance(&wallet, REWARDS_ASSET).await;

    let expected_reward_per_token: u64 =
        ((TIMESTAMP - INITIAL_TIMESTAMP) * 42 * ONE) / INITIAL_STAKE;
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

    let _receipts = get_reward(&staking_contract).await;

    let balance_after = get_balance(&wallet, REWARDS_ASSET).await;
    assert_eq!(balance_after - balance_before, expected_reward);
}

#[tokio::test]
async fn exit_with_reward() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    let expected_reward_per_token: u64 =
        ((TIMESTAMP - INITIAL_TIMESTAMP) * 42 * ONE) / INITIAL_STAKE;
    let expected_reward = expected_reward_per_token * INITIAL_STAKE / ONE;

    let staking_balance_before = get_balance(&wallet, STAKING_ASSET).await;
    let rewards_balance_before = get_balance(&wallet, REWARDS_ASSET).await;

    let _receipts = exit(&staking_contract).await;

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

#[tokio::test]
async fn can_get_reward_for_duration() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let reward_rate = reward_rate(&staking_contract).await;
    let reward_duration = reward_duration(&staking_contract).await;

    let expected_reward = reward_rate.0.value * reward_duration.0.value;
    let actual_reward = get_reward_for_duration(&staking_contract).await;

    assert_eq!(expected_reward, actual_reward.0.value);
}

#[tokio::test]
async fn can_get_last_time_reward_applicable() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let period_finish = period_finish(&staking_contract).await;
    let expected = std::cmp::min(TIMESTAMP, period_finish.0.value);
    let actual = last_time_reward_applicable(&staking_contract).await;

    assert_eq!(actual.0.value, expected);
}

#[tokio::test]
async fn can_get_last_updated() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let reciepts = notify_reward_amount(&staking_contract, 5000).await;
    let last_updated = last_update_time(&staking_contract).await;

    assert_eq!(last_updated.0.value, reciepts.1);
}

#[tokio::test]
async fn can_notify_reward_amount() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let rewardbefore = get_reward_for_duration(&staking_contract).await;
    notify_reward_amount(&staking_contract, 5000).await;
    let rewardafter = get_reward_for_duration(&staking_contract).await;

    assert_eq!(rewardbefore.0.value, 42000);
    assert_eq!(rewardafter.0.value, 41000);
}

#[tokio::test]
async fn can_get_owner() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    let actualowner = owner(&staking_contract).await;
    let expectedowner = Identity::Address(Address::from(wallet.address()));

    assert_eq!(actualowner.0.value, expectedowner);
}

#[tokio::test]
async fn can_get_period_finish() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let period_finish = period_finish(&staking_contract).await;

    assert_eq!(period_finish.0.value, 1000);
}

#[tokio::test]
async fn can_recover_tokens() {
    let (staking_contract, id, wallet, wallet2, _inittimestamp) = setup().await;

    let _receipt = wallet2
        .force_transfer_to_contract(&id, 50000, RANDOM_ASSET, TxParameters::default())
        .await
        .unwrap();

    let owner_balance_before = get_balance(&wallet, RANDOM_ASSET).await;

    recover_tokens(&staking_contract, ContractId::new([3u8; 32]), 50000).await;

    let owner_balance_after = get_balance(&wallet, RANDOM_ASSET).await;

    assert_eq!(owner_balance_before + 50000, owner_balance_after);
}

#[tokio::test]
async fn can_get_reward_per_token_stored() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let reward = reward_per_token_stored(&staking_contract).await;

    assert_eq!(reward.0.value, 0);

    notify_reward_amount(&staking_contract, 5000).await;

    let reward = reward_per_token_stored(&staking_contract).await;

    assert_eq!(reward.0.value, 516);
}

#[tokio::test]
async fn can_get_reward_per_token_paid() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;
    let wallet_identity = Identity::Address(Address::from(wallet.address()));

    let reward = reward_per_token_paid(&staking_contract, wallet_identity.clone()).await;

    assert_eq!(reward.0.value, 0);

    notify_reward_amount(&staking_contract, 5000).await;

    let reward = reward_per_token_paid(&staking_contract, wallet_identity.clone()).await;

    assert_eq!(reward.0.value, 516);
}

#[tokio::test]
async fn can_get_reward() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;
    let wallet_identity = Identity::Address(Address::from(wallet.address()));

    let reward = rewards(&staking_contract, wallet_identity.clone()).await;

    assert_eq!(reward.0.value, 0);

    notify_reward_amount(&staking_contract, 5000).await;

    let reward = rewards(&staking_contract, wallet_identity.clone()).await;

    assert_eq!(reward.0.value, 5160);
}

#[tokio::test]
async fn can_get_rewards_distribution() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;
    let wallet_identity = Identity::Address(Address::from(wallet.address()));

    let rewards_distribution = rewards_distribution(&staking_contract).await;

    assert_eq!(wallet_identity, rewards_distribution.0.value);
}

#[tokio::test]
async fn can_get_rewards_duration() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let rewards_dur = rewards_duration(&staking_contract).await;

    assert_eq!(1000, rewards_dur.0.value);
}

#[tokio::test]
async fn can_get_rewards_token() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let token = rewards_token(&staking_contract).await;

    assert_eq!(ContractId::new([2_u8; 32]), token.0.value);
}

#[tokio::test]
async fn can_set_rewards_duration() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let old_rewards_dur = rewards_duration(&staking_contract).await;

    assert_eq!(old_rewards_dur.0.value, 1000);

    set_rewards_duration(&staking_contract, 2000).await;

    let new_rewards_dur = rewards_duration(&staking_contract).await;

    assert_eq!(2000, new_rewards_dur.0.value);
}

#[tokio::test]
async fn can_get_staking_token() {
    let (staking_contract, _id, _wallet, _wallet2, _inittimestamp) = setup().await;

    let token = staking_token(&staking_contract).await;

    assert_eq!(ContractId::new([1_u8; 32]), token.0.value);
}

#[tokio::test]
async fn can_withdraw() {
    let (staking_contract, _id, wallet, _wallet2, _inittimestamp) = setup().await;

    let staking_balance_before = get_balance(&wallet, STAKING_ASSET).await;

    withdraw(&staking_contract, 500).await;

    let staking_balance_after = get_balance(&wallet, STAKING_ASSET).await;

    assert_eq!(staking_balance_before + 500, staking_balance_after);
}
