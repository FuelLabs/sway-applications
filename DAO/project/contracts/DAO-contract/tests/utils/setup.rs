use fuels::prelude::{
    abigen, launch_custom_provider_and_get_wallets, Bech32Address, Contract, ContractId,
    StorageConfiguration, TxParameters, WalletUnlocked, WalletsConfig,
};

abigen!(
    Contract(
        name = "DaoVoting",
        abi = "./contracts/DAO-contract/out/debug/DAO-contract-abi.json"
    ),
    Contract(
        name = "GovToken",
        abi = "./contracts/DAO-contract/tests/artifacts/gov_token/out/debug/gov_token-abi.json"
    ),
);

pub(crate) struct Metadata {
    pub(crate) dao_voting: DaoVoting,
    pub(crate) gov_token: Option<GovToken>,
    pub(crate) wallet: WalletUnlocked,
}

const DAO_CONTRACT_BINARY_PATH: &str = "./out/debug/DAO-contract.bin";
const DAO_CONTRACT_STORAGE_PATH: &str = "./out/debug/DAO-contract-storage_slots.json";
const GOVERNANCE_TOKEN_BINARY_PATH: &str = "./tests/artifacts/gov_token/out/debug/gov_token.bin";
const GOVERNANCE_TOKEN_STORAGE_PATH: &str =
    "./tests/artifacts/gov_token/out/debug/gov_token-storage_slots.json";

pub(crate) async fn mint(contract: &GovToken, amount: u64, address: &Bech32Address) -> bool {
    contract
        .methods()
        .mint_and_send_to_address(amount, address.into())
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
        .value
}

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

pub(crate) async fn setup() -> (GovToken, ContractId, Metadata, Metadata, u64) {
    let number_of_wallets = 2;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;

    let config = WalletsConfig::new(
        Some(number_of_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await;
    let deployer_wallet = wallets.pop().unwrap();
    let user_wallet = wallets.pop().unwrap();

    let dao_voting_id = Contract::deploy(
        DAO_CONTRACT_BINARY_PATH,
        &deployer_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(DAO_CONTRACT_STORAGE_PATH.to_string())),
    )
    .await
    .unwrap();

    let gov_token_id = Contract::deploy(
        GOVERNANCE_TOKEN_BINARY_PATH,
        &deployer_wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(GOVERNANCE_TOKEN_STORAGE_PATH.to_string())),
    )
    .await
    .unwrap();

    let gov_token = GovToken::new(gov_token_id.clone(), deployer_wallet.clone());

    let deployer = Metadata {
        dao_voting: DaoVoting::new(dao_voting_id.clone(), deployer_wallet.clone()),
        gov_token: Some(GovToken::new(gov_token_id.clone(), deployer_wallet.clone())),
        wallet: deployer_wallet,
    };

    let user = Metadata {
        dao_voting: DaoVoting::new(dao_voting_id, user_wallet.clone()),
        gov_token: None,
        wallet: user_wallet,
    };

    let asset_amount = 10;

    (gov_token, gov_token_id.into(), deployer, user, asset_amount)
}
