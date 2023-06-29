use fuels::prelude::{
    abigen, launch_custom_provider_and_get_wallets, Contract, ContractId, LoadConfiguration,
    StorageConfiguration, TxParameters, WalletUnlocked, WalletsConfig,
};

abigen!(Contract(
    name = "SimpleAsset",
    abi = "./contracts/asset-contract/out/debug/asset-contract-abi.json"
));

pub(crate) struct Metadata {
    pub(crate) asset_id: ContractId,
    pub(crate) simple_asset: SimpleAsset<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) const ASSET_CONTRACT_BINARY_PATH: &str = "./out/debug/asset-contract.bin";
pub(crate) const ASSET_CONTRACT_STORAGE_PATH: &str =
    "./out/debug/asset-contract-storage_slots.json";

pub(crate) async fn setup() -> (Metadata, Metadata, u64) {
    let number_of_wallets = 2;
    let coins_per_wallet = 1;
    let coin_amount = 1000000;

    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(number_of_wallets),
            Some(coins_per_wallet),
            Some(coin_amount),
        ),
        None,
        None,
    )
    .await;

    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();

    let storage_configuration = StorageConfiguration::load_from(ASSET_CONTRACT_STORAGE_PATH);
    let configuration =
        LoadConfiguration::default().set_storage_configuration(storage_configuration.unwrap());

    let simple_asset_id = Contract::load_from(ASSET_CONTRACT_BINARY_PATH, configuration)
        .unwrap()
        .deploy(&wallet1, TxParameters::default())
        .await
        .unwrap();

    let deployer = Metadata {
        asset_id: ContractId::new(*simple_asset_id.hash()),
        simple_asset: SimpleAsset::new(simple_asset_id.clone(), wallet1.clone()),
        wallet: wallet1,
    };

    let user = Metadata {
        asset_id: ContractId::new(*simple_asset_id.hash()),
        simple_asset: SimpleAsset::new(simple_asset_id, wallet2.clone()),
        wallet: wallet2,
    };

    let total_supply = 100;

    (deployer, user, total_supply)
}
