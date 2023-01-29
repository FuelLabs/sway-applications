use fuels::prelude::*;

abigen!(
    Vault,
    "./project/vault-contract/out/debug/vault-contract-abi.json"
);

const VAULT_CONTRACT_BINARY_PATH: &str = "./out/debug/vault-contract.bin";
const VAULT_CONTRACT_STORAGE_PATH: &str = "./out/debug/vault-contract-storage_slots.json";

pub async fn setup() -> (Vault, WalletUnlocked) {
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
        VAULT_CONTRACT_BINARY_PATH,
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(VAULT_CONTRACT_STORAGE_PATH.to_string())),
    )
    .await
    .unwrap();

    let instance = Vault::new(id, wallet.clone());

    (instance, wallet)
}
