use fuels::{prelude::*, tx::ContractId};
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(Nft, "out/debug/NFT-abi.json");
abigen!(Asset, "tests/artifacts/asset/out/debug/asset-abi.json");
/*
struct Metadata {
    asset: Option<Asset>,
    nft: Nft,
    wallet: LocalWallet,
    entity: Entity,
}

async fn setup() -> (Metadata, Metadata, Metadata, ContractId) {
    // Setup 3 test wallets
    let wallets = launch_provider_and_get_wallets(WalletsConfig {
        num_wallets: 3,
        coins_per_wallet: 1,
        coin_amount: 1000000,
    })
    .await;

    // Get the wallets from that provider
    let wallet1 = &wallets[0];
    let wallet2 = &wallets[1];
    let wallet3 = &wallets[2];

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
        wallet: wallet1.clone(),
        entity: Entity {
            address: wallet1.address(),
            contract_id: ContractId {
                value: 0
            },
            identity: 1
        }
    };

    let owner1 = Metadata {
        asset: Some(Asset::new(asset_id.to_string(), wallet2.clone())),
        nft: Nft::new(nft_id.to_string(), wallet2.clone()),
        wallet: wallet2.clone(),
        entity: Entity {
            address: wallet2.address(),
            contract_id: ContractId {
                value: 0
            },
            identity: 1
        }
    };

    let owner2 = Metadata {
        asset: Some(Asset::new(asset_id.to_string(), wallet3.clone())),
        nft: Nft::new(nft_id.to_string(), wallet3.clone()),
        wallet: wallet3.clone(),
        entity: Entity {
            address: wallet3.address(),
            contract_id: ContractId {
                value: 0
            },
            identity: 1
        }
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
        .constructor(owner.entity, access_control, token_supply, token_price, asset)
        .call()
        .await
        .unwrap()
        .value
}

async fn deploy_funds(
    deploy_wallet: &Metadata,
    owner: &LocalWallet,
    asset_amount: u64
) {
    deploy_wallet
        .asset
        .as_ref()
        .unwrap()
        .mint_and_send_to_address(asset_amount, owner.address())
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
        .value;
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

mod mint {

    // TODO: Need test for to entity being valid

    use super::*;

    #[tokio::test]
    async fn mints() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        assert!(
            owner1
                .nft
                .mint(owner1.entity, 1)
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );

        assert_eq!(
            owner1.nft.balance_of(owner1.entity).call().await.unwrap().value,
            1
        );
    }

    #[tokio::test]
    async fn mints_with_access() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, true, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let _allowed_mint = owner1.nft.allow_mint(owner1.entity).call().await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        assert!(
            owner1
                .nft
                .mint(owner1.entity, 1)
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );

        assert_eq!(
            owner1.nft.balance_of(owner1.entity).call().await.unwrap().value,
            1
        );
    }

    #[tokio::test]
    async fn mints_multiple() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 5, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 3).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(3), Some(AssetId::from(*asset_id)));

        assert!(
            owner1
                .nft
                .mint(owner1.entity, 3)
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );

        assert_eq!(
            owner1.nft.balance_of(owner1.entity).call().await.unwrap().value,
            3
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        assert!(
            owner1
                .nft
                .mint(owner1.entity, 1)
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_mint_amount_is_zero() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        assert!(
            owner1
                .nft
                .mint(owner1.entity, 0)
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_minting_more_tokens_than_supply() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        assert!(
            owner1
                .nft
                .mint(owner1.entity, 2)
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_minter_does_not_have_access() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, true, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        assert!(
            owner1
                .nft
                .mint(owner1.entity, 1)
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/pull/305 (deploy_with_salt) lands in a new release
    // #[tokio::test]
    // #[should_panic]
    // async fn panics_when_paying_incorrect_asset() {
    //     let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

    //     init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        
    //     let another_asset_id = Contract::deploy_with_salt(
    //         "./tests/artifacts/asset/out/debug/asset.bin",
    //         &deployer.wallet,
    //         TxParameters::default(),
    //         Salt::from([1u8; 32]),
    //     )
    //     .await
    //     .unwrap();

    //     let another_asset = Asset::new(another_asset_id.to_string(), deployer.wallet.clone());

    //     let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
    //     let call_params = CallParameters::new(Some(asset_amount), Some(AssetId::from(*another_asset_id)));

    //     another_asset
    //         .mint_and_send_to_address(asset_amount, buyer.wallet.address())
    //         .append_variable_outputs(1)
    //         .call()
    //         .await
    //         .unwrap();
  
    //     assert!(
    //         owner1
    //             .nft
    //             .mint(owner1.entity, 1)
    //             .tx_params(tx_params)
    //             .call_params(call_params)
    //             .call()
    //             .await
    //             .unwrap()
    //             .value
    //     );
    // }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_paying_incorrect_asset_amount() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 2, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        assert!(
            owner1
                .nft
                .mint(owner1.entity, 1)
                .tx_params(tx_params)
                .call_params(call_params)
                .call()
                .await
                .unwrap()
                .value
        );
    }
}

mod allow_mint {

    use super::*;

    #[tokio::test]
    async fn allows_mint() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, true, 1, 1, asset_id).await;

        assert! {
            owner1
            .nft
            .allow_mint(owner1.entity)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2, _asset_id) = setup().await;

        assert! {
            owner1
            .nft
            .allow_mint(owner1.entity)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_access_control_not_set() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;

        assert! {
            owner1
            .nft
            .allow_mint(owner1.entity)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_given_access_twice() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, true, 1, 1, asset_id).await;

        let _allowed_mint = owner1.nft.allow_mint(owner1.wallet.address()).call().await;

        assert! {
            owner1
            .nft
            .allow_mint(owner1.entity)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_access_control_address() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, true, 1, 1, asset_id).await;

        assert! {
            owner2
            .nft
            .allow_mint(owner2.entity)
            .call()
            .await
            .unwrap()
            .value
        };
    }
}

mod approve {

    use super::*;

    #[tokio::test]
    async fn approves() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        assert!{
            owner1
            .nft
            .approve(owner2.entity, token_id)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, owner2, _asset_id) = setup().await;

        let token_id = 0;

        assert!{
            owner1
            .nft
            .approve(owner2.entity, token_id)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_approval_given_twice() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        let _approved = owner1.nft.approve(owner2.entity, token_id).call().await;

        assert!{
            owner1
            .nft
            .approve(owner2.entity, token_id)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        assert!{
            owner2
            .nft
            .approve(owner2.entity, token_id)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_approver_is_owner() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        assert!{
            owner1
            .nft
            .approve(owner1.entity, token_id)
            .call()
            .await
            .unwrap()
            .value
        };
    }
}

mod balance_of {

    use super::*;

    #[tokio::test]
    async fn gets_balance() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        assert_eq!(
            owner1.nft.balance_of(owner1.entity).call().await.unwrap().value,
            1
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2, _asset_id) = setup().await;

        assert_eq!(
            owner1.nft.balance_of(owner1.entity).call().await.unwrap().value,
            0
        );
    }
}

mod burn {

    use super::*;

    #[tokio::test]
    async fn burns() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        assert!(
            owner1
            .nft
            .burn(token_id)
            .call()
            .await
            .unwrap()
            .value
        );

        assert_eq!(
            owner1.nft.balance_of(owner1.entity).call().await.unwrap().value,
            0
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2, _asset_id) = setup().await;

        let token_id = 0;

        assert!(
            owner1
            .nft
            .burn(token_id)
            .call()
            .await
            .unwrap()
            .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_token_does_not_exist() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = 2;

        assert!(
            owner1
            .nft
            .burn(token_id)
            .call()
            .await
            .unwrap()
            .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        assert!(
            owner2
            .nft
            .burn(token_id)
            .call()
            .await
            .unwrap()
            .value
        );
    }
}

mod get_approved {

    use super::*;

    #[tokio::test]
    async fn gets_approval() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        let _approved = owner1
            .nft
            .approve(owner2.entity, token_id)
            .call()
            .await;

        assert_eq!(
            owner1.nft.get_approved(token_id).call().await.unwrap().value,
            owner2.entity
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2, _asset_id) = setup().await;
        let token_id = 0;

        assert_eq!(
            owner1.nft.get_approved(token_id).call().await.unwrap().value,
            owner1.wallet.address()
        );
    }
}

mod get_tokens {

    use super::*;

    #[tokio::test]
    async fn gets_tokens() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.wallet.address(), 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        assert_eq!(
            owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value,
            token_id
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2, _asset_id) = setup().await;

        assert_eq!(
            owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value,
            0
        );
    }
}

mod get_total_supply {

    use super::*;

    #[tokio::test]
    async fn gets_total_supply() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 10, 1, asset_id).await;

        assert_eq!(
            owner1.nft.get_total_supply().call().await.unwrap().value,
            10
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2, _asset_id) = setup().await;

        assert_eq!(
            owner1.nft.get_total_supply().call().await.unwrap().value,
            0
        );
    }
}

mod is_approved_for_all {

    use super::*;

    #[tokio::test]
    async fn gets_approval_for_all() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;

        let _set_approval = owner1
            .nft
            .set_approval_for_all(owner1.entity, owner2.wallet.address())
            .call()
            .await;

        assert_eq!{
            owner1
                .nft
                .is_approved_for_all(owner1.entity, owner2.wallet.address())
                .call()
                .await
                .unwrap()
                .value,
            true
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, owner2, _asset_id) = setup().await;

        assert_eq!{
            owner1
                .nft
                .is_approved_for_all(owner1.entity, owner2.entity)
                .call()
                .await
                .unwrap()
                .value,
            true
        };
    }
}

mod owner_of {

    use super::*;

    #[tokio::test]
    async fn gets_owner_of() {
        let (deploy_wallet, owner1, _owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        assert_eq!(
            owner1.nft.owner_of(token_id).call().await.unwrap().value,
            owner1.entity
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2, _asset_id) = setup().await;
        let token_id = 0;

        assert_eq!(
            owner1.nft.owner_of(token_id).call().await.unwrap().value,
            owner1.entity
        );
    }
}

mod set_approval_for_all {

    use super::*;

    #[tokio::test]
    async fn sets_approval_for_all() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;

        assert!(
            owner1
            .nft
            .set_approval_for_all(owner1.entity, owner2.entity)
            .call()
            .await
            .unwrap()
            .value
        );

        assert_eq!(
            owner1
            .nft
            .is_approved_for_all(owner1.entity, owner2.entity)
            .call()
            .await
            .unwrap()
            .value,
            true
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, owner2, _asset_id) = setup().await;

        assert!(
            owner1
            .nft
            .set_approval_for_all(owner1.entity, owner2.entity)
            .call()
            .await
            .unwrap()
            .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_approval_given_twice() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;

        let _minted = owner1
            .nft
            .set_approval_for_all(owner1.entity, owner2.entity)
            .call()
            .await;

        assert!(
            owner1
            .nft
            .set_approval_for_all(owner1.entity, owner2.entity)
            .call()
            .await
            .unwrap()
            .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;

        assert!(
            owner2
            .nft
            .set_approval_for_all(owner1.entity, owner2.entity)
            .call()
            .await
            .unwrap()
            .value
        );
    }
}

mod transfer_from {

    use super::*;

    #[tokio::test]
    async fn transfers() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        assert!(
            owner1
            .nft
            .transfer_from(owner1.entity, owner2.entity, token_id)
            .call()
            .await
            .unwrap()
            .value
        );  

        assert_eq!(
            owner2
                .nft
                .owner_of(token_id)
                .call()
                .await
                .unwrap()
                .value,
            owner2.entity
        );

        assert_eq!(
            owner1
                .nft
                .balance_of(owner1.entity)
                .call()
                .await
                .unwrap()
                .value,
            0
        );

        assert_eq!(
            owner2
                .nft
                .balance_of(owner2.entity)
                .call()
                .await
                .unwrap()
                .value,
            1
        );

        assert_eq!(
            owner2
                .nft
                .get_tokens(owner2.entity)
                .call()
                .await
                .unwrap()
                .value,
            token_id
        );

        assert_eq!(
            owner1
                .nft
                .get_tokens(owner1.entity)
                .call()
                .await
                .unwrap()
                .value,
            0
        );
    }

    #[tokio::test]
    async fn transfers_by_approval() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        let _approved = owner1.nft.approve(owner2.entity, token_id).call().await;

        assert!(
            owner2
            .nft
            .transfer_from(owner1.entity, owner2.entity, token_id)
            .call()
            .await
            .unwrap()
            .value
        );  

        assert_eq!(
            owner2
                .nft
                .owner_of(token_id)
                .call()
                .await
                .unwrap()
                .value,
            owner2.entity
        );
    }

    #[tokio::test]
    async fn transfers_by_operator() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        let _set_approval = owner1
            .nft
            .set_approval_for_all(owner1.entity, owner2.entity)
            .call()
            .await;

        assert!(
            owner2
            .nft
            .transfer_from(owner1.entity, owner2.entity, token_id)
            .call()
            .await
            .unwrap()
            .value
        );  

        assert_eq!(
            owner2
                .nft
                .owner_of(token_id)
                .call()
                .await
                .unwrap()
                .value,
            owner2.entity
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, owner2, _asset_id) = setup().await;
        let token_id = 0;

        assert!(
            owner1
            .nft
            .transfer_from(owner1.entity, owner2.entity, token_id)
            .call()
            .await
            .unwrap()
            .value
        );  
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2, asset_id) = setup().await;

        init(&deploy_wallet, &owner1, false, 1, 1, asset_id).await;
        deploy_funds(&deploy_wallet, &owner1.wallet, 1).await;

        let tx_params = TxParameters::new(None, Some(1_000_000), None, None);
        let call_params = CallParameters::new(Some(1), Some(AssetId::from(*asset_id)));

        let _minted = owner1
            .nft
            .mint(owner1.entity, 1)
            .tx_params(tx_params)
            .call_params(call_params)
            .call()
            .await;

        let token_id = owner1.nft.get_tokens(owner1.entity).call().await.unwrap().value;

        assert!(
            owner2
            .nft
            .transfer_from(owner1.entity, owner2.entity, token_id)
            .call()
            .await
            .unwrap()
            .value
        );  
    }
}*/
