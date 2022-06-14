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
    let mut wallets = launch_provider_and_get_wallets(WalletsConfig {
        num_wallets: 3,
        coins_per_wallet: 1,
        coin_amount: 1000000,
    })
    .await;

    // Get the wallets from that provider
    let wallet1 = wallets.pop().unwrap();
    let wallet2 = wallets.pop().unwrap();
    let wallet3 = wallets.pop().unwrap();

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
    let response = deploy_wallet
        .nft
        .constructor(
            nft_mod::Identity::Address(owner.wallet.address()), 
            access_control, 
            token_supply)
        .call()
        .await;

    match response {
        Ok(_call_response) => true,
        Err(Error::ContractCallError(reason, receipts)) => {
            println!("ContractCall failed with reason: {}", reason);
            println!("Transaction receipts are: {:?}", receipts);
            false
        },
        _ => false
    }
}

async fn mint(
    mint_wallet: &Metadata,
    owner: &Metadata,
    amount: u64
)  -> bool {
    let response = mint_wallet
        .nft
        .mint(nft_mod::Identity::Address(owner.wallet.address()), amount)
        .call()
        .await;

    match response {
        Ok(_call_response) => true,
        Err(Error::ContractCallError(reason, receipts)) => {
            println!("ContractCall failed with reason: {}", reason);
            println!("Transaction receipts are: {:?}", receipts);
            false
        },
        _ => false
    }
}

async fn allow_mint(
    call_wallet: &Metadata,
    minter: &Metadata,
    allow: bool
) -> bool {
    let response = call_wallet
        .nft
        .allow_mint(nft_mod::Identity::Address(minter.wallet.address()), allow)
        .call()
        .await;

    match response {
        Ok(_call_response) => true,
        Err(Error::ContractCallError(reason, receipts)) => {
            println!("ContractCall failed with reason: {}", reason);
            println!("Transaction receipts are: {:?}", receipts);
            false
        },
        _ => false
    }
}

async fn approve(
    call_wallet: &Metadata,
    approve: &Metadata,
    token_id: u64
) -> bool {
    let response = call_wallet
        .nft
        .approve(nft_mod::Identity::Address(approve.wallet.address()), token_id)
        .call()
        .await;

    match response {
        Ok(_call_response) => true,
        Err(Error::ContractCallError(reason, receipts)) => {
            println!("ContractCall failed with reason: {}", reason);
            println!("Transaction receipts are: {:?}", receipts);
            false
        },
        _ => false
    }
}

async fn burn(
    call_wallet: &Metadata,
    token_id: u64
) -> bool {
    let response = call_wallet
        .nft
        .burn(token_id)
        .call()
        .await;

    match response {
        Ok(_call_response) => true,
        Err(Error::ContractCallError(reason, receipts)) => {
            println!("ContractCall failed with reason: {}", reason);
            println!("Transaction receipts are: {:?}", receipts);
            false
        },
        _ => false
    }  
}

async fn set_approval_for_all(
    call_wallet: &Metadata,
    owner: &Metadata,
    operator: &Metadata
) -> bool {
    let response = call_wallet
        .nft
        .set_approval_for_all(
            nft_mod::Identity::Address(owner.wallet.address()), 
            nft_mod::Identity::Address(operator.wallet.address()))
        .call()
        .await;

    match response {
        Ok(_call_response) => true,
        Err(Error::ContractCallError(reason, receipts)) => {
            println!("ContractCall failed with reason: {}", reason);
            println!("Transaction receipts are: {:?}", receipts);
            false
        },
        _ => false
    }  
}

async fn transfer(
    call_wallet: &Metadata,
    from: &Metadata,
    to: &Metadata,
    token_id: u64
) -> bool {
    let response = call_wallet
        .nft
        .transfer_from(
            nft_mod::Identity::Address(from.wallet.address()), 
            nft_mod::Identity::Address(to.wallet.address()), 
            token_id)
        .call()
        .await;

    match response {
        Ok(_call_response) => true,
        Err(Error::ContractCallError(reason, receipts)) => {
            println!("ContractCall failed with reason: {}", reason);
            println!("Transaction receipts are: {:?}", receipts);
            false
        },
        _ => false
    } 
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
            mint(&owner1, &owner1, 1).await
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

        allow_mint(&owner1, &owner1, true).await;

        assert!(
            mint(&owner1, &owner1, 1).await
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
            mint(&owner1, &owner1, 3).await
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
            mint(&owner1, &owner1, 1).await
        );
    }

    #[tokio::test]
    async fn does_not_mint_when_not_initalized() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        assert!(
            mint(&owner1, &owner1, 0).await
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
    async fn panics_when_minting_more_tokens_than_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        assert!(
            mint(&owner1, &owner1, 2).await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_minter_does_not_have_access() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, true, 1).await;

        assert!(
            mint(&owner1, &owner1, 1).await
        );
    }
}

