use crate::utils::{
    abi_calls::{constructor, mint, token_metadata},
    test_helpers::setup,
    TokenMetadata,
};
use fuels::{
    signers::Signer,
    types::{Identity, SizedAsciiString},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn gets_meta_data() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(1)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, minter.clone()).await;

        let example_metadata = TokenMetadata {
            name: SizedAsciiString::<7>::new("Example".to_string()).unwrap(),
        };

        assert_eq!(
            token_metadata(&owner1.contract, 0).await,
            Some(example_metadata.clone())
        );
    }

    #[tokio::test]
    async fn gets_meta_data_multiple() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        constructor(None, &deploy_wallet.contract, Some(10)).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(3, &owner1.contract, minter.clone()).await;

        let example_metadata = TokenMetadata {
            name: SizedAsciiString::<7>::new("Example".to_string()).unwrap(),
        };

        assert_eq!(
            token_metadata(&owner1.contract, 0).await,
            Some(example_metadata.clone())
        );

        assert_eq!(
            token_metadata(&owner1.contract, 1).await,
            Some(example_metadata.clone())
        );

        assert_eq!(
            token_metadata(&owner1.contract, 2).await,
            Some(example_metadata.clone())
        );
    }

    #[tokio::test]
    async fn when_token_does_not_exist() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        assert_eq!(token_metadata(&owner1.contract, 1).await, None);
    }
}
