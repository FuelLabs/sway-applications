use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Bech32Address, Config, Contract,
        LoadConfiguration, StorageConfiguration, TxParameters, WalletUnlocked, WalletsConfig,
    },
    tx::ContractId,
    types::Identity,
};

abigen!(
    Contract(
        name = "Fundraiser",
        abi = "./contracts/fundraiser-contract/out/debug/fundraiser-contract-abi.json"
    ),
    Contract(
        name = "Asset",
        abi = "./contracts/fundraiser-contract/tests/artifacts/asset/out/debug/asset-abi.json"
    )
);

const ASSET_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/asset/out/debug/asset.bin";
const ASSET_CONTRACT_STORAGE_PATH: &str =
    "./tests/artifacts/asset/out/debug/asset-storage_slots.json";
const FUNDRAISER_CONTRACT_BINARY_PATH: &str = "./out/debug/fundraiser-contract.bin";
const FUNDRAISER_CONTRACT_STORAGE_PATH: &str = "./out/debug/fundraiser-contract-storage_slots.json";

pub(crate) struct Coin {
    pub(crate) contract: Asset<WalletUnlocked>,
    pub(crate) id: ContractId,
}

pub(crate) struct DefaultParameters {
    pub(crate) asset_id: ContractId,
    pub(crate) beneficiary: Identity,
    pub(crate) deadline: u64,
    pub(crate) target_amount: u64,
}

pub(crate) struct User {
    pub(crate) contract: Fundraiser<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) async fn identity(address: &Bech32Address) -> Identity {
    Identity::Address(address.into())
}

pub(crate) async fn mint(
    contract: &Asset<WalletUnlocked>,
    amount: u64,
    address: &Bech32Address,
) -> bool {
    contract
        .methods()
        .mint_and_send_to_address(amount, address.into())
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
        .value
}

pub(crate) async fn setup() -> (User, User, Coin, Coin, DefaultParameters) {
    let number_of_wallets = 3;
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

    let deployer_wallet = wallets.pop().unwrap();
    let author_wallet = wallets.pop().unwrap();
    let user_wallet = wallets.pop().unwrap();

    let fundraiser_storage_configuration =
        StorageConfiguration::load_from(FUNDRAISER_CONTRACT_STORAGE_PATH);
    let asset_storage_configuration = StorageConfiguration::load_from(ASSET_CONTRACT_STORAGE_PATH);

    let fundraiser_configuration = LoadConfiguration::default()
        .set_storage_configuration(fundraiser_storage_configuration.unwrap());
    let asset_configuration = LoadConfiguration::default()
        .set_storage_configuration(asset_storage_configuration.unwrap());

    let id = Contract::load_from(FUNDRAISER_CONTRACT_BINARY_PATH, fundraiser_configuration)
        .unwrap()
        .deploy(&deployer_wallet, TxParameters::default())
        .await
        .unwrap();

    let asset_id = Contract::load_from(ASSET_CONTRACT_BINARY_PATH, asset_configuration.clone())
        .unwrap()
        .deploy(&deployer_wallet, TxParameters::default())
        .await
        .unwrap();

    let asset2_id = Contract::load_from(
        ASSET_CONTRACT_BINARY_PATH,
        asset_configuration.set_salt([1u8; 32]),
    )
    .unwrap()
    .deploy(&deployer_wallet, TxParameters::default())
    .await
    .unwrap();

    let author = User {
        contract: Fundraiser::new(id.clone(), author_wallet.clone()),
        wallet: author_wallet,
    };

    let user = User {
        contract: Fundraiser::new(id, user_wallet.clone()),
        wallet: user_wallet.clone(),
    };

    let asset = Coin {
        contract: Asset::new(asset_id.clone(), deployer_wallet.clone()),
        id: asset_id.clone().into(),
    };

    let asset2 = Coin {
        contract: Asset::new(asset2_id.clone(), deployer_wallet),
        id: asset2_id.into(),
    };

    let defaults = DefaultParameters {
        asset_id: asset_id.into(),
        beneficiary: Identity::Address(user_wallet.address().into()),
        deadline: 100,
        target_amount: 512,
    };

    (author, user, asset, asset2, defaults)
}
