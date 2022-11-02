use crate::utils::{
    abi_calls::{balance, deposit, withdraw},
    test_helpers::{setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn withdraws_entire_deposit_of_asset_a() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount = 100;
        let withdraw_amount = deposit_amount;

        deposit(
            &exchange.contract,
            CallParameters::new(Some(deposit_amount), Some(exchange.asset_a_asset_id), None),
        )
        .await;

        let initial_contract_balance = balance(&exchange.contract, exchange.asset_a_contract_id)
            .await
            .value;
        let initial_wallet_balance = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();

        withdraw(
            &exchange.contract,
            deposit_amount,
            exchange.asset_a_contract_id,
        )
        .await;

        let final_contract_balance = balance(&exchange.contract, exchange.asset_a_contract_id)
            .await
            .value;
        let final_wallet_balance = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();

        assert_eq!(
            final_contract_balance,
            initial_contract_balance - withdraw_amount
        );
        assert_eq!(
            final_wallet_balance,
            initial_wallet_balance + withdraw_amount
        );
    }

    #[tokio::test]
    async fn withdraws_asset_a_partially() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount = 100;
        let withdraw_amount = 50;

        deposit(
            &exchange.contract,
            CallParameters::new(Some(deposit_amount), Some(exchange.asset_a_asset_id), None),
        )
        .await;

        let initial_contract_balance = balance(&exchange.contract, exchange.asset_a_contract_id)
            .await
            .value;
        let initial_wallet_balance = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();

        withdraw(
            &exchange.contract,
            withdraw_amount,
            exchange.asset_a_contract_id,
        )
        .await;

        let final_contract_balance = balance(&exchange.contract, exchange.asset_a_contract_id)
            .await
            .value;
        let final_wallet_balance = wallet
            .get_asset_balance(&exchange.asset_a_asset_id)
            .await
            .unwrap();

        assert_eq!(
            final_contract_balance,
            initial_contract_balance - withdraw_amount
        );
        assert_eq!(
            final_wallet_balance,
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
        let (exchange, _wallet, asset_c_id) = setup_and_initialize().await;
        let deposit_amount = 100;

        deposit(
            &exchange.contract,
            CallParameters::new(Some(deposit_amount), Some(exchange.asset_a_asset_id), None),
        )
        .await;

        // sending invalid asset
        withdraw(&exchange.contract, 0, asset_c_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn on_withdraw_more_than_deposited() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount = 100;

        deposit(
            &exchange.contract,
            CallParameters::new(Some(deposit_amount), Some(exchange.asset_a_asset_id), None),
        )
        .await;

        // attempting to withdraw more than deposit amount
        withdraw(
            &exchange.contract,
            deposit_amount + 1,
            exchange.asset_a_contract_id,
        )
        .await;
    }
}
