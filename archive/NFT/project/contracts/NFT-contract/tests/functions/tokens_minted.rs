use crate::utils::{
    abi_calls::{constructor, mint, tokens_minted},
    test_helpers::setup,
};
use fuels::{signers::Signer, types::Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_tokens_minted() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;

        assert_eq!(tokens_minted(&owner1.contract).await, 1);
    }

    #[tokio::test]
    async fn gets_tokens_minted_multiple() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(4)).await;

        assert_eq!(tokens_minted(&owner1.contract).await, 0);

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;
        assert_eq!(tokens_minted(&owner1.contract).await, 1);

        mint(1, &owner1.contract, minter.clone()).await;
        assert_eq!(tokens_minted(&owner1.contract).await, 2);

        mint(2, &owner1.contract, minter.clone()).await;
        assert_eq!(tokens_minted(&owner1.contract).await, 4);
    }
}
