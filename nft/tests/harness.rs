use fuels::prelude::*;
use fuels_abigen_macro::abigen;

// Load abi from json
abigen!(Nft, "out/debug/NFT-abi.json");

struct Metadata {
    nft: Nft,
    wallet: LocalWallet,
}

async fn setup() -> (Metadata, Metadata, Metadata) {
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

    let deploy_wallet = Metadata {
        nft: Nft::new(nft_id.to_string(), wallet1.clone()),
        wallet: wallet1.clone()
    };

    let owner1 = Metadata {
        nft: Nft::new(nft_id.to_string(), wallet2.clone()),
        wallet: wallet2.clone()
    };

    let owner2 = Metadata {
        nft: Nft::new(nft_id.to_string(), wallet3.clone()),
        wallet: wallet3.clone()
    };

    (deploy_wallet, owner1, owner2)
}

async fn init(
    deploy_wallet: &Metadata,
    owner: &Metadata,
    access_control: bool,
    token_supply: u64,
) -> bool {
    deploy_wallet
        .nft
        .constructor(
            nft_mod::Identity::Address(owner.wallet.address()), 
            access_control, 
            token_supply)
        .call()
        .await
        .unwrap()
        .value
}

mod constructor {

    use super::*;

    #[tokio::test]
    async fn initalizes() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert!(
            init(
                &deploy_wallet,
                &owner1,
                true,
                1
            )
            .await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_initalized_twice() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, true, 1).await;

        assert!(
            init(
                &deploy_wallet,
                &owner1,
                true,
                1
            )
            .await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_token_supply_is_zero() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert!(
            init(
                &deploy_wallet,
                &owner1,
                true,
                0
            )
            .await
        );
    }
}

mod mint {

    use super::*;

    #[tokio::test]
    async fn mints() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        assert!(
            owner1
                .nft
                .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
                .call()
                .await
                .unwrap()
                .value
        );

        assert_eq!(
            owner1
                .nft
                .balance_of(nft_mod::Identity::Address(owner1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            1
        );
    }

    #[tokio::test]
    async fn mints_with_access() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, true, 1).await;

        let _allowed_mint = owner1
            .nft
            .allow_mint(nft_mod::Identity::Address(owner1.wallet.address()), true)
            .call()
            .await;

        assert!(
            owner1
                .nft
                .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
                .call()
                .await
                .unwrap()
                .value
        );

        assert_eq!(
            owner1
                .nft
                .balance_of(nft_mod::Identity::Address(owner1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            1
        );
    }

    #[tokio::test]
    async fn mints_multiple() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 5).await;

        assert!(
            owner1
                .nft
                .mint(nft_mod::Identity::Address(owner1.wallet.address()), 3)
                .call()
                .await
                .unwrap()
                .value
        );

