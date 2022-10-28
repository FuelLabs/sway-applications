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
        let deposit_amount_b = 400;
        let expected_liquidity = 200;
        let deadline = 1000;

        let initial_pool_info = pool_info(&exchange.contract).await.value;
        let wallet_initial_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_initial_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_initial_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();
        let balance_a_in_contract_before_deposit =
            balance(&exchange.contract, exchange.asset_a_id).await.value;
        let balance_b_in_contract_before_deposit =
            balance(&exchange.contract, exchange.asset_b_id).await.value;

        deposit_but_do_not_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            deposit_amount_a,
            asset_b_asset_id,
            deposit_amount_b,
        )
        .await;

        let balance_a_in_contract_after_deposit =
            balance(&exchange.contract, exchange.asset_a_id).await.value;
        let balance_b_in_contract_after_deposit =
            balance(&exchange.contract, exchange.asset_b_id).await.value;

        let added_liquidity = add_liquidity(
            &exchange.contract,
            CallParameters::default(),
            TxParameters::default(),
            expected_liquidity,
            deadline,
        )
        .await
        .value;

        let final_pool_info = pool_info(&exchange.contract).await.value;
        let balance_a_in_contract_after_adding_liquidity =
            balance(&exchange.contract, exchange.asset_a_id).await.value;
        let balance_b_in_contract_after_adding_liquidity =
            balance(&exchange.contract, exchange.asset_b_id).await.value;

        let wallet_final_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_final_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_final_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();

        assert_eq!(initial_pool_info.asset_a_reserve, 0);
        assert_eq!(initial_pool_info.asset_b_reserve, 0);
        assert_eq!(initial_pool_info.liquidity, 0);
        assert_eq!(balance_a_in_contract_before_deposit, 0);
        assert_eq!(balance_b_in_contract_before_deposit, 0);
        assert_eq!(balance_a_in_contract_after_deposit, deposit_amount_a);
        assert_eq!(balance_b_in_contract_after_deposit, deposit_amount_b);
        assert_eq!(added_liquidity, expected_liquidity);
        assert_eq!(final_pool_info.asset_a_reserve, deposit_amount_a);
        assert_eq!(final_pool_info.asset_b_reserve, deposit_amount_b);
        assert_eq!(final_pool_info.liquidity, added_liquidity);
        assert_eq!(balance_a_in_contract_after_adding_liquidity, 0);
        assert_eq!(balance_b_in_contract_after_adding_liquidity, 0);
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
    async fn adds_when_liquidity_exists_based_on_a_and_refunds() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let asset_a_asset_id = AssetId::new(*exchange.asset_a_id);
        let asset_b_asset_id = AssetId::new(*exchange.asset_b_id);
        let lp_asset_id = AssetId::new(*exchange.liquidity_pool_id);
        let initial_deposit_amount_a = 100;
        let initial_deposit_amount_b = 400;
        let initial_liquidity = 200;
        let deadline = 1000;
        let deposit_amount_a = initial_deposit_amount_a;
        let deposit_amount_b = initial_deposit_amount_b * 2;
        let expected_liquidity = initial_liquidity;

        let initial_pool_info = pool_info(&exchange.contract).await.value;
        let balance_a_in_contract_before_deposit =
            balance(&exchange.contract, exchange.asset_a_id).await.value;
        let balance_b_in_contract_before_deposit =
            balance(&exchange.contract, exchange.asset_b_id).await.value;
        let wallet_initial_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_initial_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_initial_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();

        deposit_and_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            initial_deposit_amount_a,
            asset_b_asset_id,
            initial_deposit_amount_b,
            initial_liquidity,
            deadline,
        )
        .await;

        let balance_a_in_contract_after_adding_liquidity_for_the_first_time =
            balance(&exchange.contract, exchange.asset_a_id).await.value;
        let balance_b_in_contract_after_adding_liquidity_for_the_first_time =
            balance(&exchange.contract, exchange.asset_b_id).await.value;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            deposit_amount_a,
            asset_b_asset_id,
            deposit_amount_b,
            expected_liquidity,
            deadline,
        )
        .await;

        let balance_a_in_contract_after_adding_liquidity_for_the_second_time =
            balance(&exchange.contract, exchange.asset_a_id).await.value;
        let balance_b_in_contract_after_adding_liquidity_for_the_second_time =
            balance(&exchange.contract, exchange.asset_b_id).await.value;
        let final_pool_info = pool_info(&exchange.contract).await.value;
        let wallet_final_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_final_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_final_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();

        assert_eq!(initial_pool_info.asset_a_reserve, 0);
        assert_eq!(initial_pool_info.asset_b_reserve, 0);
        assert_eq!(initial_pool_info.liquidity, 0);
        assert_eq!(balance_a_in_contract_before_deposit, 0);
        assert_eq!(balance_b_in_contract_before_deposit, 0);
        assert_eq!(
            balance_a_in_contract_after_adding_liquidity_for_the_first_time,
            0
        );
        assert_eq!(
            balance_b_in_contract_after_adding_liquidity_for_the_first_time,
            0
        );
        assert_eq!(added_liquidity, expected_liquidity);
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_deposit_amount_a + deposit_amount_a
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_deposit_amount_b + (deposit_amount_b / 2)
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_liquidity + added_liquidity
        );
        assert_eq!(
            balance_a_in_contract_after_adding_liquidity_for_the_second_time,
            0
        );
        assert_eq!(
            balance_b_in_contract_after_adding_liquidity_for_the_second_time,
            0
        );
        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a - (initial_deposit_amount_a + deposit_amount_a)
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b - (initial_deposit_amount_b + (deposit_amount_b / 2))
        );
        assert_eq!(
            wallet_final_balance_lp,
            wallet_initial_balance_lp + (initial_liquidity + added_liquidity)
        );
    }

    #[tokio::test]
    async fn adds_when_liquidity_exists_based_on_b_and_refunds() {
        let (exchange, wallet, _asset_c_id) = setup_and_initialize().await;
        let asset_a_asset_id = AssetId::new(*exchange.asset_a_id);
        let asset_b_asset_id = AssetId::new(*exchange.asset_b_id);
        let lp_asset_id = AssetId::new(*exchange.liquidity_pool_id);
        let initial_deposit_amount_a = 100;
        let initial_deposit_amount_b = 400;
        let initial_liquidity = 200;
        let deadline = 1000;
        let deposit_amount_a = initial_deposit_amount_a * 2;
        let deposit_amount_b = initial_deposit_amount_b;
        let expected_liquidity = initial_liquidity;

        let initial_pool_info = pool_info(&exchange.contract).await.value;
        let balance_a_in_contract_before_deposit =
            balance(&exchange.contract, exchange.asset_a_id).await.value;
        let balance_b_in_contract_before_deposit =
            balance(&exchange.contract, exchange.asset_b_id).await.value;
        let wallet_initial_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_initial_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_initial_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();

        deposit_and_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            initial_deposit_amount_a,
            asset_b_asset_id,
            initial_deposit_amount_b,
            initial_liquidity,
            deadline,
        )
        .await;

        let balance_a_in_contract_after_adding_liquidity_for_the_first_time =
            balance(&exchange.contract, exchange.asset_a_id).await.value;
        let balance_b_in_contract_after_adding_liquidity_for_the_first_time =
            balance(&exchange.contract, exchange.asset_b_id).await.value;

        let added_liquidity = deposit_and_add_liquidity(
            &exchange.contract,
            asset_a_asset_id,
            deposit_amount_a,
            asset_b_asset_id,
            deposit_amount_b,
            expected_liquidity,
            deadline,
        )
        .await;

        let balance_a_in_contract_after_adding_liquidity_for_the_second_time =
            balance(&exchange.contract, exchange.asset_a_id).await.value;
        let balance_b_in_contract_after_adding_liquidity_for_the_second_time =
            balance(&exchange.contract, exchange.asset_b_id).await.value;
        let final_pool_info = pool_info(&exchange.contract).await.value;
        let wallet_final_balance_a = wallet.get_asset_balance(&asset_a_asset_id).await.unwrap();
        let wallet_final_balance_b = wallet.get_asset_balance(&asset_b_asset_id).await.unwrap();
        let wallet_final_balance_lp = wallet.get_asset_balance(&lp_asset_id).await.unwrap();

        assert_eq!(initial_pool_info.asset_a_reserve, 0);
        assert_eq!(initial_pool_info.asset_b_reserve, 0);
        assert_eq!(initial_pool_info.liquidity, 0);
        assert_eq!(balance_a_in_contract_before_deposit, 0);
        assert_eq!(balance_b_in_contract_before_deposit, 0);
        assert_eq!(
            balance_a_in_contract_after_adding_liquidity_for_the_first_time,
            0
        );
        assert_eq!(
            balance_b_in_contract_after_adding_liquidity_for_the_first_time,
            0
        );
        assert_eq!(added_liquidity, expected_liquidity);
        assert_eq!(
            final_pool_info.asset_a_reserve,
            initial_deposit_amount_a + (deposit_amount_a / 2)
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            initial_deposit_amount_b + deposit_amount_b
        );
        assert_eq!(
            final_pool_info.liquidity,
            initial_liquidity + added_liquidity
        );
        assert_eq!(
            balance_a_in_contract_after_adding_liquidity_for_the_second_time,
            0
        );
        assert_eq!(
            balance_b_in_contract_after_adding_liquidity_for_the_second_time,
            0
        );
        assert_eq!(
            wallet_final_balance_a,
            wallet_initial_balance_a - (initial_deposit_amount_a + (deposit_amount_a / 2))
        );
        assert_eq!(
            wallet_final_balance_b,
            wallet_initial_balance_b - (initial_deposit_amount_b + deposit_amount_b)
        );
        assert_eq!(
            wallet_final_balance_lp,
            wallet_initial_balance_lp + (initial_liquidity + added_liquidity)
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_unitialized() {
        // call setup instead of setup_and_initialize
        let (exchange_instance, _wallet, _pool_asset_id, _asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;
        let min_liquidity = 200;
        let deadline = 1000;

        add_liquidity(
            &exchange_instance,
            CallParameters::default(),
            TxParameters::default(),
            min_liquidity,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deadline_has_passed() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        // deadline too low
        let deadline = 0;
        let min_liquidity = 200;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            min_liquidity,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_amount_is_not_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let deadline = 1000;
        let min_liquidity = 200;

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
                None,
                None,
            ),
            TxParameters::default(),
            min_liquidity,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_desired_liquidity_is_less_than_minimum_liquidity() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let deadline = 1000;
        // min_liquidity is less than minimum_liquidity
        let min_liquidity = 0;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            min_liquidity,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deposited_a_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        // depositing 0 amount of asset A
        let deposit_amount_a = 0;
        let deposit_amount_b = 400;
        let deadline = 1000;
        let min_liquidity = 0;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            min_liquidity,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deposited_b_is_zero() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        // depositing 0 amount of asset B
        let deposit_amount_b = 0;
        let deadline = 1000;
        let min_liquidity = 0;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            min_liquidity,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_liquidity_is_zero_but_desired_liquidity_is_too_high() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let deposit_amount_a = 100;
        let deposit_amount_b = 400;
        let deadline = 1000;
        // min_liquidity is more than 200
        let min_liquidity = 300;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            min_liquidity,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_liquidity_exists_but_desired_liquidity_is_too_high_based_on_a() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let initial_deposit_amount_a = 100;
        let initial_deposit_amount_b = 400;
        let deadline = 1000;
        let initial_liquidity = 200;
        let deposit_amount_a = initial_deposit_amount_a;
        let deposit_amount_b = initial_deposit_amount_b * 2;
        // min_liquidity is more than 200
        let min_liquidity = initial_liquidity * 2;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            initial_deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            initial_deposit_amount_b,
            initial_liquidity,
            deadline,
        )
        .await;
        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            min_liquidity,
            deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_liquidity_exists_but_desired_liquidity_is_too_high_based_on_b() {
        let (exchange, _wallet, _asset_c_id) = setup_and_initialize().await;
        let initial_deposit_amount_a = 100;
        let initial_deposit_amount_b = 400;
        let deadline = 1000;
        let initial_liquidity = 200;
        let deposit_amount_a = initial_deposit_amount_a * 2;
        let deposit_amount_b = initial_deposit_amount_b;
        // min_liquidity is more than 200
        let min_liquidity = initial_liquidity * 2;

        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            initial_deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            initial_deposit_amount_b,
            initial_liquidity,
            deadline,
        )
        .await;
        deposit_and_add_liquidity(
            &exchange.contract,
            AssetId::new(*exchange.asset_a_id),
            deposit_amount_a,
            AssetId::new(*exchange.asset_b_id),
            deposit_amount_b,
            min_liquidity,
            deadline,
        )
        .await;
    }
}
