use crate::utils::{
    abi_calls::{add_liquidity, deposit},
    test_helpers::setup,
};
use fuels::prelude::*;
use std::str::FromStr;

mod success {
    use super::*;

    #[tokio::test]
    async fn can_add_liquidity() {
        let (exchange_instance, _native_contract_id, token_asset_id, ..) = setup().await;
        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        // Deposit some Native Asset
        let call_params = CallParameters::new(Some(native_amount_deposit), None, None);
        let _t = deposit(&exchange_instance, call_params).await;

        // Deposit some Token Asset
        let call_params = CallParameters::new(
            Some(token_amount_deposit),
            Some(token_asset_id.clone()),
            None,
        );
        let _t = deposit(&exchange_instance, call_params).await;

        // Add liquidity for the second time. Keeping the proportion 1:2
        // It should return the same amount of LP as the amount of ETH deposited
        let call_params =
            CallParameters::new(Some(0), Some(token_asset_id.clone()), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        let lp_amount_received =
            add_liquidity(&exchange_instance, call_params, tx_params, 1000, 1).await;

        assert_eq!(lp_amount_received.value, native_amount_deposit);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_msg_amount_not_zero() {
        let (exchange_instance, _native_contract_id, token_asset_id, ..) = setup().await;
        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        let call_params = CallParameters::new(Some(native_amount_deposit), None, None);
        let _t = deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(token_amount_deposit),
            Some(token_asset_id.clone()),
            None,
        );
        let _t = deposit(&exchange_instance, call_params).await;

        // msg_amount 1 instead of 0
        let call_params =
            CallParameters::new(Some(1), Some(token_asset_id.clone()), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_deadline_passed() {
        let (exchange_instance, _native_contract_id, token_asset_id, ..) = setup().await;
        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        let call_params = CallParameters::new(Some(native_amount_deposit), None, None);
        let _t = deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(token_amount_deposit),
            Some(token_asset_id.clone()),
            None,
        );
        let _t = deposit(&exchange_instance, call_params).await;

        // Add liquidity for the second time. Keeping the proportion 1:2
        // It should return the same amount of LP as the amount of ETH deposited
        let call_params =
            CallParameters::new(Some(0), Some(token_asset_id.clone()), Some(100_000_000));
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
    async fn on_token_id_not_match() {
        let (exchange_instance, _native_contract_id, token_asset_id, ..) = setup().await;
        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        let call_params = CallParameters::new(Some(native_amount_deposit), None, None);
        let _t = deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(token_amount_deposit),
            Some(token_asset_id.clone()),
            None,
        );
        let _t = deposit(&exchange_instance, call_params).await;

        // invalid asset id
        let unmatched_id =
            AssetId::from_str("0x0000000000000000000000000000000000000000000000000000000000000002")
                .unwrap();

        let call_params = CallParameters::new(Some(0), Some(unmatched_id), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_not_enough_eth_balance_on_contract() {
        let (exchange_instance, _native_contract_id, token_asset_id, ..) = setup().await;
        let token_amount_deposit = 200;

        // only deposit token, not ETH
        let call_params = CallParameters::new(
            Some(token_amount_deposit),
            Some(token_asset_id.clone()),
            None,
        );
        let _t = deposit(&exchange_instance, call_params).await;

        let call_params =
            CallParameters::new(Some(0), Some(token_asset_id.clone()), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_min_liquidity_zero() {
        let (exchange_instance, _native_contract_id, token_asset_id, ..) = setup().await;
        let native_amount_deposit = 100;
        let token_amount_deposit = 200;

        let call_params = CallParameters::new(Some(native_amount_deposit), None, None);
        let _t = deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(token_amount_deposit),
            Some(token_asset_id.clone()),
            None,
        );
        let _t = deposit(&exchange_instance, call_params).await;

        let call_params =
            CallParameters::new(Some(0), Some(token_asset_id.clone()), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 1).await;

        let call_params =
            CallParameters::new(Some(0), Some(token_asset_id.clone()), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };
        // total liquidity is not 0 but min_liquidity is 0
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 0).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_insufficient_eth_balance() {
        let (exchange_instance, _native_contract_id, token_asset_id, ..) = setup().await;
        let native_amount_deposit = 1;
        let token_amount_deposit = 200;

        let call_params = CallParameters::new(Some(native_amount_deposit), None, None);
        let _t = deposit(&exchange_instance, call_params).await;

        let call_params = CallParameters::new(
            Some(token_amount_deposit),
            Some(token_asset_id.clone()),
            None,
        );
        let _t = deposit(&exchange_instance, call_params).await;

        let call_params =
            CallParameters::new(Some(0), Some(token_asset_id.clone()), Some(100_000_000));
        let tx_params = TxParameters {
            gas_price: 0,
            gas_limit: 100_000_000,
            maturity: 0,
        };

        // native amount deposit is lower than min_liquidity provided
        add_liquidity(&exchange_instance, call_params, tx_params, 1000, 10).await;
    }
}
