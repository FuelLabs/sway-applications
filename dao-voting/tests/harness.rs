#![allow(unused_variables)]
#![allow(unused_imports)]

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

struct Metadata {
    dao_voting: DaoVoting,
    gov_token: Option<GovToken>,
    wallet: LocalWallet,
}

async fn setup() -> (GovToken, ContractId, Metadata, Metadata, u64) {
    let num_wallets = 2;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;
    let config = WalletsConfig::new(
        Some(num_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_provider_and_get_wallets(config).await;
    let deployer_wallet = wallets.pop().unwrap();
    let user_wallet = wallets.pop().unwrap();

    let dao_voting_id = Contract::deploy(
        "./out/debug/dao-voting.bin",
        &deployer_wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let gov_token_id = Contract::deploy(
        "./tests/artifacts/gov_token/out/debug/gov_token.bin",
        &deployer_wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let gov_token = GovToken::new(gov_token_id.to_string(), deployer_wallet.clone());

    let deployer = Metadata {
        dao_voting: DaoVoting::new(dao_voting_id.to_string(), deployer_wallet.clone()),
        gov_token: Some(GovToken::new(
            gov_token_id.to_string(),
            deployer_wallet.clone(),
        )),
        wallet: deployer_wallet,
    };

    let user = Metadata {
        dao_voting: DaoVoting::new(dao_voting_id.to_string(), user_wallet.clone()),
        gov_token: None,
        wallet: user_wallet,
    };

    let asset_amount: u64 = 10;

    (gov_token, gov_token_id, deployer, user, asset_amount)
}

async fn initialize() -> bool {
    let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
    deployer
        .dao_voting
        .constructor(gov_token_id, 10, 10)
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
    let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
    deployer
        .dao_voting
        .constructor(gov_token_id, 10, 10)
        .call()
        .await
        .unwrap()
        .value;
    deployer
        .dao_voting
        .constructor(gov_token_id, 10, 10)
        .call()
        .await
        .unwrap()
        .value;
}

#[tokio::test]
#[should_panic]
async fn panics_with_incorrect_voting_period() {
    let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
    deployer
        .dao_voting
        .constructor(gov_token_id, 0, 10)
        .call()
        .await
        .unwrap()
        .value;
}

#[tokio::test]
#[should_panic]
async fn panics_with_incorrect_approval_percentage() {
    let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
    deployer
        .dao_voting
        .constructor(gov_token_id, 10, 0)
        .call()
        .await
        .unwrap()
        .value;
}

#[tokio::test]
async fn user_can_deposit() {
    let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

    assert!(
        deployer
            .gov_token
            .unwrap()
            .mint_and_send_to_address(100, user.wallet.address())
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
            .value
    );

    deployer
        .dao_voting
        .constructor(gov_token_id, 10, 10)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(
        deployer
            .dao_voting
            .get_balance()
            .call()
            .await
            .unwrap()
            .value,
        0
    );

    let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
    let call_params = CallParameters::new(Some(asset_amount), Some(AssetId::from(*gov_token_id)));
    assert!(
        user.dao_voting
            .deposit()
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await
            .unwrap()
            .value
    );

    assert_eq!(
        deployer
            .dao_voting
            .get_balance()
            .call()
            .await
            .unwrap()
            .value,
        asset_amount
    );
}

#[tokio::test]
#[should_panic]
async fn panics_when_not_initialized() {
    let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

    assert!(
        deployer
            .gov_token
            .unwrap()
            .mint_and_send_to_address(100, user.wallet.address())
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
            .value
    );

    let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
    let call_params = CallParameters::new(Some(asset_amount), Some(AssetId::from(*gov_token_id)));
    user.dao_voting
        .deposit()
        .tx_params(tx_params)
        .call_params(call_params)
        .call()
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic]
async fn panics_with_incorrect_asset() {
    let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

    let another_asset_id = Contract::deploy_with_salt(
        "./tests/artifacts/asset/out/debug/asset.bin",
        &deployer.wallet,
        TxParameters::default(),
        Salt::from([1u8; 32]),
    )
    .await
    .unwrap();

    let another_asset = GovToken::new(another_asset_id.to_string(), deployer.wallet.clone());

    assert!(
        another_asset
            .mint_and_send_to_address(100, user.wallet.address())
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
            .value
    );

    deployer
        .dao_voting
        .constructor(gov_token_id, 10, 10)
        .call()
        .await
        .unwrap()
        .value;

    let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
    let call_params =
        CallParameters::new(Some(asset_amount), Some(AssetId::from(*another_asset_id)));
    user.dao_voting
        .deposit()
        .tx_params(tx_params)
        .call_params(call_params)
        .call()
        .await
        .unwrap();
}

#[tokio::test]
#[should_panic]
async fn panics_with_incorrect_amount() {
    let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;

    assert!(
        deployer
            .gov_token
            .unwrap()
            .mint_and_send_to_address(100, user.wallet.address())
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
            .value
    );

    deployer
        .dao_voting
        .constructor(gov_token_id, 10, 10)
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(
        deployer
            .dao_voting
            .get_balance()
            .call()
            .await
            .unwrap()
            .value,
        0
    );

    let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
    let call_params = CallParameters::new(Some(0), Some(AssetId::from(*gov_token_id)));
    user.dao_voting
        .deposit()
        .tx_params(tx_params)
        .call_params(call_params)
        .call()
        .await
        .unwrap();
}

#[tokio::test]
async fn user_can_add_proposal() {
    let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
    deployer
        .dao_voting
        .constructor(gov_token_id, 10, 10)
        .call()
        .await
        .unwrap()
        .value;
    assert!(
        user.dao_voting
            .add_proposal([1; 32])
            .call()
            .await
            .unwrap()
            .value
    );

    let proposal = user.dao_voting.get_proposal(0).call().await.unwrap().value;

    assert_eq!(
        proposal,
        daovoting_mod::Proposal {
            approved: false,
            expired: false,
            data: [1; 32]
        }
    );
}

#[tokio::test]
#[should_panic]
async fn panics_on_incorrect_proposal_id() {
    let (gov_token, gov_token_id, deployer, user, asset_amount) = setup().await;
    user.dao_voting.get_proposal(0).call().await.unwrap();
}
