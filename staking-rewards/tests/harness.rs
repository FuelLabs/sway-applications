use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(StakingRewards, "out/debug/staking-rewards-abi.json");

const ONE: u64 = 1_000_000_000;

async fn setup() -> (StakingRewards, ContractId, LocalWallet) {
    // Launch a local network and deploy the contract

    let config = WalletsConfig::new_single(Some(1), Some(1000 * ONE));
    let wallet = &launch_custom_provider_and_get_wallets(config, None).await[0];

    let id = Contract::deploy(
        "./out/debug/staking-rewards.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/staking-rewards-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = StakingRewards::new(id.to_string(), wallet.clone());

    (instance, id, wallet.clone())
}

// For testing, staking and reward token will both be native asset
// Timestamps of each action must be specified. Contract is deployed at t=0

#[tokio::test]
async fn can_stake() {
    let (staking_contract, _id, wallet) = setup().await;

    let amount_to_stake = 10 * ONE;
    let timestamp = 0;
    let staking_call_params = CallParameters::new(Some(amount_to_stake), None, None);
    let _receipts = staking_contract
        .stake(timestamp)
        .call_params(staking_call_params)
        .call()
        .await
        .unwrap();

    // Check total_supply has updated
    let total_supply = staking_contract.total_supply().call().await.unwrap().value;
    assert_eq!(total_supply, amount_to_stake);

    // Check user balance has updated
    let wallet_identity = stakingrewards_mod::Identity::Address(wallet.address());
    let user_balance = staking_contract
        .balance_of(wallet_identity)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(user_balance, amount_to_stake)
}

#[tokio::test]
async fn can_earn() {
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

    let wallet_identity = stakingrewards_mod::Identity::Address(wallet.address());
    timestamp = 123;

    // Total accrued pe token is time_elapsed * rate / total_supply
    // So expect (123 * 2 // 10) = 24 reward per token
    let reward_per_token = staking_contract
        .reward_per_token(timestamp)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(reward_per_token, 24);

    // Our wallet staked 10 tokens, so expect 24 * 10 = 240 tokens earned
    let earned = staking_contract
        .earned(wallet_identity, timestamp)
        .call()
        .await
        .unwrap()
        .value;
    assert_eq!(earned, 240);
}
