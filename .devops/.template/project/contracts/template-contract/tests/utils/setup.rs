use fuels::prelude::*;

abigen!(Contract(
    name = "Template",
    abi = "./contracts/template-contract/out/debug/template-contract-abi.json"
));

const TEMPLATE_CONTRACT_BINARY_PATH: &str = "./out/debug/template-contract.bin";
const TEMPLATE_CONTRACT_STORAGE_PATH: &str = "./out/debug/template-contract-storage_slots.json";

pub async fn setup() -> (Template<WalletUnlocked>, WalletUnlocked) {
    let number_of_wallets = 1;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;

    let wallet_config = WalletsConfig::new(
        Some(number_of_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_custom_provider_and_get_wallets(wallet_config, None, None)
        .await
        .unwrap();

    let wallet = wallets.pop().unwrap();

    let storage_configuration = StorageConfiguration::default()
        .add_slot_overrides_from_file(TEMPLATE_CONTRACT_STORAGE_PATH);
    let configuration =
        LoadConfiguration::default().with_storage_configuration(storage_configuration.unwrap());

    let id = Contract::load_from(TEMPLATE_CONTRACT_BINARY_PATH, configuration)
        .unwrap()
        .deploy(&wallet, TxPolicies::default())
        .await
        .unwrap();

    let instance = Template::new(id, wallet.clone());

    (instance, wallet)
}
