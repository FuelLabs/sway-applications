use crate::utils::{
    abi_calls::{add_liquidity, deposit},
    test_helpers::{setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn adds_liquidity_when_total_liquidity_zero() {
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

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let added_liquidity = add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2)
            .await
            .value;

        assert_eq!(added_liquidity, base_deposit_amount);
    }

    #[tokio::test]
    async fn adds_liquidity_when_total_liquidity_not_zero_and_asset_deposit_sufficient() {
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

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let added_liquidity = add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2)
            .await
            .value;

        let later_base_deposit_amount = 100;
        let later_other_deposit_amount = 200;

        let call_params =
            CallParameters::new(Some(later_base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(later_other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let later_added_liquidity =
            add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2)
                .await
                .value;

        assert_eq!(
            added_liquidity + later_added_liquidity,
            base_deposit_amount + later_base_deposit_amount
        );
    }

    #[tokio::test]
    async fn adds_liquidity_when_total_liquidity_not_zero_and_asset_deposit_extra() {
        let (
            exchange_instance,
            wallet,
            _pool_asset_id,
            base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;
        let base_deposit_amount = 100;
        let other_deposit_amount = 200;
        let wallet_initial_balance = wallet.get_asset_balance(&other_asset_id).await.unwrap();

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let added_liquidity = add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2)
            .await
            .value;

        let later_base_deposit_amount = 100;
        // depositing extra
        let later_other_deposit_amount = 250;

        let call_params =
            CallParameters::new(Some(later_base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(later_other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let later_added_liquidity =
            add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2)
                .await
                .value;

        assert_eq!(
            added_liquidity + later_added_liquidity,
            base_deposit_amount + later_base_deposit_amount
        );

        let wallet_final_balance = wallet.get_asset_balance(&other_asset_id).await.unwrap();

        assert_eq!(
            wallet_initial_balance - wallet_final_balance,
            (added_liquidity + later_added_liquidity) * other_deposit_amount / base_deposit_amount
        );
    }

    #[tokio::test]
    async fn does_not_add_liquidity_when_total_liquidity_not_zero_but_asset_deposit_insufficient() {
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

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let added_liquidity = add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2)
            .await
            .value;

        let later_base_deposit_amount = 100;

        let call_params =
            CallParameters::new(Some(later_base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let later_added_liquidity =
            add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2)
                .await
                .value;

        assert_eq!(later_added_liquidity, 0);
        assert_eq!(added_liquidity, base_deposit_amount);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_not_initialized() {
        // call setup instead of setup_and_initialize
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            _base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup().await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deadline_passed() {
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

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // deadline is 0
        add_liquidity(&exchange_instance, call_params, tx_params, 0, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_message_amount_not_zero() {
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

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        // msg_amount set to 1
        let call_params = CallParameters::new(Some(1), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2).await;
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
            invalid_asset_id,
        ) = setup_and_initialize().await;
        let base_deposit_amount = 100;
        let other_deposit_amount = 200;

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        // send invalid asset id
        let call_params = CallParameters::new(Some(0), Some(invalid_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_base_asset_in_deposit_zero() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            _base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;
        let other_deposit_amount = 200;

        // do not deposit base asset

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_total_liquidity_not_zero_but_min_liquidity_zero() {
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

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2).await;

        // at this point, liquidity is 2

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // min_liquidity is 0
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_total_liquidity_not_zero_but_min_liquidity_too_high() {
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

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let added_liquidity =
            add_liquidity(&exchange_instance, call_params, tx_params, 1000, 2).await;

        // at this point, liquidity is 2

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // min_liquidity greater than added liquidity
        add_liquidity(
            &exchange_instance,
            call_params,
            tx_params,
            1000,
            added_liquidity.value + 1,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_total_liquidity_zero_but_base_amount_in_deposit_too_low() {
        let (
            exchange_instance,
            _wallet,
            _pool_asset_id,
            base_asset_id,
            other_asset_id,
            _invalid_asset_id,
        ) = setup_and_initialize().await;
        // base asset deposit amount too low
        let base_deposit_amount = 1;
        let other_deposit_amount = 200;

        let call_params = CallParameters::new(Some(base_deposit_amount), Some(base_asset_id), None);
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(other_deposit_amount),
            Some(other_asset_id.clone()),
            None,
        );
        deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(Some(0), Some(other_asset_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 1).await;
    }
}