        assert_eq!(
            owner1
                .nft
                .balance_of(nft_mod::Identity::Address(owner1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            3
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        assert!(
            owner1
                .nft
                .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_mint_amount_is_zero() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        assert!(
            owner1
                .nft
                .mint(nft_mod::Identity::Address(owner1.wallet.address()), 0)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_minting_more_tokens_than_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        assert!(
            owner1
                .nft
                .mint(nft_mod::Identity::Address(owner1.wallet.address()), 2)
                .call()
                .await
                .unwrap()
                .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_minter_does_not_have_access() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, true, 1).await;

        assert!(
            owner1
                .nft
                .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
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
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, true, 1).await;

        assert! {
            owner1
            .nft
            .allow_mint(nft_mod::Identity::Address(owner1.wallet.address()), true)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        assert! {
            owner1
            .nft
            .allow_mint(nft_mod::Identity::Address(owner1.wallet.address()), true)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_access_control_not_set() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        assert! {
            owner1
            .nft
            .allow_mint(nft_mod::Identity::Address(owner1.wallet.address()), true)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_given_access_twice() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, true, 1).await;

        let _allowed_mint = owner1
            .nft
            .allow_mint(nft_mod::Identity::Address(owner1.wallet.address()), true)
            .call()
            .await;

        assert! {
            owner1
            .nft
            .allow_mint(nft_mod::Identity::Address(owner1.wallet.address()), true)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_access_control_address() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, true, 1).await;

        assert! {
            owner2
            .nft
            .allow_mint(nft_mod::Identity::Address(owner2.wallet.address()), true)
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
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!{
            owner1
            .nft
            .approve(nft_mod::Identity::Address(owner2.wallet.address()), token_id)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, owner2) = setup().await;

        let token_id = 0;

        assert!{
            owner1
            .nft
            .approve(nft_mod::Identity::Address(owner2.wallet.address()), token_id)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_approval_given_twice() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        let _approved = owner1
            .nft
            .approve(nft_mod::Identity::Address(owner2.wallet.address()), token_id)
            .call()
            .await;

        assert!{
            owner1
            .nft
            .approve(nft_mod::Identity::Address(owner2.wallet.address()), token_id)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!{
            owner2
            .nft
            .approve(nft_mod::Identity::Address(owner2.wallet.address()), token_id)
            .call()
            .await
            .unwrap()
            .value
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_approver_is_owner() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!{
            owner1
            .nft
            .approve(nft_mod::Identity::Address(owner1.wallet.address()), token_id)
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
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        assert_eq!(
            owner1
                .nft
                .balance_of(nft_mod::Identity::Address(owner1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            1
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(
            owner1
                .nft
                .balance_of(nft_mod::Identity::Address(owner1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );
    }
}

mod burn {

    use super::*;

    #[tokio::test]
    async fn burns() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

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
            owner1
                .nft
                .balance_of(nft_mod::Identity::Address(owner1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

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
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
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
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

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

// Uncommment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
// mod get_approved {

//     use super::*;

//     #[tokio::test]
//     async fn gets_approval() {
//         let (deploy_wallet, owner1, owner2) = setup().await;

//         init(&deploy_wallet, &owner1, false, 1).await;

//         let _minted = owner1
//             .nft
//             .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
//             .call()
//             .await;

//         let token_id = owner1
//             .nft
//             .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
//             .call()
//             .await
//             .unwrap()
//             .value;

//         let _approved = owner1
//             .nft
//             .approve(nft_mod::Identity::Address(owner2.wallet.address()), token_id)
//             .call()
//             .await;

//         assert_eq!(
//             owner1.nft.get_approved(token_id).call().await.unwrap().value,
//             owner2.wallet.address()
//         );
//     }

//     #[tokio::test]
//     #[should_panic]
//     async fn panics_when_not_initalized() {
//         let (_deploy_wallet, owner1, _owner2) = setup().await;
//         let token_id = 0;

//         assert_eq!(
//             owner1.nft.get_approved(token_id).call().await.unwrap().value,
//             owner1.wallet.address()
//         );
//     }
// }

mod get_tokens {

    use super::*;

    #[tokio::test]
    async fn gets_tokens() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(
            owner1
                .nft
                .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            token_id
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(
            owner1
                .nft
                .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );
    }
}

mod get_total_supply {

    use super::*;

    #[tokio::test]
    async fn gets_total_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 10).await;

        assert_eq!(
            owner1.nft.get_total_supply().call().await.unwrap().value,
            10
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

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
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _set_approval = owner1
            .nft
            .set_approval_for_all(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()))
            .call()
            .await;

        assert_eq!{
            owner1
                .nft
                .is_approved_for_all(
                    nft_mod::Identity::Address(owner1.wallet.address()), 
                    nft_mod::Identity::Address(owner2.wallet.address()))
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
        let (_deploy_wallet, owner1, owner2) = setup().await;

        assert_eq!{
            owner1
                .nft
                .is_approved_for_all(
                    nft_mod::Identity::Address(owner1.wallet.address()), 
                    nft_mod::Identity::Address(owner2.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            true
        };
    }
}

// Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
// mod owner_of {

//     use super::*;

//     #[tokio::test]
//     async fn gets_owner_of() {
//         let (deploy_wallet, owner1, _owner2) = setup().await;

//         init(&deploy_wallet, &owner1, false, 1).await;

//         let _minted = owner1
//             .nft
//             .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
//             .call()
//             .await;

//         let token_id = owner1
//             .nft
//             .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
//             .call()
//             .await
//             .unwrap()
//             .value;

//         assert_eq!(
//             owner1.nft.owner_of(token_id).call().await.unwrap().value,
//             owner1.wallet.address()
//         );
//     }

//     #[tokio::test]
//     #[should_panic]
//     async fn panics_when_not_initalized() {
//         let (_deploy_wallet, owner1, _owner2) = setup().await;
//         let token_id = 0;

//         assert_eq!(
//             owner1.nft.owner_of(token_id).call().await.unwrap().value,
//             owner1.wallet.address()
//         );
//     }
// }

mod set_approval_for_all {

    use super::*;

    #[tokio::test]
    async fn sets_approval_for_all() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        assert!(
            owner1
            .nft
            .set_approval_for_all(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()))
            .call()
            .await
            .unwrap()
            .value
        );

        assert_eq!(
            owner1
            .nft
            .is_approved_for_all(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()))
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
        let (_deploy_wallet, owner1, owner2) = setup().await;

        assert!(
            owner1
            .nft
            .set_approval_for_all(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()))
            .call()
            .await
            .unwrap()
            .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_approval_given_twice() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _approved = owner1
            .nft
            .set_approval_for_all(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()))
            .call()
            .await;

        assert!(
            owner1
            .nft
            .set_approval_for_all(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()))
            .call()
            .await
            .unwrap()
            .value
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        assert!(
            owner2
            .nft
            .set_approval_for_all(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()))
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
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!(
            owner1
            .nft
            .transfer_from(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()), 
                token_id)
            .call()
            .await
            .unwrap()
            .value
        );  

        // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     owner2
        //         .nft
        //         .owner_of(token_id)
        //         .call()
        //         .await
        //         .unwrap()
        //         .value,
        //     owner2.wallet.address()
        // );

        assert_eq!(
            owner1
                .nft
                .balance_of(nft_mod::Identity::Address(owner1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );

        assert_eq!(
            owner2
                .nft
                .balance_of(nft_mod::Identity::Address(owner2.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            1
        );

        assert_eq!(
            owner2
                .nft
                .get_tokens(nft_mod::Identity::Address(owner2.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            token_id
        );

        assert_eq!(
            owner1
                .nft
                .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
                .call()
                .await
                .unwrap()
                .value,
            0
        );
    }

    #[tokio::test]
    async fn transfers_by_approval() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        let _approved = owner1
            .nft
            .approve(nft_mod::Identity::Address(owner2.wallet.address()), token_id)
            .call()
            .await;

        assert!(
            owner2
            .nft
            .transfer_from(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()), 
                token_id)
            .call()
            .await
            .unwrap()
            .value
        );  

        // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     owner2
        //         .nft
        //         .owner_of(token_id)
        //         .call()
        //         .await
        //         .unwrap()
        //         .value,
        //     owner2.wallet.address()
        // );
    }

    #[tokio::test]
    async fn transfers_by_operator() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        let _set_approval = owner1
            .nft
            .set_approval_for_all(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()))
            .call()
            .await;

        assert!(
            owner2
            .nft
            .transfer_from(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()), 
                token_id)
            .call()
            .await
            .unwrap()
            .value
        );  

        // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
        // assert_eq!(
        //     owner2
        //         .nft
        //         .owner_of(token_id)
        //         .call()
        //         .await
        //         .unwrap()
        //         .value,
        //     owner2.wallet.address()
        // );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, owner2) = setup().await;
        let token_id = 0;

        assert!(
            owner1
            .nft
            .transfer_from(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()), 
                token_id)
            .call()
            .await
            .unwrap()
            .value
        );  
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        let _minted = owner1
            .nft
            .mint(nft_mod::Identity::Address(owner1.wallet.address()), 1)
            .call()
            .await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!(
            owner2
            .nft
            .transfer_from(
                nft_mod::Identity::Address(owner1.wallet.address()), 
                nft_mod::Identity::Address(owner2.wallet.address()), 
                token_id)
            .call()
            .await
            .unwrap()
            .value
        );  
    }
}
