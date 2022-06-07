use fuel_tx::ContractId;
use fuels::prelude::*;
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
    let wallets = launch_provider_and_get_wallets(WalletsConfig {
        num_wallets: 4,
        coins_per_wallet: 1,
        coin_amount: 1000000,
    })
    .await;

    // Get the wallets from that provider
    let wallet1 = &wallets[0];
    let wallet2 = &wallets[1];
    let wallet3 = &wallets[2];
    let wallet4 = &wallets[3];

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
