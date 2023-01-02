use crate::utils::setup;
use fuels::prelude::*;
use test_utils::interface::exchange::{constructor, pool_info};

mod success {
    use super::*;

    #[tokio::test]
    async fn constructs() {
        let (exchange_instance, _wallet, assets, _deadline) = setup().await;

        constructor(&exchange_instance, (assets.asset_1, assets.asset_2)).await;
        let pool_info = pool_info(&exchange_instance).await;

        assert_eq!(pool_info.reserves.a.id, ContractId::new(*assets.asset_1));
        assert_eq!(pool_info.reserves.b.id, ContractId::new(*assets.asset_2));
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AssetPairAlreadySet")]
    async fn when_reinitialized() {
        let (exchange_instance, _wallet, assets, _deadline) = setup().await;

        constructor(&exchange_instance, (assets.asset_1, assets.asset_2)).await;

        constructor(&exchange_instance, (assets.asset_1, assets.asset_2)).await;
    }

    #[tokio::test]
    #[should_panic(expected = "IdenticalAssets")]
    async fn when_assets_in_pair_are_identical() {
        let (exchange_instance, _wallet, assets, _deadline) = setup().await;

        constructor(&exchange_instance, (assets.asset_1, assets.asset_1)).await;
    }
}
