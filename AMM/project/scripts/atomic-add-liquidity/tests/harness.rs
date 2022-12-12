use fuels::prelude::*;
use test_utils::{
    abi::{
        exchange::{deposit, preview_add_liquidity, withdraw},
        AtomicAddLiquidityScript,
    },
    data_structures::{
        ExchangeContract, ExchangeContractConfiguration, LiquidityParameters,
        TransactionParameters, WalletAssetConfiguration,
    },
    paths::ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH,
    setup::{
        common::{
            deploy_and_construct_exchange, deposit_and_add_liquidity, setup_wallet_and_provider,
        },
        scripts::transaction_inputs_outputs,
    },
};

async fn expected_liquidity(
    exchange: &ExchangeContract,
    liquidity_parameters: &LiquidityParameters,
) -> u64 {
    deposit(
        &exchange.instance,
        CallParameters::new(
            Some(liquidity_parameters.amounts.0),
            Some(exchange.pair.0),
            None,
        ),
    )
    .await;

    let preview_add_liquidity_info = preview_add_liquidity(
        &exchange.instance,
        CallParameters::default(),
        TxParameters::new(None, Some(100_000_000), None),
        liquidity_parameters.amounts.1,
        exchange.pair.1,
    )
    .await
    .value;

    withdraw(
        &exchange.instance,
        liquidity_parameters.amounts.0,
        exchange.pair.0,
    )
    .await;

    preview_add_liquidity_info.liquidity_asset_amount_to_receive
}

async fn setup(
    liquidity_parameters: &LiquidityParameters,
) -> (WalletUnlocked, ExchangeContract, TransactionParameters) {
    let (wallet, asset_ids, provider) =
        setup_wallet_and_provider(&WalletAssetConfiguration::default()).await;

    let exchange = deploy_and_construct_exchange(
        &wallet,
        &ExchangeContractConfiguration::new(Some((asset_ids[0], asset_ids[1])), None, None, None),
    )
    .await;

    let transaction_parameters = transaction_inputs_outputs(
        &wallet,
        &provider,
        &vec![exchange.id],
        &vec![*asset_ids.get(0).unwrap(), *asset_ids.get(1).unwrap()],
        Some(&vec![
            liquidity_parameters.amounts.0,
            liquidity_parameters.amounts.1,
        ]),
    )
    .await;

    (wallet, exchange, transaction_parameters)
}

mod success {
    use super::*;

    #[tokio::test]
    async fn adds_initial_liquidity_with_equal_deposit_amounts() {
        let liquidity_parameters =
            LiquidityParameters::new(Some((1000, 1000)), Some(1000), Some(1000));

        let (wallet, exchange, transaction_parameters) = setup(&liquidity_parameters).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(100_000_000), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_initial_liquidity_to_make_a_more_valuable() {
        let liquidity_parameters =
            LiquidityParameters::new(Some((1000, 2000)), Some(1000), Some(1000));

        let (wallet, exchange, transaction_parameters) = setup(&liquidity_parameters).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(100_000_000), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_initial_liquidity_to_make_b_more_valuable() {
        let liquidity_parameters =
            LiquidityParameters::new(Some((2000, 1000)), Some(1000), Some(1000));

        let (wallet, exchange, transaction_parameters) = setup(&liquidity_parameters).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(100_000_000), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_further_liquidity_without_extra_deposit_when_a_is_more_valuable() {
        let initial_liquidity_parameters =
            LiquidityParameters::new(Some((1000, 4000)), Some(1000), Some(2000));

        let liquidity_parameters = LiquidityParameters::new(Some((50, 200)), Some(1000), Some(100));

        let (wallet, exchange, transaction_parameters) = setup(&liquidity_parameters).await;
        deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(100_000_000), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_further_liquidity_with_extra_a_deposit_when_a_is_more_valuable() {
        let initial_liquidity_parameters =
            LiquidityParameters::new(Some((1000, 4000)), Some(1000), Some(2000));

        let liquidity_parameters =
            LiquidityParameters::new(Some((200, 200)), Some(1000), Some(100));

        let (wallet, exchange, transaction_parameters) = setup(&liquidity_parameters).await;
        deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(100_000_000), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_further_liquidity_without_extra_deposit_when_b_is_more_valuable() {
        let initial_liquidity_parameters =
            LiquidityParameters::new(Some((4000, 1000)), Some(1000), Some(2000));

        let liquidity_parameters = LiquidityParameters::new(Some((400, 50)), Some(1000), Some(100));

        let (wallet, exchange, transaction_parameters) = setup(&liquidity_parameters).await;
        deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(100_000_000), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }

    #[tokio::test]
    async fn adds_further_liquidity_with_extra_a_deposit_when_b_is_more_valuable() {
        let initial_liquidity_parameters =
            LiquidityParameters::new(Some((4000, 1000)), Some(1000), Some(2000));

        let liquidity_parameters = LiquidityParameters::new(Some((200, 50)), Some(1000), Some(100));

        let (wallet, exchange, transaction_parameters) = setup(&liquidity_parameters).await;
        deposit_and_add_liquidity(&initial_liquidity_parameters, &exchange).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let expected_liquidity = expected_liquidity(&exchange, &liquidity_parameters).await;

        let liquidity = script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                expected_liquidity,
            )
            .with_inputs(transaction_parameters.inputs)
            .with_outputs(transaction_parameters.outputs)
            .tx_params(TxParameters::new(None, Some(100_000_000), None))
            .call()
            .await
            .unwrap()
            .value;

        assert_eq!(expected_liquidity, liquidity);
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "DesiredLiquidityZero")]
    async fn when_desired_liquidity_zero() {
        let liquidity_parameters =
            LiquidityParameters::new(Some((1000, 1000)), Some(1000), Some(1000));

        let (wallet, exchange, _transaction_parameters) = setup(&liquidity_parameters).await;

        let script_instance =
            AtomicAddLiquidityScript::new(wallet, ATOMIC_ADD_LIQUIDITY_SCRIPT_BINARY_PATH);

        let desired_liquidity = 0;

        script_instance
            .main(
                exchange.id,
                (
                    ContractId::new(*exchange.pair.0),
                    ContractId::new(*exchange.pair.1),
                ),
                (
                    liquidity_parameters.amounts.0,
                    liquidity_parameters.amounts.1,
                ),
                desired_liquidity,
            )
            .call()
            .await
            .unwrap();
    }
}
