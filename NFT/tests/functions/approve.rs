use crate::utils::{
    abi_calls::{approve, approved, constructor, mint},
    test_helpers::setup,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn approves() {
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
    async fn approves_mutliple() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 4).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(4, &owner1.contract, &minter).await;

        // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address().into()));
        let approved_identity = Identity::Address(owner2.wallet.address().into());
        approve(&approved_identity, &owner1.contract, 0).await;
        assert_eq!(approved(&owner1.contract, 0).await, approved_identity);

        // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address().into()));
        let approved_identity = Identity::Address(owner2.wallet.address().into());
        approve(&approved_identity, &owner1.contract, 1).await;
        assert_eq!(approved(&owner1.contract, 1).await, approved_identity);

        // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address().into()));
        let approved_identity = Identity::Address(owner2.wallet.address().into());
        approve(&approved_identity, &owner1.contract, 2).await;
        assert_eq!(approved(&owner1.contract, 2).await, approved_identity);

        // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address().into()));
        let approved_identity = Identity::Address(owner2.wallet.address().into());
        approve(&approved_identity, &owner1.contract, 3).await;
        assert_eq!(approved(&owner1.contract, 3).await, approved_identity);
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_token_owner_does_not_exist() {
        let (_deploy_wallet, owner1, owner2) = setup().await;

        // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address().into()));
        let approved_identity = Identity::Address(owner2.wallet.address().into());
        approve(&approved_identity, &owner1.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_token_does_not_map_to_existing_token() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address().into()));
        let approved_identity = Identity::Address(owner2.wallet.address().into());
        approve(&approved_identity, &owner1.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;'
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;

        // let approved_identity = Option::Some(Identity::Address(owner2.wallet.address().into()));
        let approved_identity = Identity::Address(owner2.wallet.address().into());
        approve(&approved_identity, &owner2.contract, 0).await;
    }
}
