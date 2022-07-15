use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(StakingRewards, "out/debug/staking-rewards-abi.json");

const ONE: u64 = 1_000_000_000;
const BASE_ASSET: AssetId = AssetId::new([0u8; 32]);

async fn get_balance(provider: &Provider, address: Address, asset: AssetId) -> u64 {
    let balance = provider.get_asset_balance(&address, asset).await.unwrap();
    balance
}

async fn setup() -> (StakingRewards, ContractId, LocalWallet) {
    // Launch a local network and deploy the contract

    let config = WalletsConfig::new_single(Some(1), Some(10000 * ONE));
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

    // Seed the contract with some reward tokens
    let seed_amount = 1000 * ONE;
    let _receipt = wallet
        .transfer(
            &Address::new(*id),
            seed_amount,
            BASE_ASSET,
            TxParameters::default(),
        )
        .await
        .unwrap();

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

    timestamp = 123;

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
async fn can_claim_reward() {
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
