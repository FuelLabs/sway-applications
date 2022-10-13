use crate::utils::{
    abi_calls::{balance_of, constructor, max_supply, mint, owner_of, total_supply},
    test_helpers::setup,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn mints() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        assert_eq!(total_supply(&owner1.contract).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, 1);
        assert_eq!(balance_of(&owner1.contract, &minter).await, 0);

        mint(1, &owner1.contract, &minter).await;

        assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
        // assert_eq!(approved(&owner1.contract, 0).await, Option::None());
        assert_eq!(total_supply(&owner1.contract).await, 1);
        assert_eq!(max_supply(&owner1.contract).await, 1);
    }

    #[tokio::test]
    async fn mints_with_access() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        let minter = Identity::Address(owner1.wallet.address().into());
        // let admin = Option::Some(minter.clone());
        let admin = minter.clone();
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        assert_eq!(max_supply(&owner1.contract).await, 1);
        assert_eq!(total_supply(&owner1.contract).await, 0);
        assert_eq!(balance_of(&owner1.contract, &minter).await, 0);

        mint(1, &owner1.contract, &minter).await;

        assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
        // assert_eq!(approved(&owner1.contract, 0).await, Option::None());
        assert_eq!(total_supply(&owner1.contract).await, 1);
        assert_eq!(max_supply(&owner1.contract).await, 1);
    }

    #[tokio::test]
    async fn mints_multiple() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 4).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 4).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        assert_eq!(max_supply(&owner1.contract).await, 4);
        assert_eq!(total_supply(&owner1.contract).await, 0);
        assert_eq!(balance_of(&owner1.contract, &minter).await, 0);

        mint(4, &owner1.contract, &minter).await;

        assert_eq!(balance_of(&owner1.contract, &minter).await, 4);
        assert_eq!(total_supply(&owner1.contract).await, 4);
        assert_eq!(max_supply(&owner1.contract).await, 4);

        // assert_eq!(
        //     owner_of(&owner1.contract, 0).await,
        //     Option::Some(minter.clone())
        // );
        // assert_eq!(
        //     owner_of(&owner1.contract, 1).await,
        //     Option::Some(minter.clone())
        // );
        // assert_eq!(
        //     owner_of(&owner1.contract, 2).await,
        //     Option::Some(minter.clone())
        // );
        // assert_eq!(
        //     owner_of(&owner1.contract, 3).await,
        //     Option::Some(minter.clone())
        // );
        assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
        assert_eq!(owner_of(&owner1.contract, 1).await, minter.clone());
        assert_eq!(owner_of(&owner1.contract, 2).await, minter.clone());
        assert_eq!(owner_of(&owner1.contract, 3).await, minter.clone());
    }

    #[tokio::test]
    async fn mint_amount_is_zero() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, 1);
        assert_eq!(total_supply(&owner1.contract).await, 0);

        mint(0, &owner1.contract, &minter).await;

        assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
        assert_eq!(max_supply(&owner1.contract).await, 1);
        assert_eq!(total_supply(&owner1.contract).await, 0);
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_no_token_supply_set() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_minting_more_tokens_than_supply() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(
            max_supply(&owner1.contract).await + 1,
            &owner1.contract,
            &minter,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_minter_does_not_have_access() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        let minter = Identity::Address(owner2.wallet.address().into());
        // let admin = Option::Some(Identity::Address(owner1.wallet.address().into()));
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        mint(1, &owner2.contract, &minter).await;
    }
}
