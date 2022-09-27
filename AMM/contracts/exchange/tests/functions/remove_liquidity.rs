use crate::utils::{
    abi_calls::remove_liquidity,
    test_helpers::{deposit_and_add_liquidity, setup},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_remove_liquidity() {
        let (exchange_instance, _native_contract_id, token_asset_id, lp_asset_id) = setup().await;

        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        let lp_amount_received = deposit_and_add_liquidity(
            &exchange_instance,
            native_amount_deposit,
            token_amount_deposit,
            token_asset_id,
        )
        .await;

        let call_params = CallParameters::new(
            Some(lp_amount_received),
            Some(lp_asset_id.clone()),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let result = remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1, 1).await;
        assert_eq!(result.eth_amount, native_amount_deposit);
        assert_eq!(result.token_amount, token_amount_deposit);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_msg_amount_not_zero() {
        let (exchange_instance, _native_contract_id, token_asset_id, lp_asset_id) = setup().await;

        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        deposit_and_add_liquidity(
            &exchange_instance,
            native_amount_deposit,
            token_amount_deposit,
            token_asset_id,
        )
        .await;

        // msg_amount is 0
        let call_params =
            CallParameters::new(Some(0), Some(lp_asset_id.clone()), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_msg_asset_id_not_match_contract_id() {
        let (exchange_instance, _native_contract_id, token_asset_id, _lp_asset_id) = setup().await;

        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        let lp_amount_received = deposit_and_add_liquidity(
            &exchange_instance,
            native_amount_deposit,
            token_amount_deposit,
            token_asset_id,
        )
        .await;

        // msg_asset_id is invalid
        let call_params = CallParameters::new(
            Some(lp_amount_received),
            Some(token_asset_id.clone()),
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
    async fn on_deadline_passed() {
        let (exchange_instance, _native_contract_id, token_asset_id, lp_asset_id) = setup().await;

        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        let lp_amount_received = deposit_and_add_liquidity(
            &exchange_instance,
            native_amount_deposit,
            token_amount_deposit,
            token_asset_id,
        )
        .await;

        let call_params = CallParameters::new(
            Some(lp_amount_received),
            Some(lp_asset_id.clone()),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // deadline is 0
        remove_liquidity(&exchange_instance, call_params, tx_params, 0, 1, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_min_eth_zero() {
        let (exchange_instance, _native_contract_id, token_asset_id, lp_asset_id) = setup().await;

        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        let lp_amount_received = deposit_and_add_liquidity(
            &exchange_instance,
            native_amount_deposit,
            token_amount_deposit,
            token_asset_id,
        )
        .await;

        let call_params = CallParameters::new(
            Some(lp_amount_received),
            Some(lp_asset_id.clone()),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // min_eth is 0
        remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 0, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_min_tokens_zero() {
        let (exchange_instance, _native_contract_id, token_asset_id, lp_asset_id) = setup().await;

        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        let lp_amount_received = deposit_and_add_liquidity(
            &exchange_instance,
            native_amount_deposit,
            token_amount_deposit,
            token_asset_id,
        )
        .await;

        let call_params = CallParameters::new(
            Some(lp_amount_received),
            Some(lp_asset_id.clone()),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // min_tokens is 0
        remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_insufficient_eth_amount() {
        let (exchange_instance, _native_contract_id, token_asset_id, lp_asset_id) = setup().await;

        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        let lp_amount_received = deposit_and_add_liquidity(
            &exchange_instance,
            native_amount_deposit,
            token_amount_deposit,
            token_asset_id,
        )
        .await;

        let call_params = CallParameters::new(
            Some(lp_amount_received),
            Some(lp_asset_id.clone()),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // min_eth is too high
        remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1000000, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_insufficient_token_amount() {
        let (exchange_instance, _native_contract_id, token_asset_id, lp_asset_id) = setup().await;

        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        let lp_amount_received = deposit_and_add_liquidity(
            &exchange_instance,
            native_amount_deposit,
            token_amount_deposit,
            token_asset_id,
        )
        .await;

        let call_params = CallParameters::new(
            Some(lp_amount_received),
            Some(lp_asset_id.clone()),
            Some(100_000_000),
        );
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // min_tokens is too high
        remove_liquidity(&exchange_instance, call_params, tx_params, 1000, 1, 1000000).await;
    }
}
