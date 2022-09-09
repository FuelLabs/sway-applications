use crate::utils::{abi_calls::constructor, test_helpers::setup, Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn initalizes() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_asset, total_supply).await;
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_initalized_twice() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_asset, total_supply).await;
        constructor(identity.clone(), &deployer.simple_asset, total_supply).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn panics_when_asset_supply_zero() {
        let (deployer, _, _) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(identity.clone(), &deployer.simple_asset, 0).await;
    }
}
