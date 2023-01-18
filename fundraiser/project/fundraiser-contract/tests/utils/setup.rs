use fuels::{
    prelude::*,
    tx::{ContractId, Salt},
};

abigen!(
    Fundraiser,
    "./project/fundraiser-contract/out/debug/fundraiser-contract-abi.json"
);

abigen!(
    Asset,
    "./project/fundraiser-contract/tests/artifacts/asset/out/debug/asset-abi.json"
);

const ASSET_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/asset/out/debug/asset.bin";
const ASSET_CONTRACT_STORAGE_PATH: &str =
    "./tests/artifacts/asset/out/debug/asset-storage_slots.json";
const FUNDRAISER_CONTRACT_BINARY_PATH: &str = "./out/debug/fundraiser-contract.bin";
const FUNDRAISER_CONTRACT_STORAGE_PATH: &str = "./out/debug/fundraiser-contract-storage_slots.json";

pub struct Coin {
    pub contract: Asset,
    pub id: ContractId,
}

pub struct DefaultParameters {
    pub asset_id: ContractId,
    pub beneficiary: Identity,
    pub deadline: u64,
    pub target_amount: u64,
}

pub struct User {
    pub contract: Fundraiser,
    pub wallet: WalletUnlocked,
}

pub async fn identity(address: &Bech32Address) -> Identity {
    Identity::Address(address.into())
}

pub async fn mint(contract: &Asset, amount: u64, address: &Bech32Address) -> bool {
    contract
        .methods()
        .mint_and_send_to_address(amount, address.into())
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
        .value
}

pub async fn setup() -> (User, User, Coin, Coin, DefaultParameters) {
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

    let id = Contract::deploy(
        FUNDRAISER_CONTRACT_BINARY_PATH,
        &deployer_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(FUNDRAISER_CONTRACT_STORAGE_PATH.to_string())),
    )
    .await
    .unwrap();

    let asset_id = Contract::deploy(
        ASSET_CONTRACT_BINARY_PATH,
        &deployer_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(ASSET_CONTRACT_STORAGE_PATH.to_string())),
    )
    .await
    .unwrap();

    let asset2_id = Contract::deploy_with_parameters(
        ASSET_CONTRACT_BINARY_PATH,
        &deployer_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(ASSET_CONTRACT_STORAGE_PATH.to_string())),
        Salt::from([1u8; 32]),
    )
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
