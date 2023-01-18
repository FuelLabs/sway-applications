use crate::utils::{
    abi_calls::{admin, constructor, max_supply, tokens_minted},
    test_helpers::setup,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn initalizes_with_access_control_and_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, None);

        let admin_identity = Some(Identity::Address(owner1.wallet.address().into()));
        constructor(admin_identity.clone(), &deploy_wallet.contract, Some(1)).await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, Some(1));
        assert_eq!(admin(&owner1.contract).await, admin_identity.clone());
    }

    #[tokio::test]
    async fn initalizes_without_access_control() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, None);

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, Some(1));
        assert_eq!(admin(&owner1.contract).await, None);
    }

    #[tokio::test]
    async fn initalizes_without_max_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, None);

        let admin_identity = Some(Identity::Address(owner1.wallet.address().into()));
        constructor(admin_identity.clone(), &deploy_wallet.contract, None).await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, admin_identity.clone());
    }

    #[tokio::test]
    async fn initalizes_without_access_control_and_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, None);

        constructor(None, &deploy_wallet.contract, None).await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, None);
        assert_eq!(admin(&owner1.contract).await, None);
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CannotReinitialize")]
    async fn when_initalized_twice() {
        let (deploy_wallet, _owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;
        constructor(None, &deploy_wallet.contract, Some(1)).await;
    }
}
