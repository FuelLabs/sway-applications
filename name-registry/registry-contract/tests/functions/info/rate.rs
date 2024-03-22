mod success {
    use crate::utils::{
        interface::{core::set_asset, info::rate},
        setup::setup,
    };
    use fuels::prelude::AssetId;

    #[tokio::test]
    async fn asset_not_set_returns_none() {
        let (instance, _account, _wallet2) = setup().await;
        let value = rate(&instance, AssetId::new(*instance.contract_id().hash())).await;
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn returns_set_rate() {
        let (instance, _account, _wallet2) = setup().await;

        let asset_rate = Some(5);
        set_asset(
            &instance,
            AssetId::new(*instance.contract_id().hash()),
            asset_rate,
        )
        .await;

        let value = rate(&instance, AssetId::new(*instance.contract_id().hash())).await;
        assert_eq!(asset_rate, value);
    }
}
