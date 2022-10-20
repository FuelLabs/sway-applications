use crate::utils::{
    abi_calls::{constructor, mint, owner_of},
    test_helpers::setup,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_owner_of() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
    }

    #[tokio::test]
    async fn gets_owner_of_multiple() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 2).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 2).await;

        let minter1 = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter1).await;

        let minter2 = Identity::Address(owner2.wallet.address().into());
        mint(1, &owner1.contract, &minter2).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter1.clone()));
        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter2.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, minter1.clone());
        assert_eq!(owner_of(&owner1.contract, 1).await, minter2.clone());
    }

    #[tokio::test]
    #[ignore]
    async fn gets_owner_of_none() {
        let (_deploy_wallet, _owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::None());
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn gets_owner_of_none() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        owner_of(&owner1.contract, 0).await;
    }
}
