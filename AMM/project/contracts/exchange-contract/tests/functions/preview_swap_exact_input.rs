use crate::utils::setup_and_construct;
use test_utils::interface::exchange::preview_swap_exact_input;

mod success {
    use super::*;
    use crate::utils::minimum_output_given_exact_input;
    use fuels::prelude::AssetId;

    #[tokio::test]
    async fn previews_swap_of_a() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let input_amount = 10;

        let expected_min_output_amount = minimum_output_given_exact_input(
            input_amount,
            liquidity_parameters.amounts.0,
            liquidity_parameters.amounts.1,
            333,
        );
        let expected_sufficient_reserve =
            expected_min_output_amount <= liquidity_parameters.amounts.1;

        let preview_swap_info =
            preview_swap_exact_input(&exchange.instance, input_amount, exchange.pair.0, true).await;

        assert_eq!(
            AssetId::new(*preview_swap_info.other_asset.id),
            exchange.pair.1
        );
        assert_eq!(
            preview_swap_info.other_asset.amount,
            expected_min_output_amount
        );
        assert_eq!(
            preview_swap_info.sufficient_reserve,
            expected_sufficient_reserve
        );
    }

    #[tokio::test]
    async fn previews_swap_of_b() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;
        let input_amount = 10;

        let expected_min_output_amount = minimum_output_given_exact_input(
            input_amount,
            liquidity_parameters.amounts.1,
            liquidity_parameters.amounts.0,
            333,
        );
        let expected_sufficient_reserve =
            expected_min_output_amount <= liquidity_parameters.amounts.0;

        let preview_swap_info =
            preview_swap_exact_input(&exchange.instance, input_amount, exchange.pair.1, true).await;

        assert_eq!(
            AssetId::new(*preview_swap_info.other_asset.id),
            exchange.pair.0
        );
        assert_eq!(
            preview_swap_info.other_asset.amount,
            expected_min_output_amount
        );
        assert_eq!(
            preview_swap_info.sufficient_reserve,
            expected_sufficient_reserve
        );
    }
}

mod revert {
    use super::*;
    use crate::utils::setup;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_construct
        let (exchange_instance, _wallet, assets, _deadline) = setup().await;

        preview_swap_exact_input(&exchange_instance, 10, assets.asset_1, false).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, _liquidity_parameters, asset_c_id) =
            setup_and_construct(true, true).await;

        preview_swap_exact_input(
            &exchange.instance,
            10,
            // passing invalid asset
            asset_c_id,
            false,
        )
        .await;
    }
}
