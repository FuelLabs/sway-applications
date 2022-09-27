use crate::utils::{
    abi_calls::{get_swap_with_minimum, swap_with_minimum},
    test_helpers::{deposit_and_add_liquidity, setup},
};
use fuels::prelude::*;
use std::str::FromStr;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_swap_with_minimum_eth_for_tokens() {
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

        let call_params = CallParameters::new(Some(swap_amount), None, None);
        let response = swap_with_minimum(
            &exchange_instance,
            call_params,
            1000,
            amount_expected.amount,
        )
        .await;
        assert_eq!(response.value, amount_expected.amount);
    }

    #[tokio::test]
    async fn can_swap_with_minimum_tokens_for_eth() {
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

        let call_params =
            CallParameters::new(Some(swap_amount), Some(token_asset_id.clone()), None);
        let response = swap_with_minimum(
            &exchange_instance,
            call_params,
            1000,
            amount_expected.amount,
        )
        .await;
        assert_eq!(response.value, amount_expected.amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_deadline_passed() {
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

        let call_params = CallParameters::new(Some(swap_amount), None, None);
        // deadline is 0
        swap_with_minimum(&exchange_instance, call_params, 0, amount_expected.amount).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_swap_amount_zero() {
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

        let call_params = CallParameters::new(Some(swap_amount), None, None);
        // minimum swap amount is 0
        swap_with_minimum(&exchange_instance, call_params, 1000, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_msg_amount_zero() {
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

        // msg_amount is 0
        let call_params = CallParameters::new(Some(0), None, None);

        swap_with_minimum(
            &exchange_instance,
            call_params,
            1000,
            amount_expected.amount,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_msg_asset_id_invalid() {
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

        let unmatched_id =
            AssetId::from_str("0x0000000000000000000000000000000000000000000000000000000000000002")
                .unwrap();

        let call_params = CallParameters::new(Some(swap_amount), Some(unmatched_id), None);
        swap_with_minimum(
            &exchange_instance,
            call_params,
            1000,
            amount_expected.amount,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_swap_amount_insufficient() {
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

        let call_params = CallParameters::new(Some(swap_amount), None, None);
        // minimum swap amount is too high
        swap_with_minimum(&exchange_instance, call_params, 1000, 100_000_000).await;
    }
}
