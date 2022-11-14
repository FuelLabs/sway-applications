use crate::utils::{
    abi_calls::{balance, deposit, withdraw},
    test_helpers::{setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn withdraws_entire_deposit_of_asset_a() {
        let (exchange, wallet, _amounts, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount = 100;
        let withdraw_amount = deposit_amount;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(deposit_amount), Some(exchange.asset_a), None),
        )
        .await;

        let initial_contract_balance = balance(&exchange.instance, exchange.asset_a).await.value;
        let initial_wallet_balance = wallet.get_asset_balance(&exchange.asset_a).await.unwrap();

        withdraw(&exchange.instance, deposit_amount, exchange.asset_a).await;

        let contract_balance = balance(&exchange.instance, exchange.asset_a).await.value;
        let wallet_balance = wallet.get_asset_balance(&exchange.asset_a).await.unwrap();

        assert_eq!(
            contract_balance,
            initial_contract_balance - withdraw_amount
        );
        assert_eq!(
            wallet_balance,
            initial_wallet_balance + withdraw_amount
        );
    }

    #[tokio::test]
    async fn withdraws_asset_a_partially() {
        let (exchange, wallet, _amounts, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount = 100;
        let withdraw_amount = 50;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(deposit_amount), Some(exchange.asset_a), None),
        )
        .await;

        let initial_contract_balance = balance(&exchange.instance, exchange.asset_a).await.value;
        let initial_wallet_balance = wallet.get_asset_balance(&exchange.asset_a).await.unwrap();

        withdraw(&exchange.instance, withdraw_amount, exchange.asset_a).await;

        let contract_balance = balance(&exchange.instance, exchange.asset_a).await.value;
        let wallet_balance = wallet.get_asset_balance(&exchange.asset_a).await.unwrap();

        assert_eq!(
            contract_balance,
            initial_contract_balance - withdraw_amount
        );
        assert_eq!(
            wallet_balance,
            initial_wallet_balance + withdraw_amount
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;

        withdraw(&exchange_instance, 0, asset_a_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_invalid_asset() {
        let (exchange, _wallet, _amounts, asset_c_id) = setup_and_initialize().await;
        let deposit_amount = 100;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(deposit_amount), Some(exchange.asset_a), None),
        )
        .await;

        // sending invalid asset
        withdraw(&exchange.instance, 0, asset_c_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_withdraw_more_than_deposited() {
        let (exchange, _wallet, _amounts, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount = 100;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(deposit_amount), Some(exchange.asset_a), None),
        )
        .await;

        // attempting to withdraw more than deposit amount
        withdraw(&exchange.instance, deposit_amount + 1, exchange.asset_a).await;
    }
}
