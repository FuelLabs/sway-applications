use crate::utils::{contract_balances, setup, setup_and_construct, wallet_balances};
use fuels::prelude::*;
use test_utils::{
    abi::exchange::{add_liquidity, pool_info},
    data_structures::LiquidityParameters,
    setup::common::{deposit_and_add_liquidity, deposit_both_assets},
};

mod success {
    use super::*;

    #[tokio::test]
    async fn adds_when_liquidity_is_zero() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;
        let initial_contract_balances = contract_balances(&exchange).await;

        deposit_both_assets(&liquidity_parameters, &exchange).await;

        let contract_balances_after_deposit = contract_balances(&exchange).await;

        let added_liquidity = add_liquidity(
            &exchange.instance,
            CallParameters::default(),
            TxParameters::default(),
            liquidity_parameters.liquidity,
            liquidity_parameters.deadline,
        )
        .await
        .value;

        let pool_info_after_adding_liquidity = pool_info(&exchange.instance).await.value;
        let wallet_balances_after_adding_liquidity = wallet_balances(&exchange, &wallet).await;
        let contract_balances_after_adding_liquidity = contract_balances(&exchange).await;

        assert_eq!(initial_pool_info.asset_a_reserve, 0);
        assert_eq!(initial_pool_info.asset_b_reserve, 0);
        assert_eq!(initial_pool_info.liquidity, 0);
        assert_eq!(initial_contract_balances.asset_a, 0);
        assert_eq!(initial_contract_balances.asset_b, 0);
        assert_eq!(
            contract_balances_after_deposit.asset_a,
            liquidity_parameters.amounts.0
        );
        assert_eq!(
            contract_balances_after_deposit.asset_b,
            liquidity_parameters.amounts.1
        );
        assert_eq!(added_liquidity, liquidity_parameters.liquidity);
        assert_eq!(
            pool_info_after_adding_liquidity.asset_a_reserve,
            liquidity_parameters.amounts.0
        );
        assert_eq!(
            pool_info_after_adding_liquidity.asset_b_reserve,
            liquidity_parameters.amounts.1
        );
        assert_eq!(pool_info_after_adding_liquidity.liquidity, added_liquidity);
        assert_eq!(contract_balances_after_adding_liquidity.asset_a, 0);
        assert_eq!(contract_balances_after_adding_liquidity.asset_b, 0);
        assert_eq!(
            wallet_balances_after_adding_liquidity.asset_a,
            initial_wallet_balances.asset_a - liquidity_parameters.amounts.0
        );
        assert_eq!(
            wallet_balances_after_adding_liquidity.asset_b,
            initial_wallet_balances.asset_b - liquidity_parameters.amounts.1
        );
        assert_eq!(
            wallet_balances_after_adding_liquidity.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset + added_liquidity
        );
    }

    #[tokio::test]
    async fn adds_when_liquidity_exists_based_on_a_and_refunds() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let second_liquidity_amounts = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.0,
                liquidity_parameters.amounts.1 * 2,
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity),
        );

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;
        let initial_contract_balances = contract_balances(&exchange).await;

        deposit_and_add_liquidity(&liquidity_parameters, &exchange).await;

        let contract_balances_after_adding_liquidity_for_the_first_time =
            contract_balances(&exchange).await;

        let added_liquidity = deposit_and_add_liquidity(&second_liquidity_amounts, &exchange).await;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_contract_balances = contract_balances(&exchange).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

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
            final_pool_info.asset_a_reserve,
            liquidity_parameters.amounts.0 + second_liquidity_amounts.amounts.0
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            liquidity_parameters.amounts.1 + (second_liquidity_amounts.amounts.1 / 2)
        );
        assert_eq!(
            final_pool_info.liquidity,
            liquidity_parameters.liquidity + added_liquidity
        );
        assert_eq!(final_contract_balances.asset_a, 0);
        assert_eq!(final_contract_balances.asset_b, 0);
        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a
                - (liquidity_parameters.amounts.0 + second_liquidity_amounts.amounts.0)
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b
                - (liquidity_parameters.amounts.1 + (second_liquidity_amounts.amounts.1 / 2))
        );
        assert_eq!(
            final_wallet_balances.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset
                + (liquidity_parameters.liquidity + second_liquidity_amounts.liquidity)
        );
    }

    #[tokio::test]
    async fn adds_when_liquidity_exists_based_on_b_and_refunds() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let second_liquidity_amounts = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.0 * 2,
                liquidity_parameters.amounts.1,
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity),
        );

        let initial_pool_info = pool_info(&exchange.instance).await.value;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;
        let initial_contract_balances = contract_balances(&exchange).await;

        deposit_and_add_liquidity(&liquidity_parameters, &exchange).await;

        let contract_balances_after_adding_liquidity_for_the_first_time =
            contract_balances(&exchange).await;

        let added_liquidity = deposit_and_add_liquidity(&second_liquidity_amounts, &exchange).await;

        let final_pool_info = pool_info(&exchange.instance).await.value;
        let final_contract_balances = contract_balances(&exchange).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

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
            final_pool_info.asset_a_reserve,
            liquidity_parameters.amounts.0 + (second_liquidity_amounts.amounts.0 / 2)
        );
        assert_eq!(
            final_pool_info.asset_b_reserve,
            liquidity_parameters.amounts.1 + second_liquidity_amounts.amounts.1
        );
        assert_eq!(
            final_pool_info.liquidity,
            liquidity_parameters.liquidity + added_liquidity
        );
        assert_eq!(final_contract_balances.asset_a, 0);
        assert_eq!(final_contract_balances.asset_b, 0);
        assert_eq!(
            final_wallet_balances.asset_a,
            initial_wallet_balances.asset_a
                - (liquidity_parameters.amounts.0 + (second_liquidity_amounts.amounts.0 / 2))
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b
                - (liquidity_parameters.amounts.1 + second_liquidity_amounts.amounts.1)
        );
        assert_eq!(
            final_wallet_balances.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset
                + (liquidity_parameters.liquidity + second_liquidity_amounts.liquidity)
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "AssetPairNotSet")]
    async fn when_uninitialized() {
        // call setup instead of setup_and_construct
        let (exchange_instance, _wallet, _pool_asset_id, _asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;
        let min_liquidity = 20000;
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
    #[should_panic(expected = "DeadlinePassed")]
    async fn when_deadline_passed() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let override_liquidity_parameters = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.0,
                liquidity_parameters.amounts.1,
            )),
            Some(0), // deadline too low
            Some(liquidity_parameters.liquidity),
        );

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ExpectedZeroAmount")]
    async fn when_msg_amount_is_not_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, false).await;

        add_liquidity(
            &exchange.instance,
            CallParameters::new(
                // msg_amount not zero
                Some(1),
                None,
                None,
            ),
            TxParameters::default(),
            liquidity_parameters.liquidity,
            liquidity_parameters.deadline,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "CannotAddLessThanMinimumLiquidity")]
    async fn when_desired_liquidity_is_less_than_minimum_liquidity() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let override_liquidity_parameters = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.0,
                liquidity_parameters.amounts.1,
            )),
            Some(liquidity_parameters.deadline),
            Some(0), // expected_liquidity is less than MINIMUM_LIQUIDITY
        );

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ExpectedNonZeroDeposit")]
    async fn when_deposited_a_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let override_liquidity_parameters = LiquidityParameters::new(
            Some((
                0, // depositing 0 amount of asset A
                liquidity_parameters.amounts.1,
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity),
        );

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "ExpectedNonZeroDeposit")]
    async fn when_deposited_b_is_zero() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let override_liquidity_parameters = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.0,
                0, // depositing 0 amount of asset B
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity),
        );

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "DesiredAmountTooHigh")]
    async fn when_liquidity_is_zero_but_desired_liquidity_is_too_high() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let override_liquidity_parameters = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.0,
                liquidity_parameters.amounts.1,
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity + 100), // expected_liquidity is more than what can be provided with this setup
        );

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "DesiredAmountTooHigh")]
    async fn when_liquidity_exists_but_desired_liquidity_is_too_high_based_on_a() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let override_liquidity_parameters = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.0,
                liquidity_parameters.amounts.1 * 2, // setting this so that liquidity is calculated based on asset A amount
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity * 2), // expected_liquidity is more than what can be provided with this setup
        );

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange).await;
    }

    #[tokio::test]
    #[should_panic(expected = "DesiredAmountTooHigh")]
    async fn when_liquidity_exists_but_desired_liquidity_is_too_high_based_on_b() {
        let (exchange, _wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(true, true).await;

        let override_liquidity_parameters = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.0 * 2, // setting this so that liquidity is calculated based on asset B amount
                liquidity_parameters.amounts.1,
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity * 2), // expected_liquidity is more than what can be provided with this setup
        );

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange).await;
    }
}
