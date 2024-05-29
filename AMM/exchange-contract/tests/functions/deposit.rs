use crate::utils::setup_and_construct;
use test_utils::interface::exchange::deposit;

mod success {
    use super::*;
    use fuels::accounts::ViewOnlyAccount;
    use test_utils::interface::{exchange::balance, Asset, DepositEvent};

    #[tokio::test]
    async fn deposits() {
        let (exchange, wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;

        let initial_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();
        let initial_contract_balance = balance(&exchange.instance, exchange.pair.0).await;

        let response = deposit(&exchange.instance, deposit_amount, exchange.pair.0).await;
        let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
        let event = log.first().unwrap();

        let final_contract_balance = balance(&exchange.instance, exchange.pair.0).await;
        let final_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        assert_eq!(
            *event,
            DepositEvent {
                deposited_asset: Asset {
                    id: exchange.pair.0,
                    amount: deposit_amount,
                },
                new_balance: final_contract_balance,
            }
        );
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
        let initial_contract_balance = balance(&exchange.instance, exchange.pair.0).await;

        deposit(&exchange.instance, first_deposit_amount, exchange.pair.0).await;

        let contract_balance_after_deposit = balance(&exchange.instance, exchange.pair.0).await;

        let response = deposit(&exchange.instance, second_deposit_amount, exchange.pair.0).await;
        let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
        let event = log.first().unwrap();

        let final_contract_balance = balance(&exchange.instance, exchange.pair.0).await;
        let final_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        assert_eq!(
            *event,
            DepositEvent {
                deposited_asset: Asset {
                    id: exchange.pair.0,
                    amount: second_deposit_amount,
                },
                new_balance: final_contract_balance,
            }
        );
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
    use crate::utils::setup;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_construct
        let (exchange_instance, _wallet, assets, _deadline) = setup().await;
        let deposit_amount = 100;

        deposit(&exchange_instance, deposit_amount, assets.asset_1).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, _liquidity_parameters, asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;

        // send invalid asset id
        deposit(&exchange.instance, deposit_amount, asset_c_id).await;
    }
}
