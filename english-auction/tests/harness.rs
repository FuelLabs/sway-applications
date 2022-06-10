use fuels::{prelude::*, tx::ContractId};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(EnglishAuction, "out/debug/english-auction-abi.json");

struct Metadata {
    auction: EnglishAuction,
    wallet: LocalWallet,
}

async fn setup() -> (Metadata, Metadata, ContractId) {
    // Setup 2 test wallets
    let mut wallets = launch_provider_and_get_wallets(WalletsConfig {
        num_wallets: 2,
        coins_per_wallet: 1,
        coin_amount: 1000000,
    })
    .await;

    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();

    let auction_id = Contract::deploy(
        "./out/debug/english-auction-abi.json", 
        &wallet1, 
        TxParameters::default()
    )
    .await
    .unwrap();

    let deploy_wallet = Metadata {
        auction: EnglishAuction::new(auction_id.to_string(), wallet1.clone()),
        wallet: wallet1.clone(),
    };

    let seller = Metadata {
        auction: EnglishAuction::new(auction_id.to_string(), wallet2.clone()),
        wallet: wallet2.clone(),
    };

    (deploy_wallet, seller, auction_id)
}

mod constructor {

    use super::*;

    #[tokio::test]
    async fn inits() {
        let (deploy_wallet, seller, sell_asset_id) = setup().await;

        let boolean = deploy_wallet.auction.test_function().call().await.unwrap().value;

    }
}
