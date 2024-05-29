use crate::utils::setup_and_construct;
use test_utils::{
    data_structures::LiquidityParameters, interface::exchange::add_liquidity,
    setup::common::deposit_and_add_liquidity,
};

mod success {
    use super::*;
    use crate::utils::{contract_balances, wallet_balances};
    use fuels::{prelude::ContractId, tx::ContractIdExt, types::Bytes32};
    use test_utils::{
        interface::{
            exchange::{deposit, pool_info},
            AddLiquidityEvent, Asset, AssetPair,
        },
        setup::common::deposit_and_add_liquidity_with_response,
    };

    #[tokio::test]
    async fn adds_when_liquidity_is_zero() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;
        let initial_contract_balances = contract_balances(&exchange).await;

        deposit(
            &exchange.instance,
            liquidity_parameters.amounts.0,
            exchange.pair.0,
        )
        .await;

        deposit(
            &exchange.instance,
            liquidity_parameters.amounts.1,
            exchange.pair.1,
        )
        .await;

        let contract_balances_after_deposit = contract_balances(&exchange).await;

        let response = add_liquidity(
            &exchange.instance,
            liquidity_parameters.liquidity,
            liquidity_parameters.deadline,
            false,
        )
        .await;
        let log = response
            .decode_logs_with_type::<AddLiquidityEvent>()
            .unwrap();
        let event = log.first().unwrap();

        let added_liquidity = response.value;

        let pool_info_after_adding_liquidity = pool_info(&exchange.instance).await;
        let wallet_balances_after_adding_liquidity = wallet_balances(&exchange, &wallet).await;
        let contract_balances_after_adding_liquidity = contract_balances(&exchange).await;

