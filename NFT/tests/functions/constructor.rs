use crate::utils::{
    abi_calls::{constructor, max_supply, total_supply},
    test_helpers::setup,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn initalizes_with_access_control() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(total_supply(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, 0);

        // let admin = Option::Some(Identity::Address(owner1.wallet.address().into()));
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        assert_eq!(total_supply(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn initalizes_without_access_control() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(total_supply(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, 0);

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;

        assert_eq!(total_supply(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, 1);
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_initalized_twice() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;
        constructor(true, &deploy_wallet.contract, &admin, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_token_supply_is_zero() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 0).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    #[ignore]
    async fn when_access_control_set_but_no_admin() {
        let (_deploy_wallet, _owner1, _owner2) = setup().await;

        // constructor(true, &deploy_wallet.contract, &Option::None(), 0).await;
    }
}
