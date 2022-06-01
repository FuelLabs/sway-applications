use fuel_tx::{AssetId, ContractId};
use fuels::prelude::*;
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(Nft, "out/debug/nft-abi.json");

struct Metadata {
    nft: Nft,
    wallet: LocalWallet,
}

async fn setup() -> (Metadata, Metadata, Metadata) {
    // Create some addresses with the minimum amount of asset: 1 Million
    let (pk1, mut coins1) = setup_address_and_coins(1, 1000000);
    let (pk2, coins2) = setup_address_and_coins(1, 1000000);
    let (pk3, coins3) = setup_address_and_coins(1, 1000000);

    coins1.extend(coins2);
    coins1.extend(coins3);

    // Launch a provider with those coins
    let (provider, _) = setup_test_provider(coins1).await;

    // Get the wallets from that provider
    let wallet1 = LocalWallet::new_from_private_key(pk1, provider.clone());
    let wallet2 = LocalWallet::new_from_private_key(pk2, provider.clone());
    let wallet3 = LocalWallet::new_from_private_key(pk3, provider);

    let id = Contract::deploy(
        "./out/debug/NFT.bin", 
        &wallet1, 
        TxParameters::default()
    )
    .await
    .unwrap();

    let deploy_wallet = Metadata {
        nft: Nft::new(id.to_string(), wallet1.clone()),
        wallet: wallet1,
    };

    let owner1 = Metadata {
        nft: Nft::new(id.to_string(), wallet2.clone()),
        wallet: wallet2,
    };

    let owner2 = Metadata {
        nft: Nft::new(id.to_string(), wallet3.clone()),
        wallet: wallet3,
    };

    (deploy_wallet, owner1, owner2)
}

#[tokio::test]
async fn can_get_contract_id() {
    let (_instance, _id) = get_contract_instance().await;

    // Now you have an instance of your contract you can use to test each function
}
