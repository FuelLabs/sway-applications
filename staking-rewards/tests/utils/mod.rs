use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(StakingRewards, "out/debug/staking-rewards-abi.json");

pub const ONE: u64 = 1_000_000_000;
pub const BASE_ASSET: AssetId = AssetId::new([0u8; 32]);

pub async fn get_balance(provider: &Provider, address: Address, asset: AssetId) -> u64 {
    let balance = provider.get_asset_balance(&address, asset).await.unwrap();
    balance
}

pub async fn setup() -> (StakingRewards, ContractId, LocalWallet) {
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

pub async fn stake(staking_contract: &StakingRewards, amount_to_stake: u64, timestamp: u64) {
    let staking_call_params = CallParameters::new(Some(amount_to_stake), None, None);
    let _receipts = staking_contract
        .stake(timestamp)
        .call_params(staking_call_params)
        .call()
        .await
        .unwrap();
}
