use crate::utils::setup_and_construct;
use test_utils::interface::exchange::{deposit, withdraw};

mod success {
    use super::*;
    use fuels::accounts::ViewOnlyAccount;
    use test_utils::interface::{exchange::balance, Asset, WithdrawEvent};

    #[tokio::test]
    async fn withdraws_entire_deposit_of_asset_a() {
        let (exchange, wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;
        let withdraw_amount = deposit_amount;

        deposit(&exchange.instance, deposit_amount, exchange.pair.0).await;

        let initial_contract_balance = balance(&exchange.instance, exchange.pair.0).await;
        let initial_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        let response = withdraw(&exchange.instance, withdraw_amount, exchange.pair.0).await;
        let log = response.decode_logs_with_type::<WithdrawEvent>().unwrap();
        let event = log.first().unwrap();

        let final_contract_balance = balance(&exchange.instance, exchange.pair.0).await;
        let final_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        assert_eq!(
            *event,
            WithdrawEvent {
                withdrawn_asset: Asset {
                    id: exchange.pair.0,
                    amount: withdraw_amount,
                },
                remaining_balance: final_contract_balance,
            }
        );
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
        let (exchange, wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;
        let withdraw_amount = 50;

        deposit(&exchange.instance, deposit_amount, exchange.pair.0).await;

        let initial_contract_balance = balance(&exchange.instance, exchange.pair.0).await;
        let initial_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        let response = withdraw(&exchange.instance, withdraw_amount, exchange.pair.0).await;
        let log = response.decode_logs_with_type::<WithdrawEvent>().unwrap();
        let event = log.first().unwrap();

        let final_contract_balance = balance(&exchange.instance, exchange.pair.0).await;
        let final_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        assert_eq!(
            *event,
            WithdrawEvent {
                withdrawn_asset: Asset {
                    id: exchange.pair.0,
                    amount: withdraw_amount,
                },
                remaining_balance: final_contract_balance,
            }
        );
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
    use crate::utils::setup;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn on_unitialized() {
        // call setup instead of setup_and_construct
        let (exchange_instance, _wallet, assets, _deadline) = setup().await;

        withdraw(&exchange_instance, 0, assets.asset_1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn on_invalid_asset() {
        let (exchange, _wallet, _liquidity_parameters, asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;

        deposit(&exchange.instance, deposit_amount, exchange.pair.0).await;

        // passing invalid asset
        withdraw(&exchange.instance, 0, asset_c_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "DesiredAmountTooHigh")]
    async fn on_withdraw_more_than_deposited() {
        let (exchange, _wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;

        deposit(&exchange.instance, deposit_amount, exchange.pair.0).await;

        // attempting to withdraw more than deposit amount
        withdraw(&exchange.instance, deposit_amount + 1, exchange.pair.0).await;
    }
}
