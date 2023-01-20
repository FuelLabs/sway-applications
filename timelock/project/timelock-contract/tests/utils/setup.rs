use fuels::prelude::*;

// TODO: do setup instead of copy/pasted code with minor adjustments

// Load abi from json
abigen!(
    Contract(name="Timelock", abi="./project/timelock-contract/out/debug/timelock-contract-abi.json"),
);

const TIMELOCK_CONTRACT_BINARY_PATH: &str = "./out/debug/timelock-contract.bin";
const TIMELOCK_CONTRACT_STORAGE_PATH: &str = "./out/debug/timelock-contract-storage_slots.json";

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
        TIMELOCK_CONTRACT_BINARY_PATH,
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            TIMELOCK_CONTRACT_STORAGE_PATH.to_string()
        )),
    )
    .await
    .unwrap();

    let instance = Timelock::new(id.clone(), wallet.clone());

    (instance, wallet, wallet2)
}
