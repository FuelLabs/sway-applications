use crate::utils::{setup, setup_and_construct};
use fuels::prelude::*;
use test_utils::abi::exchange::{balance, deposit};

mod success {
    use super::*;

    #[tokio::test]
    async fn deposits() {
        let (exchange, wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;

        let initial_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();
        let initial_contract_balance = balance(&exchange.instance, exchange.pair.0).await.value;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(deposit_amount), Some(exchange.pair.0), None),
        )
        .await;

        let final_contract_balance = balance(&exchange.instance, exchange.pair.0).await.value;
        let final_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        assert_eq!(initial_contract_balance, 0);
        assert_eq!(final_contract_balance, deposit_amount);
        assert_eq!(
            final_wallet_balance,
            initial_wallet_balance - deposit_amount
        );
    }

    #[tokio::test]
    async fn deposits_more_than_once() {
        let (exchange, wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let first_deposit_amount = 100;
        let second_deposit_amount = 200;

        let initial_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();
        let initial_contract_balance = balance(&exchange.instance, exchange.pair.0).await.value;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(first_deposit_amount), Some(exchange.pair.0), None),
        )
        .await;

        let contract_balance_after_deposit =
            balance(&exchange.instance, exchange.pair.0).await.value;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(second_deposit_amount), Some(exchange.pair.0), None),
        )
        .await;

        let final_contract_balance = balance(&exchange.instance, exchange.pair.0).await.value;
        let final_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        assert_eq!(initial_contract_balance, 0);
        assert_eq!(contract_balance_after_deposit, first_deposit_amount);
        assert_eq!(
            final_contract_balance,
            contract_balance_after_deposit + second_deposit_amount
        );
        assert_eq!(
            final_wallet_balance,
            initial_wallet_balance - first_deposit_amount - second_deposit_amount
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_construct
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;
        let deposit_amount = 100;

        deposit(
            &exchange_instance,
            CallParameters::new(Some(deposit_amount), Some(AssetId::new(*asset_a_id)), None),
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, _liquidity_parameters, asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;

        // send invalid asset id
        deposit(
            &exchange.instance,
            CallParameters::new(Some(deposit_amount), Some(AssetId::new(*asset_c_id)), None),
        )
        .await;
    }
}
