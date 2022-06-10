use fuels::{prelude::*, tx::ContractId};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(EnglishAuction, "out/debug/english-auction-abi.json");
abigen!(Asset, "tests/artifacts/asset/out/debug/asset-abi.json");

struct Metadata {
    asset: Option<Asset>,
    auction: EnglishAuction,
    wallet: LocalWallet,
}

async fn setup() -> (Metadata, Metadata, Metadata, Metadata, ContractId) {
    // Setup 3 test wallets
    let mut wallets = launch_provider_and_get_wallets(WalletsConfig {
        num_wallets: 4,
        coins_per_wallet: 1,
        coin_amount: 1000000,
    })
    .await;

    // Get the wallets from that provider
    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();
    let wallet3 = wallets.pop().unwrap();
    let wallet4 = wallets.pop().unwrap();

    let auction_id = Contract::deploy(
        "./out/debug/english-auction-abi.bin", 
        &wallet1, 
        TxParameters::default()
    )
    .await
    .unwrap();

    let asset_id = Contract::deploy(
        "./tests/artifacts/asset/out/debug/asset.bin",
        &wallet1,
        TxParameters::default(),
    )
    .await
    .unwrap();

    let deploy_wallet = Metadata {
        asset: Some(Asset::new(asset_id.to_string(), wallet1.clone())),
        auction: EnglishAuction::new(auction_id.to_string(), wallet1.clone()),
        wallet: wallet1.clone(),
    };

    let owner1 = Metadata {
        asset: Some(Asset::new(asset_id.to_string(), wallet2.clone())),
        auction: EnglishAuction::new(auction_id.to_string(), wallet2.clone()),
        wallet: wallet2.clone(),
    };

    let owner2 = Metadata {
        asset: Some(Asset::new(asset_id.to_string(), wallet3.clone())),
        auction: EnglishAuction::new(auction_id.to_string(), wallet3.clone()),
        wallet: wallet3.clone(),
    };
    
    let owner3 = Metadata {
        asset: Some(Asset::new(asset_id.to_string(), wallet4.clone())),
        auction: EnglishAuction::new(auction_id.to_string(), wallet4.clone()),
        wallet: wallet4.clone(),
    };

    (deploy_wallet, owner1, owner2, owner3, asset_id)
}

/*async fn get_contract_instance() -> (EnglishAuction, ContractId) {
    // Launch a local network and deploy the contract
    let wallet = launch_provider_and_get_single_wallet().await;

    let id = Contract::deploy("./out/debug/test_project.bin", &wallet, TxParameters::default())
        .await
        .unwrap();

    let instance = EnglishAuction::new(id.to_string(), wallet);

    (instance, id)
}*/
