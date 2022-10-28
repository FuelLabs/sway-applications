use crate::utils::{
    abi_calls::preview_swap_with_exact_input,
    test_helpers::{deposit_and_add_liquidity, setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn previews_partial_swap_of_a() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let input_amount = 10;
        // hardcoded calculation for liquidity miner fee of 333
        let expected_min_output_amount = (input_amount * (1 - (1 / 333)) * deposit_amount_b)
            / (deposit_amount_a + (input_amount * (1 - (1 / 333))));
        let expected_sufficient_reserve = expected_min_output_amount <= deposit_amount_b;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let preview_swap_info =
            preview_swap_with_exact_input(&exchange.contract, input_amount, exchange.asset_a_id)
                .await
                .value;

        assert_eq!(preview_swap_info.amount, expected_min_output_amount);
        assert_eq!(
            preview_swap_info.sufficient_reserve,
            expected_sufficient_reserve
        );
    }

    #[tokio::test]
    async fn previews_partial_swap_of_b() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let liquidity = 200;
        let deadline = 1000;
        let input_amount = 10;
        // hardcoded calculation for liquidity miner fee of 333
        let expected_min_output_amount = (input_amount * (1 - (1 / 333)) * deposit_amount_a)
            / (deposit_amount_b + (input_amount * (1 - (1 / 333))));
        let expected_sufficient_reserve = expected_min_output_amount <= deposit_amount_a;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            liquidity,
            deadline,
        )
        .await;

        let preview_swap_info =
            preview_swap_with_exact_input(&exchange.contract, input_amount, exchange.asset_b_id)
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
    #[should_panic(expected = "Revert(42)")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;

        preview_swap_with_exact_input(&exchange_instance, 10, asset_a_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, asset_c_id) = setup_and_initialize().await;

        preview_swap_with_exact_input(
            &exchange.contract,
            10,
            // sending invalid asset
            asset_c_id,
        )
        .await;
    }
}