mod allow_mint {

    use super::*;

    #[tokio::test]
    async fn allows_mint() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, true, 1).await;

        assert! (
            allow_mint(&owner1, &owner1, true).await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        assert! (
            allow_mint(&owner1, &owner1, true).await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_access_control_not_set() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        assert! (
            allow_mint(&owner1, &owner1, true).await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_access_control_address() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, true, 1).await;

        assert! (
            allow_mint(&owner2, &owner1, true).await
        );
    }
}

mod approve {

    use super::*;

    #[tokio::test]
    async fn approves() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        mint(&owner1, &owner1, 1).await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!{
            approve(&owner1, &owner2, token_id).await
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_not_initalized() {
        let (_deploy_wallet, owner1, owner2) = setup().await;
        let token_id = 0;

        assert!{
            approve(&owner1, &owner2, token_id).await
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_approval_given_twice() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        mint(&owner1, &owner1, 1).await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        approve(&owner1, &owner2, token_id).await;

        assert!{
            approve(&owner1, &owner2, token_id).await
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        mint(&owner1, &owner1, 1).await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!{
            approve(&owner2, &owner2, token_id).await
        };
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_approver_is_owner() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        mint(&owner1, &owner1, 1).await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!{
            approve(&owner1, &owner1, token_id).await
        };
    }
}

mod balance_of {

    use super::*;

    #[tokio::test]
    async fn gets_balance() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        mint(&owner1, &owner1, 1).await;

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
}

mod burn {

    use super::*;

    #[tokio::test]
    async fn burns() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        mint(&owner1, &owner1, 1).await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!(
            burn(&owner1, token_id).await
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
            burn(&owner1, token_id).await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_token_does_not_exist() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        mint(&owner1, &owner1, 1).await;

        let token_id = 2;

        assert!(
            burn(&owner1, token_id).await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        mint(&owner1, &owner1, 1).await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!(
            burn(&owner2, token_id).await
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
//         mint(&owner1, &owner1, 1).await;

//         let token_id = owner1
//             .nft
//             .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
//             .call()
//             .await
//             .unwrap()
//             .value;

//         approve(&owner1, &owner2, token_id).await;

//         assert_eq!(
//             owner1.nft.get_approved(token_id).call().await.unwrap().value,
//             owner2.wallet.address()
//         );
//     }
// }

mod get_tokens {

    use super::*;

    #[tokio::test]
    async fn gets_tokens() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        mint(&owner1, &owner1, 1).await;

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
}

mod is_approved_for_all {

    use super::*;

    #[tokio::test]
    async fn gets_approval_for_all() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        set_approval_for_all(&owner1, &owner1, &owner2).await;

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
//         mint(&owner1, &owner1, 1).await;

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
// }

mod set_approval_for_all {

    use super::*;

    #[tokio::test]
    async fn sets_approval_for_all() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        assert!(
            set_approval_for_all(&owner1, &owner1, &owner2).await
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
    async fn panics_when_approval_given_twice() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        set_approval_for_all(&owner1, &owner1, &owner2).await;

        assert!(
            set_approval_for_all(&owner1, &owner1, &owner2).await
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;

        assert!(
            set_approval_for_all(&owner2, &owner1, &owner2).await
        );
    }
}

mod transfer_from {

    use super::*;

    #[tokio::test]
    async fn transfers() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        mint(&owner1, &owner1, 1).await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!(
            transfer(&owner1, &owner1, &owner2, token_id).await
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
        mint(&owner1, &owner1, 1).await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        approve(&owner1, &owner2, token_id).await;

        assert!(
            transfer(&owner2, &owner1, &owner2, token_id).await
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
        mint(&owner1, &owner1, 1).await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        set_approval_for_all(&owner1, &owner1, &owner2).await;

        assert!(
            transfer(&owner2, &owner1, &owner2, token_id).await
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
            transfer(&owner1, &owner1, &owner2, token_id).await
        );  
    }

    #[tokio::test]
    #[should_panic]
    async fn panics_when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        init(&deploy_wallet, &owner1, false, 1).await;
        mint(&owner1, &owner1, 1).await;

        let token_id = owner1
            .nft
            .get_tokens(nft_mod::Identity::Address(owner1.wallet.address()))
            .call()
            .await
            .unwrap()
            .value;

        assert!(
            transfer(&owner2, &owner1, &owner2, token_id).await
        );  
    }
}
