use crate::utils::{
    abi_calls::{balance, deposit, withdraw},
    test_helpers::{setup, setup_and_initialize},
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn withdraws_entire_deposit() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount = 100;
        let asset = exchange.asset_a_id;

        deposit(
            &exchange.contract,
            CallParameters::new(Some(deposit_amount), Some(AssetId::new(*asset)), None),
        )
        .await;

        let initial_contract_balance = balance(&exchange.contract, asset).await.value;
        let initial_wallet_balance = wallet
            .get_asset_balance(&AssetId::new(*asset))
            .await
            .unwrap();

        let withdraw_amount = deposit_amount;
        withdraw(&exchange.contract, deposit_amount, asset).await;

        let final_contract_balance = balance(&exchange.contract, asset).await.value;
        let final_wallet_balance = wallet
            .get_asset_balance(&AssetId::new(*asset))
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
    async fn withdraws_partially() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount = 100;
        let asset = exchange.asset_a_id;

        deposit(
            &exchange.contract,
            CallParameters::new(
                Some(deposit_amount),
                Some(AssetId::new(*exchange.asset_a_id)),
                None,
            ),
        )
        .await;

        let initial_contract_balance = balance(&exchange.contract, asset).await.value;
        let initial_wallet_balance = wallet
            .get_asset_balance(&AssetId::new(*asset))
            .await
            .unwrap();

        let withdraw_amount = 50;
        withdraw(&exchange.contract, withdraw_amount, exchange.asset_a_id).await;

        let final_contract_balance = balance(&exchange.contract, asset).await.value;
        let final_wallet_balance = wallet
            .get_asset_balance(&AssetId::new(*asset))
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
            CallParameters::new(
                Some(deposit_amount),
                Some(AssetId::new(*exchange.asset_a_id)),
                None,
            ),
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
            CallParameters::new(
                Some(deposit_amount),
                Some(AssetId::new(*exchange.asset_a_id)),
                None,
            ),
        )
        .await;
        // attempting to withdraw more than deposit amount
        withdraw(&exchange.contract, deposit_amount + 1, exchange.asset_a_id).await;
    }
}
