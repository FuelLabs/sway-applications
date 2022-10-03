use crate::utils::{
    abi_calls::remove_liquidity,
    test_helpers::{deposit_and_add_liquidity, setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_remove_liquidity() {
        let (
            exchange_instance,
            _wallet,
            pool_asset_id,
            _base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;

        let base_deposit_amount = 100;
        let other_deposit_amount = 200;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange_instance,
            base_deposit_amount,
            other_deposit_amount,
            other_asset_id,
        )
        .await;

        let call_params = CallParameters::new(
            Some(added_liquidity),
            Some(pool_asset_id),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let result = remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1, 1).await;
        assert_eq!(result.base_asset_amount, base_deposit_amount);
        assert_eq!(result.other_asset_amount, other_deposit_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_not_initialized() {
        // call setup instead of setup and initialize
        let (
            exchange_instance,
            _wallet,
            pool_asset_id,
            _base_asset_id,
            _other_asset_id,
            _invalid_asset_id,
        ) = setup().await;

        let call_params = CallParameters::new(Some(0), Some(pool_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_asset_invalid() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;

        let base_deposit_amount = 100;
        let other_deposit_amount = 200;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange_instance,
            base_deposit_amount,
            other_deposit_amount,
            other_asset_id,
        )
        .await;

        // sending invalid asset id
        let call_params = CallParameters::new(
            Some(added_liquidity),
            Some(base_asset_id),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_minimum_base_asset_value_zero() {
        let (
            exchange_instance,
            _wallet,
            pool_asset_id,
            _base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;

        let base_deposit_amount = 100;
        let other_deposit_amount = 200;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange_instance,
            base_deposit_amount,
            other_deposit_amount,
            other_asset_id,
        )
        .await;

        let call_params = CallParameters::new(
            Some(added_liquidity),
            Some(pool_asset_id),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // setting min_base_asset to 0
        remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 0, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_minimum_other_asset_value_zero() {
        let (
            exchange_instance,
            _wallet,
            pool_asset_id,
            _base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;

        let base_deposit_amount = 100;
        let other_deposit_amount = 200;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange_instance,
            base_deposit_amount,
            other_deposit_amount,
            other_asset_id,
        )
        .await;

        let call_params = CallParameters::new(
            Some(added_liquidity),
            Some(pool_asset_id),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // setting min_other_asset to 0
        remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deadline_passed() {
        let (
            exchange_instance,
            _wallet,
            pool_asset_id,
            _base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;

        let base_deposit_amount = 100;
        let other_deposit_amount = 200;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange_instance,
            base_deposit_amount,
            other_deposit_amount,
            other_asset_id,
        )
        .await;

        let call_params = CallParameters::new(
            Some(added_liquidity),
            Some(pool_asset_id),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // setting deadline to 0
        remove_liquidity(&exchange_instance, call_params, tx_params, 0, 1, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_message_amount_zero() {
        let (
            exchange_instance,
            _wallet,
            pool_asset_id,
            _base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;

        let base_deposit_amount = 100;
        let other_deposit_amount = 200;

        deposit_and_add_liquidity(
            &exchange_instance,
            base_deposit_amount,
            other_deposit_amount,
            other_asset_id,
        )
        .await;

        // setting amount to 0
        let call_params = CallParameters::new(Some(0), Some(pool_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1, 1).await;
    }
}
