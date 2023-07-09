mod success {
    use crate::utils::{
        interface::{core::set_asset, info::rate},
        setup::setup,
    };

    #[tokio::test]
    #[ignore]
    async fn asset_not_set_returns_none() {
        // TODO: bug in older compiler, remove ignore when bumping versions, use matches! > assert_eq
        let (instance, _account, _wallet2) = setup().await;
        let value = rate(&instance, instance.contract_id().into()).await;
        assert_eq!(None, value);
    }

    #[tokio::test]
    async fn returns_set_rate() {
        let (instance, _account, _wallet2) = setup().await;

        let asset_rate = Some(5);
        set_asset(&instance, instance.contract_id().into(), asset_rate).await;

        let value = rate(&instance, instance.contract_id().into()).await;
        assert_eq!(asset_rate, value);
    }
}
