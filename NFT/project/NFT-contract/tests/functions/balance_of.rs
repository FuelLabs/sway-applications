use crate::utils::{
    abi_calls::{balance_of, constructor, mint},
    test_helpers::setup,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_balance_of_owned() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;

        assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
    }

    #[tokio::test]
    async fn gets_balance_of_multiple_owned() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 4).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(4, &owner1.contract, &minter).await;

        assert_eq!(balance_of(&owner1.contract, &minter).await, 4);
    }

    #[tokio::test]
    async fn gets_balance_none_owned() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;

        let not_minter = Identity::Address(owner2.wallet.address().into());
        assert_eq!(balance_of(&owner1.contract, &not_minter).await, 0);
    }

    #[tokio::test]
    async fn gets_balance_before_initalized() {
        let (_deploy_wallet, owner1, owner2) = setup().await;

        let balance_identity_1 = Identity::Address(owner1.wallet.address().into());
        assert_eq!(balance_of(&owner1.contract, &balance_identity_1).await, 0);

        let balance_identity_2 = Identity::Address(owner2.wallet.address().into());
        assert_eq!(balance_of(&owner1.contract, &balance_identity_2).await, 0);
    }
}
