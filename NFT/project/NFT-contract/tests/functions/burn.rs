use crate::utils::{
    abi_calls::{balance_of, burn, constructor, mint, tokens_minted},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn burns() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;

        assert_eq!(tokens_minted(&owner1.contract).await, 1);
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 1);

        burn(&owner1.contract, 0).await;

        assert_eq!(tokens_minted(&owner1.contract).await, 1);
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 0);
    }

    #[tokio::test]
    async fn burns_multiple() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(4)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(4, &owner1.contract, minter.clone()).await;

        assert_eq!(tokens_minted(&owner1.contract).await, 4);
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 4);

        burn(&owner1.contract, 0).await;
        assert_eq!(tokens_minted(&owner1.contract).await, 4);
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 3);

        burn(&owner1.contract, 1).await;
        assert_eq!(tokens_minted(&owner1.contract).await, 4);
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 2);

        burn(&owner1.contract, 2).await;
        assert_eq!(tokens_minted(&owner1.contract).await, 4);
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 1);

        burn(&owner1.contract, 3).await;
        assert_eq!(tokens_minted(&owner1.contract).await, 4);
        assert_eq!(balance_of(&owner1.contract, minter.clone()).await, 0);
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "TokenDoesNotExist")]
    async fn when_token_owner_does_not_exist() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        burn(&owner1.contract, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "TokenDoesNotExist")]
    async fn when_token_does_not_exist() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;

        burn(&owner1.contract, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "NoContractAdmin")]
    async fn when_sender_is_not_owner() {
        let (deploy_wallet, owner1, owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;

        burn(&owner2.contract, 0).await;
    }
}
