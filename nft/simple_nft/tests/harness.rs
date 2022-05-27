use fuels_abigen_macro::abigen;
use fuels::prelude::*;

// Load abi from json
abigen!(SimpleNFT, "out/debug/simple_nft-abi.json");

struct Metadata {
    nft: SimpleNFT,
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
        "./out/debug/simple_nft.bin", 
        &wallet1, 
        TxParameters::default()
    )
    .await
    .unwrap();

    let deploy_wallet = Metadata {
        nft: SimpleNFT::new(id.to_string(), wallet1.clone()),
        wallet: wallet1,
    };

    let owner1 = Metadata {
        nft: SimpleNFT::new(id.to_string(), wallet2.clone()),
        wallet: wallet2,
    };

    let owner2 = Metadata {
        nft: SimpleNFT::new(id.to_string(), wallet3.clone()),
        wallet: wallet3,
    };

    (deploy_wallet, owner1, owner2)
}

async fn mint(
    deploy_wallet: &Metadata,
    owner1: &Metadata,
) -> bool {
    deploy_wallet
        .nft
        .mint(owner1.wallet.address())
        .call()
        .await
        .unwrap()
        .value
}

mod mint {
    
    use super::*;
    
    #[tokio::test]
    async fn mints() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert!(
            mint(
                &deploy_wallet,
                &owner1
            )
            .await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_minted_twice() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        mint(&deploy_wallet, &owner1).await;

        assert!(
            mint(
                &deploy_wallet,
                &owner1
            )
            .await
        );   
    }

    // TODO: Test for wallet with 0 address
    // #[tokio::test]
    // #[should_panic]
    // async fn panics_when_minted_to_zero_address() {
    //     let (deploy_wallet, owner1, owner2) = setup().await;

    //     let empty_address = Metadata {
    //         nft: owner1.nft,
    //         wallet: LocalWallet::, // Something needs to happen here
    //     };

    //     assert!(
    //         mint(
    //             &deploy_wallet,
    //             &empty_address
    //         )
    //         .await
    //     ); 
    // }
}

mod burn {

    use super::*;

    #[tokio::test]
    async fn burns() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        mint(&deploy_wallet, &owner1).await;

        assert!(
            owner1
            .nft
            .burn()
            .call()
            .await
            .unwrap()
            .value
        );

        assert_ne!(
            owner1.nft.owner().call().await.unwrap().value,
            owner1.wallet.address()
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_burn_not_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        mint(&deploy_wallet, &owner1).await;

        assert!(
            owner2
            .nft
            .burn()
            .call()
            .await
            .unwrap()
            .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        assert!(
            owner1
            .nft
            .burn()
            .call()
            .await
            .unwrap()
            .value
        );
    }
}

mod appoval {

    use super::*;

    #[tokio::test]
    async fn sets_approval() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        mint(&deploy_wallet, &owner1).await;

        assert_eq!(
            owner1.nft.set_approval(
                owner2
                    .wallet
                    .address())
                    .call()
                    .await
                    .unwrap()
                    .value,
            true
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_approval_not_from_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        mint(&deploy_wallet, &owner1).await;

        assert!(
            owner2
                .nft
                .set_approval(owner2.wallet.address())
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_approval_is_owner() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        mint(&deploy_wallet, &owner1).await;

        assert!(
            owner1
            .nft
            .set_approval(owner1.wallet.address())
            .call()
            .await
            .unwrap()
            .value
        );
    }

    #[tokio::test]
    async fn gets_approval() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        mint(&deploy_wallet, &owner1).await;

        let _result = owner1
            .nft
            .set_approval(owner2.wallet.address())
            .call().
            await;

        assert_eq!(
            owner1.nft.get_approval().call().await.unwrap().value,
            owner2.wallet.address()
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, owner2) = setup().await;

        assert_eq!(
            owner1
                .nft
                .set_approval(owner2.wallet.address())
                .call()
                .await
                .unwrap()
                .value,
            true
        );
    }
}

mod is_owner {

    use super::*;

    #[tokio::test]
    async fn owner_matches() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        mint(&deploy_wallet, &owner1).await;

        assert_eq!(
            owner1.nft.owner().call().await.unwrap().value,
            owner1.wallet.address()
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(
            owner1.nft.owner().call().await.unwrap().value,
            owner1.wallet.address()
        );
    }
}

mod transfer { 

    use super::*;

    #[tokio::test]
    async fn transfers_by_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        mint(&deploy_wallet,&owner1).await;

        assert!(
            owner1
                .nft
                .transfer(owner1.wallet.address(), owner2.wallet.address())
                .call()
                .await
                .unwrap()
                .value
        );

        assert_eq!(
            owner2.nft.owner().call().await.unwrap().value,
            owner2.wallet.address()
        );
    }

    #[tokio::test]
    async fn transfers_by_approval() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        mint(&deploy_wallet, &owner1).await;

        let _result = owner1
            .nft
            .set_approval(owner2.wallet.address())
            .call()
            .await;

        assert!(
            owner2
                .nft
                .transfer(owner1.wallet.address(), owner2.wallet.address())
                .call()
                .await
                .unwrap()
                .value
        );

        assert_eq!(
            owner2.nft.owner().call().await.unwrap().value,
            owner2.wallet.address()
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panic_when_not_approved() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        mint(&deploy_wallet, &owner1).await;

        assert!(
            owner2
                .nft
                .transfer(owner1.wallet.address(), owner2.wallet.address())
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, owner2) = setup().await;

        assert!(
            owner1
                .nft
                .transfer(owner1.wallet.address(), owner2.wallet.address())
                .call()
                .await
                .unwrap()
                .value
        );
    }
}