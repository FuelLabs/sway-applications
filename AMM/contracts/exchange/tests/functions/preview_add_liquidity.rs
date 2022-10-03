use crate::utils::{abi_calls::preview_add_liquidity, test_helpers::setup_and_initialize};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn previews_add_base_asset_when_total_liquidity_zero() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            base_asset_id,
            _other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;
        let base_preview_amount = 99;

        let preview = preview_add_liquidity(
            &exchange_instance,
            CallParameters::default(),
            TxParameters::default(),
            base_preview_amount,
            base_asset_id,
        )
        .await;

        assert_eq!(preview.received_liquidity, base_preview_amount);
        assert_eq!(preview.other_asset_amount, 0);
    }
}
