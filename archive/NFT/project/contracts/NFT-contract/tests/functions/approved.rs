use crate::utils::{
    abi_calls::{approve, approved, constructor, mint},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_approval() {
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
    async fn gets_approval_multiple() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(3)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(3, &owner1.contract, minter.clone()).await;

        assert_eq!(approved(&owner1.contract, 0).await, None);
        assert_eq!(approved(&owner1.contract, 1).await, None);
        assert_eq!(approved(&owner1.contract, 2).await, None);

        let approved_identity = Some(Identity::Address(owner2.wallet.address().into()));
        approve(approved_identity.clone(), &owner1.contract, 0).await;
        approve(approved_identity.clone(), &owner1.contract, 1).await;
        approve(approved_identity.clone(), &owner1.contract, 2).await;

        assert_eq!(
            approved(&owner1.contract, 0).await,
            approved_identity.clone()
        );
        assert_eq!(
            approved(&owner1.contract, 1).await,
            approved_identity.clone()
        );
        assert_eq!(
            approved(&owner1.contract, 2).await,
            approved_identity.clone()
        );
    }

    #[tokio::test]
    async fn gets_approval_for_none() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(3)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;

        assert_eq!(approved(&owner1.contract, 0).await, None);
    }

    #[tokio::test]
    async fn gets_approval_for_non_existing_token() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        assert_eq!(approved(&owner1.contract, 0).await, None);
    }
}
