use fuels::prelude::{
    abigen, setup_multiple_assets_coins, setup_test_provider, AssetId, Contract, ContractId,
    LoadConfiguration, StorageConfiguration, TxParameters, WalletUnlocked,
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
    let (mut wallets, mut asset_ids) = setup_wallets().await;

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

    let gov_token_id: [u8; 32] = asset_ids.pop().unwrap().into();
    let other_token_id: [u8; 32] = asset_ids.pop().unwrap().into();

    (
        gov_token_id.into(),
        other_token_id.into(),
        deployer,
        user,
        asset_amount,
    )
}

pub(crate) async fn setup_wallets() -> (Vec<WalletUnlocked>, Vec<AssetId>) {
    let mut wallets = vec![
        WalletUnlocked::new_random(None),
        WalletUnlocked::new_random(None),
    ];

    let num_assets = 3;
    let coins_per_asset = 1;
    let amount_per_coin = 1_000_000;

    let mut all_coins = vec![];
    let mut asset_ids = vec![];

    for wallet in &mut wallets {
        let (mut coin, assets) = setup_multiple_assets_coins(
            wallet.address(),
            num_assets,
            coins_per_asset,
            amount_per_coin,
        );

        all_coins.append(&mut coin);
        asset_ids = assets;
    }

    let (provider, _) = setup_test_provider(all_coins, vec![], None, None).await;

    for wallet in &mut wallets {
        wallet.set_provider(provider.clone());
    }

    (wallets, asset_ids)
}
