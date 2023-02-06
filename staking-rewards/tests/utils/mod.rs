pub mod abi;

use core::fmt::Debug;
use fuels::prelude::*;
use fuels::core::traits::Tokenizable;
use fuels::types::Identity;
use fuels::{
    client::types::TransactionStatus,
    programs::{call_response::FuelCallResponse, contract::ContractCallHandler},
};

// Load abi from json
// abigen!(StakingRewards, "out/debug/staking-rewards-abi.json");
abigen!(Contract(
    name = "StakingRewards",
    abi = "out/debug/staking-rewards-abi.json"
));

pub const PRECISION: u32 = 9; // Should match precision in staking contract
pub const ONE: u64 = 10_i32.pow(PRECISION) as u64;
pub const BASE_ASSET: AssetId = AssetId::new([0u8; 32]);
pub const STAKING_ASSET: AssetId = AssetId::new([1u8; 32]);
pub const REWARDS_ASSET: AssetId = AssetId::new([2u8; 32]);
pub const RANDOM_ASSET: AssetId = AssetId::new([3u8; 32]);

pub const INITIAL_STAKE: u64 = 10 * ONE;

pub async fn get_balance(wallet: &Wallet, asset: AssetId) -> u64 {
    let provider = wallet.get_provider().unwrap();
    let balance = provider
        .get_asset_balance(&wallet.address(), asset)
        .await
        .unwrap();
    balance
}

pub async fn setup() -> (
    StakingRewards,
    Bech32ContractId,
    WalletUnlocked,
    WalletUnlocked,
    u64,
) {
    // Configure wallet with assets
    let assets = [BASE_ASSET, STAKING_ASSET, REWARDS_ASSET, RANDOM_ASSET];
    let wallet_config = WalletsConfig::new_multiple_assets(
        2,
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

    let wallets = &launch_custom_provider_and_get_wallets(wallet_config, None, None).await;
    let wallet = &wallets[0];
    let wallet2 = &wallets[1];

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

    let staking_contract = StakingRewards::new(id.clone().into(), wallet.clone());

    // Seed the contract with some reward tokens
    let seed_amount = 100_000_000 * ONE;
    let _receipt = wallet
        .force_transfer_to_contract(&id, seed_amount, REWARDS_ASSET, TxParameters::default())
        .await
        .unwrap();

    // Stake some tokens from the wallet
    let staking_call_params = CallParameters::new(Some(INITIAL_STAKE), Some(STAKING_ASSET), None);
    staking_contract.methods().stake().call_params(staking_call_params).call().await.unwrap();
    let init_timestamp = wallet.get_provider().unwrap().chain_info().await.unwrap().latest_block.header.time.unwrap().timestamp() as u64;

    (
        staking_contract,
        id,
        wallet.clone(),
        wallet2.clone(),
        init_timestamp,
    )
}

// async fn get_timestamp_and_call<T>(handler: ContractCallHandler<T>) -> (FuelCallResponse<T>, u64)
// where
//     T: Tokenizable + Debug,
// {
//     let script = handler.get_call_execution_script().await.unwrap();
//     let tx_id = script.tx.id().to_string();
//     let provider = handler.provider.clone();
//     let call_response = handler.call().await.unwrap();
//     let tx_status = provider.get_transaction_by_id(&tx_id).await.unwrap().status;

//     let time = match tx_status {
//         TransactionStatus::Success { time, .. } => time,
//         _ => panic!("tx failed"),
//     };
//     let time = time.timestamp() as u64;

//     (call_response, time)
// }










// pub async fn reward_rate(instance: &StakingRewards) -> (FuelCallResponse<u64>, u64) {
//     get_timestamp_and_call(instance.methods().reward_rate()).await
// }

// pub async fn reward_duration(instance: &StakingRewards) -> (FuelCallResponse<u64>, u64) {
//     get_timestamp_and_call(instance.methods().rewards_duration()).await
// }

// pub async fn get_reward_for_duration(instance: &StakingRewards) -> (FuelCallResponse<u64>, u64) {
//     get_timestamp_and_call(instance.methods().get_reward_for_duration()).await
// }

// pub async fn period_finish(instance: &StakingRewards) -> (FuelCallResponse<u64>, u64) {
//     get_timestamp_and_call(instance.methods().period_finish()).await
// }

// pub async fn last_time_reward_applicable(instance: &StakingRewards) -> (FuelCallResponse<u64>, u64) {
//     get_timestamp_and_call(instance.methods().last_time_reward_applicable()).await
// }

// pub async fn last_update_time(instance: &StakingRewards) -> (FuelCallResponse<u64>, u64) {
//     get_timestamp_and_call(instance.methods().last_update_time()).await
// }



// pub async fn owner(instance: &StakingRewards) -> (FuelCallResponse<Identity>, u64) {
//     get_timestamp_and_call(instance.methods().owner()).await
// }

// pub async fn recover_tokens(
//     instance: &StakingRewards,
//     asset_id: ContractId,
//     amount: u64,
// ) -> (FuelCallResponse<()>, u64) {
//     get_timestamp_and_call(
//         instance
//             .methods()
//             .recover_tokens(asset_id, amount)
//             .append_variable_outputs(1),
//     )
//     .await
// }



// pub async fn reward_per_token_paid(
//     instance: &StakingRewards,
//     account: Identity,
// ) -> (FuelCallResponse<u64>, u64) {
//     get_timestamp_and_call(instance.methods().reward_per_token_paid(account)).await
// }

// pub async fn rewards(instance: &StakingRewards, account: Identity) -> (FuelCallResponse<u64>, u64) {
//     get_timestamp_and_call(instance.methods().rewards(account)).await
// }

// pub async fn rewards_distribution(instance: &StakingRewards) -> (FuelCallResponse<Identity>, u64) {
//     get_timestamp_and_call(instance.methods().rewards_distribution()).await
// }

// pub async fn rewards_duration(instance: &StakingRewards) -> (FuelCallResponse<u64>, u64) {
//     get_timestamp_and_call(instance.methods().rewards_duration()).await
// }

// pub async fn rewards_token(instance: &StakingRewards) -> (FuelCallResponse<ContractId>, u64) {
//     get_timestamp_and_call(instance.methods().rewards_token()).await
// }

// pub async fn set_rewards_duration(
//     instance: &StakingRewards,
//     rewards_duration: u64,
// ) -> (FuelCallResponse<()>, u64) {
//     get_timestamp_and_call(instance.methods().set_rewards_duration(rewards_duration)).await
// }

// pub async fn staking_token(instance: &StakingRewards) -> (FuelCallResponse<ContractId>, u64) {
//     get_timestamp_and_call(instance.methods().staking_token()).await
// }

// pub async fn withdraw(instance: &StakingRewards, amount: u64) -> (FuelCallResponse<()>, u64) {
//     get_timestamp_and_call(
//         instance
//             .methods()
//             .withdraw(amount)
//             .append_variable_outputs(1),
//     )
//     .await
// }
