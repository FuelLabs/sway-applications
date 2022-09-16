use crate::utils::{abi_calls::constructor, test_helpers::setup, Identity};

mod success {

    use super::*;

    #[tokio::test]
    async fn initalizes() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(total_supply, &deployer.simple_asset, identity.clone()).await;
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_initalized_twice() {
        let (deployer, _, total_supply) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(total_supply, &deployer.simple_asset, identity.clone()).await;
        constructor(total_supply, &deployer.simple_asset, identity.clone()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_asset_supply_zero() {
        let (deployer, _, _) = setup().await;

        let identity = Identity::Address(deployer.wallet.address().into());
        constructor(0, &deployer.simple_asset, identity.clone()).await;
    }
}
