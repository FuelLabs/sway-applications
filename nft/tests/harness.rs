use fuel_tx::{AssetId, ContractId};
use fuels::prelude::*;
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(Nft, "out/debug/nft-abi.json");

struct Metadata {
    asset: Option<Asset>,
    nft: Nft,
    wallet: LocalWallet,
}

async fn setup() -> (Metadata, Metadata, Metadata, ContractId) {
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

    let nft_id = Contract::deploy(
        "./out/debug/NFT.bin", 
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
        nft: Nft::new(nft_id.to_string(), wallet1.clone()),
        wallet: wallet1,
    };

    let owner1 = Metadata {
        asset: Some(Asset::new(asset_id.to_string(), wallet2.clone())),
        nft: Nft::new(nft_id.to_string(), wallet2.clone()),
        wallet: wallet2,
    };

    let owner2 = Metadata {
        asset: Some(Asset::new(asset_id.to_string(), wallet3.clone())),
        nft: Nft::new(nft_id.to_string(), wallet3.clone()),
        wallet: wallet3,
    };

    (deploy_wallet, owner1, owner2, asset_id)
}

async fn init(
    deploy_wallet: &Metadata,
    owner: &Metadata,
    access_control: bool,
    token_supply: u64,
    token_price: u64,
    asset: ContractId
) -> bool {
    deploy_wallet
        .nft
        .constructor(owner.address(), access_control, token_supply, token_price, asset)
        .call()
        .await
        .unwrap()
        .value
}

mod constructor {

    use super::*;

    #[tokio::test]
    async fn initalizes() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        assert!(
            init(
                &deploy_wallet,
                &owner1,
                true,
                1,
                1,
                asset_id
            )
            .await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_initalized_twice() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, true, 1, 1, asset_id).await;

        assert!(
            init(
                &deploy_wallet,
                &owner1,
                true,
                1,
                1,
                asset_id
            )
            .await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_token_supply_is_zero() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        assert!(
            init(
                &deploy_wallet,
                &owner1,
                true,
                0,
                0,
                asset_id
            )
            .await
        );
    }
}
