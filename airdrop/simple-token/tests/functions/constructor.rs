use crate::utils::{abi_calls::constructor, test_helpers::setup, Identity};
use fuels::signers::Signer;

mod success {

    use super::*;

    #[tokio::test]
    async fn initalizes() {
        let (deployer, _) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_token, 100).await;
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_initalized_twice() {
        let (deployer, _) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_token, 100).await;
        constructor(identity.clone(), &deployer.simple_token, 100).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_token_supply_zero() {
        let (deployer, _) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_token, 0).await;
    }
}
