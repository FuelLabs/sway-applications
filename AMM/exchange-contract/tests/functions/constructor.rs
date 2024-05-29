use crate::utils::setup;
use test_utils::interface::exchange::constructor;

mod success {
    use super::*;
    use test_utils::interface::{exchange::pool_info, DefineAssetPairEvent};

    #[tokio::test]
    async fn constructs() {
        let (exchange_instance, _wallet, assets, _deadline) = setup().await;

        let response = constructor(&exchange_instance, (assets.asset_1, assets.asset_2)).await;
        let log = response
            .decode_logs_with_type::<DefineAssetPairEvent>()
            .unwrap();
        let event = log.first().unwrap();

        let pool_info = pool_info(&exchange_instance).await;

        assert_eq!(
            *event,
            DefineAssetPairEvent {
                asset_a_id: assets.asset_1,
                asset_b_id: assets.asset_2,
            }
        );
        assert_eq!(pool_info.reserves.a.id, assets.asset_1);
        assert_eq!(pool_info.reserves.b.id, assets.asset_2);
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
