use crate::utils::{
    abi_calls::{
        approve, balance_of, constructor, mint, owner_of, set_approval_for_all, transfer_from,
    },
    test_helpers::setup,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use super::*;

    #[tokio::test]
    async fn transfers() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        let to = Identity::Address(owner2.wallet.address().into());

        mint(1, &owner1.contract, &minter).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
        assert_eq!(balance_of(&owner2.contract, &to).await, 0);

        transfer_from(&owner1.contract, &minter, &to, 0).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
        assert_eq!(balance_of(&owner2.contract, &to).await, 1);
    }

    #[tokio::test]
    async fn transfers_by_approval() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        let to = Identity::Address(owner2.wallet.address().into());
        // let approved_identity = Option::Some(to.clone());
        let approved_identity = to.clone();

        mint(1, &owner1.contract, &minter).await;

        approve(&approved_identity, &owner1.contract, 0).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
        assert_eq!(balance_of(&owner2.contract, &to).await, 0);

        transfer_from(&owner2.contract, &minter, &to, 0).await;

        assert_eq!(owner_of(&owner1.contract, 0).await, approved_identity);
        assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
        assert_eq!(balance_of(&owner2.contract, &to).await, 1);
    }

    #[tokio::test]
    async fn transfers_by_operator() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        let operator = Identity::Address(owner2.wallet.address().into());

        mint(1, &owner1.contract, &minter).await;

        set_approval_for_all(true, &owner1.contract, &operator).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
        assert_eq!(balance_of(&owner2.contract, &operator).await, 0);

        transfer_from(&owner2.contract, &minter, &operator, 0).await;

        // assert_eq!(
        //     owner_of(&owner1.contract, 0).await,
        //     Option::Some(operator.clone())
        // );
        assert_eq!(owner_of(&owner1.contract, 0).await, operator.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
        assert_eq!(balance_of(&owner2.contract, &operator).await, 1);
    }

    #[tokio::test]
    async fn transfers_multiple() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 4).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        let to = Identity::Address(owner2.wallet.address().into());

        mint(4, &owner1.contract, &minter).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(minter.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, minter.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 4);
        assert_eq!(balance_of(&owner2.contract, &to).await, 0);

        transfer_from(&owner1.contract, &minter, &to, 0).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 3);
        assert_eq!(balance_of(&owner2.contract, &to).await, 1);

        transfer_from(&owner1.contract, &minter, &to, 1).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 2);
        assert_eq!(balance_of(&owner2.contract, &to).await, 2);

        transfer_from(&owner1.contract, &minter, &to, 2).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 1);
        assert_eq!(balance_of(&owner2.contract, &to).await, 3);

        transfer_from(&owner1.contract, &minter, &to, 3).await;

        // assert_eq!(owner_of(&owner1.contract, 0).await, Option::Some(to.clone()));
        assert_eq!(owner_of(&owner1.contract, 0).await, to.clone());
        assert_eq!(balance_of(&owner1.contract, &minter).await, 0);
        assert_eq!(balance_of(&owner2.contract, &to).await, 4);
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_token_does_not_exist() {
        let (_deploy_wallet, owner1, owner2) = setup().await;

        let from = Identity::Address(owner1.wallet.address().into());
        let to = Identity::Address(owner2.wallet.address().into());
        transfer_from(&owner1.contract, &from, &to, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_sender_is_not_owner_or_approved() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 1).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 1).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;

        let to = Identity::Address(owner2.wallet.address().into());
        transfer_from(&owner2.contract, &minter, &to, 0).await;
    }
}
