use fuels::prelude::*;

abigen!(
    Template,
    "./project/template-contract/out/debug/template-contract-abi.json"
);

const TEMPLATE_CONTRACT_BINARY_PATH: &str = "./out/debug/template-contract.bin";
const TEMPLATE_CONTRACT_STORAGE_PATH: &str = "./out/debug/template-contract-storage_slots.json";

pub async fn setup() -> (Template, WalletUnlocked) {
    let number_of_wallets = 1;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;

    let wallet_config = WalletsConfig::new(
        Some(number_of_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let provider_config = Config {
        manual_blocks_enabled: true, // Necessary so the `produce_blocks` API can be used locally
        ..Config::local_node()
    };

    let mut wallets =
        launch_custom_provider_and_get_wallets(wallet_config, Some(provider_config), None).await;

    let wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        TEMPLATE_CONTRACT_BINARY_PATH,
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(TEMPLATE_CONTRACT_STORAGE_PATH.to_string())),
    )
    .await
    .unwrap();

    let instance = Template::new(id, wallet.clone());

    (instance, wallet)
}