        assert_eq!(
            *event,
            AddLiquidityEvent {
                added_assets: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: liquidity_parameters.amounts.1,
                    },
                },
                liquidity: Asset {
                    id: ContractId::new(*exchange.id).asset_id(&Bytes32::zeroed()),
                    amount: liquidity_parameters.liquidity,
                },
            }
        );
        assert_eq!(initial_pool_info.reserves.a.amount, 0);
        assert_eq!(initial_pool_info.reserves.b.amount, 0);
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
            pool_info_after_adding_liquidity.reserves.a.amount,
            liquidity_parameters.amounts.0
        );
        assert_eq!(
            pool_info_after_adding_liquidity.reserves.b.amount,
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
        let second_liquidity_parameters = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.0,
                liquidity_parameters.amounts.1 * 2,
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity),
        );

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;
        let initial_contract_balances = contract_balances(&exchange).await;

        deposit_and_add_liquidity(&liquidity_parameters, &exchange, false).await;

        let contract_balances_after_adding_liquidity_for_the_first_time =
            contract_balances(&exchange).await;

        let response =
            deposit_and_add_liquidity_with_response(&second_liquidity_parameters, &exchange, true)
                .await;
        let log = response
            .decode_logs_with_type::<AddLiquidityEvent>()
            .unwrap();
        let event = log.first().unwrap();

        let added_liquidity = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_contract_balances = contract_balances(&exchange).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            AddLiquidityEvent {
                added_assets: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: second_liquidity_parameters.amounts.0,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: second_liquidity_parameters.amounts.1 / 2,
                    },
                },
                liquidity: Asset {
                    id: ContractId::new(*exchange.id).asset_id(&Bytes32::zeroed()),
                    amount: second_liquidity_parameters.liquidity,
                },
            }
        );
        assert_eq!(initial_pool_info.reserves.a.amount, 0);
        assert_eq!(initial_pool_info.reserves.b.amount, 0);
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
        assert_eq!(added_liquidity, second_liquidity_parameters.liquidity);
        assert_eq!(
            final_pool_info.reserves.a.amount,
            liquidity_parameters.amounts.0 + second_liquidity_parameters.amounts.0
        );
        assert_eq!(
            final_pool_info.reserves.b.amount,
            liquidity_parameters.amounts.1 + (second_liquidity_parameters.amounts.1 / 2)
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
                - (liquidity_parameters.amounts.0 + second_liquidity_parameters.amounts.0)
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b
                - (liquidity_parameters.amounts.1 + (second_liquidity_parameters.amounts.1 / 2))
        );
        assert_eq!(
            final_wallet_balances.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset
                + (liquidity_parameters.liquidity + second_liquidity_parameters.liquidity)
        );
    }

    #[tokio::test]
    async fn adds_when_liquidity_exists_based_on_b_and_refunds() {
        let (exchange, wallet, liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let second_liquidity_parameters = LiquidityParameters::new(
            Some((
                liquidity_parameters.amounts.0 * 2,
                liquidity_parameters.amounts.1,
            )),
            Some(liquidity_parameters.deadline),
            Some(liquidity_parameters.liquidity),
        );

        let initial_pool_info = pool_info(&exchange.instance).await;
        let initial_wallet_balances = wallet_balances(&exchange, &wallet).await;
        let initial_contract_balances = contract_balances(&exchange).await;

        deposit_and_add_liquidity(&liquidity_parameters, &exchange, false).await;

        let contract_balances_after_adding_liquidity_for_the_first_time =
            contract_balances(&exchange).await;

        let response =
            deposit_and_add_liquidity_with_response(&second_liquidity_parameters, &exchange, true)
                .await;
        let log = response
            .decode_logs_with_type::<AddLiquidityEvent>()
            .unwrap();
        let event = log.first().unwrap();

        let added_liquidity = response.value;

        let final_pool_info = pool_info(&exchange.instance).await;
        let final_contract_balances = contract_balances(&exchange).await;
        let final_wallet_balances = wallet_balances(&exchange, &wallet).await;

        assert_eq!(
            *event,
            AddLiquidityEvent {
                added_assets: AssetPair {
                    a: Asset {
                        id: exchange.pair.0,
                        amount: second_liquidity_parameters.amounts.0 / 2,
                    },
                    b: Asset {
                        id: exchange.pair.1,
                        amount: second_liquidity_parameters.amounts.1,
                    },
                },
                liquidity: Asset {
                    id: ContractId::new(*exchange.id).asset_id(&Bytes32::zeroed()),
                    amount: liquidity_parameters.liquidity,
                },
            }
        );
        assert_eq!(initial_pool_info.reserves.a.amount, 0);
        assert_eq!(initial_pool_info.reserves.b.amount, 0);
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
        assert_eq!(added_liquidity, second_liquidity_parameters.liquidity);
        assert_eq!(
            final_pool_info.reserves.a.amount,
            liquidity_parameters.amounts.0 + (second_liquidity_parameters.amounts.0 / 2)
        );
        assert_eq!(
            final_pool_info.reserves.b.amount,
            liquidity_parameters.amounts.1 + second_liquidity_parameters.amounts.1
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
                - (liquidity_parameters.amounts.0 + (second_liquidity_parameters.amounts.0 / 2))
        );
        assert_eq!(
            final_wallet_balances.asset_b,
            initial_wallet_balances.asset_b
                - (liquidity_parameters.amounts.1 + second_liquidity_parameters.amounts.1)
        );
        assert_eq!(
            final_wallet_balances.liquidity_pool_asset,
            initial_wallet_balances.liquidity_pool_asset
                + (liquidity_parameters.liquidity + second_liquidity_parameters.liquidity)
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
        let (exchange_instance, _wallet, _assets, deadline) = setup().await;
        let min_liquidity = 20000;

        add_liquidity(&exchange_instance, min_liquidity, deadline, false).await;
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

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange, false).await;
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

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange, false).await;
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

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange, false).await;
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

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange, false).await;
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

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange, false).await;
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

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange, true).await;
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

        deposit_and_add_liquidity(&override_liquidity_parameters, &exchange, true).await;
    }
}
