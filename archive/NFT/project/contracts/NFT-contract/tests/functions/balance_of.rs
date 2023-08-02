use crate::utils::{
    abi_calls::{balance_of, constructor, mint},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_balance_of_owned() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;

        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 1);
    }

    #[tokio::test]
    async fn gets_balance_of_multiple_owned() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(4)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(4, &owner1.contract, minter.clone()).await;

        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 4);
    }

    #[tokio::test]
    async fn gets_balance_none_owned() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;

        let not_minter = Identity::Address(owner2.wallet.address().into());
        assert_eq!(balance_of(&owner1.contract, not_minter.clone()).await, 0);
    }

    #[tokio::test]
    async fn gets_balance_before_initalized() {
        let (_deploy_wallet, owner1, owner2) = setup().await;

        let balance_identity_1 = Identity::Address(owner1.wallet.address().into());
        assert_eq!(
            balance_of(&owner1.contract, balance_identity_1.clone()).await,
            0
        );

        let balance_identity_2 = Identity::Address(owner2.wallet.address().into());
        assert_eq!(
            balance_of(&owner1.contract, balance_identity_2.clone()).await,
            0
        );
    }
}
