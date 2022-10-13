pub mod abi;

use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(NameRegistry, "out/debug/name-registry-abi.json");

pub async fn get_contract_instance() -> (NameRegistry, ContractId, WalletUnlocked, WalletUnlocked) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(2),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/name-registry.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/name-registry-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = NameRegistry::new(id.to_string(), wallet.clone());

    (instance, id.into(), wallet, wallet2)
}
