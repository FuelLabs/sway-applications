use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(MyContract, "out/debug/script_vault-abi.json");

async fn get_contract_instance() -> (MyContract, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new_single(Some(1), Some(1000000)),
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/script_vault.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/script_vault-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MyContract::new(id.to_string(), wallet);

    (instance, id)
}

#[tokio::test]
async fn can_get_contract_id() {
    let (_instance, _id) = get_contract_instance().await;

    enum PoolSpecialization {
        GENERAL: (),
        MINIMAL_SWAP_INFO: (),
        TWO_TOKEN: (),
    }

    // Now you have an instance of your contract you can use to test each function
    let result = contract_instance
        .register_pool(0x34e50d5b80a0391081fa756ba16c499d6e3ca990f0e3a62f8417e54092d24934, PoolSpecialization::TWO_TOKEN)
        .call()
        .await
        .unwrap();

    assert_eq!(0x34e50d5b80a0391081fa756ba16c499d6e3ca990f0e3a62f8417e54092d24934, result.value);
}
