use fuels::{
    accounts::ViewOnlyAccount,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Address, AssetId, Config, Contract,
        ContractId, LoadConfiguration, StorageConfiguration, TxParameters, WalletUnlocked,
        WalletsConfig,
    },
    types::Identity,
};

abigen!(
    Contract(
        name = "Escrow",
        abi = "./contracts/escrow-contract/out/debug/escrow-contract-abi.json"
    ),
    Contract(
        name = "MyAsset",
        abi = "./contracts/escrow-contract/tests/artifacts/asset/out/debug/asset-abi.json"
    )
);

const ASSET_CONTRACT_BINARY_PATH: &str = "./tests/artifacts/asset/out/debug/asset.bin";
const ASSET_CONTRACT_STORAGE_PATH: &str =
    "./tests/artifacts/asset/out/debug/asset-storage_slots.json";
const ESCROW_CONTRACT_BINARY_PATH: &str = "./out/debug/escrow-contract.bin";
const ESCROW_CONTRACT_STORAGE_PATH: &str = "./out/debug/escrow-contract-storage_slots.json";

pub(crate) struct Defaults {
    pub(crate) asset: MyAsset<WalletUnlocked>,
    pub(crate) asset_amount: u64,
    pub(crate) asset_id: ContractId,
    pub(crate) deadline: u64,
}

pub(crate) struct User {
    pub(crate) contract: Escrow<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) async fn asset_amount(asset: &ContractId, user: &User) -> u64 {
    user.wallet
        .clone()
        .get_asset_balance(&AssetId::from(**asset))
        .await
        .unwrap()
}

pub(crate) async fn create_arbiter(user: &User, asset: ContractId, fee_amount: u64) -> Arbiter {
    Arbiter {
        address: Identity::Address(user.wallet.address().into()),
        asset,
        fee_amount,
    }
}

pub(crate) async fn create_asset(amount: u64, id: ContractId) -> Asset {
    Asset { amount, id }
}

pub(crate) async fn create_asset_with_salt(
    salt: [u8; 32],
    wallet: WalletUnlocked,
) -> (ContractId, MyAsset<WalletUnlocked>) {
    let storage_configuration = StorageConfiguration::load_from(ASSET_CONTRACT_STORAGE_PATH);
    let configuration = LoadConfiguration::default()
        .set_storage_configuration(storage_configuration.unwrap())
        .set_salt(salt);

    let asset_id = Contract::load_from(ASSET_CONTRACT_BINARY_PATH, configuration)
        .unwrap()
        .deploy(&wallet, TxParameters::default())
        .await
        .unwrap();

    (asset_id.clone().into(), MyAsset::new(asset_id, wallet))
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn escrow_info(
    arbiter: Arbiter,
    asset_count: u64,
    buyer: &User,
    asset: Option<ContractId>,
    deposited_amount: u64,
    deadline: u64,
    disputed: bool,
    first_asset_index: u64,
    seller: &User,
    state: bool,
) -> EscrowInfo {
    EscrowInfo {
        arbiter,
        asset_count,
        buyer: Buyer {
            address: Identity::Address(Address::from(buyer.wallet.address())),
            asset,
            deposited_amount,
        },
        deadline,
        disputed,
        first_asset_index,
        seller: Seller {
            address: Identity::Address(Address::from(seller.wallet.address())),
        },
        state: match state {
            true => State::Completed,
            false => State::Pending,
        },
    }
}

pub(crate) async fn mint(user: &User, amount: u64, contract: &MyAsset<WalletUnlocked>) {
    contract
        .methods()
        .mint_and_send_to_address(amount, user.wallet.address().into())
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();
}

pub(crate) async fn setup() -> (User, User, User, Defaults) {
    let number_of_wallets = 4;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;

    let wallet_config = WalletsConfig::new(
        Some(number_of_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );
    let provider_config = Config {
        manual_blocks_enabled: true,
        ..Config::local_node()
    };

    let mut wallets =
        launch_custom_provider_and_get_wallets(wallet_config, Some(provider_config), None).await;

    let deployer_wallet = wallets.pop().unwrap();
    let arbiter_wallet = wallets.pop().unwrap();
    let buyer_wallet = wallets.pop().unwrap();
    let seller_wallet = wallets.pop().unwrap();

    let escrow_storage_configuration =
        StorageConfiguration::load_from(ESCROW_CONTRACT_STORAGE_PATH);
    let escrow_configuration = LoadConfiguration::default()
        .set_storage_configuration(escrow_storage_configuration.unwrap());

    let asset_storage_configuration = StorageConfiguration::load_from(ASSET_CONTRACT_STORAGE_PATH);
    let asset_configuration = LoadConfiguration::default()
        .set_storage_configuration(asset_storage_configuration.unwrap());

    let escrow_id = Contract::load_from(ESCROW_CONTRACT_BINARY_PATH, escrow_configuration)
        .unwrap()
        .deploy(&deployer_wallet, TxParameters::default())
        .await
        .unwrap();

    let asset_id = Contract::load_from(ASSET_CONTRACT_BINARY_PATH, asset_configuration)
        .unwrap()
        .deploy(&deployer_wallet, TxParameters::default())
        .await
        .unwrap();

    let asset = MyAsset::new(asset_id.clone(), deployer_wallet);

    let arbiter = User {
        contract: Escrow::new(escrow_id.clone(), arbiter_wallet.clone()),
        wallet: arbiter_wallet,
    };

    let buyer = User {
        contract: Escrow::new(escrow_id.clone(), buyer_wallet.clone()),
        wallet: buyer_wallet,
    };

    let seller = User {
        contract: Escrow::new(escrow_id.clone(), seller_wallet.clone()),
        wallet: seller_wallet,
    };

    let defaults = Defaults {
        asset,
        asset_id: asset_id.into(),
        asset_amount: 100,
        deadline: 100,
    };

    (arbiter, buyer, seller, defaults)
}
