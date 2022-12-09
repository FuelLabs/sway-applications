use crate::utils::{setup, setup_and_construct};
use test_utils::abi::exchange::preview_swap_exact_output;

mod success {
    use super::*;

    #[tokio::test]
    async fn previews_swap_of_a() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_max_input_amount = ((liquidity_parameters.amounts.0 * output_amount)
            / (liquidity_parameters.amounts.1 - output_amount)
            * (1 - (1 / 333)))
            + 1;
        let expected_sufficient_reserve = output_amount < liquidity_parameters.amounts.1;

        let preview_swap_info =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.1)
                .await
                .value;

        assert_eq!(preview_swap_info.amount, expected_max_input_amount);
        assert_eq!(
            preview_swap_info.sufficient_reserve,
            expected_sufficient_reserve
        );
    }

    #[tokio::test]
    async fn previews_swap_of_b() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = 10;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_max_input_amount = ((liquidity_parameters.amounts.1 * output_amount)
            / (liquidity_parameters.amounts.0 - output_amount)
            * (1 - (1 / 333)))
            + 1;
        let expected_sufficient_reserve = output_amount <= liquidity_parameters.amounts.0;

        let preview_swap_info =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.0)
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
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = liquidity_parameters.amounts.1 - 1;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_max_input_amount = ((liquidity_parameters.amounts.0 * output_amount)
            / (liquidity_parameters.amounts.1 - output_amount)
            * (1 - (1 / 333)))
            + 1;
        let expected_sufficient_reserve = output_amount < liquidity_parameters.amounts.1;

        let preview_swap_info =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.1)
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
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = liquidity_parameters.amounts.0 - 1;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_max_input_amount = ((liquidity_parameters.amounts.1 * output_amount)
            / (liquidity_parameters.amounts.0 - output_amount)
            * (1 - (1 / 333)))
            + 1;
        let expected_sufficient_reserve = output_amount < liquidity_parameters.amounts.0;

        let preview_swap_info =
            preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.0)
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
    #[should_panic(expected = "RevertTransactionError(\"NotInitialized\"")]
    async fn when_unitialized() {
        // call setup instead of setup_and_construct
        let (exchange_instance, _wallet, _pool_asset_id, _asset_a_id, asset_b_id, _asset_c_id) =
            setup().await;

        preview_swap_exact_output(&exchange_instance, 10, asset_b_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"InvalidAsset\"")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, _liquidity_parameters, asset_c_id) =
            setup_and_construct(true, true).await;

        preview_swap_exact_output(
            &exchange.instance,
            10,
            // sending invalid asset
            asset_c_id,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"DesiredAmountTooHigh(40001)\"")]
    async fn when_output_b_amount_is_greater_than_reserve() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = liquidity_parameters.amounts.1 + 1;

        preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"DesiredAmountTooHigh(10001)\"")]
    async fn when_output_a_amount_is_greater_than_reserve() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let output_amount = liquidity_parameters.amounts.0 + 1;

        preview_swap_exact_output(&exchange.instance, output_amount, exchange.pair.0).await;
    }
}
