use fuels::prelude::*;

// Load abi from json
abigen!(
    Timelock,
    "./project/timelock-contract/out/debug/timelock-contract-abi.json"
);

pub async fn setup() -> (Timelock, WalletUnlocked, WalletUnlocked) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(2),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;

    let wallet = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/timelock-contract.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/timelock-contract-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = Timelock::new(id.clone(), wallet.clone());

    (instance, wallet, wallet2)
}
