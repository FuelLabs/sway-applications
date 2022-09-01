use fuels::prelude::*;

// Load abi from json
abigen!(StakingRewards, "out/debug/staking-rewards-abi.json");

pub const PRECISION: u32 = 9; // Should match precision in staking contract
pub const ONE: u64 = 10_i32.pow(PRECISION) as u64;
pub const BASE_ASSET: AssetId = AssetId::new([0u8; 32]);
pub const STAKING_ASSET: AssetId = AssetId::new([1u8; 32]);
pub const REWARDS_ASSET: AssetId = AssetId::new([2u8; 32]);

const INITIAL_STAKE: u64 = 10 * ONE;
const INITIAL_TIMESTAMP: u64 = 0;

pub async fn get_balance(wallet: &LocalWallet, asset: AssetId) -> u64 {
    let provider = wallet.get_provider().unwrap();
    let balance = provider
        .get_asset_balance(&wallet.address(), asset)
        .await
        .unwrap();
    balance
}

pub async fn setup() -> (StakingRewards, Bech32ContractId, LocalWallet) {
    // Configure wallet with assets
    let assets = [BASE_ASSET, STAKING_ASSET, REWARDS_ASSET];
    let wallet_config = WalletsConfig::new_multiple_assets(
        1,
        assets
            .map(|asset| AssetConfig {
                id: asset,
                num_coins: 1,
                coin_amount: 1_000_000_000 * ONE,
            })
            .iter()
            .cloned()
            .collect::<Vec<_>>(),
    );

    let wallet = &launch_custom_provider_and_get_wallets(wallet_config, None).await[0];

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

    let staking_contract = StakingRewardsBuilder::new(id.to_string(), wallet.clone()).build();
    let walletidentity = Identity::Address(Address::from(wallet.address()));

    staking_contract
        .constructor(walletidentity)
        .call()
        .await
        .unwrap();

    // Seed the contract with some reward tokens
    let seed_amount = 100_000 * ONE;
    let _receipt = wallet
        .force_transfer_to_contract(&id, seed_amount, REWARDS_ASSET, TxParameters::default())
        .await
        .unwrap();

    // Stake some tokens from the wallet
    let staking_call_params = CallParameters::new(Some(INITIAL_STAKE), Some(STAKING_ASSET), None);
    let _receipts = staking_contract
        .stake(INITIAL_TIMESTAMP)
        .call_params(staking_call_params)
        .call()
        .await
        .unwrap();

    (staking_contract, id, wallet.clone())
}

pub async fn balance_of(instance: &StakingRewards, id: &Identity) -> u64 {
    instance
        .balance_of(id.to_owned())
        .call()
        .await
        .unwrap()
        .value
}

pub async fn earned(instance: &StakingRewards, wallet_identity: Identity, timestamp: u64) -> u64 {
    instance
        .earned(wallet_identity, timestamp)
        .call()
        .await
        .unwrap()
        .value
}

pub async fn total_supply(instance: &StakingRewards) -> u64 {
    instance.total_supply().call().await.unwrap().value
}

pub async fn reward_per_token(instance: &StakingRewards, timestamp: u64) -> u64 {
    instance
        .reward_per_token(timestamp)
        .call()
        .await
        .unwrap()
        .value
}

pub async fn get_reward(instance: &StakingRewards, timestamp: u64) {
    instance
        .get_reward(timestamp)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
}

pub async fn exit(instance: &StakingRewards, timestamp: u64) {
    instance
        .exit(timestamp)
        .append_variable_outputs(2)
        .call()
        .await
        .unwrap();
}

pub async fn stake(instance: &StakingRewards, timestamp: u64, amount: u64) {
    instance
        .stake(timestamp)
        .call_params(CallParameters {
            amount,
            asset_id: STAKING_ASSET,
            gas_forwarded: 1000000,
        })
        .call()
        .await
        .unwrap();
}

pub async fn reward_rate(instance: &StakingRewards) -> u64 {
    instance.reward_rate().call().await.unwrap().value
}

pub async fn reward_duration(instance: &StakingRewards) -> u64 {
    instance.rewards_duration().call().await.unwrap().value
}

pub async fn get_reward_for_duration(instance: &StakingRewards) -> u64 {
    instance
        .get_reward_for_duration()
        .call()
        .await
        .unwrap()
        .value
}

pub async fn period_finish(instance: &StakingRewards) -> u64 {
    instance.period_finish().call().await.unwrap().value
}

pub async fn last_time_reward_applicable(instance: &StakingRewards, timestamp: u64) -> u64 {
    instance
        .last_time_reward_applicable(timestamp)
        .call()
        .await
        .unwrap()
        .value
}
