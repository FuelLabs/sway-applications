use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, AssetConfig, AssetId, Contract,
        LoadConfiguration, StorageConfiguration, TxPolicies, WalletUnlocked, WalletsConfig,
        BASE_ASSET_ID,
    },
    types::ContractId,
};

abigen!(Contract(
    name = "DaoVoting",
    abi = "./contracts/DAO-contract/out/debug/DAO-contract-abi.json"
),);

pub(crate) struct Metadata {
    pub(crate) dao_voting: DaoVoting<WalletUnlocked>,
    pub(crate) wallet: WalletUnlocked,
}

const DAO_CONTRACT_BINARY_PATH: &str = "./out/debug/DAO-contract.bin";
const DAO_CONTRACT_STORAGE_PATH: &str = "./out/debug/DAO-contract-storage_slots.json";

pub(crate) fn proposal_transaction(asset_id: AssetId) -> Proposal {
    let call_data = CallData {
        id: ContractId::from(*asset_id),
        function_selector: 0,
        arguments: 0,
    };

    Proposal {
        call_data,
        amount: 0,
        asset: asset_id,
        gas: 20000,
    }
}

pub(crate) async fn setup() -> (AssetId, AssetId, Metadata, Metadata, u64) {
    let number_of_coins = 1;
    let coin_amount = 1_000_000;
    let number_of_wallets = 2;

    let base_asset = AssetConfig {
        id: BASE_ASSET_ID,
        num_coins: number_of_coins,
        coin_amount,
    };
    let gov_asset_id = AssetId::new([1; 32]);
    let gov_asset = AssetConfig {
        id: gov_asset_id,
        num_coins: number_of_coins,
        coin_amount,
    };
    let other_asset_id = AssetId::new([2; 32]);
    let other_asset = AssetConfig {
        id: other_asset_id,
        num_coins: number_of_coins,
        coin_amount,
    };
    let assets = vec![base_asset, gov_asset, other_asset];

    let wallet_config = WalletsConfig::new_multiple_assets(number_of_wallets, assets);
    let mut wallets = launch_custom_provider_and_get_wallets(wallet_config, None, None)
        .await
        .unwrap();

    let deployer_wallet = wallets.pop().unwrap();
    let user_wallet = wallets.pop().unwrap();

    let storage_configuration =
        StorageConfiguration::default().add_slot_overrides_from_file(DAO_CONTRACT_STORAGE_PATH);
    let configuration =
        LoadConfiguration::default().with_storage_configuration(storage_configuration.unwrap());
    let dao_voting_id = Contract::load_from(DAO_CONTRACT_BINARY_PATH, configuration)
        .unwrap()
        .deploy(&deployer_wallet, TxPolicies::default())
        .await
        .unwrap();

    let deployer = Metadata {
        dao_voting: DaoVoting::new(dao_voting_id.clone(), deployer_wallet.clone()),
        wallet: deployer_wallet,
    };
    let user = Metadata {
        dao_voting: DaoVoting::new(dao_voting_id, user_wallet.clone()),
        wallet: user_wallet,
    };

    let asset_amount = 10;

    (gov_asset_id, other_asset_id, deployer, user, asset_amount)
}
