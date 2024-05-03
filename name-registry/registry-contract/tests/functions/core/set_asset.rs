use crate::utils::{interface::core::set_asset, setup::setup};
use fuels::prelude::AssetId;

mod success {
    use super::*;
    use crate::utils::setup::AssetRateEvent;

    #[tokio::test]
    async fn sets_asset() {
        let (instance, _account, _wallet2) = setup().await;
        let rate = Some(5);
        let response = set_asset(
            &instance,
            AssetId::new(*instance.contract_id().hash()),
            rate,
        )
        .await;
        let log = response.decode_logs_with_type::<AssetRateEvent>().unwrap();
        assert_eq!(
            log,
            vec![AssetRateEvent {
                asset: AssetId::new(*instance.contract_id().hash()),
                rate,
            }]
        )
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "SenderNotOwner")]
    async fn not_owner() {
        let (instance, _account, wallet2) = setup().await;
        let rate = Some(5);
        set_asset(
            &instance.clone().with_account(wallet2),
            AssetId::new(*instance.contract_id().hash()),
            rate,
        )
        .await;
    }
}
