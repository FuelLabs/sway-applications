use crate::utils::{
    abi_calls::{approve, approved, constructor, mint},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn approves() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;

        assert_eq!(approved(&owner1.contract, 0).await, None);

        let approved_identity = Some(Identity::Address(owner2.wallet.address().into()));
        approve(approved_identity.clone(), &owner1.contract, 0).await;

        assert_eq!(
            approved(&owner1.contract, 0).await,
            approved_identity.clone()
        );
    }

    #[tokio::test]
    async fn approves_mutliple() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(4)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(4, &owner1.contract, minter.clone()).await;

        let approved_identity = Some(Identity::Address(owner2.wallet.address().into()));
        assert_eq!(approved(&owner1.contract, 0).await, None);
        approve(approved_identity.clone(), &owner1.contract, 0).await;
        assert_eq!(
            approved(&owner1.contract, 0).await,
            approved_identity.clone()
        );

        let approved_identity = Some(Identity::Address(owner2.wallet.address().into()));
        assert_eq!(approved(&owner1.contract, 1).await, None);
        approve(approved_identity.clone(), &owner1.contract, 1).await;
        assert_eq!(
            approved(&owner1.contract, 1).await,
            approved_identity.clone()
        );

        let approved_identity = Some(Identity::Address(owner2.wallet.address().into()));
        assert_eq!(approved(&owner1.contract, 2).await, None);
        approve(approved_identity.clone(), &owner1.contract, 2).await;
        assert_eq!(
            approved(&owner1.contract, 2).await,
            approved_identity.clone()
        );

        let approved_identity = Some(Identity::Address(owner2.wallet.address().into()));
        assert_eq!(approved(&owner1.contract, 3).await, None);
        approve(approved_identity.clone(), &owner1.contract, 3).await;
        assert_eq!(
            approved(&owner1.contract, 3).await,
            approved_identity.clone()
        );
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "TokenDoesNotExist")]
    async fn when_token_does_not_map_to_existing_token() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let approved_identity = Some(Identity::Address(owner2.wallet.address().into()));
        approve(approved_identity.clone(), &owner1.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "SenderNotOwner")]
    async fn when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;

        let approved_identity = Some(Identity::Address(owner2.wallet.address().into()));
        approve(approved_identity.clone(), &owner2.contract, 0).await;
    }
}
