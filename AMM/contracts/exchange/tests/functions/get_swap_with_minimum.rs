use crate::utils::{
    abi_calls::get_swap_with_minimum,
    test_helpers::{deposit_and_add_liquidity, setup},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_get_swap_with_minimum_eth_for_tokens() {
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
            get_swap_with_minimum(&exchange_instance, CallParameters::default(), swap_amount).await;
        assert!(amount_expected.has_liquidity);
    }

    #[tokio::test]
    async fn can_get_swap_with_minimum_tokens_for_eth() {
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

        let call_params = CallParameters::new(Some(0), Some(token_asset_id.clone()), None);
        let amount_expected =
            get_swap_with_minimum(&exchange_instance, call_params, swap_amount).await;
        assert!(amount_expected.has_liquidity);
    }
}
