use crate::utils::{
    abi_calls::{constructor, mint, owner_of},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_owner_of() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;

        assert_eq!(owner_of(&owner1.contract, 0).await, Some(minter.clone()));
    }

    #[tokio::test]
    async fn gets_owner_of_multiple() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(2)).await;

        let minter1 = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter1.clone()).await;

        let minter2 = Identity::Address(owner2.wallet.address().into());
        mint(1, &owner1.contract, minter2.clone()).await;

        assert_eq!(owner_of(&owner1.contract, 0).await, Some(minter1.clone()));
        assert_eq!(owner_of(&owner1.contract, 1).await, Some(minter2.clone()));
    }

    #[tokio::test]
    async fn gets_owner_of_none() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        assert_eq!(owner_of(&owner1.contract, 0).await, None);
    }
}
