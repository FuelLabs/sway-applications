use crate::utils::{
    abi_calls::{constructor, meta_data, mint},
    test_helpers::setup,
    TokenMetaData,
};
use fuels::{prelude::Identity, signers::Signer};

mod success {

    use fuels::prelude::SizedAsciiString;

    use super::*;

    #[tokio::test]
    async fn gets_meta_data() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 10).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 10).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(1, &owner1.contract, &minter).await;

        assert_eq!(
            meta_data(&owner1.contract, 0).await,
            TokenMetaData {
                name: SizedAsciiString::<7>::new("Example".to_string()).unwrap()
            }
        );
    }

    #[tokio::test]
    async fn gets_meta_data_multiple() {
        let (deploy_wallet, owner1, _owner2) = setup().await;

        // constructor(false, &deploy_wallet.contract, &Option::None(), 10).await;
        let admin = Identity::Address(owner1.wallet.address().into());
        constructor(true, &deploy_wallet.contract, &admin, 10).await;

        let minter = Identity::Address(owner1.wallet.address().into());
        mint(3, &owner1.contract, &minter).await;

        assert_eq!(
            meta_data(&owner1.contract, 0).await,
            TokenMetaData {
                name: SizedAsciiString::<7>::new("Example".to_string()).unwrap()
            }
        );

        assert_eq!(
            meta_data(&owner1.contract, 1).await,
            TokenMetaData {
                name: SizedAsciiString::<7>::new("Example".to_string()).unwrap()
            }
        );

        assert_eq!(
            meta_data(&owner1.contract, 2).await,
            TokenMetaData {
                name: SizedAsciiString::<7>::new("Example".to_string()).unwrap()
            }
        );
    }
}

mod reverts {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_token_does_not_exist() {
        let (_deploy_wallet, owner1, _owner2) = setup().await;

        meta_data(&owner1.contract, 1).await;
    }
}
