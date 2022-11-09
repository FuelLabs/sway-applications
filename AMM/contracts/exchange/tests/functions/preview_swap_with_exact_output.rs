use crate::utils::{
    abi_calls::preview_swap_with_exact_output,
    test_helpers::{setup, setup_initialize_deposit_and_add_liquidity},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn previews_partial_swap_of_a() {
        let (exchange, _wallet, amounts, _asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;

        let output_amount = 10;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_max_input_amount = ((amounts.amount_a * output_amount)
            / (amounts.amount_b - output_amount)
            * (1 - (1 / 333)))
            + 1;
        let expected_sufficient_reserve = output_amount < amounts.amount_b;

        let preview_swap_info =
            preview_swap_with_exact_output(&exchange.instance, output_amount, exchange.asset_b)
                .await
                .value;

        assert_eq!(preview_swap_info.amount, expected_max_input_amount);
        assert_eq!(
            preview_swap_info.sufficient_reserve,
            expected_sufficient_reserve
        );
    }

    #[tokio::test]
    async fn previews_partial_swap_of_b() {
        let (exchange, _wallet, amounts, _asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;

        let output_amount = 10;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_max_input_amount = ((amounts.amount_b * output_amount)
            / (amounts.amount_a - output_amount)
            * (1 - (1 / 333)))
            + 1;
        let expected_sufficient_reserve = output_amount <= amounts.amount_a;

        let preview_swap_info =
            preview_swap_with_exact_output(&exchange.instance, output_amount, exchange.asset_a)
                .await
                .value;

        assert_eq!(preview_swap_info.amount, expected_max_input_amount);
        assert_eq!(
            preview_swap_info.sufficient_reserve,
            expected_sufficient_reserve
        );
    }

    #[tokio::test]
    async fn previews_maximum_swap_of_a() {
        let (exchange, _wallet, amounts, _asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;

        let output_amount = amounts.amount_b - 1;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_max_input_amount = ((amounts.amount_a * output_amount)
            / (amounts.amount_b - output_amount)
            * (1 - (1 / 333)))
            + 1;
        let expected_sufficient_reserve = output_amount < amounts.amount_b;

        let preview_swap_info =
            preview_swap_with_exact_output(&exchange.instance, output_amount, exchange.asset_b)
                .await
                .value;

        assert_eq!(preview_swap_info.amount, expected_max_input_amount);
        assert_eq!(
            preview_swap_info.sufficient_reserve,
            expected_sufficient_reserve
        );
    }

    #[tokio::test]
    async fn previews_maximum_swap_of_b() {
        let (exchange, _wallet, amounts, _asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;

        let output_amount = amounts.amount_a - 1;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_max_input_amount = ((amounts.amount_b * output_amount)
            / (amounts.amount_a - output_amount)
            * (1 - (1 / 333)))
            + 1;
        let expected_sufficient_reserve = output_amount < amounts.amount_a;

        let preview_swap_info =
            preview_swap_with_exact_output(&exchange.instance, output_amount, exchange.asset_a)
                .await
                .value;

        assert_eq!(preview_swap_info.amount, expected_max_input_amount);
        assert_eq!(
            preview_swap_info.sufficient_reserve,
            expected_sufficient_reserve
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, _asset_a_id, asset_b_id, _asset_c_id) =
            setup().await;

        preview_swap_with_exact_output(&exchange_instance, 10, asset_b_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, _amounts, asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;

        preview_swap_with_exact_output(
            &exchange.instance,
            10,
            // sending invalid asset
            asset_c_id,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_output_b_amount_is_greater_than_reserve() {
        let (exchange, _wallet, amounts, _asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;

        let output_amount = amounts.amount_b + 1;

        preview_swap_with_exact_output(&exchange.instance, output_amount, exchange.asset_b).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(18446744073709486080)")]
    async fn when_output_a_amount_is_greater_than_reserve() {
        let (exchange, _wallet, amounts, _asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;

        let output_amount = amounts.amount_a + 1;

        preview_swap_with_exact_output(&exchange.instance, output_amount, exchange.asset_a).await;
    }
}
