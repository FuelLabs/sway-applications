use crate::utils::{
    abi_calls::{add_liquidity, balance, pool_info},
    test_helpers::{
        deposit_and_add_liquidity, deposit_but_do_not_add_liquidity, setup, setup_and_initialize,
    },
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn adds_when_liquidity_is_zero() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let asset_a_asset_id = AssetId::new(*exchange.asset_a_id);
        let asset_b_asset_id = AssetId::new(*exchange.asset_b_id);
        let lp_asset_id = AssetId::new(*exchange.liquidity_pool_id);
        let deposit_amount_a = 100;
        let deposit_amount_b = 200;

        let initial_pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(initial_pool_info.asset_a_reserve, 0);
        assert_eq!(initial_pool_info.asset_b_reserve, 0);
        assert_eq!(initial_pool_info.liquidity, 0);

        assert_eq!(
            balance(&exchange.contract, exchange.asset_a_id).await.value,
            0
        );
        assert_eq!(
            balance(&exchange.contract, exchange.asset_b_id).await.value,
            0
        );

        let wallet_initial_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_initial_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_initial_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();

        deposit_but_do_not_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            deposit_amount_a,
            asset_b_asset_id,
            deposit_amount_b,
        )
        .await;
        assert_eq!(
            balance(&exchange.contract, exchange.asset_a_id).await.value,
            deposit_amount_a
        );
        assert_eq!(
            balance(&exchange.contract, exchange.asset_b_id).await.value,
            deposit_amount_b
        );

        let added_liquidity = add_liquidity(
            &exchange.contract,
            CallParameters::new(Some(0), Some(asset_b_asset_id), Some(100_000_000)),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            2,
        )
        .await
        .value;
        assert_eq!(added_liquidity, deposit_amount_a);

        let final_pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(final_pool_info.asset_a_reserve, deposit_amount_a);
        assert_eq!(final_pool_info.asset_b_reserve, deposit_amount_b);
        assert_eq!(final_pool_info.liquidity, added_liquidity);

        assert_eq!(
            balance(&exchange.contract, exchange.asset_a_id).await.value,
            0
        );
        assert_eq!(
            balance(&exchange.contract, exchange.asset_b_id).await.value,
            0
        );

        let wallet_final_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_final_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_final_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();
        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a - deposit_amount_a
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b - deposit_amount_b
        );
        assert_eq!(
            wallet_final_balance_lp,
            wallet_initial_balance_lp + added_liquidity
        );
    }

    #[tokio::test]
    async fn adds_when_liquidity_is_not_zero() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let asset_a_asset_id = AssetId::new(*exchange.asset_a_id);
        let asset_b_asset_id = AssetId::new(*exchange.asset_b_id);
        let lp_asset_id = AssetId::new(*exchange.liquidity_pool_id);
        let initial_deposit_amount_a = 100;
        let initial_deposit_amount_b = 200;
        let deadline = 1000;
        let min_liquidity = 2;

        let first_added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            initial_deposit_amount_a,
            asset_b_asset_id,
            initial_deposit_amount_b,
            deadline,
            min_liquidity,
        )
        .await;

        let initial_pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(initial_pool_info.asset_a_reserve, initial_deposit_amount_a);
        assert_eq!(initial_pool_info.asset_b_reserve, initial_deposit_amount_b);
        assert_eq!(initial_pool_info.liquidity, first_added_liquidity);

        assert_eq!(
            balance(&exchange.contract, exchange.asset_a_id).await.value,
            0
        );
        assert_eq!(
            balance(&exchange.contract, exchange.asset_b_id).await.value,
            0
        );

        let wallet_initial_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_initial_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_initial_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();

        let later_deposit_amount_a = 200;
        let later_deposit_amount_b = 400;

        let later_added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            later_deposit_amount_a,
            asset_b_asset_id,
            later_deposit_amount_b,
            deadline,
            min_liquidity,
        )
        .await;

        assert_eq!(later_added_liquidity, later_deposit_amount_a);

        let final_pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve + later_deposit_amount_a
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve + later_deposit_amount_b
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_pool_info.liquidity + later_added_liquidity
        );

        assert_eq!(
            balance(&exchange.contract, exchange.asset_a_id).await.value,
            0
        );
        assert_eq!(
            balance(&exchange.contract, exchange.asset_b_id).await.value,
            0
        );

        let wallet_final_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_final_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_final_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();
        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a - later_deposit_amount_a
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b - later_deposit_amount_b
        );
        assert_eq!(
            wallet_final_balance_lp,
            wallet_initial_balance_lp + later_added_liquidity
        );
    }

    #[tokio::test]
    async fn adds_when_liquidity_is_not_zero_and_sends_extra_deposit_back() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let asset_a_asset_id = AssetId::new(*exchange.asset_a_id);
        let asset_b_asset_id = AssetId::new(*exchange.asset_b_id);
        let lp_asset_id = AssetId::new(*exchange.liquidity_pool_id);
        let initial_deposit_amount_a = 100;
        let initial_deposit_amount_b = 200;
        let deadline = 1000;
        let min_liquidity = 2;

        let first_added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            initial_deposit_amount_a,
            asset_b_asset_id,
            initial_deposit_amount_b,
            deadline,
            min_liquidity,
        )
        .await;

        let initial_pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(initial_pool_info.asset_a_reserve, initial_deposit_amount_a);
        assert_eq!(initial_pool_info.asset_b_reserve, initial_deposit_amount_b);
        assert_eq!(initial_pool_info.liquidity, first_added_liquidity);

        assert_eq!(
            balance(&exchange.contract, exchange.asset_a_id).await.value,
            0
        );
        assert_eq!(
            balance(&exchange.contract, exchange.asset_b_id).await.value,
            0
        );

        let wallet_initial_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_initial_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_initial_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();

        let later_deposit_amount_a = 200;
        let extra_deposit_b = 100;
        let later_deposit_amount_b = 400 + extra_deposit_b;

        let later_added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            later_deposit_amount_a,
            asset_b_asset_id,
            later_deposit_amount_b,
            deadline,
            min_liquidity,
        )
        .await;

        assert_eq!(later_added_liquidity, later_deposit_amount_a);

        let final_pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve + later_deposit_amount_a
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve + later_deposit_amount_b - extra_deposit_b
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_pool_info.liquidity + later_added_liquidity
        );

        assert_eq!(
            balance(&exchange.contract, exchange.asset_a_id).await.value,
            0
        );
        assert_eq!(
            balance(&exchange.contract, exchange.asset_b_id).await.value,
            0
        );

        let wallet_final_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_final_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_final_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();
        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a - later_deposit_amount_a
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b - later_deposit_amount_b + extra_deposit_b
        );
        assert_eq!(
            wallet_final_balance_lp,
            wallet_initial_balance_lp + later_added_liquidity
        );
    }

    #[tokio::test]
    async fn returns_current_balances_without_adding_liquidity_when_deposit_is_insufficient() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let asset_a_asset_id = AssetId::new(*exchange.asset_a_id);
        let asset_b_asset_id = AssetId::new(*exchange.asset_b_id);
        let lp_asset_id = AssetId::new(*exchange.liquidity_pool_id);
        let initial_deposit_amount_a = 100;
        let initial_deposit_amount_b = 200;
        let deadline = 1000;
        let min_liquidity = 2;

        let first_added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            initial_deposit_amount_a,
            asset_b_asset_id,
            initial_deposit_amount_b,
            deadline,
            min_liquidity,
        )
        .await;

        let initial_pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(initial_pool_info.asset_a_reserve, initial_deposit_amount_a);
        assert_eq!(initial_pool_info.asset_b_reserve, initial_deposit_amount_b);
        assert_eq!(initial_pool_info.liquidity, first_added_liquidity);

        assert_eq!(
            balance(&exchange.contract, exchange.asset_a_id).await.value,
            0
        );
        assert_eq!(
            balance(&exchange.contract, exchange.asset_b_id).await.value,
            0
        );

        let wallet_initial_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_initial_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_initial_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();

        let later_deposit_amount_a = 200;
        let missing_deposit_b = 100;
        let later_deposit_amount_b = 400 - missing_deposit_b;

        let later_added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            later_deposit_amount_a,
            asset_b_asset_id,
            later_deposit_amount_b,
            deadline,
            min_liquidity,
        )
        .await;

        assert_eq!(later_added_liquidity, 0);

        let final_pool_info = pool_info(&exchange.contract).await.value;
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_pool_info.asset_a_reserve
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_pool_info.asset_b_reserve
        );
        assert_eq!(final_pool_info.liquidity, initial_pool_info.liquidity);

        assert_eq!(
            balance(&exchange.contract, exchange.asset_a_id).await.value,
            0
        );
        assert_eq!(
            balance(&exchange.contract, exchange.asset_b_id).await.value,
            0
        );

        let wallet_final_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_final_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_final_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();
        assert_eq!(wallet_final_balance_a, wallet_initial_balance_a);
        assert_eq!(wallet_final_balance_b, wallet_initial_balance_b);
        assert_eq!(wallet_final_balance_lp, wallet_initial_balance_lp);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;
        add_liquidity(
            &exchange_instance,
            CallParameters::new(Some(0), Some(AssetId::new(*asset_a_id)), Some(100_000_000)),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            1000,
            2,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deadline_has_passed() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 200;
        // deadline too low
        let deadline = 0;
        let min_liquidity = 2;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            deadline,
            min_liquidity,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_amount_is_not_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 200;
        let deadline = 1000;
        let min_liquidity = 2;

        deposit_but_do_not_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
        )
        .await;
        add_liquidity(
            &exchange.contract,
            CallParameters::new(
                // msg_amount not zero
                Some(1),
                Some(AssetId::new(*exchange.asset_b_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            deadline,
            min_liquidity,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_asset_id_is_invalid() {
        let (exchange, _wallet, asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 200;
        let deadline = 1000;
        let min_liquidity = 2;

        deposit_but_do_not_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
        )
        .await;
        add_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(0),
                // asset not in exchange contract pool
                Some(AssetId::new(*asset_c_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            deadline,
            min_liquidity,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_asset_a_in_deposit_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        // depositing zero
        let deposit_amount_a = 0;
        let deposit_amount_b = 200;
        let deadline = 1000;
        let min_liquidity = 2;

        deposit_but_do_not_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
        )
        .await;
        add_liquidity(
            &exchange.contract,
            CallParameters::new(
                Some(0),
                Some(AssetId::new(*exchange.asset_b_id)),
                Some(100_000_000),
            ),
            TxParameters {
                gas_price: 0,
                gas_limit: 100_000_000,
                maturity: 0,
            },
            deadline,
            min_liquidity,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_liquidity_is_not_zero_and_min_liquidity_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 200;
        let deadline = 1000;
        let mut min_liquidity = 2;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            deadline,
            min_liquidity,
        )
        .await;

        // min_liquidity is zero
        min_liquidity = 0;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            deadline,
            min_liquidity,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_liquidity_is_not_zero_and_min_liquidity_is_too_high() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 200;
        let deadline = 1000;
        let mut min_liquidity = 2;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            deadline,
            min_liquidity,
        )
        .await;

        // min_liquidity is too high
        min_liquidity = 1000;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            deadline,
            min_liquidity,
        )
        .await;
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    // Currently unimplemented because minimum_liquidity is a config-time constant for the exchange contract
    async fn when_liquidity_is_not_zero_and_minimum_liquidity_is_not_satisfied() {
        let (_exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
    }
}
