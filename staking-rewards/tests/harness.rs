use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(StakingRewards, "out/debug/staking-rewards-abi.json");

async fn get_contract_instance() -> (StakingRewards, ContractId) {
    // Launch a local network and deploy the contract
    let wallet = launch_provider_and_get_wallet().await;

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

    let instance = StakingRewards::new(id.to_string(), wallet);

    (instance, id)
}

#[tokio::test]
async fn can_get_contract_id() {
    let (_instance, _id) = get_contract_instance().await;

    // Now you have an instance of your contract you can use to test each function
}
