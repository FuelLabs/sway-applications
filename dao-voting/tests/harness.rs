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

async fn setup() -> (ContractId, Metadata, Metadata) {
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
        "./tests/artifacts/gov_token/out/debug/gov_token-abi.json",
        &deployer_wallet,
        TxParameters::default(),
    )
    .await
    .unwrap();

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

    (gov_token_id, deployer, user)
}

async fn initialize() -> bool {
    let (gov_token_id, deployer, _user) = setup().await;
    deployer
        .dao_voting
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
    let (gov_token_id, deployer, user) = setup().await;
    deployer
        .dao_voting
        .constructor(gov_token_id, 10, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
    deployer
        .dao_voting
        .constructor(gov_token_id, 10, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
}

#[tokio::test]
#[should_panic]
async fn panics_with_incorrect_voting_period() {
    let (gov_token_id, deployer, user) = setup().await;
    deployer
        .dao_voting
        .constructor(gov_token_id, 0, 10, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
}

#[tokio::test]
#[should_panic]
async fn panics_with_incorrect_approval_percentage() {
    let (gov_token_id, deployer, user) = setup().await;
    deployer
        .dao_voting
        .constructor(gov_token_id, 10, 0, [0; 32])
        .call()
        .await
        .unwrap()
        .value;
}

#[tokio::test]
async fn user_can_deposit() {
    let (gov_token_id, deployer, user) = setup().await;

    println!("{:?}", gov_token_id);

    let res = deployer
        .gov_token
        .unwrap()
        .mint_and_send_to_address(100, user.wallet.address())
        .append_variable_outputs(1)
        .call()
        .await.unwrap();
    // match res {
    //     Ok(call_response) => {
    //         print!("call response: {:?}", call_response.logs);
    //     }

    //     Err(Error::ContractCallError(reason)) => {
    //         println!("Contract Call failed with reason : {}", reason);
    //     }

    //     _ => {
    //         println!("boo");
    //     }
    // }

    // deployer
    //     .dao_voting
    //     .constructor(gov_token_id, 10, 10, [0; 32])
    //     .call()
    //     .await
    //     .unwrap()
    //     .value;

    // assert_eq!(
    //     deployer
    //         .dao_voting
    //         .get_balance()
    //         .call()
    //         .await
    //         .unwrap()
    //         .value,
    //     0
    // );

    // let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
    // let call_params = CallParameters::new(Some(10), Some(AssetId::from(*gov_token_id)));
    // assert!(
    //     dao_voting
    //         .deposit()
    //         .tx_params(tx_params)
    //         .call_params(call_params)
    //         .call()
    //         .await
    //         .unwrap()
    //         .value
    // );
}
