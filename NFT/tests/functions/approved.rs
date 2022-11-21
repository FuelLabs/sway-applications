use crate::utils::{
    abi_calls::{approve, approved, constructor, mint},
    test_helpers::setup,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_approval() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;

        // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address().into()));
        let approved_identity = Identity::Address(owner2.wallet.address().into());
        approve(&approved_identity, &owner1.contract, 0).await;

        assert_eq!(approved(&owner1.contract, 0).await, approved_identity);
    }

    #[tokio::test]
    async fn gets_approval_multiple() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 3).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(3, &owner1.contract, &minter).await;

        // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address().into()));
        let approved_identity = Identity::Address(owner2.wallet.address().into());
        approve(&approved_identity, &owner1.contract, 0).await;
        approve(&approved_identity, &owner1.contract, 1).await;
        approve(&approved_identity, &owner1.contract, 2).await;

        assert_eq!(approved(&owner1.contract, 0).await, approved_identity);
        assert_eq!(approved(&owner1.contract, 1).await, approved_identity);
        assert_eq!(approved(&owner1.contract, 2).await, approved_identity);
    }

    #[tokio::test]
    #[ignore]
    async fn gets_approval_for_none() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;

        // assert_eq!(approved(&owner1.contract, 0).await, Option::None());
    }

    #[tokio::test]
    #[ignore]
    async fn gets_approval_for_non_existing_token() {
        let (_deploy_wallet, _owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;

        // assert_eq!(approved(&owner1.contract, 0).await, Option::None());
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn gets_approval_for_none() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;

        approved(&owner1.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn gets_approval_for_non_existing_token() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        approved(&owner1.contract, 0).await;
    }
}
