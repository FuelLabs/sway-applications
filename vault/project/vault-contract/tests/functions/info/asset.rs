mod success {
    use crate::utils::{interface::info::asset, setup::setup};
    use fuels::prelude::{AssetId, BASE_ASSET_ID};

    #[tokio::test]
    async fn gets_the_asset_id() {
        let (instance, wallet) = setup().await;
        let contract_id = asset(&instance).await.value;
        assert_eq!(*contract_id, *BASE_ASSET_ID);
    }
}
