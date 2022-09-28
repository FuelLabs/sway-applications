use crate::utils::{
    abi_calls::preview_swap_with_maximum,
    test_helpers::{deposit_and_add_liquidity, setup},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_preview_swap_with_maximum_eth_for_tokens() {
        let (exchange_instance, _native_contract_id, token_asset_id, _lp_asset_id) = setup().await;

        let swap_amount: u64 = 10;
        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        deposit_and_add_liquidity(
            &exchange_instance,
            native_amount_deposit,
            token_amount_deposit,
            token_asset_id,
        )
        .await;

        let amount_expected =
            preview_swap_with_maximum(&exchange_instance, CallParameters::default(), swap_amount)
                .await;
        assert!(amount_expected.has_liquidity);
    }

    #[tokio::test]
    async fn can_preview_swap_with_maximum_tokens_for_eth() {
        let (exchange_instance, _native_contract_id, token_asset_id, _lp_asset_id) = setup().await;

        let swap_amount: u64 = 10;
        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        deposit_and_add_liquidity(
            &exchange_instance,
            native_amount_deposit,
            token_amount_deposit,
            token_asset_id,
        )
        .await;

        let call_params = CallParameters::new(None, Some(token_asset_id.clone()), None);
        let amount_expected =
            preview_swap_with_maximum(&exchange_instance, call_params, swap_amount).await;
        assert!(amount_expected.has_liquidity);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_insufficient_reserve() {
        let (exchange_instance, _native_contract_id, token_asset_id, _lp_asset_id) = setup().await;

        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        deposit_and_add_liquidity(
            &exchange_instance,
            native_amount_deposit,
            token_amount_deposit,
            token_asset_id,
        )
        .await;

        // swap amount more than reserve
        preview_swap_with_maximum(
            &exchange_instance,
            CallParameters::default(),
            token_amount_deposit + 1,
        )
        .await;
    }
}
