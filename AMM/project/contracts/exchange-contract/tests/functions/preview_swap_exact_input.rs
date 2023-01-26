use crate::utils::{
    abi_calls::preview_swap_exact_input,
    test_helpers::{setup, setup_initialize_deposit_and_add_liquidity},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn previews_swap_of_a() {
        let (exchange, _wallet, amounts, _asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;
        let input_amount = 10;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_min_output_amount = (input_amount * (1 - (1 / 333)) * amounts.amount_b)
            / (amounts.amount_a + (input_amount * (1 - (1 / 333))));
        let expected_sufficient_reserve = expected_min_output_amount <= amounts.amount_b;

        let preview_swap_info =
            preview_swap_exact_input(&exchange.instance, input_amount, exchange.asset_a)
                .await
                .value;

        assert_eq!(preview_swap_info.amount, expected_min_output_amount);
        assert_eq!(
            preview_swap_info.sufficient_reserve,
            expected_sufficient_reserve
        );
    }

    #[tokio::test]
    async fn previews_swap_of_b() {
        let (exchange, _wallet, amounts, _asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;
        let input_amount = 10;

        // hardcoded calculation for liquidity miner fee of 333
        let expected_min_output_amount = (input_amount * (1 - (1 / 333)) * amounts.amount_a)
            / (amounts.amount_b + (input_amount * (1 - (1 / 333))));
        let expected_sufficient_reserve = expected_min_output_amount <= amounts.amount_a;

        let preview_swap_info =
            preview_swap_exact_input(&exchange.instance, input_amount, exchange.asset_b)
                .await
                .value;

        assert_eq!(preview_swap_info.amount, expected_min_output_amount);
        assert_eq!(
            preview_swap_info.sufficient_reserve,
            expected_sufficient_reserve
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;

        preview_swap_exact_input(&exchange_instance, 10, asset_a_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, _amounts, asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;

        preview_swap_exact_input(
            &exchange.instance,
            10,
            // sending invalid asset
            asset_c_id,
        )
        .await;
    }
}
