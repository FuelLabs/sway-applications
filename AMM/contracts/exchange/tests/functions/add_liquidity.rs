use crate::utils::{
    abi_calls::{add_liquidity, pool_info},
    test_helpers::{
        contract_balances, deposit_and_add_liquidity, deposit_but_do_not_add_liquidity, setup,
        setup_and_initialize, setup_initialize_and_deposit_but_do_not_add_liquidity,
        setup_initialize_deposit_and_add_liquidity, wallet_balances,
    },
    MetaAmounts,
};
use fuels::prelude::*;

mod success {
    use super::*;

    #[tokio::test]
    async fn adds_when_liquidity_is_zero() {
        let (exchange, wallet, amounts, _asset_c_id) = setup_and_initialize().await;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;
        let initial_contract_balances = contract_balances(&exchange).await;

        deposit_but_do_not_add_liquidity(&amounts, &exchange).await;

        let contract_balances_after_deposit = contract_balances(&exchange).await;

        let added_liquidity = add_liquidity(
            &exchange.instance,
            CallParameters::default(),
            TxParameters::default(),
            amounts.liquidity,
            amounts.deadline,
        )
        .await
        .value;

        let pool_info = pool_info(&exchange.instance).await.value;
        let wallet_balances = wallet_balances(&exchange, &wallet).await;
        let contract_balances = contract_balances(&exchange).await;

        assert_eq!(initial_pool_info.asset_a_reserve, 0);
        assert_eq!(initial_pool_info.asset_b_reserve, 0);
        assert_eq!(initial_pool_info.liquidity, 0);
        assert_eq!(initial_contract_balances.asset_a, 0);
        assert_eq!(initial_contract_balances.asset_b, 0);
        assert_eq!(contract_balances_after_deposit.asset_a, amounts.amount_a);
        assert_eq!(contract_balances_after_deposit.asset_b, amounts.amount_b);
        assert_eq!(added_liquidity, amounts.liquidity);
        assert_eq!(
            pool_info.asset_a_reserve,
            amounts.amount_a
        );
        assert_eq!(
            pool_info.asset_b_reserve,
            amounts.amount_b
        );
        assert_eq!(pool_info.liquidity, added_liquidity);
        assert_eq!(contract_balances.asset_a, 0);
        assert_eq!(contract_balances.asset_b, 0);
        assert_eq!(
            wallet_balances.asset_a,
            initial_wallet_balances.asset_a - amounts.amount_a
        );
        assert_eq!(
            wallet_balances.asset_b,
            initial_wallet_balances.asset_b - amounts.amount_b
        );
        assert_eq!(
            wallet_balances.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset + added_liquidity
        );
    }

    #[tokio::test]
    async fn adds_when_liquidity_exists_based_on_a_and_refunds() {
        let (exchange, wallet, amounts, _asset_c_id) = setup_and_initialize().await;
        let second_liquidity_amounts = MetaAmounts {
            amount_a: amounts.amount_a,
            amount_b: amounts.amount_b * 2,
            deadline: amounts.deadline,
            liquidity: amounts.liquidity,
        };

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;
        let initial_contract_balances = contract_balances(&exchange).await;

        deposit_and_add_liquidity(&amounts, &exchange).await;

        let contract_balances_after_adding_liquidity_for_the_first_time =
            contract_balances(&exchange).await;

        let added_liquidity = deposit_and_add_liquidity(&second_liquidity_amounts, &exchange).await;

        let pool_info = pool_info(&exchange.instance).await.value;
        let contract_balances = contract_balances(&exchange).await;
        let wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(initial_pool_info.asset_a_reserve, 0);
        assert_eq!(initial_pool_info.asset_b_reserve, 0);
        assert_eq!(initial_pool_info.liquidity, 0);
        assert_eq!(initial_contract_balances.asset_a, 0);
        assert_eq!(initial_contract_balances.asset_b, 0);
        assert_eq!(
            contract_balances_after_adding_liquidity_for_the_first_time.asset_a,
            0
        );
        assert_eq!(
            contract_balances_after_adding_liquidity_for_the_first_time.asset_b,
            0
        );
        assert_eq!(added_liquidity, second_liquidity_amounts.liquidity);
        assert_eq!(
            pool_info.asset_a_reserve,
            amounts.amount_a + second_liquidity_amounts.amount_a
        );
        assert_eq!(
            pool_info.asset_b_reserve,
            amounts.amount_b + (second_liquidity_amounts.amount_b / 2)
        );
        assert_eq!(
            pool_info.liquidity,
            amounts.liquidity + added_liquidity
        );
        assert_eq!(contract_balances.asset_a, 0);
        assert_eq!(contract_balances.asset_b, 0);
        assert_eq!(
            wallet_balances.asset_a,
            initial_wallet_balances.asset_a
                - (amounts.amount_a + second_liquidity_amounts.amount_a)
        );
        assert_eq!(
            wallet_balances.asset_b,
            initial_wallet_balances.asset_b
                - (amounts.amount_b + (second_liquidity_amounts.amount_b / 2))
        );
        assert_eq!(
            wallet_balances.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset
                + (amounts.liquidity + second_liquidity_amounts.liquidity)
        );
    }

