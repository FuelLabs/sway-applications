use fuels::prelude::{
    abigen, launch_custom_provider_and_get_wallets, AssetConfig, AssetId, Contract, ContractId,
    LoadConfiguration, StorageConfiguration, TxParameters, WalletUnlocked, WalletsConfig,
    BASE_ASSET_ID,
};
use rand::Fill;

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

pub(crate) fn proposal_transaction(asset_id: ContractId) -> Proposal {
    let call_data = CallData {
        id: asset_id,
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

pub(crate) async fn setup() -> (ContractId, ContractId, Metadata, Metadata, u64) {
    let mut rng = rand::thread_rng();
    let num_coins = 1;
    let coin_amount = 1_000_000;

    let base_asset = AssetConfig {
        id: BASE_ASSET_ID,
        num_coins,
        coin_amount,
    };
    let mut gov_token_id = AssetId::zeroed();
    gov_token_id.try_fill(&mut rng).unwrap();
    let gov_token = AssetConfig {
        id: gov_token_id,
        num_coins,
        coin_amount,
    };
    let mut other_token_id = AssetId::zeroed();
    other_token_id.try_fill(&mut rng).unwrap();
    let other_token = AssetConfig {
        id: other_token_id,
        num_coins,
        coin_amount,
    };
    let assets = vec![base_asset, gov_token, other_token];

    let num_wallets = 2;
    let wallet_config = WalletsConfig::new_multiple_assets(num_wallets, assets);
    let mut wallets = launch_custom_provider_and_get_wallets(wallet_config, None, None).await;

    let deployer_wallet = wallets.pop().unwrap();
    let user_wallet = wallets.pop().unwrap();

    let storage_configuration = StorageConfiguration::load_from(DAO_CONTRACT_STORAGE_PATH);
    let configuration =
        LoadConfiguration::default().set_storage_configuration(storage_configuration.unwrap());
    let dao_voting_id = Contract::load_from(DAO_CONTRACT_BINARY_PATH, configuration)
        .unwrap()
        .deploy(&deployer_wallet, TxParameters::default())
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

    (
        ContractId::from(*gov_token_id),
        ContractId::from(*other_token_id),
        deployer,
        user,
        asset_amount,
    )
}
