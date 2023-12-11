use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, AssetConfig, AssetId, Bech32Address,
        Contract, LoadConfiguration, StorageConfiguration, TxPolicies,
        WalletUnlocked, WalletsConfig, BASE_ASSET_ID,
    },
    types::Identity,
};

abigen!(Contract(
    name = "Fundraiser",
    abi = "./contracts/fundraiser-contract/out/debug/fundraiser-contract-abi.json"
),);

const FUNDRAISER_CONTRACT_BINARY_PATH: &str = "./out/debug/fundraiser-contract.bin";
const FUNDRAISER_CONTRACT_STORAGE_PATH: &str = "./out/debug/fundraiser-contract-storage_slots.json";

pub(crate) struct Coin {
    pub(crate) id: AssetId,
}

pub(crate) struct DefaultParameters {
    pub(crate) asset_id: AssetId,
    pub(crate) beneficiary: Identity,
    pub(crate) deadline: u64,
    pub(crate) initial_wallet_amount: u64,
    pub(crate) target_amount: u64,
}

pub(crate) struct User {
    pub(crate) contract: Fundraiser<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) async fn identity(address: &Bech32Address) -> Identity {
    Identity::Address(address.into())
}

pub(crate) async fn setup() -> (User, User, Coin, Coin, DefaultParameters) {
    let number_of_coins = 1;
    let coin_amount = 1_000_000;
    let number_of_wallets = 3;

    let base_asset = AssetConfig {
        id: BASE_ASSET_ID,
        num_coins: number_of_coins,
        coin_amount,
    };
    let asset_id = AssetId::new([1; 32]);
    let asset = AssetConfig {
        id: asset_id,
        num_coins: number_of_coins,
        coin_amount,
    };
    let asset2_id = AssetId::new([2; 32]);
    let asset2 = AssetConfig {
        id: asset2_id,
        num_coins: number_of_coins,
        coin_amount,
    };
    let assets = vec![base_asset, asset, asset2];

    let wallet_config = WalletsConfig::new_multiple_assets(number_of_wallets, assets);

    let mut wallets =
        launch_custom_provider_and_get_wallets(wallet_config, None , None).await.unwrap();

    let deployer_wallet = wallets.pop().unwrap();
    let author_wallet = wallets.pop().unwrap();
    let user_wallet = wallets.pop().unwrap();

    let fundraiser_storage_configuration =
        StorageConfiguration::default().add_slot_overrides_from_file(FUNDRAISER_CONTRACT_STORAGE_PATH);
    let fundraiser_configuration = LoadConfiguration::default()
        .with_storage_configuration(fundraiser_storage_configuration.unwrap());
    let fundraiser_id =
        Contract::load_from(FUNDRAISER_CONTRACT_BINARY_PATH, fundraiser_configuration)
            .unwrap()
            .deploy(&deployer_wallet, TxPolicies::default())
            .await
            .unwrap();

    let author = User {
        contract: Fundraiser::new(fundraiser_id.clone(), author_wallet.clone()),
        wallet: author_wallet,
    };

    let user = User {
        contract: Fundraiser::new(fundraiser_id, user_wallet.clone()),
        wallet: user_wallet.clone(),
    };

    let asset = Coin {
        id: asset_id,
    };

    let asset2 = Coin {
        id: asset2_id,
    };

    let defaults = DefaultParameters {
        asset_id: asset_id,
        beneficiary: Identity::Address(user_wallet.address().into()),
        deadline: 100,
        initial_wallet_amount: coin_amount,
        target_amount: 512,
    };

    (author, user, asset, asset2, defaults)
}