    #[tokio::test]
    async fn adds_when_liquidity_exists_based_on_b_and_refunds() {
        let (exchange, wallet, amounts, _asset_c_id) = setup_and_initialize().await;
        let second_liquidity_amounts = MetaAmounts {
            amount_a: amounts.amount_a * 2,
            amount_b: amounts.amount_b,
            deadline: amounts.deadline,
            liquidity: amounts.liquidity,
        };
        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;
        let initial_contract_balances = contract_balances(&exchange).await;

        deposit_and_add_liquidity(&amounts, &exchange).await;

        let contract_balances_after_adding_liquidity_for_the_first_time =
            contract_balances(&exchange).await;

        let added_liquidity = deposit_and_add_liquidity(&second_liquidity_amounts, &exchange).await;

        let pool_info = pool_info(&exchange.instance).await.value;
        let contract_balances = contract_balances(&exchange).await;
        let wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(initial_pool_info.asset_a_reserve, 0);
        assert_eq!(initial_pool_info.asset_b_reserve, 0);
        assert_eq!(initial_pool_info.liquidity, 0);
        assert_eq!(initial_contract_balances.asset_a, 0);
        assert_eq!(initial_contract_balances.asset_b, 0);
        assert_eq!(
            contract_balances_after_adding_liquidity_for_the_first_time.asset_a,
            0
        );
        assert_eq!(
            contract_balances_after_adding_liquidity_for_the_first_time.asset_b,
            0
        );
        assert_eq!(added_liquidity, second_liquidity_amounts.liquidity);
        assert_eq!(
            pool_info.asset_a_reserve,
            amounts.amount_a + (second_liquidity_amounts.amount_a / 2)
        );
        assert_eq!(
            pool_info.asset_b_reserve,
            amounts.amount_b + second_liquidity_amounts.amount_b
        );
        assert_eq!(
            pool_info.liquidity,
            amounts.liquidity + added_liquidity
        );
        assert_eq!(contract_balances.asset_a, 0);
        assert_eq!(contract_balances.asset_b, 0);
        assert_eq!(
            wallet_balances.asset_a,
            initial_wallet_balances.asset_a
                - (amounts.amount_a + (second_liquidity_amounts.amount_a / 2))
        );
        assert_eq!(
            wallet_balances.asset_b,
            initial_wallet_balances.asset_b
                - (amounts.amount_b + second_liquidity_amounts.amount_b)
        );
        assert_eq!(
            wallet_balances.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset
                + (amounts.liquidity + second_liquidity_amounts.liquidity)
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
        let (exchange, _wallet, amounts, _asset_c_id) = setup_and_initialize().await;

        let override_amounts = MetaAmounts {
            amount_a: amounts.amount_a,
            amount_b: amounts.amount_b,
            deadline: 0, // deadline too low
            liquidity: amounts.liquidity,
        };

        deposit_and_add_liquidity(&override_amounts, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_msg_amount_is_not_zero() {
        let (exchange, _wallet, amounts, _asset_c_id) =
            setup_initialize_and_deposit_but_do_not_add_liquidity().await;

        add_liquidity(
            &exchange.instance,
            CallParameters::new(
                // msg_amount not zero
                Some(1),
                None,
                None,
            ),
            TxParameters::default(),
            amounts.liquidity,
            amounts.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_desired_liquidity_is_less_than_minimum_liquidity() {
        let (exchange, _wallet, amounts, _asset_c_id) = setup_and_initialize().await;

        let override_amounts = MetaAmounts {
            amount_a: amounts.amount_a,
            amount_b: amounts.amount_b,
            deadline: amounts.deadline,
            liquidity: 0, // expected_liquidity is less than MINIMUM_LIQUIDITY
        };

        deposit_and_add_liquidity(&override_amounts, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deposited_a_is_zero() {
        let (exchange, _wallet, amounts, _asset_c_id) = setup_and_initialize().await;

        let override_amounts = MetaAmounts {
            amount_a: 0, // depositing 0 amount of asset A
            amount_b: amounts.amount_b,
            deadline: amounts.deadline,
            liquidity: amounts.liquidity,
        };

        deposit_and_add_liquidity(&override_amounts, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_deposited_b_is_zero() {
        let (exchange, _wallet, amounts, _asset_c_id) = setup_and_initialize().await;

        let override_amounts = MetaAmounts {
            amount_a: amounts.amount_a,
            amount_b: 0, // depositing 0 amount of asset B
            deadline: amounts.deadline,
            liquidity: amounts.liquidity,
        };

        deposit_and_add_liquidity(&override_amounts, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_liquidity_is_zero_but_desired_liquidity_is_too_high() {
        let (exchange, _wallet, amounts, _asset_c_id) = setup_and_initialize().await;

        let override_amounts = MetaAmounts {
            amount_a: amounts.amount_a,
            amount_b: amounts.amount_b,
            deadline: amounts.deadline,
            liquidity: 300, // expected_liquidity is more than 200
        };

        deposit_and_add_liquidity(&override_amounts, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_liquidity_exists_but_desired_liquidity_is_too_high_based_on_a() {
        let (exchange, _wallet, amounts, _asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;

        let override_amounts = MetaAmounts {
            amount_a: amounts.amount_a,
            amount_b: amounts.amount_b * 2, // setting this so that liquidity is calculated based on asset A amount
            deadline: amounts.deadline,
            liquidity: amounts.liquidity * 2, // expected_liquidity is more than 200
        };

        deposit_and_add_liquidity(&override_amounts, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Revert(42)")]
    async fn when_liquidity_exists_but_desired_liquidity_is_too_high_based_on_b() {
        let (exchange, _wallet, amounts, _asset_c_id, _added_liquidity) =
            setup_initialize_deposit_and_add_liquidity().await;

        let override_amounts = MetaAmounts {
            amount_a: amounts.amount_a * 2, // setting this so that liquidity is calculated based on asset B amount
            amount_b: amounts.amount_b,
            deadline: amounts.deadline,
            liquidity: amounts.liquidity * 2, // expected_liquidity is more than 200
        };

        deposit_and_add_liquidity(&override_amounts, &exchange).await;
    }
}
