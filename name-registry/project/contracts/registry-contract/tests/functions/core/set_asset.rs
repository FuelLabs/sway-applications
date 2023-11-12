use crate::utils::{interface::core::set_asset, setup::setup};

mod success {
    use fuels::types::Bits256;

    use super::*;
    use crate::utils::setup::AssetRateEvent;

    #[tokio::test]
    async fn sets_asset() {
        let (instance, _account, _wallet2) = setup().await;
        let rate = Some(5);
        let response = set_asset(&instance, instance.contract_id().asset_id(&Bits256::zeroed()), rate).await;
        let log = response.decode_logs_with_type::<AssetRateEvent>().unwrap();
        assert_eq!(
            log,
            vec![AssetRateEvent {
                id: instance.contract_id().asset_id(&Bits256::zeroed()),
                rate,
            }]
        )
    }
}

mod revert {
    use fuels::types::Bits256;

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "SenderNotOwner")]
    async fn not_owner() {
        let (instance, _account, wallet2) = setup().await;
        let rate = Some(5);
        set_asset(
            &instance.with_account(wallet2).unwrap(),
            instance.contract_id().asset_id(&Bits256::zeroed()),
            rate,
        )
        .await;
    }
}
