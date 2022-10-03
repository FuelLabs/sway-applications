use crate::utils::{
    abi_calls::{preview_swap_with_maximum, swap_with_maximum},
    test_helpers::{deposit_and_add_liquidity, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_swap_maximum_base_for_other() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            _base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;

        let swap_amount = 10;
        let base_deposit_amount = 100;
        let other_deposit_amount = 200;

        deposit_and_add_liquidity(
            &exchange_instance,
            base_deposit_amount,
            other_deposit_amount,
            other_asset_id,
        )
        .await;

        let amount_expected =
            preview_swap_with_maximum(&exchange_instance, CallParameters::default(), swap_amount)
                .await;

        let call_params = CallParameters::new(Some(amount_expected.amount), None, None);
        let response = swap_with_maximum(&exchange_instance, call_params, swap_amount, 1000).await;
        assert_eq!(response.value, amount_expected.amount);
    }

    #[tokio::test]
    async fn can_swap_maximum_other_for_base() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            _base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;

        let swap_amount = 10;
        let base_deposit_amount = 100;
        let other_deposit_amount = 200;

        deposit_and_add_liquidity(
            &exchange_instance,
            base_deposit_amount,
            other_deposit_amount,
            other_asset_id,
        )
        .await;

        let call_params = CallParameters::new(None, Some(other_asset_id.clone()), None);
        let amount_expected =
            preview_swap_with_maximum(&exchange_instance, call_params, swap_amount).await;

        let call_params = CallParameters::new(
            Some(amount_expected.amount),
            Some(other_asset_id.clone()),
            None,
        );
        let response = swap_with_maximum(&exchange_instance, call_params, swap_amount, 1000).await;
        assert_eq!(response.value, amount_expected.amount);
    }
}
