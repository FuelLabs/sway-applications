use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Contract, LoadConfiguration, TxPolicies,
        WalletsConfig,
    },
};

// TODO: do setup instead of copy/pasted code with minor adjustments

// Load abi from json
abigen!(Contract(
    name = "Timelock",
    abi = "./timelock-contract/out/debug/timelock-contract-abi.json"
));
const TIMELOCK_CONTRACT_BINARY_PATH: &str = "./out/debug/timelock-contract.bin";
const TIMELOCK_CONTRACT_STORAGE_PATH: &str = "./out/debug/timelock-contract-storage_slots.json";

pub async fn setup() -> (Timelock<WalletUnlocked>, WalletUnlocked, WalletUnlocked) {
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
    .await
    .unwrap();

    let wallet = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();

    let id = Contract::load_from(TIMELOCK_CONTRACT_BINARY_PATH, LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet, TxPolicies::default())
        .await
        .unwrap();

    let instance = Timelock::new(id.clone(), wallet.clone());

    (instance, wallet, wallet2)
}
