use fuels::{
    prelude::*,
    tx::{AssetId, ContractId},
};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(DaoVoting, "out/debug/dao-voting-abi.json");
abigen!(
    GovToken,
    "tests/artifacts/gov_token/out/debug/gov_token-abi.json"
);

async fn setup() -> (DaoVoting, ContractId, GovToken, ContractId, LocalWallet) {
    let num_wallets = 1;
    let coins_per_wallet = 1000;
    let amount_per_coin = 1_000_000;
    let config = WalletsConfig::new(
        Some(num_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_provider_and_get_wallets(config).await;
    let wallet = wallets.pop().unwrap();

    //let wallet = launch_provider_and_get_single_wallet().await;

    let dao_voting_id = Contract::deploy(
        "./out/debug/dao-voting.bin",
        &wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let gov_token_id = Contract::deploy(
        "./tests/artifacts/gov_token/out/debug/gov_token-abi.json",
        &wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let dao_voting = DaoVoting::new(dao_voting_id.to_string(), wallet.clone());

    let gov_token = GovToken::new(gov_token_id.to_string(), wallet.clone());

    (dao_voting, dao_voting_id, gov_token, gov_token_id, wallet)
}

async fn initialize() -> bool {
    let (dao_voting, _dao_voting_id, _gov_token, gov_token_id, _wallet) = setup().await;
    dao_voting
        .constructor(gov_token_id, 10, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value
}

#[tokio::test]
async fn initializes() {
    assert!(initialize().await);
}

#[tokio::test]
#[should_panic]
async fn panics_when_reinitialized() {
    let (dao_voting, _dao_voting_id, _gov_token, gov_token_id, _wallet) = setup().await;
    dao_voting
        .constructor(gov_token_id, 10, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
    dao_voting
        .constructor(gov_token_id, 10, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
}

#[tokio::test]
#[should_panic]
async fn panics_with_incorrect_voting_period() {
    let (dao_voting, _dao_voting_id, _gov_token, gov_token_id, _wallet) = setup().await;
    dao_voting
        .constructor(gov_token_id, 0, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
}

#[tokio::test]
#[should_panic]
async fn panics_with_incorrect_approval_percentage() {
    let (dao_voting, _dao_voting_id, _gov_token, gov_token_id, _wallet) = setup().await;
    dao_voting
        .constructor(gov_token_id, 10, 0, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
}

#[tokio::test]
async fn user_can_deposit() {
    let (dao_voting, _dao_voting_id, gov_token, gov_token_id, wallet) = setup().await;

    let res = gov_token
        .mint_and_send_to_address(100, wallet.address())
        .append_variable_outputs(1)
        .call()
        .await;
    match res {
        Ok(call_response) => {
            print!("call response: {:?}", call_response.logs);
        }

        Err(Error::ContractCallError(reason)) => {
            println!("Contract Call failed with reason : {}", reason);
        }

        _ => {
            println!("boo");
        }
    }

    dao_voting
        .constructor(gov_token_id, 10, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(dao_voting.get_balance().call().await.unwrap().value, 0);

    let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
    let call_params = CallParameters::new(Some(10), Some(AssetId::from(*gov_token_id)));
    assert!(
        dao_voting
            .deposit()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value
    );
}
